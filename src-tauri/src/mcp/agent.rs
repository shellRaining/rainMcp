//! Agent types and utilities
//!
//! This module defines supported AI coding agents and their configuration paths.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::AgentServers;

/// Supported AI coding agent types
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

/// Information about a supported agent
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SupportedAgent {
    pub agent_type: AgentType,
    pub name: String,
    pub config_path: PathBuf,
    pub is_configured: bool,
    pub enabled: bool,
    pub mcp_config: Option<AgentServers>,
}

/// Returns all supported agent types
pub fn get_all_agent_types() -> Vec<AgentType> {
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

/// Parse agent name string to AgentType
pub fn parse_agent_name(name: &str) -> Result<AgentType, String> {
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

/// Get the global configuration file path for an agent
pub fn get_global_config_path(agent: AgentType) -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("Cannot find home directory")?;
    let config_dir = dirs::config_dir().ok_or("Cannot find config directory")?;

    let path = match agent {
        AgentType::ClaudeCode => home.join(".claude.json"),
        AgentType::Cursor => home.join(".cursor/mcp.json"),
        AgentType::Windsurf => {
            // Windsurf uses .codeium/windsurf/mcp_config.json on all platforms in HOME
            home.join(".codeium/windsurf/mcp_config.json")
        }
        AgentType::Cline => {
            // macOS: ~/Library/Application Support/Code/User/...
            // Windows: %APPDATA%\Code\User\...
            // Linux: ~/.config/Code/User/...
            // dirs::config_dir() handles the prefix for all these cases.
            config_dir.join(
                "Code/User/globalStorage/saoudrizwan.claude-dev/settings/cline_mcp_settings.json",
            )
        }
        AgentType::ClaudeDesktop => {
            // macOS: ~/Library/Application Support/Claude/...
            // Windows: %APPDATA%\Claude\...
            // Linux: Not officially supported, but XDG would be ~/.config/Claude/...
            config_dir.join("Claude/claude_desktop_config.json")
        }
        AgentType::RooCode => {
            // Same as VSCode/Cline structure
            config_dir.join(
                "Code/User/globalStorage/rooveterinaryinc.roo-cline/settings/mcp_settings.json",
            )
        }
        AgentType::Trae => {
            // macOS: ~/Library/Application Support/Trae/User/...
            // Windows: %APPDATA%\Trae\User\...
            // Linux: ~/.config/Trae/User/...
            config_dir.join("Trae/User/mcp.json")
        }
        AgentType::GeminiCli => home.join(".gemini/settings.json"),
        AgentType::Kiro => home.join(".kiro/settings/mcp.json"),
        AgentType::OpenAiCodex => home.join(".codex/config.toml"),
    };

    Ok(path)
}
