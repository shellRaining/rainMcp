# OpenAI Codex

## 配置位置

- 全局: `~/.codex/config.toml`

## 支持的传输类型

- stdio
- HTTP

## 示例配置

stdio 传输:

```toml
[mcp_servers.context7]
command = "npx"
args = ["-y", "@upstash/context7-mcp"]

[mcp_servers.context7.env]
MY_ENV_VAR = "MY_ENV_VALUE"
```

HTTP 传输:

```toml
[mcp_servers.figma]
url = "https://mcp.figma.com/mcp"
bearer_token_env_var = "FIGMA_OAUTH_TOKEN"
```

## 官方文档

https://developers.openai.com/codex/mcp/
