use std::collections::{HashMap, HashSet};

use serde_json::Value;

use crate::mcp::{AgentServerEntry, BaseServerEntry, LocalServerEntry, RemoteServerEntry};

pub fn parse_server_entry(value: &Value) -> Result<AgentServerEntry, String> {
    let obj = value.as_object().ok_or_else(|| "Server config must be an object".to_string())?;

    if obj.contains_key("command") {
        let command = obj
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "command must be a string".to_string())?
            .to_string();

        let args: Option<Vec<String>> =
            obj.get("args").and_then(|v| serde_json::from_value(v.clone()).ok());
        let env: Option<HashMap<String, String>> =
            obj.get("env").and_then(|v| serde_json::from_value(v.clone()).ok());

        let timeout = obj
            .get("timeout")
            .or_else(|| obj.get("startupTimeoutMs"))
            .and_then(|v| v.as_u64())
            .map(|v| v as u32);

        Ok(AgentServerEntry::Local(LocalServerEntry {
            base: BaseServerEntry { timeout },
            command,
            args,
            env,
        }))
    } else if obj.contains_key("url") || obj.contains_key("httpUrl") {
        let url = obj
            .get("url")
            .or_else(|| obj.get("httpUrl"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| "url must be a string".to_string())?
            .to_string();

        let headers: Option<HashMap<String, String>> =
            obj.get("headers").and_then(|v| serde_json::from_value(v.clone()).ok());

        let timeout = obj
            .get("timeout")
            .or_else(|| obj.get("startupTimeoutMs"))
            .and_then(|v| v.as_u64())
            .map(|v| v as u32);

        Ok(AgentServerEntry::Remote(RemoteServerEntry {
            base: BaseServerEntry { timeout },
            url,
            headers,
        }))
    } else {
        Err("Server config must have either 'command' or 'url' field".to_string())
    }
}

pub fn server_entry_to_value(entry: &AgentServerEntry) -> Result<Value, String> {
    match entry {
        AgentServerEntry::Local(local) => serde_json::to_value(local).map_err(|e| e.to_string()),
        AgentServerEntry::Remote(remote) => serde_json::to_value(remote).map_err(|e| e.to_string()),
    }
}

pub fn merge_server_value(
    base: Value,
    defaults: serde_json::Map<String, Value>,
    existing: Option<&Value>,
) -> Result<Value, String> {
    let base_map =
        base.as_object().cloned().ok_or_else(|| "Server config must be an object".to_string())?;
    let base_keys: HashSet<String> = base_map.keys().cloned().collect();

    let mut merged = base_map;
    for (key, value) in defaults {
        if !merged.contains_key(&key) {
            merged.insert(key, value);
        }
    }

    if let Some(Value::Object(existing_map)) = existing {
        for (key, value) in existing_map {
            if !base_keys.contains(key) {
                merged.insert(key.clone(), value.clone());
            }
        }
    }

    Ok(Value::Object(merged))
}
