//! User-configured server types

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::AgentServerEntry;

/// User-configured MCP server instance
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserServer {
    /// Unique identifier for this user server
    pub id: String,

    /// Display name for the server
    pub name: String,

    /// Server configuration (local command or remote URL)
    pub config: AgentServerEntry,

    /// Origin information for UI display (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<ServerOrigin>,

    /// Timestamp of creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// Origin information for a user server (for UI display only)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ServerOrigin {
    /// Origin type
    pub origin_type: OriginType,

    /// Registry schema name (e.g., "io.jina/mcp-jina")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,

    /// Package identifier: "{registry_type}:{identifier}" (e.g., "npm:@jina-ai/mcp-server")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
}

/// Origin type for a user server
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum OriginType {
    /// Server configured from MCP Registry
    Registry,
    /// Server manually configured by user
    Custom,
}
