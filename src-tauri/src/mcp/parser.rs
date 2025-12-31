use super::{
    AgentType, BaseMcpConfig, LocalMcpConfig, McpConfig, McpServerConfig, RemoteMcpConfig,
};
use std::collections::HashMap;
use std::path::Path;

/// 转换通用 JSON 配置到内部格式
fn convert_generic_server_config(value: serde_json::Value) -> Result<McpServerConfig, String> {
    // 尝试解析为对象
    let obj = value.as_object().ok_or_else(|| "Server config must be an object".to_string())?;

    // 判断是本地还是远程配置（根据是否有 command 或 url 字段）
    if obj.contains_key("command") {
        // 本地配置
        let command = obj
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "command must be a string".to_string())?
            .to_string();

        let args = obj.get("args").and_then(|v| serde_json::from_value(v.clone()).ok());

        let env = obj.get("env").and_then(|v| serde_json::from_value(v.clone()).ok());

        let timeout = obj
            .get("timeout")
            .or_else(|| obj.get("startupTimeoutMs"))
            .and_then(|v| v.as_u64())
            .map(|v| v as u32);

        Ok(McpServerConfig::Local(LocalMcpConfig {
            base: BaseMcpConfig { timeout },
            command,
            args,
            env,
        }))
    } else if obj.contains_key("url") || obj.contains_key("httpUrl") {
        // 远程配置
        let url = obj
            .get("url")
            .or_else(|| obj.get("httpUrl"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| "url must be a string".to_string())?
            .to_string();

        let headers = obj.get("headers").and_then(|v| serde_json::from_value(v.clone()).ok());

        let timeout = obj
            .get("timeout")
            .or_else(|| obj.get("startupTimeoutMs"))
            .and_then(|v| v.as_u64())
            .map(|v| v as u32);

        Ok(McpServerConfig::Remote(RemoteMcpConfig {
            base: BaseMcpConfig { timeout },
            url,
            headers,
        }))
    } else {
        Err("Server config must have either 'command' or 'url' field".to_string())
    }
}

/// 解析通用 JSON 格式（大多数 agent）
fn parse_generic_json_config(path: &Path) -> Result<McpConfig, String> {
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

    #[derive(serde::Deserialize)]
    struct GenericJsonConfig {
        #[serde(rename = "mcpServers")]
        mcp_servers: HashMap<String, serde_json::Value>,
    }

    let config: GenericJsonConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse JSON config: {}", e))?;

    // 转换每个服务器配置到内部格式
    let mut servers = HashMap::new();
    for (name, value) in config.mcp_servers {
        let server_config = convert_generic_server_config(value)
            .map_err(|e| format!("Failed to parse server '{}': {}", name, e))?;
        servers.insert(name, server_config);
    }

    Ok(McpConfig { servers })
}

/// 解析 Claude Code 格式
fn parse_claude_code_config(path: &Path) -> Result<McpConfig, String> {
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

    #[derive(serde::Deserialize)]
    struct ClaudeCodeJsonConfig {
        #[serde(rename = "mcpServers", default)]
        mcp_servers: HashMap<String, serde_json::Value>,
    }

    let config: ClaudeCodeJsonConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse Claude Code config: {}", e))?;

    // 转换到内部格式
    let mut servers = HashMap::new();
    for (name, value) in config.mcp_servers {
        let server_config = convert_generic_server_config(value)
            .map_err(|e| format!("Failed to parse server '{}': {}", name, e))?;
        servers.insert(name, server_config);
    }

    Ok(McpConfig { servers })
}

/// 解析 TOML 格式（OpenAI Codex）
fn parse_toml_config(path: &Path) -> Result<McpConfig, String> {
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

    let toml_value: toml::Value =
        toml::from_str(&content).map_err(|e| format!("Failed to parse TOML config: {}", e))?;

    let mcp_servers_table = toml_value
        .get("mcp_servers")
        .and_then(|v| v.as_table())
        .ok_or_else(|| "mcp_servers section not found in TOML config".to_string())?;

    // 转换 TOML table 到内部格式
    let mut servers = HashMap::new();
    for (name, value) in mcp_servers_table {
        // 将 TOML 值转为 JSON，然后使用通用转换函数
        let json_value = serde_json::to_value(value)
            .map_err(|e| format!("Failed to convert TOML to JSON: {}", e))?;

        let server_config = convert_generic_server_config(json_value)
            .map_err(|e| format!("Failed to parse server '{}': {}", name, e))?;
        servers.insert(name.clone(), server_config);
    }

    Ok(McpConfig { servers })
}

