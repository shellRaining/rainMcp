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
