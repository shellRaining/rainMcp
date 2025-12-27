# Roo Code

## 配置位置

- 全局: `~/.roo/mcp.json`
- 项目: `.roo/mcp.json`

## 支持的传输类型

- stdio
- HTTP
- SSE

## 示例配置

stdio 传输:

```json
{
  "mcpServers": {
    "local-server": {
      "command": "node",
      "args": ["server.js"],
      "env": {
        "API_KEY": "your_api_key"
      }
    }
  }
}
```

HTTP 传输:

```json
{
  "mcpServers": {
    "remote-server": {
      "type": "streamable-http",
      "url": "https://your-server.com/api/mcp-endpoint",
      "headers": {
        "X-API-Key": "your-secure-api-key"
      }
    }
  }
}
```

SSE 传输:

```json
{
  "mcpServers": {
    "legacy-server": {
      "type": "sse",
      "url": "https://your-legacy-server.com/mcp-base",
      "headers": {
        "Authorization": "Bearer token"
      }
    }
  }
}
```

## 官方文档

https://docs.roocode.com/features/mcp/using-mcp-in-roo
