use std::fs;

use rstest::rstest;
use tempfile::TempDir;

use crate::mcp::{get_supported_agents_command, AgentType};

use super::fixtures::{test_env, CLAUDE_CODE_CONFIG_JSON};
use super::EnvGuard;

#[rstest]
fn get_supported_agents_command_returns_all_agents(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    let agents = get_supported_agents_command().unwrap();

    assert_eq!(agents.len(), 14);

    let agent_types: Vec<AgentType> = agents.iter().map(|a| a.agent_type).collect();
    assert!(agent_types.contains(&AgentType::ClaudeCode));
    assert!(agent_types.contains(&AgentType::Cursor));
    assert!(agent_types.contains(&AgentType::Windsurf));
    assert!(agent_types.contains(&AgentType::Cline));
    assert!(agent_types.contains(&AgentType::ClaudeDesktop));
    assert!(agent_types.contains(&AgentType::RooCode));
    assert!(agent_types.contains(&AgentType::Trae));
    assert!(agent_types.contains(&AgentType::GeminiCli));
    assert!(agent_types.contains(&AgentType::Kiro));
    assert!(agent_types.contains(&AgentType::OpenAiCodex));
    assert!(agent_types.contains(&AgentType::Comate));
    assert!(agent_types.contains(&AgentType::VsCodeCopilot));
    assert!(agent_types.contains(&AgentType::CopilotCli));
    assert!(agent_types.contains(&AgentType::Alma));
}

#[rstest]
fn get_supported_agents_command_detects_configured_agents(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let config_path = temp_dir.path().join(".claude.json");
    fs::write(&config_path, CLAUDE_CODE_CONFIG_JSON).unwrap();

    let agents = get_supported_agents_command().unwrap();

    let claude_code = agents.iter().find(|a| a.agent_type == AgentType::ClaudeCode).unwrap();
    assert!(claude_code.is_configured);
    assert!(claude_code.mcp_config.is_some());
    assert_eq!(claude_code.mcp_config.as_ref().unwrap().servers.len(), 2);

    let cursor = agents.iter().find(|a| a.agent_type == AgentType::Cursor).unwrap();
    assert!(!cursor.is_configured);
    assert!(cursor.mcp_config.is_none());
}

#[rstest]
fn get_supported_agents_command_reflects_enabled_status(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (_temp_dir, _env_guard, _lock) = test_env;

    use crate::config::get_app_config_path;
    let expected_path = get_app_config_path().unwrap();

    fs::create_dir_all(expected_path.parent().unwrap()).unwrap();
    fs::write(
        &expected_path,
        r#"{
  "clients": {
    "ClaudeCode": {
      "enabled": true
    },
    "Cursor": {
      "enabled": false
    },
    "Windsurf": {
      "enabled": false
    }
  }
}
"#,
    )
    .unwrap();

    let agents = get_supported_agents_command().unwrap();

    let claude_code = agents.iter().find(|a| a.agent_type == AgentType::ClaudeCode).unwrap();
    assert!(claude_code.enabled, "Expected ClaudeCode to be enabled");

    let cursor = agents.iter().find(|a| a.agent_type == AgentType::Cursor).unwrap();
    assert!(!cursor.enabled);

    let windsurf = agents.iter().find(|a| a.agent_type == AgentType::Windsurf).unwrap();
    assert!(!windsurf.enabled);
}

#[rstest]
fn get_supported_agents_command_has_correct_config_paths(
    test_env: (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>),
) {
    let (temp_dir, _env_guard, _lock) = test_env;

    let agents = get_supported_agents_command().unwrap();

    let claude_code = agents.iter().find(|a| a.agent_type == AgentType::ClaudeCode).unwrap();
    assert_eq!(claude_code.config_path, temp_dir.path().join(".claude.json"));

    let cursor = agents.iter().find(|a| a.agent_type == AgentType::Cursor).unwrap();
    assert_eq!(cursor.config_path, temp_dir.path().join(".cursor").join("mcp.json"));

    let codex = agents.iter().find(|a| a.agent_type == AgentType::OpenAiCodex).unwrap();
    assert_eq!(codex.config_path, temp_dir.path().join(".codex").join("config.toml"));

    let comate = agents.iter().find(|a| a.agent_type == AgentType::Comate).unwrap();
    assert_eq!(comate.config_path, temp_dir.path().join(".comate").join("mcp.json"));

    let copilot_cli = agents.iter().find(|a| a.agent_type == AgentType::CopilotCli).unwrap();
    assert_eq!(copilot_cli.config_path, temp_dir.path().join(".copilot").join("mcp-config.json"));

    let alma = agents.iter().find(|a| a.agent_type == AgentType::Alma).unwrap();
    assert_eq!(alma.config_path, temp_dir.path().join(".config").join("alma").join("mcp.json"));
}
