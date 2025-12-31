use std::collections::HashMap;
use std::fs;

use rstest::rstest;
use tempfile::TempDir;

use crate::mcp::{
    update_agent_mcp_config_command, BaseMcpConfig, LocalMcpConfig, McpConfig, McpServerConfig,
    RemoteMcpConfig,
};

use super::fixtures::{
    test_env, CLAUDE_CODE_CONFIG_WITH_FIELDS_JSON, OPENAI_CODEX_CONFIG_WITH_FIELDS_TOML,
};
use super::EnvGuard;

#[rstest]
fn update_agent_mcp_config_command_preserves_claude_code_fields(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let config_path = temp_dir.path().join(".claude.json");
    fs::write(&config_path, CLAUDE_CODE_CONFIG_WITH_FIELDS_JSON).unwrap();

    let mut servers = HashMap::new();
    let mut local_env = HashMap::new();
    local_env.insert("API_KEY".to_string(), "value".to_string());
    servers.insert(
        "local-server".to_string(),
        McpServerConfig::Local(LocalMcpConfig {
            base: BaseMcpConfig { timeout: Some(120) },
            command: "npx".to_string(),
            args: Some(vec!["-y".to_string(), "new-local".to_string()]),
            env: Some(local_env),
        }),
    );
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer token".to_string());
    servers.insert(
        "remote-server".to_string(),
        McpServerConfig::Remote(RemoteMcpConfig {
            base: BaseMcpConfig { timeout: None },
            url: "https://mcp.example.com/mcp".to_string(),
            headers: Some(headers),
        }),
    );

    let config = McpConfig { servers };
    update_agent_mcp_config_command("claude-code".to_string(), config).unwrap();

    let updated: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&config_path).unwrap()).unwrap();
    assert_eq!(updated.get("installMethod").and_then(|v| v.as_str()), Some("homebrew"));
    assert_eq!(updated.get("numStartups").and_then(|v| v.as_u64()), Some(12));

    let mcp_servers = updated.get("mcpServers").and_then(|v| v.as_object()).unwrap();
    assert!(mcp_servers.get("old-server").is_none());

    let local = mcp_servers.get("local-server").and_then(|v| v.as_object()).unwrap();
    assert_eq!(local.get("command").and_then(|v| v.as_str()), Some("npx"));
    assert_eq!(local.get("timeout").and_then(|v| v.as_u64()), Some(120));
    let args = local.get("args").and_then(|v| v.as_array()).unwrap();
    assert_eq!(args.len(), 2);
    assert_eq!(args[0].as_str(), Some("-y"));
    assert_eq!(args[1].as_str(), Some("new-local"));
    let env = local.get("env").and_then(|v| v.as_object()).unwrap();
    assert_eq!(env.get("API_KEY").and_then(|v| v.as_str()), Some("value"));

    let remote = mcp_servers.get("remote-server").and_then(|v| v.as_object()).unwrap();
    assert_eq!(remote.get("url").and_then(|v| v.as_str()), Some("https://mcp.example.com/mcp"));
    let headers = remote.get("headers").and_then(|v| v.as_object()).unwrap();
    assert_eq!(headers.get("Authorization").and_then(|v| v.as_str()), Some("Bearer token"));
}