/// 统一读取接口
pub fn read_agent_config(agent: AgentType) -> Result<McpConfig, String> {
    let path = super::paths::get_global_config_path(agent)?;

    if !path.exists() {
        return Err(format!("Config file not found: {:?}", path));
    }

    match agent {
        AgentType::OpenAiCodex => parse_toml_config(&path),
        AgentType::ClaudeCode => parse_claude_code_config(&path),
        _ => parse_generic_json_config(&path),
    }
}

/// 获取指定 server 的原始配置字符串（包含 server 名称作为 key）
pub fn get_server_raw_config(agent: AgentType, server_name: &str) -> Result<String, String> {
    let path = super::paths::get_global_config_path(agent)?;

    if !path.exists() {
        return Err(format!("Config file not found: {:?}", path));
    }

    let content =
        std::fs::read_to_string(&path).map_err(|e| format!("Failed to read config file: {}", e))?;

    match agent {
        AgentType::OpenAiCodex => {
            let toml_value: toml::Value = toml::from_str(&content)
                .map_err(|e| format!("Failed to parse TOML config: {}", e))?;

            let server_config = toml_value
                .get("mcp_servers")
                .and_then(|v| v.get(server_name))
                .ok_or_else(|| format!("Server '{}' not found", server_name))?
                .clone();

            // 构建包含 server 名称的 table
            let mut wrapper = toml::Table::new();
            wrapper.insert(server_name.to_string(), server_config);

            toml::to_string_pretty(&wrapper)
                .map_err(|e| format!("Failed to serialize server config: {}", e))
        }
        _ => {
            let json_value: serde_json::Value = serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse JSON config: {}", e))?;

            let server_config = json_value
                .get("mcpServers")
                .and_then(|v| v.get(server_name))
                .ok_or_else(|| format!("Server '{}' not found", server_name))?
                .clone();

            // 构建包含 server 名称的对象
            let mut wrapper = serde_json::Map::new();
            wrapper.insert(server_name.to_string(), server_config);

            serde_json::to_string_pretty(&wrapper)
                .map_err(|e| format!("Failed to serialize server config: {}", e))
        }
    }
}

fn convert_server_config_to_value(config: &McpServerConfig) -> Result<serde_json::Value, String> {
    match config {
        McpServerConfig::Local(c) => serde_json::to_value(c).map_err(|e| e.to_string()),
        McpServerConfig::Remote(c) => serde_json::to_value(c).map_err(|e| e.to_string()),
    }
}

pub fn save_agent_config(agent: AgentType, config: McpConfig) -> Result<(), String> {
    let path = super::paths::get_global_config_path(agent)?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    // 如果文件不存在，创建一个新的空结构
    let file_content = if path.exists() {
        std::fs::read_to_string(&path).map_err(|e| e.to_string())?
    } else {
        String::new()
    };

    match agent {
        AgentType::OpenAiCodex => {
            // TOML 处理
            let mut toml_value: toml::Table = if file_content.trim().is_empty() {
                toml::Table::new()
            } else {
                toml::from_str(&file_content).map_err(|e| format!("Failed to parse TOML: {}", e))?
            };

            let mut mcp_servers = toml::Table::new();
            for (name, server_config) in config.servers {
                let json_value = convert_server_config_to_value(&server_config)?;
                // JSON Value -> TOML Value
                let toml_server_value: toml::Value = toml::Value::try_from(json_value)
                    .map_err(|e| format!("Failed to convert server config to TOML: {}", e))?;

                mcp_servers.insert(name, toml_server_value);
            }

            toml_value.insert("mcp_servers".to_string(), toml::Value::Table(mcp_servers));

            let string_output = toml::to_string_pretty(&toml_value)
                .map_err(|e| format!("Failed to serialize TOML: {}", e))?;

            std::fs::write(path, string_output).map_err(|e| e.to_string())?;
        }
        _ => {
            // JSON 处理 (Generic & ClaudeCode structure)
            let mut json_value: serde_json::Map<String, serde_json::Value> =
                if file_content.trim().is_empty() {
                    serde_json::Map::new()
                } else {
                    serde_json::from_str(&file_content)
                        .map_err(|e| format!("Failed to parse JSON: {}", e))?
                };

            let mut mcp_servers_map = serde_json::Map::new();
            for (name, server_config) in config.servers {
                let server_value = convert_server_config_to_value(&server_config)?;
                mcp_servers_map.insert(name, server_value);
            }

            json_value.insert("mcpServers".to_string(), serde_json::Value::Object(mcp_servers_map));

            let string_output = serde_json::to_string_pretty(&json_value)
                .map_err(|e| format!("Failed to serialize JSON: {}", e))?;

            std::fs::write(path, string_output).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}
