//! OpenCode-specific adapter
//!
//! OpenCode uses a different configuration format:
//! - Config key is `mcp` instead of `mcpServers`
//! - Local server `command` is an array (not command + args)
//! - Uses `environment` instead of `env`
//! - Has explicit `type` field ("local" or "remote")
//! - Has `enabled` field

use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::mcp::agent::get_global_config_path;
use crate::mcp::{
    AgentServerEntry, AgentServers, AgentType, BaseServerEntry, LocalServerEntry, RemoteServerEntry,
};

use super::AgentConfigAdapter;

pub struct OpenCodeAdapter {
    agent_type: AgentType,
}

impl OpenCodeAdapter {
    pub fn new() -> Self {
        Self { agent_type: AgentType::OpenCode }
    }

    fn read_root_value(&self) -> Result<Value, String> {
        let path = get_global_config_path(self.agent_type)?;
        if !path.exists() {
            return Err(format!("Config file not found: {:?}", path));
        }

        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        if content.trim().is_empty() {
            return Ok(Value::Object(Map::new()));
        }

        let content = strip_jsonc_comments(&content);

        serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON config: {}", e))
    }

    fn parse_server_entry(value: &Value) -> Result<AgentServerEntry, String> {
        let obj = value.as_object().ok_or_else(|| "Server config must be an object".to_string())?;

        let server_type = obj
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Server config must have 'type' field".to_string())?;

        match server_type {
            "local" => {
                let command_array: Vec<String> = obj
                    .get("command")
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .ok_or_else(|| "local server must have 'command' array".to_string())?;

                if command_array.is_empty() {
                    return Err("command array cannot be empty".to_string());
                }

                let command = command_array[0].clone();
                let args =
                    if command_array.len() > 1 { Some(command_array[1..].to_vec()) } else { None };

                let env: Option<HashMap<String, String>> =
                    obj.get("environment").and_then(|v| serde_json::from_value(v.clone()).ok());

                let timeout = obj.get("timeout").and_then(|v| v.as_u64()).map(|v| v as u32);

                Ok(AgentServerEntry::Local(LocalServerEntry {
                    base: BaseServerEntry { timeout },
                    command,
                    args,
                    env,
                }))
            }
            "remote" => {
                let url = obj
                    .get("url")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "remote server must have 'url' field".to_string())?
                    .to_string();

                let headers: Option<HashMap<String, String>> =
                    obj.get("headers").and_then(|v| serde_json::from_value(v.clone()).ok());

                let timeout = obj.get("timeout").and_then(|v| v.as_u64()).map(|v| v as u32);

                Ok(AgentServerEntry::Remote(RemoteServerEntry {
                    base: BaseServerEntry { timeout },
                    url,
                    headers,
                }))
            }
            _ => Err(format!("Unknown server type: {}", server_type)),
        }
    }

    fn server_entry_to_value(entry: &AgentServerEntry) -> Value {
        match entry {
            AgentServerEntry::Local(local) => {
                let mut obj = Map::new();
                obj.insert("type".to_string(), Value::String("local".to_string()));

                let mut command_array = vec![local.command.clone()];
                if let Some(args) = &local.args {
                    command_array.extend(args.clone());
                }
                obj.insert(
                    "command".to_string(),
                    Value::Array(command_array.into_iter().map(Value::String).collect()),
                );

                if let Some(env) = &local.env {
                    obj.insert(
                        "environment".to_string(),
                        serde_json::to_value(env).unwrap_or(Value::Object(Map::new())),
                    );
                }

                if let Some(timeout) = local.base.timeout {
                    obj.insert("timeout".to_string(), Value::Number(timeout.into()));
                }

                obj.insert("enabled".to_string(), Value::Bool(true));
                Value::Object(obj)
            }
            AgentServerEntry::Remote(remote) => {
                let mut obj = Map::new();
                obj.insert("type".to_string(), Value::String("remote".to_string()));
                obj.insert("url".to_string(), Value::String(remote.url.clone()));

                if let Some(headers) = &remote.headers {
                    obj.insert(
                        "headers".to_string(),
                        serde_json::to_value(headers).unwrap_or(Value::Object(Map::new())),
                    );
                }

                if let Some(timeout) = remote.base.timeout {
                    obj.insert("timeout".to_string(), Value::Number(timeout.into()));
                }

                obj.insert("enabled".to_string(), Value::Bool(true));
                Value::Object(obj)
            }
        }
    }
}

impl AgentConfigAdapter for OpenCodeAdapter {
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

        let mcp_value = root_obj.get("mcp");
        let mcp_obj = match mcp_value {
            Some(Value::Object(map)) => map,
            Some(_) => {
                return Err("mcp must be an object".to_string());
            }
            None => {
                return Ok(AgentServers { servers: HashMap::new() });
            }
        };

