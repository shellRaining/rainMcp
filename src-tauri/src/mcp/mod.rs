pub mod parser;
pub mod paths;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::config::{load_app_config, save_app_config, AppConfig};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Hash)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SupportedAgent {
    pub agent_type: AgentType,
    pub name: String,
    pub config_path: PathBuf,
    pub is_configured: bool,
    pub enabled: bool,
    pub mcp_config: Option<McpConfig>,
}

/// Returns all supported agent types
fn get_all_agent_types() -> Vec<AgentType> {
    vec![
        AgentType::ClaudeCode,
        AgentType::Cursor,
        AgentType::Windsurf,
        AgentType::Cline,
        AgentType::ClaudeDesktop,
        AgentType::RooCode,
        AgentType::Trae,
        AgentType::GeminiCli,
        AgentType::Kiro,
        AgentType::OpenAiCodex,
    ]
}

#[tauri::command]
pub fn get_agent_mcp_config_command(agent_name: String) -> Result<McpConfig, String> {
    let agent = parse_agent_name(&agent_name)?;
    parser::read_agent_config(agent)
}

#[tauri::command]
pub fn get_supported_agents_command() -> Result<Vec<SupportedAgent>, String> {
    let agents = get_all_agent_types();
    let app_config = load_app_config();
    let mut supported_agents = Vec::new();

    for agent in agents {
        let path = paths::get_global_config_path(agent)
            .map_err(|e| format!("Error getting path for {:?}: {}", agent, e))?;

        let is_configured = path.exists();
        let mcp_config = if is_configured { parser::read_agent_config(agent).ok() } else { None };

        let agent_name = format!("{:?}", agent);
        let enabled = app_config.clients.get(&agent_name).map(|c| c.enabled).unwrap_or(false);

        supported_agents.push(SupportedAgent {
            agent_type: agent,
            name: agent_name,
            config_path: path,
            is_configured,
            enabled,
            mcp_config,
        });
    }

    Ok(supported_agents)
}

#[tauri::command]
pub fn get_enabled_agents_command() -> Result<Vec<AgentType>, String> {
    let app_config = load_app_config();
    let mut enabled_agents = Vec::new();

    for (agent_name, client_config) in app_config.clients {
        if client_config.enabled {
            if let Ok(agent) = parse_agent_name(&agent_name) {
                enabled_agents.push(agent);
            }
        }
    }

    Ok(enabled_agents)
}

#[tauri::command]
pub fn update_enabled_agents_command(enabled_agents: Vec<String>) -> Result<(), String> {
    let mut app_config = load_app_config();

    // Parse all agent names first to validate them
    let mut parsed_agents = Vec::new();
    for agent_name in &enabled_agents {
        let agent = parse_agent_name(agent_name)?;
        parsed_agents.push((agent_name.clone(), agent));
    }

    // Get all supported agent types
    let all_agents = get_all_agent_types();

    // Update all agents' enabled status
    for agent in all_agents {
        let agent_name = format!("{:?}", agent);
        let is_enabled = parsed_agents.iter().any(|(_, a)| *a == agent);

        app_config.clients.entry(agent_name).and_modify(|c| c.enabled = is_enabled).or_insert(
            crate::config::ClientConfigItem { enabled: is_enabled, custom_config_path: None },
        );
    }

    save_app_config(&app_config)
}

#[tauri::command]
pub fn update_agent_mcp_config_command(
    agent_name: String,
    config: McpConfig,
) -> Result<(), String> {
    let agent = parse_agent_name(&agent_name)?;
    parser::save_agent_config(agent, config)
}

#[tauri::command]
pub fn get_app_config_command() -> AppConfig {
    load_app_config()
}

#[tauri::command]
pub fn update_app_config_command(config: AppConfig) -> Result<(), String> {
    save_app_config(&config)
}

#[tauri::command]
pub fn open_config_file_command(agent_name: String) -> Result<(), String> {
    let agent = parse_agent_name(&agent_name)?;
    let path = paths::get_global_config_path(agent)?;

    if !path.exists() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        // Initialize with valid empty config
        match agent {
            AgentType::OpenAiCodex => {
                std::fs::write(&path, "").map_err(|e| e.to_string())?;
            }
            _ => {
                std::fs::write(&path, "{}").map_err(|e| e.to_string())?;
            }
        }
    }

    open::that(path).map_err(|e| e.to_string())
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
        "openai-codex" | "openai_codex" | "openaicodex" | "codex" => Ok(AgentType::OpenAiCodex),
        _ => Err(format!("Unknown agent: {}", name)),
    }
}

#[cfg(test)]
mod tests;
