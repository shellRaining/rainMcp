use std::fs;

use rstest::rstest;
use tempfile::TempDir;

use crate::mcp::get_server_raw_config_command;

use super::fixtures::{test_env, CLAUDE_CODE_CONFIG_JSON, OPENAI_CODEX_CONFIG_TOML};
use super::EnvGuard;

#[rstest]
fn get_server_raw_config_command_reads_claude_code_local_server(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let config_path = temp_dir.path().join(".claude.json");
    fs::write(&config_path, CLAUDE_CODE_CONFIG_JSON).unwrap();

    let raw_config =
        get_server_raw_config_command("claude-code".to_string(), "server-name".to_string())
            .unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&raw_config).unwrap();
    let server = parsed.get("server-name").unwrap();
    assert_eq!(server.get("command").unwrap(), "npx");
    assert_eq!(server.get("args").unwrap(), &serde_json::json!(["-y", "mcp-server"]));
    assert_eq!(server.get("env").unwrap().get("API_KEY").unwrap(), "<redacted>");
    assert_eq!(server.get("timeout").unwrap(), 123);
}

#[rstest]
fn get_server_raw_config_command_reads_claude_code_remote_server(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let config_path = temp_dir.path().join(".claude.json");
    fs::write(&config_path, CLAUDE_CODE_CONFIG_JSON).unwrap();

    let raw_config =
        get_server_raw_config_command("claude-code".to_string(), "remote".to_string()).unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&raw_config).unwrap();
    let server = parsed.get("remote").unwrap();
    assert_eq!(server.get("url").unwrap(), "https://mcp.example.com/mcp");
    assert_eq!(server.get("headers").unwrap().get("Authorization").unwrap(), "Bearer <redacted>");
}

#[rstest]
fn get_server_raw_config_command_reads_openai_codex_local_server(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let config_path = temp_dir.path().join(".codex").join("config.toml");
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(&config_path, OPENAI_CODEX_CONFIG_TOML).unwrap();

    let raw_config =
        get_server_raw_config_command("openai-codex".to_string(), "context7".to_string()).unwrap();

    let parsed: toml::Value = toml::from_str(&raw_config).unwrap();
    let server = parsed.get("context7").unwrap();
    assert_eq!(server.get("command").unwrap().as_str().unwrap(), "npx");
    assert_eq!(
        server.get("args").unwrap().as_array().unwrap(),
        &vec![
            toml::Value::String("-y".to_string()),
            toml::Value::String("@upstash/context7-mcp".to_string())
        ]
    );
    assert_eq!(
        server.get("env").unwrap().get("MY_ENV_VAR").unwrap().as_str().unwrap(),
        "<redacted>"
    );
}

#[rstest]
fn get_server_raw_config_command_reads_openai_codex_remote_server(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let config_path = temp_dir.path().join(".codex").join("config.toml");
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(&config_path, OPENAI_CODEX_CONFIG_TOML).unwrap();

    let raw_config =
        get_server_raw_config_command("openai-codex".to_string(), "figma".to_string()).unwrap();

    let parsed: toml::Value = toml::from_str(&raw_config).unwrap();
    let server = parsed.get("figma").unwrap();
    assert_eq!(server.get("url").unwrap().as_str().unwrap(), "https://mcp.figma.com/mcp");
}

#[rstest]
fn get_server_raw_config_command_returns_error_when_config_missing(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let result =
        get_server_raw_config_command("claude-code".to_string(), "server-name".to_string());
    let err = result.unwrap_err();
    assert!(err.contains("Config file not found"));
}

#[rstest]
fn get_server_raw_config_command_returns_error_when_server_not_found(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let config_path = temp_dir.path().join(".claude.json");
    fs::write(&config_path, CLAUDE_CODE_CONFIG_JSON).unwrap();

    let result =
        get_server_raw_config_command("claude-code".to_string(), "nonexistent".to_string());
    let err = result.unwrap_err();
    assert!(err.contains("Server 'nonexistent' not found"));
}

#[rstest]
fn get_server_raw_config_command_returns_error_for_invalid_agent(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let result =
        get_server_raw_config_command("invalid-agent".to_string(), "server-name".to_string());
    let err = result.unwrap_err();
    assert!(err.contains("Unknown agent"));
}
