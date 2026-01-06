//! MCP Registry API client and schema store management

use std::collections::HashMap;
use std::path::PathBuf;

use log::{debug, error, info};
use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::mcp::server_schema::ServerSchema;

// ============================================================================
// Types
// ============================================================================

/// Schema store containing all cached server schemas
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct SchemaStore {
    /// List of server schemas
    pub servers: Vec<ServerSchema>,

    /// Timestamp of last update (ISO 8601 format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// API response from MCP Registry
#[derive(Debug, Clone, Deserialize)]
pub struct RegistryResponse {
    pub servers: Vec<ServerResponseItem>,
    pub metadata: RegistryMetadata,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerResponseItem {
    pub server: ServerSchema,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryMetadata {
    pub next_cursor: Option<String>,
    pub count: i32,
}

// ============================================================================
// Constants
// ============================================================================

const REGISTRY_API_URL: &str = "https://registry.modelcontextprotocol.io/v0/servers";
const SCHEMA_STORE_FILENAME: &str = "schema_store.json";

/// Get the path to the schema store file
pub fn get_schema_store_path() -> Result<PathBuf, String> {
    // Check XDG_CONFIG_HOME first (for testing and Linux compatibility)
    if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        let path = PathBuf::from(xdg_config);
        if path.is_absolute() {
            return Ok(path.join("rain-mcp").join(SCHEMA_STORE_FILENAME));
        }
    }

    let config_dir =
        dirs::config_dir().ok_or_else(|| "Could not determine config directory".to_string())?;
    Ok(config_dir.join("rain-mcp").join(SCHEMA_STORE_FILENAME))
}

/// Fetch all servers from MCP Registry API (handles pagination with incremental deduplication)
pub async fn fetch_registry_servers(app: &AppHandle) -> Result<Vec<ServerSchema>, String> {
    info!("Starting to fetch registry servers");
    let client = reqwest::Client::new();
    let mut server_map: HashMap<String, ServerSchema> = HashMap::new();
    let mut cursor: Option<String> = None;
    let mut page = 0;

    loop {
        page += 1;
        let mut url = REGISTRY_API_URL.to_string();
        if let Some(ref c) = cursor {
            url = format!("{}?cursor={}", url, c);
        }

        let registry_response = fetch_single_page(&client, &url, page).await?;
        let server_count = registry_response.servers.len();
        debug!("Page {} parsed successfully: {} servers", page, server_count);

        // Incremental deduplication - merge new servers immediately
        let new_servers: Vec<ServerSchema> =
            registry_response.servers.into_iter().map(|item| item.server).collect();
        merge_servers_incremental(&mut server_map, new_servers);

        let _ = app.emit(
            "refresh-registry-progress",
            serde_json::json!({
                "page": page,
                "fetched": server_count,
                "total": server_map.len()
            }),
        );

        match registry_response.metadata.next_cursor {
            Some(next) if !next.is_empty() => {
                debug!("Next cursor: {}", next);
                cursor = Some(next);
            }
            _ => {
                debug!("No more pages");
                break;
            }
        }
    }

    let unique_servers: Vec<ServerSchema> = server_map.into_values().collect();
    info!("Fetched {} unique servers from {} pages", unique_servers.len(), page);

    Ok(unique_servers)
}

/// Fetch a single page from the registry API
async fn fetch_single_page(
    client: &reqwest::Client,
    url: &str,
    page: usize,
) -> Result<RegistryResponse, String> {
    debug!("Fetching page {} from: {}", page, url);

    let response =
        client.get(url).header("Accept", "application/json").send().await.map_err(|e| {
            error!("Failed to fetch registry page {}: {}", page, e);
            format!("Failed to fetch registry: {}", e)
        })?;

    let status = response.status();
    debug!("Page {} response status: {}", page, status);

    if !status.is_success() {
        error!("Registry API returned non-success status: {}", status);
        return Err(format!("Registry API returned status: {}", status));
    }

    let text = response.text().await.map_err(|e| {
        error!("Failed to read registry response text for page {}: {}", page, e);
        format!("Failed to read registry response: {}", e)
    })?;

    debug!("Page {} response length: {} bytes", page, text.len());

    parse_registry_response(&text, page)
}

/// Parse registry response with detailed error handling
fn parse_registry_response(text: &str, page: usize) -> Result<RegistryResponse, String> {
    serde_json::from_str(text).map_err(|e| {
        error!(
            "Failed to parse registry response for page {}: {} at line {}, column {}",
            page,
            e,
            e.line(),
            e.column()
        );
        // Log a snippet around the error
        let error_pos = e.column();
        if error_pos < text.len() {
            let start = error_pos.saturating_sub(100);
            let end = (error_pos + 100).min(text.len());
            error!("Error context: ...{}...", &text[start..end]);
        }
        format!(
            "Failed to parse registry response: {} at line {}, column {}",
            e,
            e.line(),
            e.column()
        )
    })
}

/// Incrementally merge new servers into the map, keeping latest versions
fn merge_servers_incremental(
    server_map: &mut HashMap<String, ServerSchema>,
    new_servers: Vec<ServerSchema>,
) {
    for server in new_servers {
        let name = server.name.clone();
        if let Some(existing) = server_map.get(&name) {
            if is_version_newer(&server.version, &existing.version) {
                server_map.insert(name, server);
            }
        } else {
            server_map.insert(name, server);
        }
    }
}

/// Check if version1 is newer than version2 using proper semver parsing
/// Falls back to string comparison if semver parsing fails
fn is_version_newer(v1: &str, v2: &str) -> bool {
    match (Version::parse(v1), Version::parse(v2)) {
        (Ok(ver1), Ok(ver2)) => ver1 > ver2,
        _ => {
            // Fallback to simple string-based comparison for non-semver versions
            debug!("Failed to parse versions as semver: '{}' vs '{}', using fallback", v1, v2);
            fallback_version_compare(v1, v2)
        }
    }
}

/// Fallback version comparison for non-semver strings
fn fallback_version_compare(v1: &str, v2: &str) -> bool {
    let parse_version = |v: &str| -> Vec<u32> {
        v.split(|c: char| !c.is_ascii_digit()).filter_map(|s| s.parse::<u32>().ok()).collect()
    };

    let parts1 = parse_version(v1);
    let parts2 = parse_version(v2);

    for (p1, p2) in parts1.iter().zip(parts2.iter()) {
        if p1 > p2 {
            return true;
        }
        if p1 < p2 {
            return false;
        }
    }

    // If all compared parts are equal, longer version is considered newer
    parts1.len() > parts2.len()
}

/// Save schema store to local file
pub fn save_schema_store(store: &SchemaStore) -> Result<(), String> {
    let path = get_schema_store_path()?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    let content = serde_json::to_string_pretty(store)
        .map_err(|e| format!("Failed to serialize schema store: {}", e))?;

    std::fs::write(&path, content).map_err(|e| format!("Failed to write schema store: {}", e))?;

    Ok(())
}

/// Load schema store from local file
pub fn load_schema_store() -> Result<SchemaStore, String> {
    let path = get_schema_store_path()?;

    if !path.exists() {
        return Ok(SchemaStore::default());
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read schema store: {}", e))?;

    serde_json::from_str(&content).map_err(|e| format!("Failed to parse schema store: {}", e))
}

/// Refresh schema store by fetching from registry
pub async fn refresh_schema_store_impl(app: &AppHandle) -> Result<SchemaStore, String> {
    let servers = fetch_registry_servers(app).await?;

    let store = SchemaStore { servers, updated_at: Some(chrono::Utc::now().to_rfc3339()) };

    save_schema_store(&store)?;

    Ok(store)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semver_comparison() {
        // Standard semver cases
        assert!(is_version_newer("1.2.0", "1.1.0"));
        assert!(is_version_newer("2.0.0", "1.9.9"));
        assert!(is_version_newer("1.0.1", "1.0.0"));
        assert!(!is_version_newer("1.0.0", "1.0.1"));
        assert!(!is_version_newer("1.1.0", "1.2.0"));

        // Pre-release versions
        assert!(is_version_newer("1.0.0", "1.0.0-alpha"));
        assert!(is_version_newer("1.0.0-beta", "1.0.0-alpha"));
    }

    #[test]
    fn test_fallback_comparison() {
        // Non-semver formats that should use fallback
        assert!(is_version_newer("v1.2", "v1.1"));
        assert!(is_version_newer("2024.1", "2023.12"));
        assert!(is_version_newer("1.0.0.1", "1.0.0"));
        assert!(!is_version_newer("1.0", "1.0.1"));
    }

    #[test]
    fn test_edge_cases() {
        // Same versions
        assert!(!is_version_newer("1.0.0", "1.0.0"));

        // Empty or invalid versions
        assert!(!is_version_newer("", "1.0.0"));
        assert!(is_version_newer("1.0.0", ""));

        // Mixed formats
        assert!(is_version_newer("2.0.0", "v1.9"));
    }
}
