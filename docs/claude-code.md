# Claude Code

## 配置位置

- 用户级: `~/.claude.json`
- 项目级: `.mcp.json`
- 本地级: `.claude/settings.local.json`

## 支持的传输类型

- stdio
- HTTP

## 示例配置

stdio 传输:

```json
{
  "mcpServers": {
    "server-name": {
      "command": "npx",
      "args": ["-y", "mcp-server"],
      "env": {
        "API_KEY": "value"
      }
    }
  }
}
```

HTTP 传输:

```json
{
  "mcpServers": {
    "server-name": {
      "url": "https://mcp.example.com/mcp",
      "headers": {
        "Authorization": "Bearer token"
      }
    }
  }
}
```

## 官方文档

https://code.claude.com/docs/en/mcp
