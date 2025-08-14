# OBS Reminder

*Other language versions: [简体中文](README-CN.md) | [繁體中文](README-TW.md)*

A lightweight toast notification tool for OBS Studio. Perfect for users who need periodic reminders during live streaming or recording.

## Screenshots

[see in here](screenshots.md)

## Features

- **Light, fast, and simple**: Built with Rust and egui for an easy-to-use interface
- **Multi-software integration**: Works with any streaming or recording tool that supports browser sources (even browsers)
- **Highly customizable settings**: Configure multiple titles, messages, colors, display durations, etc.
- **Live preview**: Test toast notifications in real time

## Next Steps

- [ ] Add Windows system tray
- [ ] Improve visual clarity
- [ ] Add sound notifications

## Quick Start

### 1. Download and Run

1. Download the latest version from the release page
2. Run `obs-reminder-client.exe`
3. The application will launch both the desktop interface and the embedded web server

### 2. Configure Reminders

1. **Add Titles**: Input multiple reminder titles (e.g., "Drink Water!", "Check Chat", "Stretch")
2. **Add Messages**: Input corresponding message content
3. **Set Colors**: Choose background gradient and text colors
4. **Set Duration**: Configure how long each toast displays (1–60 seconds)
5. **Set Interval**: Choose reminder frequency (1–1440 minutes)
6. **Select Mode**: Random or sequential display order

### 3. Add to OBS

1. In OBS Studio, add a new **Browser Source**
2. Copy the URL from the desktop app (usually `http://localhost:8080`)
3. Paste it into the Browser Source URL field
4. Set width: `265`, height: `85` (or adjust as needed)
5. Enable "Shutdown source when not visible" and "Refresh browser when scene becomes active"

### 4. Start Reminders

1. Click **Save** to save your configuration
2. Click **Test Toast** to preview in OBS
3. Click **Start** to begin automatic reminders
4. Click **Stop** anytime to pause

## Configuration

Settings are saved in `config.toml` in the same directory. You can also edit it directly:

```toml
[app]
version = "0.0.1"

[toaster]
titles = ["bbb", "114514", "alright"]
contents = ["im content", "hoho", "what can i say?"]
interval_time = 30  # minutes
duration = 5        # seconds
color_1 = "#FF6B6B"
color_2 = "#4ECDC4"
text_color = "#FFFFFF"
content_switch_mode = "random"  # or "sequential"
```

## Technical Details

### Architecture

* **Desktop App**: Rust + egui for UI
* **Web Server**: Embedded HTTP server using hyper-rs
* **Browser Component**: Svelte + svelte-toast for notifications
* **Communication**: WebSocket for real-time messages

### Ports

* **HTTP Server**: `localhost:8080` (for OBS browser source)
* **WebSocket**: `localhost:7981` (internal communication)

### System Requirements

* Windows 10/11 (64-bit)
* OBS Studio 28.0+ (supports browser source)
* Available ports 8080 and 7981

## Build from Source

### Prerequisites

* Latest Rust
* Node.js 22+ and pnpm (or other package manager)
* Git

### Build Steps

Note: Method one Windows must install 7zip
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

OR

```bash
# Clone repository
git clone https://github.com/HugoQwQ/obs-reminder.git
cd obs-reminder

# Build browser component
cd browser
pnpm install
pnpm run build
cd ..

# Build Rust application
cargo build --release

# Executable located at target/release/obs-reminder-client.exe
```

## Troubleshooting

### Common Issues

**Toast not showing in OBS:**

* Verify browser source URL is `http://localhost:8080`
* Ensure desktop app is running
* Try refreshing the browser source
* Make sure Windows Firewall is not blocking the app

**Connection issues:**

* Ensure ports 8080 and 7981 are available
* Check Windows Defender/antivirus settings
* Try running as administrator if needed

**Performance issues:**

* Reduce toast display duration if there is lag
* Ensure OBS hardware acceleration is enabled
* Close unnecessary applications

## Contributing

Contributions are welcome! Feel free to submit pull requests. For major changes, please open an issue first.

### Development Setup

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Test thoroughly
5. Commit (`git commit -m 'Add amazing feature'`)
6. Push branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

This project is licensed under GPL-3. See [LICENSE](LICENSE) for details.

## Acknowledgements

* [egui](https://github.com/emilk/egui) for desktop UI
* [Svelte](https://svelte.dev/) and [svelte-toast](https://github.com/zerodevx/svelte-toast) for browser notifications
* [hyper](https://hyper.rs/) for embedded web server support

## Support

If you find this project helpful, please consider:

* ⭐ Starring the repository
* 🐛 Reporting issues
* 💡 Suggesting features
* 📖 Improving documentation
* 😍 Buy me coffee

---

**Happy Streaming!** 🎮✨
