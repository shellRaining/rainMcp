# Gemini CLI

## 配置位置

- 用户级: `~/.gemini/settings.json`
- 项目级: `.gemini/settings.json`

## 支持的传输类型

- stdio
- HTTP
- SSE

## 示例配置

stdio 传输:

```json
{
  "mcpServers": {
    "pythonTools": {
      "command": "python",
      "args": ["-m", "my_mcp_server"],
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
    "httpServer": {
      "httpUrl": "http://localhost:3000/mcp"
    }
  }
}
```

SSE 传输:

```json
{
  "mcpServers": {
    "sseServer": {
      "url": "https://api.example.com/sse",
      "headers": {
        "Authorization": "Bearer token"
      }
    }
  }
}
```

## 官方文档

https://geminicli.com/docs/tools/mcp-server/
