use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use rstest::rstest;
use tempfile::TempDir;

use crate::mcp::agent_config::{add_server_to_agent, read_agent_config, save_agent_config};
use crate::mcp::user_server::UserServer;
use crate::mcp::{
    AgentServerEntry, AgentServers, AgentType, BaseServerEntry, LocalServerEntry, RemoteServerEntry,
};

use super::fixtures::test_env;
use super::EnvGuard;

fn create_opencode_config_dir() -> PathBuf {
    let config_dir = dirs::home_dir().unwrap().join(".config/opencode");
    fs::create_dir_all(&config_dir).unwrap();
    config_dir
}

#[rstest]
fn read_opencode_local_server(test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>)) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let config_dir = create_opencode_config_dir();
    fs::write(
        config_dir.join("opencode.json"),
        r#"{
            "mcp": {
                "test-server": {
                    "type": "local",
                    "command": ["npx", "-y", "mcp-server"],
                    "environment": {
                        "API_KEY": "test-key"
                    },
                    "enabled": true,
                    "timeout": 5000
                }
            }
        }"#,
    )
    .unwrap();

    let config = read_agent_config(AgentType::OpenCode).unwrap();
    assert!(config.servers.contains_key("test-server"));

    let server = config.servers.get("test-server").unwrap();
    match server {
        AgentServerEntry::Local(local) => {
            assert_eq!(local.command, "npx");
            assert_eq!(local.args, Some(vec!["-y".to_string(), "mcp-server".to_string()]));
            assert_eq!(local.env.as_ref().unwrap().get("API_KEY").unwrap(), "test-key");
            assert_eq!(local.base.timeout, Some(5000));
        }
        _ => panic!("Expected Local server"),
    }
}

#[rstest]
fn read_opencode_remote_server(test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>)) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let config_dir = create_opencode_config_dir();
    fs::write(
        config_dir.join("opencode.json"),
        r#"{
            "mcp": {
                "remote-server": {
                    "type": "remote",
                    "url": "https://mcp.example.com/mcp",
                    "headers": {
                        "Authorization": "Bearer token"
                    },
                    "enabled": true
                }
            }
        }"#,
    )
    .unwrap();

    let config = read_agent_config(AgentType::OpenCode).unwrap();
    assert!(config.servers.contains_key("remote-server"));

    let server = config.servers.get("remote-server").unwrap();
    match server {
        AgentServerEntry::Remote(remote) => {
            assert_eq!(remote.url, "https://mcp.example.com/mcp");
            assert_eq!(
                remote.headers.as_ref().unwrap().get("Authorization").unwrap(),
                "Bearer token"
            );
        }
        _ => panic!("Expected Remote server"),
    }
}

#[rstest]
fn read_opencode_skips_disabled_servers(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let config_dir = create_opencode_config_dir();
    fs::write(
        config_dir.join("opencode.json"),
        r#"{
            "mcp": {
                "enabled-server": {
                    "type": "local",
                    "command": ["npx", "enabled"],
                    "enabled": true
                },
                "disabled-server": {
                    "type": "local",
                    "command": ["npx", "disabled"],
                    "enabled": false
                }
            }
        }"#,
    )
    .unwrap();

    let config = read_agent_config(AgentType::OpenCode).unwrap();
    assert!(config.servers.contains_key("enabled-server"));
    assert!(!config.servers.contains_key("disabled-server"));
}

