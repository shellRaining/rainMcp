# macOS 图标生成指南

## 问题背景

在 macOS Big Sur 及以后版本中，应用图标需要遵循新的设计规范。如果图标内容占满整个画布而没有预留透明边距，会导致应用图标在 Dock 中显示得比其他系统应用图标更大，视觉效果不协调。

## 设计规范

根据 Apple 官方设计指南：

- macOS 图标需要在画布四周预留约 10% 的透明边距
- 对于 1024x1024 的标准画布，实际图标内容应控制在约 820x820 区域内
- 系统会自动应用圆角、阴影等视觉效果
- 图标内容应居中放置

参考资料：
- [Apple HIG - App Icons](https://developer.apple.com/design/human-interface-guidelines/app-icons)
- [Tauri Icons Documentation](https://v2.tauri.app/develop/icons)

## 解决方案

### 步骤 1: 调整源图标尺寸

使用 ImageMagick 为图标添加适当的边距：

```bash
magick original-icon.png -resize 820x820 -gravity center -background none -extent 1024x1024 icon-with-padding.png
```

参数说明：
- `-resize 820x820`: 将图标内容缩小到 820x820（占画布 80%）
- `-gravity center`: 将内容居中放置
- `-background none`: 使用透明背景
- `-extent 1024x1024`: 扩展画布到标准尺寸

### 步骤 2: 使用 Tauri CLI 生成所有平台图标

```bash
pnpm tauri icon icon-with-padding.png -o src-tauri/icons
```

或使用默认输入文件：

```bash
pnpm tauri icon
```

Tauri CLI 会自动生成：
- macOS: icon.icns（包含所有必需的尺寸和圆角处理）
- Windows: icon.ico
- 各种 PNG 尺寸：32x32.png, 128x128.png, 128x128@2x.png
- 移动端图标（可选）：android/, ios/

### 步骤 3: 清理多余文件

对于桌面端应用，只需保留以下文件：
- `32x32.png`
- `128x128.png`
- `128x128@2x.png`
- `icon.icns`（macOS）
- `icon.ico`（Windows）

如果不需要移动端或应用商店支持，可以删除：
- `android/` 目录
- `ios/` 目录
- `Square*.png`（Windows Store 图标）
- `StoreLogo.png`

```bash
cd src-tauri/icons
rm -rf android ios Square*.png StoreLogo.png
```

## 验证配置

检查 `src-tauri/tauri.conf.json` 中的图标配置：

```json
{
  "bundle": {
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

## 重要提示

1. 不要手动使用 `iconutil` 生成 .icns 文件，这会导致系统圆角丢失
2. 必须使用 Tauri 官方 CLI 工具，确保符合各平台规范
3. 源图标文件应该是正方形的 PNG 或 SVG 格式，建议尺寸 1024x1024
4. 透明背景很重要，不要使用纯色背景填充边距
5. 重新构建应用后才能看到图标更新效果

## 测试

构建应用并检查图标：

```bash
pnpm tauri build
```

或在开发模式下测试：

```bash
pnpm tauri dev
```

检查 macOS Dock 中的图标大小是否与其他系统应用保持一致。

## 故障排查

如果图标仍然显示过大：
1. 确认源图标已正确添加边距（四周约 102px 透明区域）
2. 重新运行 `pnpm tauri icon` 命令
3. 删除旧的构建产物并重新构建
4. 重启应用查看效果

如果图标圆角丢失：
1. 确认使用的是 Tauri CLI 而不是手动 iconutil
2. 检查 Tauri 版本是否为最新稳定版
