#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;
mod config;
mod fonts;
mod http_server;
mod timer;
mod websocket;

use app::ObsReminderApp;
use catppuccin_egui;
use config::Config;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config_path = "config.toml";
    let config = Config::load_or_create_default(config_path);

    log::info!("OBS Reminder v{} starting", config.app.version);
    log::info!("Configuration loaded successfully");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_min_inner_size([350.0, 500.0])
            .with_resizable(true),
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
