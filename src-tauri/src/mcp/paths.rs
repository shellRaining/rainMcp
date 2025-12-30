use super::AgentType;
use std::path::PathBuf;

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
