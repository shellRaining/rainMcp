//! Agent tools for MCP configuration

use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

use crate::mcp::server_schema::ServerSchema;

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Tool to fetch webpage content using Jina Reader API
#[derive(Deserialize, Serialize)]
pub struct FetchWebpage;

#[derive(Deserialize)]
pub struct FetchWebpageArgs {
    pub url: String,
}

impl Tool for FetchWebpage {
    const NAME: &'static str = "fetch_webpage";
    type Error = ToolError;
    type Args = FetchWebpageArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "fetch_webpage".to_string(),
            description: "Fetch and extract text content from a webpage URL using Jina Reader. Use this to read documentation, README files, or any web page to understand how to configure an MCP server.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The URL of the webpage to fetch"
                    }
                },
                "required": ["url"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        fetch_url_with_jina(&args.url).await
    }
}

/// Fetch webpage content using Jina Reader API
async fn fetch_url_with_jina(url: &str) -> Result<String, ToolError> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| ToolError::Http(e.to_string()))?;

    // Jina Reader API: prepend https://r.jina.ai/ to the URL
    let jina_url = format!("https://r.jina.ai/{}", url);

    let response = client
        .get(&jina_url)
        .header("Accept", "text/markdown")
        .send()
        .await
        .map_err(|e| ToolError::Http(format!("Failed to fetch URL via Jina: {}", e)))?;

    if !response.status().is_success() {
        return Err(ToolError::Http(format!("Jina API returned status: {}", response.status())));
    }

    let markdown = response
        .text()
        .await
        .map_err(|e| ToolError::Http(format!("Failed to read Jina response: {}", e)))?;

    // Truncate to avoid token limit
    let truncated = if markdown.len() > 15000 {
        format!("{}...\n\n[Content truncated due to length]", &markdown[..15000])
    } else {
        markdown
    };

    Ok(truncated)
}

/// Tool to generate MCP ServerSchema
#[derive(Deserialize, Serialize)]
pub struct GenerateServerSchema;

impl Tool for GenerateServerSchema {
    const NAME: &'static str = "generate_server_schema";
    type Error = ToolError;
    type Args = ServerSchema;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "generate_server_schema".to_string(),
            description: "Generate a complete MCP ServerSchema conforming to the official schema specification. Call this when you have gathered enough information from documentation to create a complete schema with packages or remotes arrays.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Server name in reverse-DNS format (e.g., 'io.github.username.server-name')"
                    },
                    "description": {
                        "type": "string",
                        "description": "Human-readable description of server functionality"
                    },
                    "version": {
                        "type": "string",
                        "description": "Version string following semantic versioning (e.g., '1.0.0')"
                    },
                    "title": {
                        "type": "string",
                        "description": "Optional human-readable title"
                    },
                    "repository": {
                        "type": "object",
                        "description": "Repository metadata",
                        "properties": {
                            "url": { "type": "string", "description": "Repository URL" },
                            "source": { "type": "string", "description": "Hosting service (e.g., 'github')" },
                            "subfolder": { "type": "string" },
                            "id": { "type": "string" }
                        }
                    },
                    "packages": {
                        "type": "array",
                        "description": "Array of package configurations for local installation. Use this for npm, pypi, docker packages.",
                        "items": {
                            "type": "object",
                            "properties": {
                                "registryType": {
                                    "type": "string",
                                    "description": "Registry type (npm, pypi, docker, etc.)"
                                },
                                "identifier": {
                                    "type": "string",
                                    "description": "Package identifier (e.g., '@modelcontextprotocol/server-filesystem')"
                                },
                                "version": { "type": "string", "description": "Package version" },
                                "runtimeHint": {
                                    "type": "string",
                                    "description": "Runtime hint (npx, uvx, docker, python, node, etc.)"
                                },
                                "transport": {
                                    "type": "object",
                                    "description": "Transport configuration (usually {\"type\": \"stdio\"})",
                                    "properties": {
                                        "type": { "type": "string", "enum": ["stdio", "sse", "streamable-http"] }
                                    },
                                    "required": ["type"]
                                },
                                "environmentVariables": {
                                    "type": "array",
                                    "items": {
                                        "type": "object",
                                        "properties": {
                                            "name": { "type": "string" },
                                            "description": { "type": "string" },
                                            "isRequired": { "type": "boolean" },
                                            "isSecret": { "type": "boolean" },
                                            "default": { "type": "string" },
                                            "placeholder": { "type": "string" },
                                            "value": { "type": "string" },
                                            "format": { "type": "string" }
                                        },
                                        "required": ["name"]
                                    }
                                },
                                "packageArguments": {
                                    "type": "array",
                                    "description": "Arguments passed to the package",
                                    "items": {
                                        "type": "object",
                                        "properties": {
                                            "type": { "type": "string", "enum": ["positional", "named"] },
                                            "name": { "type": "string" },
                                            "value": { "type": "string" },
                                            "description": { "type": "string" }
                                        }
                                    }
                                },
                                "runtimeArguments": {
                                    "type": "array",
                                    "description": "Arguments passed to the runtime (e.g., npx -y)",
                                    "items": {
                                        "type": "object",
                                        "properties": {
                                            "type": { "type": "string", "enum": ["positional", "named"] },
                                            "name": { "type": "string" },
                                            "value": { "type": "string" },
                                            "description": { "type": "string" }
                                        }
                                    }
                                }
                            },
                            "required": ["registryType", "identifier", "transport"]
                        }
                    },
                    "remotes": {
                        "type": "array",
                        "description": "Array of remote transport configurations. Use this for hosted MCP servers.",
                        "items": {
                            "type": "object",
                            "properties": {
                                "type": {
                                    "type": "string",
                                    "enum": ["sse", "streamable-http"],
                                    "description": "Transport type"
                                },
                                "url": {
                                    "type": "string",
                                    "description": "Remote server URL"
                                },
                                "headers": {
                                    "type": "array",
                                    "description": "HTTP headers for remote connection",
                                    "items": {
                                        "type": "object",
                                        "properties": {
                                            "name": { "type": "string" },
                                            "description": { "type": "string" },
                                            "isRequired": { "type": "boolean" },
                                            "isSecret": { "type": "boolean" },
                                            "value": { "type": "string" }
                                        },
                                        "required": ["name"]
                                    }
                                }
                            },
                            "required": ["type", "url"]
                        }
                    }
                },
                "required": ["name", "description", "version"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Validate that we have at least packages or remotes
        if args.packages.is_empty() && args.remotes.is_empty() {
            return Err(ToolError::Validation(
                "Schema must contain at least one package or remote configuration".to_string(),
            ));
        }

        // Ensure $schema field is set
        let mut schema = args;
        if schema.schema.is_none() {
            schema.schema = Some(
                "https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json"
                    .to_string(),
            );
        }

        serde_json::to_string_pretty(&schema).map_err(|e| ToolError::Serialization(e.to_string()))
    }
}
