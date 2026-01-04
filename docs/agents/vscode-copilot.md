# VS Code Copilot

## 配置位置

- 项目级: `.vscode/mcp.json`
- 用户级: VS Code 用户配置 `mcp.json` (通过 MCP: Open User Configuration 打开)
- 用户配置目录参考 (settings.json 位置):
  - Windows: `%APPDATA%\Code\User\settings.json`
  - macOS: `$HOME/Library/Application Support/Code/User/settings.json`
  - Linux: `$HOME/.config/Code/User/settings.json`
- Profiles 目录参考 (settings.json 位置):
  - Windows: `%APPDATA%\Code\User\profiles\<profile ID>\settings.json`
  - macOS: `$HOME/Library/Application Support/Code/User/profiles/<profile ID>/settings.json`
  - Linux: `$HOME/.config/Code/User/profiles/<profile ID>/settings.json`

说明: `mcp.json` 位于对应 profile 目录中，和 `settings.json` 同目录。

## 支持的传输类型

- stdio
- HTTP
- SSE

## 示例配置

stdio 传输:

```json
{
  "servers": {
    "memory": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-memory"]
    }
  }
}
```

HTTP 传输:

```json
{
  "servers": {
    "context7": {
      "type": "http",
      "url": "https://mcp.context7.com/mcp"
    }
  }
}
```

包含输入变量的配置:

```json
{
  "inputs": [
    {
      "type": "promptString",
      "id": "perplexity-key",
      "description": "Perplexity API Key",
      "password": true
    }
  ],
  "servers": {
    "perplexity": {
      "type": "stdio",
      "command": "npx",
      "args": ["-y", "server-perplexity-ask"],
      "env": {
        "PERPLEXITY_API_KEY": "${input:perplexity-key}"
      }
    }
  }
}
```

## 官方文档

https://code.visualstudio.com/docs/copilot/customization/mcp-servers
https://code.visualstudio.com/docs/getstarted/settings
https://docs.github.com/en/copilot/how-tos/provide-context/use-mcp/extend-copilot-chat-with-mcp
