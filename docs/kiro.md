# Kiro

## 配置位置

- 用户级: `~/.kiro/settings/mcp.json`
- 工作区级: `.kiro/settings/mcp.json`

## 支持的传输类型

- stdio
- HTTP

## 示例配置

stdio 传输:

```json
{
  "mcpServers": {
    "web-search": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-bravesearch"],
      "env": {
        "BRAVE_API_KEY": "value"
      },
      "disabled": false,
      "autoApprove": []
    }
  }
}
```

HTTP 传输:

```json
{
  "mcpServers": {
    "remote-server": {
      "url": "https://your-server.com/mcp",
      "headers": {
        "Authorization": "Bearer token"
      },
      "timeout": 20000
    }
  }
}
```

注意: `disabled` 字段控制是否禁用服务器，`autoApprove` 字段用于自动批准特定工具的调用。

## 官方文档

https://kiro.dev/docs/mcp/configuration/
