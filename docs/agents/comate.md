# Comate

## 配置位置

- 全局: `~/.comate/mcp.json`

## 支持的传输类型

- stdio
- streamableHttp

## 示例配置

stdio 传输:

```json
{
  "mcpServers": {
    "drawio": {
      "command": "npx",
      "args": ["@next-ai-drawio/mcp-server@latest"],
      "disabled": false
    }
  }
}
```

streamableHttp 传输:

```json
{
  "mcpServers": {
    "github": {
      "url": "https://api.githubcopilot.com/mcp/",
      "headers": {
        "Authorization": "Bearer YOUR_TOKEN"
      },
      "disabled": true
    }
  }
}
```

## 官方文档

暂未找到公开文档。
