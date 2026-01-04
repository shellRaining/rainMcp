use super::agent_adapters;
use super::user_server::UserServer;
use super::{AgentServers, AgentType};
use std::collections::HashMap;

/// Add a server to agent's configuration
pub fn add_server_to_agent(
    agent: AgentType,
    server: &UserServer,
    server_name: Option<String>,
) -> Result<(), String> {
    let name = server_name.unwrap_or_else(|| server.name.clone());

    // Read existing config or create new one
    let mut agent_servers = match read_agent_config(agent) {
        Ok(c) => c,
        Err(_) => AgentServers { servers: HashMap::new() },
    };

    // Check if server already exists
    if agent_servers.servers.contains_key(&name) {
        return Err(format!("Server '{}' already exists in agent config", name));
    }

    agent_servers.servers.insert(name, server.config.clone());
    save_agent_config(agent, agent_servers)
}

/// 统一读取接口
pub fn read_agent_config(agent: AgentType) -> Result<AgentServers, String> {
    agent_adapters::get_adapter(agent).read_config()
}

/// 获取指定 server 的原始配置字符串（包含 server 名称作为 key）
pub fn get_server_raw_config(agent: AgentType, server_name: &str) -> Result<String, String> {
    agent_adapters::get_adapter(agent).get_server_raw_config(server_name)
}

pub fn save_agent_config(agent: AgentType, config: AgentServers) -> Result<(), String> {
    agent_adapters::get_adapter(agent).write_config(config)
}
