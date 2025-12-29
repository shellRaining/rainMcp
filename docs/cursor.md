# Cursor

## 配置位置

- 全局: `~/.cursor/mcp.json`
- 项目: `.cursor/mcp.json`

## 支持的传输类型

- stdio
- HTTP
- SSE

## 示例配置

stdio 传输:

```json
{
  "mcpServers": {
    "server-name": {
      "transport": "stdio",
      "command": "npx",
      "args": ["-y", "mcp-server"],
      "env": {
        "API_KEY": "value"
      },
      "startupTimeoutMs": 20000
    }
  }
}
```

HTTP 传输:

```json
{
  "mcpServers": {
    "server-name": {
      "transport": "http",
      "url": "http://localhost:3000/mcp",
      "headers": {
        "API_KEY": "value"
      },
      "timeout": 60
    }
  }
}
```

注意: `transport` 字段是必需的，`startupTimeoutMs` 和 `timeout` 字段是可选的。

## 官方文档

https://cursor.com/docs/context/mcp
