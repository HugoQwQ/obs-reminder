use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub toaster: ToasterConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToasterConfig {
    pub titles: Vec<String>,
    pub contents: Vec<String>,
    pub interval_time: u32, // minutes
    pub duration: u32, // seconds
    pub color_1: String, // hex color
    pub color_2: String, // hex color
    pub text_color: String, // hex color for text
    pub content_switch_mode: ContentSwitchMode,
    pub enable_sound: bool, // enable/disable sound notifications
    pub sound_file_id: Option<String>, // UUID of the cached sound file
    pub sound_file_name: Option<String>, // original filename for display
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentSwitchMode {
    #[serde(rename = "random")]
    Random,
    #[serde(rename = "sequential")]
    Sequential,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app: AppConfig {
                version: "0.0.1".to_string(),
            },
            toaster: ToasterConfig {
                titles: vec!["提醒标题1".to_string(), "提醒标题2".to_string()],
                contents: vec!["提醒内容1".to_string(), "提醒内容2".to_string()],
                interval_time: 5,
                duration: 5, // 5 seconds default
                color_1: "#FF6B6B".to_string(),
                color_2: "#4ECDC4".to_string(),
                text_color: "#FFFFFF".to_string(), // white text default
                content_switch_mode: ContentSwitchMode::Random,
                enable_sound: false, // disabled by default
                sound_file_id: None, // no custom sound file by default
                sound_file_name: None, // no filename by default
            },
        }
    }
}

impl Config {
    pub fn validate(&self) -> Result<(), String> {
        // Validate titles and contents are not empty
        if self.toaster.titles.is_empty() {
            return Err("At least one title is required".to_string());
        }
        
        if self.toaster.contents.is_empty() {
            return Err("At least one content is required".to_string());
        }
        
        // Validate interval time is reasonable (1-1440 minutes = 1 day)
        if self.toaster.interval_time == 0 || self.toaster.interval_time > 1440 {
            return Err("Interval time must be between 1 and 1440 minutes".to_string());
        }
        
        // Validate duration is reasonable (1-60 seconds)
        if self.toaster.duration == 0 || self.toaster.duration > 60 {
            return Err("Toast duration must be between 1 and 60 seconds".to_string());
        }
        
        // Validate hex colors
        if !is_valid_hex_color(&self.toaster.color_1) {
            return Err("Invalid hex color format for color_1".to_string());
        }
        
        if !is_valid_hex_color(&self.toaster.color_2) {
            return Err("Invalid hex color format for color_2".to_string());
        }
        
        if !is_valid_hex_color(&self.toaster.text_color) {
            return Err("Invalid hex color format for text_color".to_string());
        }
        
        Ok(())
    }
    
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }
    
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        self.validate()?;
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(path, toml_string)?;
        Ok(())
    }
    
    pub fn load_or_create_default<P: AsRef<Path>>(path: P) -> Self {
        match Self::load_from_file(&path) {
            Ok(config) => {
                log::info!("Configuration loaded from file");
                config
            }
            Err(e) => {
                log::warn!("Failed to load config: {}. Using default configuration.", e);
                let default_config = Self::default();
                if let Err(save_err) = default_config.save_to_file(&path) {
                    log::error!("Failed to save default configuration: {}", save_err);
                }
                default_config
            }
        }
    }
}

fn is_valid_hex_color(color: &str) -> bool {
    if !color.starts_with('#') || color.len() != 7 {
        return false;
    }
    
    color[1..].chars().all(|c| c.is_ascii_hexdigit())
}