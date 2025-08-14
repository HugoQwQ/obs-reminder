# OBS Reminder

*其他语言版本：[English](README.md) | 简体中文 | [繁體中文](README-TW.md)*

一个可在 OBS Studio 中显示可自定义的 Toast 通知。非常适合需要在直播/录制过程中定期提醒的用户。

## 截图

[在这里](screenshots.md)

## 功能特性

- **轻、小、快**：使用 Rust 和 egui 构建易用界面
- **多软件集成**：可添加浏览器源的直播、录制工具（甚至浏览器）都可以使用
- **高度可自定义 设置**：配置多个标题、内容、颜色、显示时长等
- **实时预览**：可随时测试 Toast 通知效果

## 下一步计划

- [ ] 添加 Windows 系统托盘
- [ ] 提高清晰度
- [ ] 添加音效提醒

## 快速开始

### 1. 下载并运行
1. 从发布页面下载最新版本
2. 运行 `obs-reminder-client.exe`
3. 应用程序将同时启动桌面界面和 Web 服务器

### 2. 配置提醒
1. **添加标题**：输入多个提醒标题（如"喝水！"、"查看聊天"、"伸展休息"）
2. **添加内容**：输入对应的内容消息
3. **设置颜色**：选择背景渐变色和文字颜色
4. **设置时长**：配置每个 Toast 显示多长时间（1-60 秒）
5. **设置间隔**：选择提醒出现频率（1-1440 分钟）
6. **选择模式**：选择随机或顺序显示顺序

### 3. 添加到 OBS
1. 在 OBS Studio 中，添加新的**浏览器源**
2. 从桌面应用复制 URL（通常是 `http://localhost:8080`）
3. 将其粘贴到浏览器源 URL 字段
4. 设置宽度：`265`，高度：`85`（或根据需要调整）
5. 勾选"源不可见时关闭"和"场景变为活动状态时刷新浏览器"

### 4. 开始提醒
1. 点击**保存**以保存配置
2. 点击**测试 Toast** 预览在 OBS 中的效果
3. 点击**开始**开始自动提醒
4. 随时使用**停止**暂停提醒

## 配置

应用程序将设置保存到同目录下的 `config.toml` 文件。如需要，可以直接编辑此文件：

```toml
[app]
version = "0.0.1"

[toaster]
titles = ["bbb", "114514", "alright"]
contents = ["im content", "hoho", "what can i say?"]
interval_time = 30  # 分钟
duration = 5        # 秒
color_1 = "#FF6B6B"
color_2 = "#4ECDC4"
text_color = "#FFFFFF"
content_switch_mode = "random"  # 或 "sequential" 即随机、顺序
```

## 技术细节

### 架构
- **桌面应用**：Rust + egui 控制界面
- **Web 服务器**：使用 hyper-rs 的内嵌 HTTP 服务器
- **浏览器组件**：Svelte 配合 svelte-toast 显示通知
- **通信**：WebSocket 实时消息传递

### 端口
- **HTTP 服务器**：`localhost:8080`（用于 OBS 浏览器源）
- **WebSocket**：`localhost:7981`（内部通信）

### 系统要求
- Windows 10/11（64位）
- OBS Studio 28.0+（支持浏览器源）
- 可用端口 8080 和 7981

## 从源码构建

### 前置要求
- Rust 最新版
- Node.js 22+ 和 pnpm（或其他包管理器）
- Git

### 构建步骤
```bash
# 克隆仓库
git clone https://github.com/HugoQwQ/obs-reminder.git
cd obs-reminder

# 构建浏览器组件
cd browser
pnpm install
pnpm run build
cd ..

# 构建 Rust 应用程序
cargo build --release

# 可执行文件位于 target/release/obs-reminder-client.exe
```

## 故障排除

### 常见问题

**Toast 在 OBS 中不显示：**
- 验证浏览器源 URL 是否为 `http://localhost:8080`
- 检查桌面应用是否正在运行
- 尝试刷新浏览器源
- 确保 Windows 防火墙没有阻止应用程序

**连接问题：**
- 确保端口 8080 和 7981 可用
- 检查 Windows Defender/杀毒软件设置
- 如需要，尝试以管理员身份运行

**性能问题：**
- 如果遇到延迟，减少 Toast 显示时长
- 确保 OBS 硬件加速已启用
- 关闭不必要的应用程序

## 贡献

欢迎贡献！请随时提交 Pull Request。对于重大更改，请先开启 issue 讨论您想要更改的内容。

### 开发设置
1. Fork 仓库
2. 创建功能分支（`git checkout -b feature/amazing-feature`）
3. 进行更改
4. 彻底测试
5. 提交更改（`git commit -m 'Add amazing feature'`）
6. 推送到分支（`git push origin feature/amazing-feature`）
7. 开启 Pull Request

## 许可证

本项目采用 GPL-3 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 致谢

- 使用 [egui](https://github.com/emilk/egui) 构建桌面界面
- 使用 [Svelte](https://svelte.dev/) 和 [svelte-toast](https://github.com/zerodevx/svelte-toast) 显示浏览器通知
- 由 [hyper](https://hyper.rs/) 提供内嵌 Web 服务器支持

## 支持

如果您觉得这个项目有帮助，请考虑：
- ⭐ 给仓库点星
- 🐛 通过 Issues 报告错误
- 💡 通过 Issues 建议功能
- 📖 改进文档
- 😍 老登快给我爆米

---

**祝直播愉快！** 🎮✨
