# rainMcp

## 项目背景

现在市面上有很多 coding agent 的工具都支持使用 MCP。但是他们的配置方式不尽相同，导致用户在某个 coding agent 配置一个 MCP 工具后，不能够快速的迁移到另外一个 Agent 使用

## 技术栈

### 核心框架

- Tauri 2.0 - 跨平台桌面应用框架
- Vue 3 + TypeScript - 前端框架
- Vite - 构建工具

### 状态管理与工具

- Pinia - Vue 官方状态管理方案
- VueUse - Vue Composition API 工具集
- Reka UI - 无头 UI 组件库（Vue 版 Radix）

### 样式方案

- UnoCSS - 原子化 CSS 引擎（含 Attributify 和 Icons 预设）

### 开发工具

- Bun - 包管理器和运行时
- Oxlint + oxfmt - 前端代码检查和格式化
- rustfmt + clippy - Rust 代码格式化和检查

### 日志系统

- tauri-plugin-log - Tauri 官方日志插件
  - Linux: ~/.local/state/com.shellraining.rainMcp/ (XDG STATE)
  - macOS: ~/Library/Logs/com.shellraining.rainMcp/
  - Windows: %LOCALAPPDATA%\com.shellraining.rainMcp\logs\

## 项目结构

```
rainMcp/
├── src/
│   ├── components/      # Vue 组件
│   ├── composables/     # 组合式函数
│   ├── stores/          # Pinia 状态管理
│   ├── utils/           # 工具函数
│   ├── views/           # 视图组件（按功能模块分组）
│   │   ├── overview/    # 主仪表板
│   │   ├── agents/      # Agents 相关视图
│   │   ├── servers/     # Servers 相关视图
│   │   └── settings/    # Settings 相关视图
│   ├── styles/          # 全局样式
│   ├── App.vue          # 根组件
│   └── main.ts          # 入口文件
├── src-tauri/
│   ├── src/             # Rust 源码
│   ├── icons/           # 应用图标
│   ├── Cargo.toml       # Rust 依赖配置
│   ├── tauri.conf.json  # Tauri 配置
│   ├── rustfmt.toml     # Rust 格式化配置
│   └── clippy.toml      # Clippy 检查配置
├── uno.config.ts        # UnoCSS 配置
├── vite.config.ts       # Vite 配置
└── package.json         # 项目配置
```

## 特性

- 无需路由：使用 Pinia 状态管理实现视图切换，更适合桌面应用
- 无头组件：使用 Reka UI 完全控制 UI 样式
- 高性能样式：UnoCSS 按需生成，构建体积小
- 完整的代码质量保障：前后端 lint + format
- 跨平台日志：自动遵循各平台日志存储规范
