# GitHub Copilot CLI

## 配置位置

- MCP 配置: `~/.copilot/mcp-config.json`
- 其他配置: `~/.copilot/config.json`

说明: 可以通过设置 `XDG_CONFIG_HOME` 更改配置目录。

## 支持的传输类型

- local
- stdio
- HTTP
- SSE

## 示例配置

local 传输:

```json
{
  "mcpServers": {
    "filesystem": {
      "type": "local",
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem"],
      "tools": ["*"]
    }
  }
}
```

HTTP 传输:

```json
{
  "mcpServers": {
    "jina": {
      "type": "http",
      "url": "https://mcp.jina.ai/v1",
      "headers": {},
      "tools": ["*"]
    }
  }
}


```

注意: `tools` 和 `type` 为必填字段。local 需要 `command` 和 `args`，HTTP/SSE 需要 `url`。

## 官方文档

https://docs.github.com/en/copilot/how-tos/use-copilot-agents/use-copilot-cli
https://docs.github.com/en/copilot/how-tos/use-copilot-agents/coding-agent/extend-coding-agent-with-mcp
