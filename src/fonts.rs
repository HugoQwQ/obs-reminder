use eframe::egui;
use rust_embed::RustEmbed;
use std::sync::Arc;

#[derive(RustEmbed)]
#[folder = "fonts/"]
struct Assets;

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    if let Some(font_file) = Assets::get("MiSans-Regular.ttf") {
        let font_data = font_file.data.into_owned();
        fonts.font_data.insert(
            "MiSans".to_owned(),
            Arc::new(egui::FontData::from_owned(font_data)),
        );

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "MiSans".to_owned());

        log::info!("Font loaded successfully");
    } else {
        log::warn!("Font load failed, using default fonts");
    }

    ctx.set_fonts(fonts);
    ctx.request_repaint();
}
