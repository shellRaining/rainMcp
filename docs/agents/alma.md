# Alma

## 配置位置

- 全局: `~/.config/alma/mcp.json`

## 支持的传输类型

- stdio
- HTTP

## 示例配置

stdio 传输:

```json
{
  "mcpServers": {
    "nano-banana": {
      "command": "npx",
      "args": ["nano-banana-mcp"],
      "env": {
        "GEMINI_API_KEY": "YOUR_API_KEY"
      }
    }
  }
}
```

HTTP 传输:

```json
{
  "mcpServers": {
    "context7": {
      "url": "https://mcp.context7.com/mcp",
      "headers": {
        "CONTEXT7_API_KEY": "YOUR_API_KEY"
      }
    }
  }
}
```

## 官方文档

暂未找到公开文档。
