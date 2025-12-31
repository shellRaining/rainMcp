use std::fs;

use rstest::rstest;
use tempfile::TempDir;

use crate::mcp::{get_agent_mcp_config_command, McpServerConfig};

use super::fixtures::{test_env, CLAUDE_CODE_CONFIG_JSON, OPENAI_CODEX_CONFIG_TOML};
use super::EnvGuard;

#[rstest]
fn get_agent_mcp_config_command_reads_claude_code(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let config_path = temp_dir.path().join(".claude.json");
    fs::write(&config_path, CLAUDE_CODE_CONFIG_JSON).unwrap();

    let config = get_agent_mcp_config_command("claude-code".to_string()).unwrap();
    assert_eq!(config.servers.len(), 2);

    let server = config.servers.get("server-name").unwrap();
    match server {
        McpServerConfig::Local(local) => {
            assert_eq!(local.command, "npx");
            assert_eq!(
                local.args.clone().unwrap(),
                vec!["-y".to_string(), "mcp-server".to_string()]
            );
            assert_eq!(local.env.as_ref().unwrap().get("API_KEY").unwrap(), "<redacted>");
            assert_eq!(local.base.timeout, Some(123));
        }
        _ => panic!("expected local config"),
    }

    let remote_server = config.servers.get("remote").unwrap();
    match remote_server {
        McpServerConfig::Remote(remote) => {
            assert_eq!(remote.url, "https://mcp.example.com/mcp");
            assert_eq!(
                remote.headers.as_ref().unwrap().get("Authorization").unwrap(),
                "Bearer <redacted>"
            );
        }
        _ => panic!("expected remote config"),
    }
}

#[rstest]
fn get_agent_mcp_config_command_reads_openai_codex(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let config_path = temp_dir.path().join(".codex").join("config.toml");
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(&config_path, OPENAI_CODEX_CONFIG_TOML).unwrap();

    let config = get_agent_mcp_config_command("openai-codex".to_string()).unwrap();
    assert_eq!(config.servers.len(), 2);

    let server = config.servers.get("context7").unwrap();
    match server {
        McpServerConfig::Local(local) => {
            assert_eq!(local.command, "npx");
            assert_eq!(
                local.args.clone().unwrap(),
                vec!["-y".to_string(), "@upstash/context7-mcp".to_string()]
            );
            assert_eq!(local.env.as_ref().unwrap().get("MY_ENV_VAR").unwrap(), "<redacted>");
            assert_eq!(local.base.timeout, None);
        }
        _ => panic!("expected local config"),
    }

    let remote_server = config.servers.get("figma").unwrap();
    match remote_server {
        McpServerConfig::Remote(remote) => {
            assert_eq!(remote.url, "https://mcp.figma.com/mcp");
            assert!(remote.headers.is_none());
            assert_eq!(remote.base.timeout, None);
        }
        _ => panic!("expected remote config"),
    }
}

#[rstest]
fn get_agent_mcp_config_command_returns_error_when_missing(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let result = get_agent_mcp_config_command("claude-code".to_string());
    let err = result.unwrap_err();
    assert!(err.contains("Config file not found"));
}
