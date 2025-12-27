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
      "command": "npx",
      "args": ["-y", "mcp-server"],
      "env": {
        "API_KEY": "value"
      }
    }
  }
}
```

HTTP/SSE 传输:

```json
{
  "mcpServers": {
    "server-name": {
      "url": "http://localhost:3000/mcp",
      "headers": {
        "API_KEY": "value"
      }
    }
  }
}
```

## 官方文档

https://cursor.com/docs/context/mcp
