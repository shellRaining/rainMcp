# Cline

## 配置位置

- macOS: `~/Library/Application Support/Code/User/globalStorage/saoudrizwan.claude-dev/settings/cline_mcp_settings.json`
- Windows: `%APPDATA%\Code\User\globalStorage\saoudrizwan.claude-dev\settings\cline_mcp_settings.json`
- Linux: `~/.config/Code/User/globalStorage/saoudrizwan.claude-dev/settings/cline_mcp_settings.json`

## 支持的传输类型

- stdio
- SSE

## 示例配置

stdio 传输:

```json
{
  "mcpServers": {
    "local-server": {
      "command": "node",
      "args": ["/path/to/server.js"],
      "env": {
        "API_KEY": "your_api_key"
      }
    }
  }
}
```

SSE 传输:

```json
{
  "mcpServers": {
    "remote-server": {
      "url": "https://your-server-url.com/mcp",
      "headers": {
        "Authorization": "Bearer your-token"
      }
    }
  }
}
```

## 官方文档

https://docs.cline.bot/mcp/configuring-mcp-servers
