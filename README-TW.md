# OBS Reminder (繁體中文)

*其他語言版本：[English](README.md) | [简体中文](README-CN.md) | 繁體中文*

一款可在 OBS Studio 顯示自訂 Toast 提醒的輕量工具，非常適合在直播或錄製過程中需要定期提醒的使用者。

## 屏幕截圖

[在這裏](screenshots.md)

## 功能特性

- **輕量、快速、簡單**：使用 Rust 與 egui 建構簡單易用介面
- **多軟體整合**：可用於支援瀏覽器來源的直播或錄製工具（甚至瀏覽器）
- **高度自訂設定**：可配置多個標題、內容、顏色、顯示時間等
- **即時預覽**：可隨時測試 Toast 效果

## 下一步計畫

* [ ] 新增 Windows 系統托盤
* [ ] 提高清晰度
* [ ] 新增音效提醒

## 快速開始

### 1. 下載並運行

1. 從發佈頁面下載最新版本
2. 運行 `obs-reminder-client.exe`
3. 應用程式將同時啟動桌面介面與嵌入式 Web 伺服器

### 2. 配置提醒

1. **新增標題**：輸入多個提醒標題（如"喝水！"、"查看聊天"、"伸展休息"）
2. **新增內容**：輸入對應訊息內容
3. **設定顏色**：選擇背景漸層與文字顏色
4. **設定顯示時間**：每個 Toast 顯示多久（1–60 秒）
5. **設定間隔**：提醒出現頻率（1–1440 分鐘）
6. **選擇模式**：隨機或順序顯示

### 3. 加入 OBS

1. 在 OBS Studio 新增 **瀏覽器來源**
2. 從桌面應用複製 URL（通常為 `http://localhost:8080`）
3. 貼上至瀏覽器來源 URL 欄位
4. 設定寬度：`265`，高度：`85`（或依需求調整）
5. 勾選「來源不可見時關閉」與「場景變為活動狀態時刷新瀏覽器」

### 4. 開始提醒

1. 點擊 **保存** 儲存配置
2. 點擊 **測試 Toast** 預覽 OBS 效果
3. 點擊 **開始** 啟動自動提醒
4. 點擊 **停止** 隨時暫停

## 配置

設定將保存在同目錄下的 `config.toml`，也可直接編輯：

```toml
[app]
version = "0.0.1"

[toaster]
titles = ["bbb", "114514", "alright"]
contents = ["im content", "hoho", "what can i say?"]
interval_time = 30  # 分鐘
duration = 5        # 秒
color_1 = "#FF6B6B"
color_2 = "#4ECDC4"
text_color = "#FFFFFF"
content_switch_mode = "random"  # 或 "sequential" 即隨機、順序
```

## 技術細節

### 架構

* **桌面應用**：Rust + egui
* **Web 伺服器**：hyper-rs 內嵌 HTTP 伺服器
* **瀏覽器組件**：Svelte + svelte-toast
* **通信**：WebSocket 實時訊息傳遞

### 端口

* **HTTP 伺服器**：`localhost:8080`（供 OBS 瀏覽器來源使用）
* **WebSocket**：`localhost:7981`（內部通信）

### 系統需求

* Windows 10/11（64位）
* OBS Studio 28.0+（支援瀏覽器來源）
* 可用端口 8080 與 7981

## 從源碼構建

### 前置需求

* 最新 Rust
* Node.js 22+ 與 pnpm（或其他包管理器）
* Git

### 構建步驟

注意：方法一中Windows必須安裝7zip
```bash
# Clone repository
git clone https://github.com/HugoQwQ/obs-reminder.git
cd obs-reminder

# Linux & Macos
chmod +x ./scripts/build.sh
./scripts/build_and_package.sh

# Windows
.\scripts\build.bat

```

或者

```bash
# 克隆倉庫
git clone https://github.com/HugoQwQ/obs-reminder.git
cd obs-reminder

# 构建瀏覽器組件
cd browser
pnpm install
pnpm run build
cd ..

# 構建 Rust 應用
cargo build --release

# 可執行文件位於 target/release/obs-reminder-client.exe
```

## 故障排除

### 常見問題

**Toast 在 OBS 不顯示：**

* 確認瀏覽器來源 URL 為 `http://localhost:8080`
* 檢查桌面應用是否運行
* 嘗試刷新瀏覽器來源
* 確保防火牆未阻擋應用

**連線問題：**

* 確認端口 8080 與 7981 可用
* 檢查防毒軟體設定
* 如有需要，嘗試以管理員身份運行

**性能問題：**

* 若有延遲，縮短 Toast 顯示時間
* 確保 OBS 硬體加速已啟用
* 關閉不必要應用

## 貢獻

歡迎貢獻！可隨時提交 Pull Request。重大更改請先開 issue 討論。

### 開發設定

1. Fork 倉庫
2. 創建功能分支 (`git checkout -b feature/amazing-feature`)
3. 進行更改
4. 徹底測試
5. 提交更改 (`git commit -m 'Add amazing feature'`)
6. 推送分支 (`git push origin feature/amazing-feature`)
7. 開啟 Pull Request

## 許可證

本項目採用 GPL-3 許可證，詳見 [LICENSE](LICENSE) 文件。

## 致謝

* [egui](https://github.com/emilk/egui) 提供桌面 UI
* [Svelte](https://svelte.dev/) 與 [svelte-toast](https://github.com/zerodevx/svelte-toast) 提供瀏覽器通知
* [hyper](https://hyper.rs/) 提供內嵌 Web 伺服器支持

## 支持

如果您覺得本項目有幫助，請考慮：

* ⭐ 點星支持
* 🐛 透過 Issues 回報錯誤
* 💡 提出功能建議
* 📖 改進文件
* 😍 請我喝咖啡

---

**祝直播愉快！** 🎮✨
