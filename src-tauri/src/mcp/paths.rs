use super::AgentType;
use std::path::PathBuf;

pub fn get_global_config_path(agent: AgentType) -> Result<PathBuf, String> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| "Cannot find home directory".to_string())?;

    let path = match agent {
        AgentType::ClaudeCode => format!("{}/.claude.json", home),
        AgentType::Cursor => format!("{}/.cursor/mcp.json", home),
        AgentType::Windsurf => {
            #[cfg(target_os = "windows")]
            {
                format!("{}/.codeium/windsurf/mcp_config.json", home)
            }
            #[cfg(not(target_os = "windows"))]
            {
                format!("{}/.codeium/windsurf/mcp_config.json", home)
            }
        }
        AgentType::Cline => {
            #[cfg(target_os = "macos")]
            {
                format!("{}/Library/Application Support/Code/User/globalStorage/saoudrizwan.claude-dev/settings/cline_mcp_settings.json", home)
            }
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA")
                    .map_err(|_| "Cannot find APPDATA directory".to_string())?;
                format!("{}/Code/User/globalStorage/saoudrizwan.claude-dev/settings/cline_mcp_settings.json", appdata)
            }
            #[cfg(target_os = "linux")]
            {
                format!("{}/.config/Code/User/globalStorage/saoudrizwan.claude-dev/settings/cline_mcp_settings.json", home)
            }
        }
        AgentType::ClaudeDesktop => {
            #[cfg(target_os = "macos")]
            {
                format!("{}/Library/Application Support/Claude/claude_desktop_config.json", home)
            }
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA")
                    .map_err(|_| "Cannot find APPDATA directory".to_string())?;
                format!("{}/Claude/claude_desktop_config.json", appdata)
            }
            #[cfg(target_os = "linux")]
            {
                return Err("Claude Desktop is not available on Linux".to_string());
            }
        }
        AgentType::RooCode => {
            #[cfg(target_os = "macos")]
            {
                format!("{}/Library/Application Support/Code/User/globalStorage/rooveterinaryinc.roo-cline/settings/mcp_settings.json", home)
            }
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA")
                    .map_err(|_| "Cannot find APPDATA directory".to_string())?;
                format!("{}/Code/User/globalStorage/rooveterinaryinc.roo-cline/settings/mcp_settings.json", appdata)
            }
            #[cfg(target_os = "linux")]
            {
                format!("{}/.config/Code/User/globalStorage/rooveterinaryinc.roo-cline/settings/mcp_settings.json", home)
            }
        }
        AgentType::Trae => {
            #[cfg(target_os = "macos")]
            {
                format!("{}/Library/Application Support/Trae/User/mcp.json", home)
            }
            #[cfg(target_os = "windows")]
            {
                let appdata = std::env::var("APPDATA")
                    .map_err(|_| "Cannot find APPDATA directory".to_string())?;
                format!("{}/Trae/User/mcp.json", appdata)
            }
            #[cfg(target_os = "linux")]
            {
                format!("{}/.config/Trae/User/mcp.json", home)
            }
        }
        AgentType::GeminiCli => format!("{}/.gemini/settings.json", home),
        AgentType::Kiro => format!("{}/.kiro/settings/mcp.json", home),
        AgentType::OpenAiCodex => format!("{}/.codex/config.toml", home),
    };

    Ok(PathBuf::from(path))
}
