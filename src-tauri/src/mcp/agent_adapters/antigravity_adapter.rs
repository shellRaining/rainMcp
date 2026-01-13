use std::collections::HashMap;

use serde_json::Value;

use crate::mcp::agent::get_global_config_path;
use crate::mcp::{AgentServerEntry, AgentServers, AgentType};

use super::common::{merge_server_value, parse_server_entry, server_entry_to_value};
use super::AgentConfigAdapter;

pub struct AntigravityAdapter {
    agent_type: AgentType,
}

impl AntigravityAdapter {
    pub fn new() -> Self {
        Self { agent_type: AgentType::Antigravity }
    }

    fn read_root_value(&self) -> Result<Value, String> {
        let path = get_global_config_path(self.agent_type)?;
        if !path.exists() {
            return Err(format!("Config file not found: {:?}", path));
        }

        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        if content.trim().is_empty() {
            return Ok(Value::Object(serde_json::Map::new()));
        }

        serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON config: {}", e))
    }

    fn server_entry_to_value_antigravity(entry: &AgentServerEntry) -> Result<Value, String> {
        match entry {
            AgentServerEntry::Local(_) => server_entry_to_value(entry),
            AgentServerEntry::Remote(_) => {
                let mut value = server_entry_to_value(entry)?;
                let obj = value
                    .as_object_mut()
                    .ok_or_else(|| "Server config must be an object".to_string())?;
                if let Some(url) = obj.remove("url") {
                    obj.insert("serverUrl".to_string(), url);
                }
                Ok(value)
            }
        }
    }
}

impl Default for AntigravityAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentConfigAdapter for AntigravityAdapter {
    fn agent_type(&self) -> AgentType {
        self.agent_type
    }

    fn config_path(&self) -> Result<std::path::PathBuf, String> {
        get_global_config_path(self.agent_type)
    }

    fn read_config(&self) -> Result<AgentServers, String> {
        let root = self.read_root_value()?;
        let root_obj =
            root.as_object().ok_or_else(|| "Config root must be an object".to_string())?;

        let servers_value = root_obj.get("mcpServers");
        let servers_obj = match servers_value {
            Some(Value::Object(map)) => map,
            Some(_) => {
                return Err("mcpServers must be an object".to_string());
            }
            None => {
                return Ok(AgentServers { servers: HashMap::new() });
            }
        };

        let mut servers = HashMap::new();
        for (name, value) in servers_obj {
            let server_config = parse_server_entry(value)
                .map_err(|e| format!("Failed to parse server '{}': {}", name, e))?;
            servers.insert(name.clone(), server_config);
        }

        Ok(AgentServers { servers })
    }

    fn write_config(&self, config: AgentServers) -> Result<(), String> {
        let path = get_global_config_path(self.agent_type)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let content = if path.exists() {
            std::fs::read_to_string(&path).map_err(|e| e.to_string())?
        } else {
            String::new()
        };

        let mut root_value = if content.trim().is_empty() {
            Value::Object(serde_json::Map::new())
        } else {
            serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))?
        };

        let root_obj = root_value
            .as_object_mut()
            .ok_or_else(|| "Config root must be an object".to_string())?;

        let existing_servers =
            root_obj.get("mcpServers").and_then(|v| v.as_object()).cloned().unwrap_or_default();

        let mut new_servers = serde_json::Map::new();
        for (name, server_config) in config.servers {
            let base = Self::server_entry_to_value_antigravity(&server_config)?;
            let existing = existing_servers.get(&name);
            let merged = merge_server_value(base, serde_json::Map::new(), existing)?;
            new_servers.insert(name, merged);
        }

        root_obj.insert("mcpServers".to_string(), Value::Object(new_servers));

        let output = serde_json::to_string_pretty(&root_value)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?;
        std::fs::write(path, output).map_err(|e| e.to_string())
    }

    fn get_server_raw_config(&self, server_name: &str) -> Result<String, String> {
        let root = self.read_root_value()?;
        let root_obj =
            root.as_object().ok_or_else(|| "Config root must be an object".to_string())?;

        let server_value = root_obj
            .get("mcpServers")
            .and_then(|v| v.get(server_name))
            .ok_or_else(|| format!("Server '{}' not found", server_name))?
            .clone();

        let mut wrapper = serde_json::Map::new();
        wrapper.insert(server_name.to_string(), server_value);

        serde_json::to_string_pretty(&wrapper)
            .map_err(|e| format!("Failed to serialize server config: {}", e))
    }
}
