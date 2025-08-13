mod config;
mod websocket;
mod timer;
mod app;
mod fonts;

use config::Config;
use app::ObsReminderApp;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    
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
            // Setup custom fonts
            fonts::setup_custom_fonts(&cc.egui_ctx);
            
            Ok(Box::new(ObsReminderApp::new(config, config_path.to_string())))
        }),
    )
}
