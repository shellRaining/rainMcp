pub mod parser;
pub mod paths;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BaseMcpConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LocalMcpConfig {
    #[serde(flatten)]
    pub base: BaseMcpConfig,

    pub command: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RemoteMcpConfig {
    #[serde(flatten)]
    pub base: BaseMcpConfig,

    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum McpServerConfig {
    Local(LocalMcpConfig),
    Remote(RemoteMcpConfig),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct McpConfig {
    pub servers: HashMap<String, McpServerConfig>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentType {
    ClaudeCode,
    Cursor,
    Windsurf,
    Cline,
    ClaudeDesktop,
    RooCode,
    Trae,
    GeminiCli,
    Kiro,
    OpenAiCodex,
}

#[tauri::command]
pub fn get_mcp_servers(agent_name: String) -> Result<McpConfig, String> {
    let agent = parse_agent_name(&agent_name)?;
    parser::read_agent_config(agent)
}

fn parse_agent_name(name: &str) -> Result<AgentType, String> {
    match name.to_lowercase().as_str() {
        "claude-code" | "claude_code" | "claudecode" => Ok(AgentType::ClaudeCode),
        "cursor" => Ok(AgentType::Cursor),
        "windsurf" => Ok(AgentType::Windsurf),
        "cline" => Ok(AgentType::Cline),
        "claude-desktop" | "claude_desktop" | "claudedesktop" => Ok(AgentType::ClaudeDesktop),
        "roo-code" | "roo_code" | "roocode" => Ok(AgentType::RooCode),
        "trae" => Ok(AgentType::Trae),
        "gemini-cli" | "gemini_cli" | "geminicli" => Ok(AgentType::GeminiCli),
        "kiro" => Ok(AgentType::Kiro),
        "openai-codex" | "openai_codex" | "codex" => Ok(AgentType::OpenAiCodex),
        _ => Err(format!("Unknown agent: {}", name)),
    }
}
