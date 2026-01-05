pub mod common;
pub mod json_adapter;
pub mod opencode_adapter;
pub mod toml_adapter;

use crate::mcp::agent::AgentType;

use json_adapter::{JsonAdapter, JsonServerExtras};
use opencode_adapter::OpenCodeAdapter;
use toml_adapter::TomlAdapter;

pub trait AgentConfigAdapter {
    fn agent_type(&self) -> AgentType;
    fn config_path(&self) -> Result<std::path::PathBuf, String>;
    fn read_config(&self) -> Result<crate::mcp::AgentServers, String>;
    fn write_config(&self, config: crate::mcp::AgentServers) -> Result<(), String>;
    fn get_server_raw_config(&self, server_name: &str) -> Result<String, String>;
}

pub fn get_adapter(agent: AgentType) -> Box<dyn AgentConfigAdapter> {
    match agent {
        AgentType::OpenAiCodex => Box::new(TomlAdapter::new(agent)),
        AgentType::OpenCode => Box::new(OpenCodeAdapter::new()),
        AgentType::VsCodeCopilot => {
            Box::new(JsonAdapter::new(agent, "servers", JsonServerExtras::VsCodeCopilot))
        }
        AgentType::CopilotCli => {
            Box::new(JsonAdapter::new(agent, "mcpServers", JsonServerExtras::CopilotCli))
        }
        AgentType::ClaudeCode => {
            Box::new(JsonAdapter::new(agent, "mcpServers", JsonServerExtras::ClaudeCode))
        }
        AgentType::Cursor
        | AgentType::Windsurf
        | AgentType::Cline
        | AgentType::ClaudeDesktop
        | AgentType::RooCode
        | AgentType::Trae
        | AgentType::GeminiCli
        | AgentType::Kiro
        | AgentType::Comate
        | AgentType::Alma => {
            Box::new(JsonAdapter::new(agent, "mcpServers", JsonServerExtras::None))
        }
    }
}
