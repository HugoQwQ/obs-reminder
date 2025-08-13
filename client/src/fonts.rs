use eframe::egui;
use std::sync::Arc;

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Try multiple possible font file names and paths
    let font_paths = [
        "client/fonts/MiSans-Regular.ttf",
        "client/fonts/MiSans.ttf",
        "fonts/MiSans-Regular.ttf", 
        "fonts/MiSans.ttf",
        "./client/fonts/MiSans-Regular.ttf",
        "./client/fonts/MiSans.ttf"
    ];

    let mut font_loaded = false;
    
    for path in &font_paths {
        match std::fs::read(path) {
            Ok(font_data) => {
                log::info!("Successfully read font file from: {}", path);
                log::info!("Font file size: {} bytes", font_data.len());
                
                fonts.font_data.insert(
                    "MiSans".to_owned(),
                    Arc::new(egui::FontData::from_owned(font_data)),
                );

                // Set MiSans as the primary font for proportional text
                fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, "MiSans".to_owned());

                log::info!("MiSans font loaded and configured successfully from {}", path);
                font_loaded = true;
                break;
            }
            Err(e) => {
                log::debug!("Font not found at {}: {}", path, e);
            }
        }
    }
    
    if !font_loaded {
        log::warn!("Failed to load MiSans font from any path, using default fonts");
        log::warn!("Tried paths: {:?}", font_paths);
        
        // Check if the directory exists
        if let Ok(entries) = std::fs::read_dir("client/fonts") {
            log::info!("Files in client/fonts directory:");
            for entry in entries {
                if let Ok(entry) = entry {
                    log::info!("  - {}", entry.file_name().to_string_lossy());
                }
            }
        } else {
            log::warn!("client/fonts directory not found or not accessible");
        }
    }

    ctx.set_fonts(fonts);
    
    // Force a repaint to apply the new fonts immediately
    ctx.request_repaint();
    
    log::info!("Font configuration applied to egui context");
}
