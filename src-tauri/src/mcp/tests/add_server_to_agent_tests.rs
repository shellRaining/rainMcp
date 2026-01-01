use std::collections::HashMap;
use std::fs;

use rstest::rstest;
use tempfile::TempDir;

use crate::config::{load_app_config, save_app_config};
use crate::mcp::agent_config::{add_server_to_agent, read_agent_config};
use crate::mcp::user_server::{OriginType, ServerOrigin, UserServer};
use crate::mcp::{
    AgentServerEntry, AgentType, BaseServerEntry, LocalServerEntry, RemoteServerEntry,
};

use super::fixtures::test_env;
use super::EnvGuard;

fn create_remote_user_server() -> UserServer {
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer token123".to_string());

    UserServer {
        id: "remote-server".to_string(),
        name: "Remote MCP".to_string(),
        config: AgentServerEntry::Remote(RemoteServerEntry {
            base: BaseServerEntry { timeout: None },
            url: "https://mcp.example.com/sse".to_string(),
            headers: Some(headers),
        }),
        origin: Some(ServerOrigin {
            origin_type: OriginType::Custom,
            schema_name: None,
            package_id: None,
        }),
        created_at: None,
    }
}

fn create_custom_command_user_server() -> UserServer {
    let mut env = HashMap::new();
    env.insert("API_KEY".to_string(), "secret-key".to_string());

    UserServer {
        id: "custom-server".to_string(),
        name: "Custom Server".to_string(),
        config: AgentServerEntry::Local(LocalServerEntry {
            base: BaseServerEntry { timeout: None },
            command: "python".to_string(),
            args: Some(vec!["-m".to_string(), "mcp_server".to_string()]),
            env: Some(env),
        }),
        origin: Some(ServerOrigin {
            origin_type: OriginType::Custom,
            schema_name: None,
            package_id: None,
        }),
        created_at: None,
    }
}

fn create_registry_user_server() -> UserServer {
    let mut env = HashMap::new();
    env.insert("JINA_API_KEY".to_string(), "jina-key-123".to_string());

    UserServer {
        id: "registry-server".to_string(),
        name: "Jina MCP".to_string(),
        config: AgentServerEntry::Local(LocalServerEntry {
            base: BaseServerEntry { timeout: None },
            command: "npx".to_string(),
            args: Some(vec!["-y".to_string(), "@jina-ai/mcp-server@1.2.0".to_string()]),
            env: Some(env),
        }),
        origin: Some(ServerOrigin {
            origin_type: OriginType::Registry,
            schema_name: Some("io.jina/mcp-jina".to_string()),
            package_id: Some("npm:@jina-ai/mcp-server".to_string()),
        }),
        created_at: None,
    }
}

#[rstest]
fn add_remote_server_to_claude_code(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    fs::write(temp_dir.path().join(".claude.json"), "{}").unwrap();

    let server = create_remote_user_server();
    add_server_to_agent(AgentType::ClaudeCode, &server, None).unwrap();

    let config = read_agent_config(AgentType::ClaudeCode).unwrap();
    assert!(config.servers.contains_key("Remote MCP"));

    let saved = config.servers.get("Remote MCP").unwrap();
    match saved {
        AgentServerEntry::Remote(remote) => {
            assert_eq!(remote.url, "https://mcp.example.com/sse");
            assert!(remote.headers.is_some());
        }
        _ => panic!("Expected Remote config"),
    }
}

#[rstest]
fn add_custom_command_server_to_claude_code(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    fs::write(temp_dir.path().join(".claude.json"), "{}").unwrap();

    let server = create_custom_command_user_server();
    add_server_to_agent(AgentType::ClaudeCode, &server, None).unwrap();

    let config = read_agent_config(AgentType::ClaudeCode).unwrap();
    assert!(config.servers.contains_key("Custom Server"));

    let saved = config.servers.get("Custom Server").unwrap();
    match saved {
        AgentServerEntry::Local(local) => {
            assert_eq!(local.command, "python");
            assert_eq!(local.args, Some(vec!["-m".to_string(), "mcp_server".to_string()]));
        }
        _ => panic!("Expected Local config"),
    }
}

#[rstest]
fn add_registry_server_to_claude_code(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    fs::write(temp_dir.path().join(".claude.json"), "{}").unwrap();

    let server = create_registry_user_server();
    add_server_to_agent(AgentType::ClaudeCode, &server, None).unwrap();

    let config = read_agent_config(AgentType::ClaudeCode).unwrap();
    assert!(config.servers.contains_key("Jina MCP"));

    let saved = config.servers.get("Jina MCP").unwrap();
    match saved {
        AgentServerEntry::Local(local) => {
            assert_eq!(local.command, "npx");
            let args = local.args.as_ref().unwrap();
            assert!(args.contains(&"-y".to_string()));
            assert!(args.contains(&"@jina-ai/mcp-server@1.2.0".to_string()));
        }
        _ => panic!("Expected Local config"),
    }
}

#[rstest]
fn add_server_with_custom_name(test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>)) {
    let (temp_dir, _env_guard, _lock) = test_env;

    fs::write(temp_dir.path().join(".claude.json"), "{}").unwrap();

    let server = create_remote_user_server();
    add_server_to_agent(AgentType::ClaudeCode, &server, Some("my-custom-name".to_string()))
        .unwrap();

    let config = read_agent_config(AgentType::ClaudeCode).unwrap();
    assert!(config.servers.contains_key("my-custom-name"));
    assert!(!config.servers.contains_key("Remote MCP"));
}

#[rstest]
fn add_server_fails_when_name_exists(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    fs::write(
        temp_dir.path().join(".claude.json"),
        r#"{"mcpServers": {"Remote MCP": {"command": "echo"}}}"#,
    )
    .unwrap();

    let server = create_remote_user_server();
    let result = add_server_to_agent(AgentType::ClaudeCode, &server, None);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("already exists"));
}

#[rstest]
fn add_server_creates_config_if_not_exists(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let server = create_remote_user_server();
    add_server_to_agent(AgentType::ClaudeCode, &server, None).unwrap();

    let config = read_agent_config(AgentType::ClaudeCode).unwrap();
    assert!(config.servers.contains_key("Remote MCP"));
}

#[rstest]
fn add_server_to_openai_codex(test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>)) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let config_dir = temp_dir.path().join(".codex");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(config_dir.join("config.toml"), "").unwrap();

    let server = create_custom_command_user_server();
    add_server_to_agent(AgentType::OpenAiCodex, &server, None).unwrap();

    let config = read_agent_config(AgentType::OpenAiCodex).unwrap();
    assert!(config.servers.contains_key("Custom Server"));
}

#[rstest]
fn add_server_to_agent_command_works(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    fs::write(temp_dir.path().join(".claude.json"), "{}").unwrap();

    // Save user server to app config
    let server = create_remote_user_server();
    let mut app_config = load_app_config();
    app_config.user_servers.push(server);
    save_app_config(&app_config).unwrap();

    // Call the function that the command uses
    let app_config = load_app_config();
    let server = app_config.user_servers.iter().find(|s| s.id == "remote-server").unwrap();
    add_server_to_agent(AgentType::ClaudeCode, server, None).unwrap();

    let config = read_agent_config(AgentType::ClaudeCode).unwrap();
    assert!(config.servers.contains_key("Remote MCP"));
}
