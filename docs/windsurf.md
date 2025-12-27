# Windsurf

## 配置位置

- macOS: `~/.codeium/windsurf/mcp_config.json`
- Windows: `%USERPROFILE%\.codeium\windsurf\mcp_config.json`

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
      "url": "https://your-server-url/mcp",
      "headers": {
        "Authorization": "Bearer token"
      }
    }
  }
}
```

## 官方文档

https://docs.windsurf.com/windsurf/cascade/mcp
