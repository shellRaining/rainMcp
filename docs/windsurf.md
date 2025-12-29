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
      },
      "disabled": false,
      "disabledTools": []
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
      },
      "timeout": 20000
    }
  }
}
```

注意: `disabled` 字段控制是否禁用服务器，`disabledTools` 字段用于禁用服务器中的特定工具。

## 官方文档

https://docs.windsurf.com/windsurf/cascade/mcp