#[rstest]
fn update_agent_mcp_config_command_preserves_codex_fields(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let config_path = temp_dir.path().join(".codex").join("config.toml");
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(&config_path, OPENAI_CODEX_CONFIG_WITH_FIELDS_TOML).unwrap();

    let mut servers = HashMap::new();
    let mut local_env = HashMap::new();
    local_env.insert("TOKEN".to_string(), "value".to_string());
    servers.insert(
        "context7".to_string(),
        McpServerConfig::Local(LocalMcpConfig {
            base: BaseMcpConfig { timeout: None },
            command: "npx".to_string(),
            args: Some(vec!["-y".to_string(), "new-context7".to_string()]),
            env: Some(local_env),
        }),
    );
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer new".to_string());
    servers.insert(
        "remote".to_string(),
        McpServerConfig::Remote(RemoteMcpConfig {
            base: BaseMcpConfig { timeout: Some(45) },
            url: "https://mcp.example.com/mcp".to_string(),
            headers: Some(headers),
        }),
    );

    let config = McpConfig { servers };
    update_agent_mcp_config_command("openai-codex".to_string(), config).unwrap();

    let updated: toml::Value = toml::from_str(&fs::read_to_string(&config_path).unwrap()).unwrap();
    assert_eq!(updated.get("model").and_then(|v| v.as_str()), Some("gpt-4.1"));
    assert_eq!(updated.get("model_reasoning_effort").and_then(|v| v.as_str()), Some("medium"));
    let notice = updated.get("notice").and_then(|v| v.as_table()).unwrap();
    assert_eq!(notice.get("version").and_then(|v| v.as_integer()), Some(1));
    assert_eq!(notice.get("message").and_then(|v| v.as_str()), Some("keep"));

    let mcp_servers = updated.get("mcp_servers").and_then(|v| v.as_table()).unwrap();
    assert!(mcp_servers.get("context7").is_some());
    assert!(mcp_servers.get("remote").is_some());
    assert!(mcp_servers.get("jina").is_none());

    let context7 = mcp_servers.get("context7").and_then(|v| v.as_table()).unwrap();
    assert_eq!(context7.get("command").and_then(|v| v.as_str()), Some("npx"));
    let args = context7.get("args").and_then(|v| v.as_array()).unwrap();
    assert_eq!(args.len(), 2);
    assert_eq!(args[0].as_str(), Some("-y"));
    assert_eq!(args[1].as_str(), Some("new-context7"));
    let env = context7.get("env").and_then(|v| v.as_table()).unwrap();
    assert_eq!(env.get("TOKEN").and_then(|v| v.as_str()), Some("value"));

    let remote = mcp_servers.get("remote").and_then(|v| v.as_table()).unwrap();
    assert_eq!(remote.get("url").and_then(|v| v.as_str()), Some("https://mcp.example.com/mcp"));
    assert_eq!(remote.get("timeout").and_then(|v| v.as_integer()), Some(45));
    let headers = remote.get("headers").and_then(|v| v.as_table()).unwrap();
    assert_eq!(headers.get("Authorization").and_then(|v| v.as_str()), Some("Bearer new"));
}

#[rstest]
fn update_agent_mcp_config_command_creates_parent_dirs(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let config_path = temp_dir.path().join(".cursor").join("mcp.json");
    assert!(!config_path.exists());

    let mut servers = HashMap::new();
    servers.insert(
        "local".to_string(),
        McpServerConfig::Local(LocalMcpConfig {
            base: BaseMcpConfig { timeout: None },
            command: "npx".to_string(),
            args: Some(vec!["-y".to_string(), "server".to_string()]),
            env: None,
        }),
    );
    let config = McpConfig { servers };
    update_agent_mcp_config_command("cursor".to_string(), config).unwrap();

    assert!(config_path.exists());
    let updated: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&config_path).unwrap()).unwrap();
    let mcp_servers = updated.get("mcpServers").and_then(|v| v.as_object()).unwrap();
    assert!(mcp_servers.get("local").is_some());
}

#[rstest]
fn update_agent_mcp_config_command_rejects_unknown_agent(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let config = McpConfig { servers: HashMap::new() };
    let err = update_agent_mcp_config_command("unknown-agent".to_string(), config).unwrap_err();
    assert!(err.contains("Unknown agent"));
}

#[rstest]
fn update_agent_mcp_config_command_errors_on_invalid_json(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let config_path = temp_dir.path().join(".claude.json");
    fs::write(&config_path, "{").unwrap();

    let mut servers = HashMap::new();
    servers.insert(
        "local".to_string(),
        McpServerConfig::Local(LocalMcpConfig {
            base: BaseMcpConfig { timeout: None },
            command: "npx".to_string(),
            args: None,
            env: None,
        }),
    );
    let config = McpConfig { servers };
    let err = update_agent_mcp_config_command("claude-code".to_string(), config).unwrap_err();
    assert!(err.contains("Failed to parse JSON"));
}

#[rstest]
fn update_agent_mcp_config_command_errors_on_invalid_toml(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let config_path = temp_dir.path().join(".codex").join("config.toml");
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(&config_path, "not = [").unwrap();

    let mut servers = HashMap::new();
    servers.insert(
        "local".to_string(),
        McpServerConfig::Local(LocalMcpConfig {
            base: BaseMcpConfig { timeout: None },
            command: "npx".to_string(),
            args: None,
            env: None,
        }),
    );
    let config = McpConfig { servers };
    let err = update_agent_mcp_config_command("openai-codex".to_string(), config).unwrap_err();
    assert!(err.contains("Failed to parse TOML"));
}
