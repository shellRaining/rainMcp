use serde_json::Value;

use crate::mcp::agent::get_global_config_path;
use crate::mcp::{AgentServers, AgentType};

use super::common::{merge_server_value, parse_server_entry, server_entry_to_value};
use super::AgentConfigAdapter;

pub struct TomlAdapter {
    agent_type: AgentType,
}

impl TomlAdapter {
    pub fn new(agent_type: AgentType) -> Self {
        Self { agent_type }
    }

    fn read_root_table(&self) -> Result<toml::Table, String> {
        let path = get_global_config_path(self.agent_type)?;
        if !path.exists() {
            return Err(format!("Config file not found: {:?}", path));
        }

        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        if content.trim().is_empty() {
            return Ok(toml::Table::new());
        }

        toml::from_str(&content).map_err(|e| format!("Failed to parse TOML config: {}", e))
    }
}

impl AgentConfigAdapter for TomlAdapter {
    fn agent_type(&self) -> AgentType {
        self.agent_type
    }

    fn config_path(&self) -> Result<std::path::PathBuf, String> {
        get_global_config_path(self.agent_type)
    }

    fn read_config(&self) -> Result<AgentServers, String> {
        let root = self.read_root_table()?;
        let servers_table = root.get("mcp_servers").and_then(|v| v.as_table());

        let mut servers = std::collections::HashMap::new();
        if let Some(table) = servers_table {
            for (name, value) in table {
                let json_value = serde_json::to_value(value)
                    .map_err(|e| format!("Failed to convert TOML to JSON: {}", e))?;
                let server_config = parse_server_entry(&json_value)
                    .map_err(|e| format!("Failed to parse server '{}': {}", name, e))?;
                servers.insert(name.clone(), server_config);
            }
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

        let mut root: toml::Table = if content.trim().is_empty() {
            toml::Table::new()
        } else {
            toml::from_str(&content).map_err(|e| format!("Failed to parse TOML: {}", e))?
        };

        let existing_servers =
            root.get("mcp_servers").and_then(|v| v.as_table()).cloned().unwrap_or_default();

        let mut new_servers = toml::Table::new();
        for (name, server_config) in config.servers {
            let base_json = server_entry_to_value(&server_config)?;
            let existing_json: Option<Value> =
                existing_servers.get(&name).and_then(|value| serde_json::to_value(value).ok());
            let merged_json =
                merge_server_value(base_json, serde_json::Map::new(), existing_json.as_ref())?;
            let merged_toml = toml::Value::try_from(merged_json)
                .map_err(|e| format!("Failed to convert server config to TOML: {}", e))?;
            new_servers.insert(name, merged_toml);
        }

        root.insert("mcp_servers".to_string(), toml::Value::Table(new_servers));

        let output = toml::to_string_pretty(&root)
            .map_err(|e| format!("Failed to serialize TOML: {}", e))?;
        std::fs::write(path, output).map_err(|e| e.to_string())
    }

    fn get_server_raw_config(&self, server_name: &str) -> Result<String, String> {
        let root = self.read_root_table()?;
        let server_value = root
            .get("mcp_servers")
            .and_then(|v| v.get(server_name))
            .ok_or_else(|| format!("Server '{}' not found", server_name))?
            .clone();

        let mut wrapper = toml::Table::new();
        wrapper.insert(server_name.to_string(), server_value);

        toml::to_string_pretty(&wrapper)
            .map_err(|e| format!("Failed to serialize server config: {}", e))
    }
}
