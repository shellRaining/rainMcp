use std::fs;

use rstest::rstest;
use tempfile::TempDir;

use crate::mcp::registry::{load_schema_store, save_schema_store, SchemaStore};
use crate::mcp::server_schema::{
    EnvironmentVariable, Icon, LocalTransport, Package, Repository, ServerSchema,
};

use super::fixtures::test_env;
use super::EnvGuard;

fn create_test_server_schema() -> ServerSchema {
    ServerSchema {
        schema: Some(
            "https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json"
                .to_string(),
        ),
        name: "io.github.test/test-server".to_string(),
        description: "A test MCP server".to_string(),
        version: "1.0.0".to_string(),
        title: Some("Test Server".to_string()),
        repository: Some(Repository {
            url: Some("https://github.com/test/test-server".to_string()),
            source: Some("github".to_string()),
            subfolder: None,
            id: None,
        }),
        website_url: None,
        icons: vec![Icon {
            src: "https://example.com/icon.png".to_string(),
            mime_type: Some("image/png".to_string()),
            sizes: vec!["32x32".to_string()],
            theme: None,
        }],
        packages: vec![Package {
            registry_type: "npm".to_string(),
            identifier: "test-server".to_string(),
            version: Some("1.0.0".to_string()),
            runtime_hint: Some("npx".to_string()),
            transport: LocalTransport::Stdio,
            environment_variables: vec![EnvironmentVariable {
                name: "API_KEY".to_string(),
                description: Some("API key for the service".to_string()),
                is_required: true,
                is_secret: true,
                default: None,
                placeholder: Some("your-api-key".to_string()),
                value: None,
                format: None,
                choices: vec![],
            }],
            package_arguments: vec![],
            runtime_arguments: vec![],
            file_sha256: None,
            registry_base_url: None,
        }],
        remotes: vec![],
    }
}

#[rstest]
fn load_schema_store_returns_empty_when_no_file(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let store = load_schema_store().unwrap();
    assert!(store.servers.is_empty());
    assert!(store.updated_at.is_none());
}

#[rstest]
fn save_and_load_schema_store(test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>)) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let store = SchemaStore {
        servers: vec![create_test_server_schema()],
        updated_at: Some("2025-01-01T00:00:00Z".to_string()),
    };

    save_schema_store(&store).unwrap();

    let loaded = load_schema_store().unwrap();
    assert_eq!(loaded.servers.len(), 1);
    assert_eq!(loaded.servers[0].name, "io.github.test/test-server");
    assert_eq!(loaded.servers[0].description, "A test MCP server");
    assert_eq!(loaded.servers[0].version, "1.0.0");
    assert_eq!(loaded.updated_at, Some("2025-01-01T00:00:00Z".to_string()));
}

#[rstest]
fn save_schema_store_creates_parent_directories(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    // Ensure the config directory doesn't exist yet
    let config_dir = temp_dir.path().join(".config").join("rain-mcp");
    assert!(!config_dir.exists());

    let store = SchemaStore { servers: vec![create_test_server_schema()], updated_at: None };

    save_schema_store(&store).unwrap();

    // Now the directory should exist
    assert!(config_dir.exists());
}

#[rstest]
fn schema_store_preserves_package_details(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let store = SchemaStore { servers: vec![create_test_server_schema()], updated_at: None };

    save_schema_store(&store).unwrap();

    let loaded = load_schema_store().unwrap();
    let server = &loaded.servers[0];
    assert_eq!(server.packages.len(), 1);

    let package = &server.packages[0];
    assert_eq!(package.registry_type, "npm");
    assert_eq!(package.identifier, "test-server");
    assert_eq!(package.runtime_hint, Some("npx".to_string()));

    let env_var = &package.environment_variables[0];
    assert_eq!(env_var.name, "API_KEY");
    assert!(env_var.is_required);
    assert!(env_var.is_secret);
}

#[rstest]
fn schema_store_handles_invalid_json(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let config_dir = temp_dir.path().join(".config").join("rain-mcp");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(config_dir.join("schema_store.json"), "invalid json").unwrap();

    let result = load_schema_store();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to parse schema store"));
}
