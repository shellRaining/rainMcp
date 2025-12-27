# Trae

## 配置位置

- 全局: `~/.trae/mcp.json`
- 项目级: `.trae/mcp.json`

## 支持的传输类型

- stdio
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

SSE 传输:

```json
{
  "mcpServers": {
    "server-name": {
      "url": "https://your-server-url.com/sse",
      "headers": {
        "Authorization": "Bearer token"
      }
    }
  }
}
```

## 官方文档

https://docs.trae.ai/ide/model-context-protocol
