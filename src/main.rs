#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod audio_manager;
mod config;
mod fonts;
mod http_server;
mod timer;
mod websocket;

use app::ObsReminderApp;
use config::Config;

use env_logger::Env;
use eframe::egui;
use egui::viewport::IconData;
use std::io::Cursor;

fn load_icon() -> IconData {
    let icon_bytes = include_bytes!("../icon.ico");
    let image = image::load(Cursor::new(icon_bytes), image::ImageFormat::Ico)
        .expect("Failed to load icon")
        .into_rgba8();

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    IconData { rgba, width, height }
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config_path = "config.toml";
    let config = Config::load_or_create_default(config_path);
    let icon = load_icon();

    log::info!("OBS Reminder v{} starting", std::env!("CARGO_PKG_VERSION"));
    log::info!("Configuration loaded successfully");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 650.0])
            .with_min_inner_size([350.0, 500.0])
            .with_resizable(true)
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "OBS Reminder",
        options,
        Box::new(|cc| {
            fonts::setup_custom_fonts(&cc.egui_ctx);

            catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MOCHA);
            Ok(Box::new(ObsReminderApp::new(
                config,
                config_path.to_string(),
            )))
        }),
    )
}
