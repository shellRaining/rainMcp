# Trae

## 配置位置

macOS:

- 全局: `~/Library/Application Support/Trae/User/mcp.json`
- 项目级: `.trae/mcp.json`

Windows:

- 全局: `%APPDATA%\Trae\User\mcp.json`
- 项目级: `.trae/mcp.json`

Linux:

- 全局: `~/.config/Trae/User/mcp.json`
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
      },
      "disabled": false
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
      },
      "disabled": false
    }
  }
}
```

注意: `disabled` 字段控制是否禁用服务器。

## 官方文档

https://docs.trae.ai/ide/model-context-protocol