#[rstest]
fn write_opencode_local_server(test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>)) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let config_dir = create_opencode_config_dir();
    fs::write(config_dir.join("opencode.json"), "{}").unwrap();

    let mut env = HashMap::new();
    env.insert("API_KEY".to_string(), "secret".to_string());

    let mut servers = HashMap::new();
    servers.insert(
        "my-server".to_string(),
        AgentServerEntry::Local(LocalServerEntry {
            base: BaseServerEntry { timeout: Some(10000) },
            command: "npx".to_string(),
            args: Some(vec!["-y".to_string(), "my-mcp-server".to_string()]),
            env: Some(env),
        }),
    );

    save_agent_config(AgentType::OpenCode, AgentServers { servers }).unwrap();

    let content = fs::read_to_string(config_dir.join("opencode.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    let server = &json["mcp"]["my-server"];
    assert_eq!(server["type"], "local");
    assert_eq!(server["command"], serde_json::json!(["npx", "-y", "my-mcp-server"]));
    assert_eq!(server["environment"]["API_KEY"], "secret");
    assert_eq!(server["timeout"], 10000);
    assert_eq!(server["enabled"], true);
}

#[rstest]
fn write_opencode_remote_server(test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>)) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let config_dir = create_opencode_config_dir();
    fs::write(config_dir.join("opencode.json"), "{}").unwrap();

    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer token".to_string());

    let mut servers = HashMap::new();
    servers.insert(
        "remote-server".to_string(),
        AgentServerEntry::Remote(RemoteServerEntry {
            base: BaseServerEntry { timeout: None },
            url: "https://mcp.example.com/mcp".to_string(),
            headers: Some(headers),
        }),
    );

    save_agent_config(AgentType::OpenCode, AgentServers { servers }).unwrap();

    let content = fs::read_to_string(config_dir.join("opencode.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    let server = &json["mcp"]["remote-server"];
    assert_eq!(server["type"], "remote");
    assert_eq!(server["url"], "https://mcp.example.com/mcp");
    assert_eq!(server["headers"]["Authorization"], "Bearer token");
    assert_eq!(server["enabled"], true);
}

#[rstest]
fn add_server_to_opencode(test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>)) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let config_dir = create_opencode_config_dir();
    fs::write(config_dir.join("opencode.json"), "{}").unwrap();

    let mut env = HashMap::new();
    env.insert("API_KEY".to_string(), "test-key".to_string());

    let server = UserServer {
        id: "test-id".to_string(),
        name: "Test Server".to_string(),
        config: AgentServerEntry::Local(LocalServerEntry {
            base: BaseServerEntry { timeout: None },
            command: "npx".to_string(),
            args: Some(vec!["-y".to_string(), "test-mcp".to_string()]),
            env: Some(env),
        }),
        origin: None,
        created_at: None,
    };

    add_server_to_agent(AgentType::OpenCode, &server, None).unwrap();

    let config = read_agent_config(AgentType::OpenCode).unwrap();
    assert!(config.servers.contains_key("Test Server"));
}

#[rstest]
fn read_opencode_jsonc_with_comments(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let config_dir = create_opencode_config_dir();
    fs::write(
        config_dir.join("opencode.json"),
        r#"{
            // This is a comment
            "mcp": {
                /* Block comment */
                "test-server": {
                    "type": "local",
                    "command": ["echo", "hello"],
                    "enabled": true
                }
            }
        }"#,
    )
    .unwrap();

    let config = read_agent_config(AgentType::OpenCode).unwrap();
    assert!(config.servers.contains_key("test-server"));
}

#[rstest]
fn opencode_preserves_disabled_servers_on_write(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let config_dir = create_opencode_config_dir();
    fs::write(
        config_dir.join("opencode.json"),
        r#"{
            "mcp": {
                "disabled-server": {
                    "type": "local",
                    "command": ["npx", "disabled"],
                    "enabled": false
                }
            }
        }"#,
    )
    .unwrap();

    let mut servers = HashMap::new();
    servers.insert(
        "new-server".to_string(),
        AgentServerEntry::Local(LocalServerEntry {
            base: BaseServerEntry { timeout: None },
            command: "npx".to_string(),
            args: Some(vec!["new".to_string()]),
            env: None,
        }),
    );

    save_agent_config(AgentType::OpenCode, AgentServers { servers }).unwrap();

    let content = fs::read_to_string(config_dir.join("opencode.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert!(json["mcp"]["new-server"].is_object());
    assert!(json["mcp"]["disabled-server"].is_object());
    assert_eq!(json["mcp"]["disabled-server"]["enabled"], false);
}

#[rstest]
fn opencode_preserves_extra_config_fields(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let config_dir = create_opencode_config_dir();
    fs::write(
        config_dir.join("opencode.json"),
        r#"{
            "other_config": "should_be_preserved",
            "mcp": {
                "test-server": {
                    "type": "local",
                    "command": ["npx", "test"],
                    "enabled": true
                }
            }
        }"#,
    )
    .unwrap();

    let config = read_agent_config(AgentType::OpenCode).unwrap();
    save_agent_config(AgentType::OpenCode, config).unwrap();

    let content = fs::read_to_string(config_dir.join("opencode.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(json["other_config"], "should_be_preserved");
}
