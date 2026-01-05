# OpenCode

## 配置位置

- 项目级: `opencode.json` 或 `opencode.jsonc`（项目根目录）
- 全局: `~/.config/opencode/opencode.json`

## 支持的传输类型

- stdio (local)
- HTTP (remote)
- OAuth (remote with authentication)

## 示例配置

stdio 传输 (local):

```json
{
  "mcp": {
    "server-name": {
      "type": "local",
      "command": ["npx", "-y", "mcp-server"],
      "environment": {
        "API_KEY": "value"
      },
      "enabled": true,
      "timeout": 5000
    }
  }
}
```

HTTP 传输 (remote):

```json
{
  "mcp": {
    "server-name": {
      "type": "remote",
      "url": "https://mcp.example.com/mcp",
      "headers": {
        "Authorization": "Bearer token"
      },
      "enabled": true,
      "timeout": 5000
    }
  }
}
```

OAuth 认证 (remote):

```json
{
  "mcp": {
    "server-name": {
      "type": "remote",
      "url": "https://mcp.example.com/mcp",
      "oauth": {
        "clientId": "{env:MY_CLIENT_ID}",
        "clientSecret": "{env:MY_CLIENT_SECRET}",
        "scope": "tools:read tools:execute"
      }
    }
  }
}
```

注意:
- `type` 字段是必需的，值为 `"local"` 或 `"remote"`
- `enabled` 字段控制是否启用服务器，默认为 true
- `timeout` 字段指定获取工具的超时时间（毫秒），默认为 5000
- `environment` 字段（local）用于设置环境变量
- `oauth` 字段（remote）支持自动 OAuth 认证流程，设为 `false` 可禁用

## 官方文档

https://opencode.ai/docs/mcp-servers/
