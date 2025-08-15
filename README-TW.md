# OBS Reminder (繁體中文)

<p align="center">
    <a href="https://discord.gg/Hvz9Axh84z">
        <img alt="BilibiliShowcase  " src="https://img.shields.io/badge/Bilibili-Showcase-blue"/></a>
    <a href="README_CN.md"><img alt="CN doc" src="https://img.shields.io/badge/docs-简体中文-white" /></a>
    <a href="README_TW.md"><img alt="TW doc" src="https://img.shields.io/badge/docs-繁體中文-white.svg" /></a>
    <a href="LICENSE"><img src="https://img.shields.io/static/v1?label=license&message=GPL&color=white&style=flat" alt="License"/></a>
</p>

一款可在 OBS Studio 顯示自訂通知提醒的輕量工具，非常適合在直播或錄製過程中需要定期提醒的使用者。

## 螢幕截圖

[在這裡](screenshots.md)

## 功能特性

- **輕量、快速、簡單**：使用 Rust 與 egui 建構簡單易用介面
- **多軟體整合**：可用於支援瀏覽器來源的直播或錄製工具（甚至瀏覽器）
- **高度自訂設定**：可設定多個標題、內容、顏色、顯示時間等
- **即時預覽**：可隨時測試 Toast 效果

## 下一步計畫

-  新增 Windows 系統匣

## 快速開始

### 1. 下載並執行

1. 從發佈頁面下載最新版本
2. 執行 `obs-reminder-client.exe`
3. 應用程式將同時啟動桌面介面與內建 Web 伺服器

### 2. 設定提醒

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
5. 勾選「來源不可見時關閉」與「場景變為活動狀態時重新整理瀏覽器」

### 4. 開始提醒

1. 點擊 **保存** 儲存設定
2. 點擊 **測試 Toast** 預覽 OBS 效果
3. 點擊 **開始** 啟動自動提醒
4. 點擊 **停止** 隨時暫停

## 設定檔

設定會保存在同目錄下的 `config.toml`，也可直接編輯：

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

- **桌面應用**：Rust + egui
- **Web 伺服器**：hyper-rs 內嵌 HTTP 伺服器
- **瀏覽器組件**：Svelte + svelte-toast
- **通訊**：WebSocket 即時訊息傳遞

### 連接埠

- **HTTP 伺服器**：`localhost:8080`（供 OBS 瀏覽器來源使用）
- **WebSocket**：`localhost:7981`（內部通訊）

### 系統需求

- Windows 10/11（64 位元）
- OBS Studio 28.0+（支援瀏覽器來源）
- 可用連接埠 8080 與 7981

## 從原始碼建置

### 前置需求

- 最新版本的 Rust
- Node.js 22+ 與 pnpm（或其他套件管理工具）
- Git

### 建置步驟

> [!WARNING]
>  方式 1 依賴 [7zip](https://www.7-zip.org/)、[taskfile](https://taskfile.dev/docs/installation) 請自行安裝

```bash
# 複製儲存庫
git clone https://github.com/HugoQwQ/obs-reminder.git
cd obs-reminder

# 建置並打包
task
```

或者

```bash
# 複製儲存庫
git clone https://github.com/HugoQwQ/obs-reminder.git
cd obs-reminder

# 建置瀏覽器組件
cd browser
pnpm install
pnpm run build
cd ..

# 建置 Rust 應用
cargo build --release

# 可執行檔位於 target/release/obs-reminder-client.exe
```

## 疑難排解

### 常見問題

**Toast 在 OBS 不顯示：**

- 確認瀏覽器來源 URL 為 `http://localhost:8080`
- 檢查桌面應用是否正在執行
- 嘗試重新整理瀏覽器來源
- 確保防火牆未封鎖應用程式

**連線問題：**

- 確認連接埠 8080 與 7981 可用
- 檢查防毒軟體設定
- 如有需要，嘗試以系統管理員身分執行

**效能問題：**

- 若有延遲，縮短 Toast 顯示時間
- 確保 OBS 硬體加速已啟用
- 關閉不必要的應用程式

## 貢獻

歡迎貢獻！可隨時提交 Pull Request。重大更改請先開 issue 討論。

### 開發設定

1. Fork 儲存庫
2. 建立功能分支 (`git checkout -b feature/amazing-feature`)
3. 進行修改
4. 徹底測試
5. 提交變更 (`git commit -m 'Add amazing feature'`)
6. 推送分支 (`git push origin feature/amazing-feature`)
7. 開啟 Pull Request

## 授權

本專案採用 GPL-3 授權，詳見 LICENSE 文件。

## 致謝

- [egui](https://github.com/emilk/egui) 提供桌面 UI
- [Svelte](https://svelte.dev/) 與 [svelte-toast](https://github.com/zerodevx/svelte-toast) 提供瀏覽器通知
- [hyper](https://hyper.rs/) 提供內嵌 Web 伺服器支援

## 支援

如果您覺得本專案有幫助，請考慮：

- ⭐ 點星支持
- 🐛 透過 Issues 回報錯誤
- 💡 提出功能建議
- 📖 改進文件
- 😍 請我喝咖啡

------

**祝直播愉快！** 🎮✨
