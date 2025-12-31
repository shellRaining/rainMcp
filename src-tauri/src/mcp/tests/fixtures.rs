use rstest::fixture;
use tempfile::TempDir;

use super::{set_temp_home, EnvGuard, ENV_LOCK};

#[fixture]
pub fn test_env() -> (TempDir, EnvGuard, std::sync::MutexGuard<'static, ()>) {
    let lock = ENV_LOCK.lock().unwrap();
    let temp_dir = TempDir::new().unwrap();
    let env_guard = set_temp_home(&temp_dir);
    (temp_dir, env_guard, lock)
}

pub const CLAUDE_CODE_CONFIG_JSON: &str = r#"{
  "mcpServers": {
    "server-name": {
      "command": "npx",
      "args": ["-y", "mcp-server"],
      "env": {
        "API_KEY": "<redacted>"
      },
      "timeout": 123
    },
    "remote": {
      "url": "https://mcp.example.com/mcp",
      "headers": {
        "Authorization": "Bearer <redacted>"
      }
    }
  }
}
"#;

pub const CLAUDE_CODE_CONFIG_WITH_FIELDS_JSON: &str = r#"{
  "installMethod": "homebrew",
  "numStartups": 12,
  "mcpServers": {
    "old-server": {
      "command": "npx",
      "args": ["-y", "old"]
    }
  }
}
"#;

pub const OPENAI_CODEX_CONFIG_TOML: &str = r#"[mcp_servers.context7]
command = "npx"
args = ["-y", "@upstash/context7-mcp"]

[mcp_servers.context7.env]
MY_ENV_VAR = "<redacted>"

[mcp_servers.figma]
url = "https://mcp.figma.com/mcp"
bearer_token_env_var = "REDACTED_TOKEN_ENV"
"#;

pub const OPENAI_CODEX_CONFIG_WITH_FIELDS_TOML: &str = r#"model = "gpt-4.1"
model_reasoning_effort = "medium"

[notice]
version = 1
message = "keep"

[mcp_servers.context7]
command = "npx"
args = ["-y", "@upstash/context7-mcp"]

[mcp_servers.jina]
url = "https://mcp.jina.ai"
http_headers = { Authorization = "Bearer old" }
"#;