        let mut servers = HashMap::new();
        for (name, value) in mcp_obj {
            if let Some(enabled) = value.get("enabled").and_then(|v| v.as_bool()) {
                if !enabled {
                    continue;
                }
            }

            let server_config = Self::parse_server_entry(value)
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

        let content = strip_jsonc_comments(&content);

        let mut root_value = if content.trim().is_empty() {
            Value::Object(Map::new())
        } else {
            serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))?
        };

        let root_obj = root_value
            .as_object_mut()
            .ok_or_else(|| "Config root must be an object".to_string())?;

        let existing_mcp =
            root_obj.get("mcp").and_then(|v| v.as_object()).cloned().unwrap_or_default();

        let mut new_mcp = Map::new();

        for (name, server_config) in config.servers {
            let mut value = Self::server_entry_to_value(&server_config);
            if let (Some(existing), Some(new_obj)) =
                (existing_mcp.get(&name), value.as_object_mut())
            {
                if let Some(existing_obj) = existing.as_object() {
                    for (key, val) in existing_obj {
                        if !new_obj.contains_key(key) {
                            new_obj.insert(key.clone(), val.clone());
                        }
                    }
                }
            }
            new_mcp.insert(name, value);
        }

        for (name, value) in existing_mcp {
            if !new_mcp.contains_key(&name) {
                if let Some(enabled) = value.get("enabled").and_then(|v| v.as_bool()) {
                    if !enabled {
                        new_mcp.insert(name, value);
                    }
                }
            }
        }

        root_obj.insert("mcp".to_string(), Value::Object(new_mcp));

        let output = serde_json::to_string_pretty(&root_value)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?;
        std::fs::write(path, output).map_err(|e| e.to_string())
    }

    fn get_server_raw_config(&self, server_name: &str) -> Result<String, String> {
        let root = self.read_root_value()?;
        let root_obj =
            root.as_object().ok_or_else(|| "Config root must be an object".to_string())?;

        let server_value = root_obj
            .get("mcp")
            .and_then(|v| v.get(server_name))
            .ok_or_else(|| format!("Server '{}' not found", server_name))?
            .clone();

        let mut wrapper = Map::new();
        wrapper.insert(server_name.to_string(), server_value);

        serde_json::to_string_pretty(&wrapper)
            .map_err(|e| format!("Failed to serialize server config: {}", e))
    }
}

fn strip_jsonc_comments(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();
    let mut in_string = false;
    let mut escape_next = false;

    while let Some(c) = chars.next() {
        if escape_next {
            result.push(c);
            escape_next = false;
            continue;
        }

        if c == '\\' && in_string {
            result.push(c);
            escape_next = true;
            continue;
        }

        if c == '"' && !escape_next {
            in_string = !in_string;
            result.push(c);
            continue;
        }

        if !in_string && c == '/' {
            if let Some(&next) = chars.peek() {
                if next == '/' {
                    // Line comment, skip until newline
                    chars.next();
                    while let Some(&ch) = chars.peek() {
                        if ch == '\n' {
                            break;
                        }
                        chars.next();
                    }
                    continue;
                } else if next == '*' {
                    // Block comment, skip until */
                    chars.next();
                    while let Some(ch) = chars.next() {
                        if ch == '*' {
                            if let Some(&'/') = chars.peek() {
                                chars.next();
                                break;
                            }
                        }
                    }
                    continue;
                }
            }
        }

        result.push(c);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_jsonc_comments_line_comment() {
        let input = r#"{
  // This is a comment
  "key": "value"
}"#;
        let expected = r#"{
  
  "key": "value"
}"#;
        assert_eq!(strip_jsonc_comments(input), expected);
    }

    #[test]
    fn test_strip_jsonc_comments_block_comment() {
        let input = r#"{
  /* This is a
     block comment */
  "key": "value"
}"#;
        let expected = r#"{
  
  "key": "value"
}"#;
        assert_eq!(strip_jsonc_comments(input), expected);
    }

    #[test]
    fn test_strip_jsonc_comments_in_string() {
        let input = r#"{"key": "value // not a comment"}"#;
        assert_eq!(strip_jsonc_comments(input), input);
    }

    #[test]
    fn test_parse_local_server() {
        let value = serde_json::json!({
            "type": "local",
            "command": ["npx", "-y", "mcp-server"],
            "environment": {
                "API_KEY": "test"
            },
            "enabled": true,
            "timeout": 5000
        });

        let result = OpenCodeAdapter::parse_server_entry(&value).unwrap();
        match result {
            AgentServerEntry::Local(local) => {
                assert_eq!(local.command, "npx");
                assert_eq!(local.args, Some(vec!["-y".to_string(), "mcp-server".to_string()]));
                assert_eq!(local.env.unwrap().get("API_KEY").unwrap(), "test");
                assert_eq!(local.base.timeout, Some(5000));
            }
            _ => panic!("Expected Local server"),
        }
    }

    #[test]
    fn test_parse_remote_server() {
        let value = serde_json::json!({
            "type": "remote",
            "url": "https://mcp.example.com/mcp",
            "headers": {
                "Authorization": "Bearer token"
            },
            "enabled": true
        });

        let result = OpenCodeAdapter::parse_server_entry(&value).unwrap();
        match result {
            AgentServerEntry::Remote(remote) => {
                assert_eq!(remote.url, "https://mcp.example.com/mcp");
                assert_eq!(remote.headers.unwrap().get("Authorization").unwrap(), "Bearer token");
            }
            _ => panic!("Expected Remote server"),
        }
    }

    #[test]
    fn test_server_entry_to_value_local() {
        let entry = AgentServerEntry::Local(LocalServerEntry {
            base: BaseServerEntry { timeout: Some(5000) },
            command: "npx".to_string(),
            args: Some(vec!["-y".to_string(), "mcp-server".to_string()]),
            env: Some(HashMap::from([("API_KEY".to_string(), "test".to_string())])),
        });

        let value = OpenCodeAdapter::server_entry_to_value(&entry);
        let obj = value.as_object().unwrap();

        assert_eq!(obj.get("type").unwrap(), "local");
        assert_eq!(obj.get("command").unwrap(), &serde_json::json!(["npx", "-y", "mcp-server"]));
        assert_eq!(obj.get("environment").unwrap().get("API_KEY").unwrap(), "test");
        assert_eq!(obj.get("timeout").unwrap(), 5000);
        assert_eq!(obj.get("enabled").unwrap(), true);
    }
}
