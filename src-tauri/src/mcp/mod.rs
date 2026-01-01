//! MCP (Model Context Protocol) module
//!
//! This module handles MCP server configuration for various AI coding agents.

pub mod agent;
pub mod agent_config;
pub mod registry;
pub mod server_schema;
pub mod user_server;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::config::{load_app_config, save_app_config, AppConfig};

// Re-export commonly used types
pub use agent::{get_all_agent_types, parse_agent_name, AgentType, SupportedAgent};

// ============================================================================
// Agent server entry types (internal representation of agent config)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BaseServerEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LocalServerEntry {
    #[serde(flatten)]
    pub base: BaseServerEntry,

    pub command: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RemoteServerEntry {
    #[serde(flatten)]
    pub base: BaseServerEntry,

    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AgentServerEntry {
    Local(LocalServerEntry),
    Remote(RemoteServerEntry),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AgentServers {
    pub servers: HashMap<String, AgentServerEntry>,
}

// ============================================================================
// Tauri commands
// ============================================================================

#[tauri::command]
pub fn get_agent_mcp_config_command(agent_name: String) -> Result<AgentServers, String> {
    let agent = parse_agent_name(&agent_name)?;
    agent_config::read_agent_config(agent)
}

#[tauri::command]
pub fn get_server_raw_config_command(
    agent_name: String,
    server_name: String,
) -> Result<String, String> {
    let agent = parse_agent_name(&agent_name)?;
    agent_config::get_server_raw_config(agent, &server_name)
}

#[tauri::command]
pub fn get_supported_agents_command() -> Result<Vec<SupportedAgent>, String> {
    let agents = get_all_agent_types();
    let app_config = load_app_config();
    let mut supported_agents = Vec::new();

    for agent in agents {
        let path = agent::get_global_config_path(agent)
            .map_err(|e| format!("Error getting path for {:?}: {}", agent, e))?;

        let is_configured = path.exists();
        let mcp_config =
            if is_configured { agent_config::read_agent_config(agent).ok() } else { None };

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
    config: AgentServers,
) -> Result<(), String> {
    let agent = parse_agent_name(&agent_name)?;
    agent_config::save_agent_config(agent, config)
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
    let path = agent::get_global_config_path(agent)?;

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

#[tauri::command]
pub async fn refresh_schema_store_command() -> Result<registry::SchemaStore, String> {
    registry::refresh_schema_store_impl().await
}

#[tauri::command]
pub fn get_schema_store_command() -> Result<registry::SchemaStore, String> {
    registry::load_schema_store()
}

#[tauri::command]
pub fn get_user_servers_command() -> Result<Vec<user_server::UserServer>, String> {
    let app_config = load_app_config();
    Ok(app_config.user_servers)
}

#[tauri::command]
pub fn add_user_server_command(
    server: user_server::UserServer,
) -> Result<user_server::UserServer, String> {
    let mut app_config = load_app_config();

    // Check for duplicate ID
    if app_config.user_servers.iter().any(|s| s.id == server.id) {
        return Err(format!("Server with ID '{}' already exists", server.id));
    }

    // Add created_at timestamp if not provided
    let server = if server.created_at.is_none() {
        user_server::UserServer { created_at: Some(chrono::Utc::now().to_rfc3339()), ..server }
    } else {
        server
    };

    app_config.user_servers.push(server.clone());
    save_app_config(&app_config)?;

    Ok(server)
}

#[tauri::command]
pub fn update_user_server_command(
    server: user_server::UserServer,
) -> Result<user_server::UserServer, String> {
    let mut app_config = load_app_config();

    let index = app_config
        .user_servers
        .iter()
        .position(|s| s.id == server.id)
        .ok_or_else(|| format!("Server with ID '{}' not found", server.id))?;

    app_config.user_servers[index] = server.clone();
    save_app_config(&app_config)?;

    Ok(server)
}

#[tauri::command]
pub fn delete_user_server_command(server_id: String) -> Result<(), String> {
    let mut app_config = load_app_config();

    let index = app_config
        .user_servers
        .iter()
        .position(|s| s.id == server_id)
        .ok_or_else(|| format!("Server with ID '{}' not found", server_id))?;

    app_config.user_servers.remove(index);
    save_app_config(&app_config)?;

    Ok(())
}

#[tauri::command]
pub fn add_server_to_agent_command(
    agent_name: String,
    server_id: String,
    server_name: Option<String>,
) -> Result<(), String> {
    let agent = parse_agent_name(&agent_name)?;
    let app_config = load_app_config();

    let server = app_config
        .user_servers
        .iter()
        .find(|s| s.id == server_id)
        .ok_or_else(|| format!("User server with ID '{}' not found", server_id))?;

    agent_config::add_server_to_agent(agent, server, server_name)
}

#[cfg(test)]
mod tests;
