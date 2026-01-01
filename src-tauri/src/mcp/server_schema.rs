//! Official MCP Registry server.schema.json types
//!
//! These structures correspond to the official MCP Registry schema:
//! https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json
//!
//! Note: Some types use relaxed definitions (e.g., Argument as struct instead of enum)
//! to handle edge cases where API returns non-conforming data.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Main server schema structure from MCP Registry
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ServerSchema {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    /// Server name in reverse-DNS format (e.g., "io.github.user/weather")
    pub name: String,

    /// Human-readable description of server functionality
    pub description: String,

    /// Version string (should follow semantic versioning)
    pub version: String,

    /// Optional human-readable title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Repository metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,

    /// Optional website URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,

    /// Icons for UI display
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub icons: Vec<Icon>,

    /// Package configurations for local installation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<Package>,

    /// Remote transport configurations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remotes: Vec<RemoteTransport>,
}

/// Repository metadata
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct Repository {
    /// Repository URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Repository hosting service (e.g., "github")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Optional subfolder path in monorepo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subfolder: Option<String>,

    /// Repository ID from hosting service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Icon for UI display
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    /// URL to icon resource
    pub src: String,

    /// MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// Available sizes (e.g., ["48x48", "96x96"])
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sizes: Vec<String>,

    /// Theme this icon is designed for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
}

/// Package configuration for local installation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    /// Registry type (npm, pypi, oci, nuget, mcpb)
    pub registry_type: String,

    /// Package identifier or URL
    pub identifier: String,

    /// Package version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Runtime hint (npx, uvx, docker, dnx)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_hint: Option<String>,

    /// Transport configuration
    pub transport: LocalTransport,

    /// Environment variables
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub environment_variables: Vec<EnvironmentVariable>,

    /// Package arguments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub package_arguments: Vec<Argument>,

    /// Runtime arguments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runtime_arguments: Vec<Argument>,

    /// SHA-256 hash for integrity verification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_sha256: Option<String>,

    /// Base URL of the package registry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_base_url: Option<String>,
}

/// Local transport configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LocalTransport {
    Stdio,
    #[serde(rename_all = "camelCase")]
    Sse {
        url: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        headers: Vec<EnvironmentVariable>,
    },
    #[serde(rename = "streamable-http", rename_all = "camelCase")]
    StreamableHttp {
        url: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        headers: Vec<EnvironmentVariable>,
    },
}

/// Remote transport configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum RemoteTransport {
    #[serde(rename_all = "camelCase")]
    Sse {
        url: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        headers: Vec<EnvironmentVariable>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        variables: Option<serde_json::Value>,
    },
    #[serde(rename = "streamable-http", rename_all = "camelCase")]
    StreamableHttp {
        url: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        headers: Vec<EnvironmentVariable>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        variables: Option<serde_json::Value>,
    },
}

/// Environment variable or header configuration (corresponds to Input/KeyValueInput in schema)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentVariable {
    /// Variable name
    pub name: String,

    /// Description of the variable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the variable is required
    #[serde(default)]
    pub is_required: bool,

    /// Whether the variable contains sensitive data
    #[serde(default)]
    pub is_secret: bool,

    /// Default value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    /// Placeholder text for UI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// Fixed value (not user-configurable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Input format (string, number, boolean, filepath)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Possible choices
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<String>,
}

/// Command-line argument (corresponds to PositionalArgument | NamedArgument in schema)
///
/// Note: Using struct instead of enum because API may return empty or unknown type values.
/// See: https://github.com/modelcontextprotocol/registry for edge cases.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Argument {
    /// Argument type: "positional", "named", or may be empty
    #[serde(rename = "type", default)]
    pub arg_type: String,

    /// Argument name (for named arguments, including leading dashes like "--port")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Argument value (fixed value, not user-configurable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Hint for what kind of value is expected (for positional arguments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_hint: Option<String>,

    /// Whether the argument can be repeated
    #[serde(default)]
    pub is_repeated: bool,

    /// Description of the argument
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the argument is required
    #[serde(default)]
    pub is_required: bool,

    /// Whether the argument contains sensitive data (e.g., password, token)
    #[serde(default)]
    pub is_secret: bool,

    /// Default value for the argument
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    /// Placeholder text for UI display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// Input format: "string", "number", "boolean", "filepath"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Possible choices for the argument value
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<String>,
}
