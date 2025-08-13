use crate::config::{Config, ContentSwitchMode};
use crate::timer::TimerService;
use crate::websocket::{WebSocketMessage, WebSocketServer};
use eframe::egui;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct ObsReminderApp {
    config: Config,
    config_path: String,

    // UI state
    new_title: String,
    new_content: String,

    // Services
    websocket_server: Option<Arc<WebSocketServer>>,
    websocket_sender: Option<broadcast::Sender<WebSocketMessage>>,
    timer_service: Option<TimerService>,

    // Status
    is_running: bool,
    connection_status: String,
    client_count: usize,

    // Test toast cooldown
    test_toast_cooldown: Option<std::time::Instant>,
}

impl ObsReminderApp {
    pub fn new(config: Config, config_path: String) -> Self {
        Self {
            timer_service: Some(TimerService::new(&config)),
            websocket_server: None,
            websocket_sender: None,
            config,
            config_path,
            new_title: String::new(),
            new_content: String::new(),
            is_running: false,
            connection_status: "Not Started".to_string(),
            client_count: 0,
            test_toast_cooldown: None,
        }
    }

    fn is_test_toast_on_cooldown(&self) -> bool {
        if let Some(cooldown_end) = self.test_toast_cooldown {
            std::time::Instant::now() < cooldown_end
        } else {
            false
        }
    }

    fn get_test_toast_cooldown_remaining(&self) -> Option<std::time::Duration> {
        if let Some(cooldown_end) = self.test_toast_cooldown {
            let now = std::time::Instant::now();
            if cooldown_end > now {
                Some(cooldown_end - now)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn initialize_websocket(&mut self) {
        if self.websocket_server.is_none() {
            let websocket_server = Arc::new(WebSocketServer::new());
            let websocket_sender = websocket_server.get_sender();

            // Start WebSocket server in background
            let server_clone = websocket_server.clone();
            tokio::spawn(async move {
                if let Err(e) = server_clone.start().await {
                    log::error!("WebSocket server error: {}", e);
                }
            });

            self.websocket_server = Some(websocket_server);
            self.websocket_sender = Some(websocket_sender);
            self.connection_status = "WebSocket Started".to_string();
            log::info!("WebSocket server initialized");
        }
    }

    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.heading("OBS Reminder v0.0.1");

        ui.separator();
    }

    fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");

        // Toaster titles section
        ui.label("toaster-title: (allow multi)");

        // Display existing titles
        let mut titles_to_remove = Vec::new();
        for (i, title) in self.config.toaster.titles.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(title);
                if ui.button("Remove").clicked() {
                    titles_to_remove.push(i);
                }
            });
        }

        // Remove titles marked for deletion (in reverse order to maintain indices)
        for &i in titles_to_remove.iter().rev() {
            self.config.toaster.titles.remove(i);
        }

        // Add new title
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.new_title);
            if ui.button("Add Title").clicked() && !self.new_title.is_empty() {
                self.config.toaster.titles.push(self.new_title.clone());
                self.new_title.clear();
            }
        });

        ui.separator();

        // Toaster contents section
        ui.label("toaster-content: (allow multi)");

        // Display existing contents
        let mut contents_to_remove = Vec::new();
        for (i, content) in self.config.toaster.contents.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(content);
                if ui.button("Remove").clicked() {
                    contents_to_remove.push(i);
                }
            });
        }

        // Remove contents marked for deletion
        for &i in contents_to_remove.iter().rev() {
            self.config.toaster.contents.remove(i);
        }

        // Add new content
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.new_content);
            if ui.button("Add Content").clicked() && !self.new_content.is_empty() {
                self.config.toaster.contents.push(self.new_content.clone());
                self.new_content.clear();
            }
        });

        ui.separator();

        // Interval time
        ui.horizontal(|ui| {
            ui.label("interval time: (min)");
            ui.add(egui::DragValue::new(&mut self.config.toaster.interval_time).range(1..=1440));
        });

        // Duration
        ui.horizontal(|ui| {
            ui.label("toast duration: (sec)");
            ui.add(egui::DragValue::new(&mut self.config.toaster.duration).range(1..=60));
        });

        ui.separator();

        // Colors
        ui.horizontal(|ui| {
            ui.label("toaster-color #1:");

            // Convert hex string to Color32
            let mut color1 = hex_to_color32(&self.config.toaster.color_1);

            if ui.color_edit_button_srgba(&mut color1).changed() {
                self.config.toaster.color_1 = color32_to_hex(color1);
            }

            ui.label(&self.config.toaster.color_1);
        });

        ui.horizontal(|ui| {
            ui.label("toaster-color #2:");

            // Convert hex string to Color32
            let mut color2 = hex_to_color32(&self.config.toaster.color_2);

            if ui.color_edit_button_srgba(&mut color2).changed() {
                self.config.toaster.color_2 = color32_to_hex(color2);
            }

            ui.label(&self.config.toaster.color_2);
        });

        ui.separator();

        // Content switch mode
        ui.horizontal(|ui| {
            ui.label("content-switch-mode:");
            egui::ComboBox::from_id_salt("switch_mode")
                .selected_text(match self.config.toaster.content_switch_mode {
                    ContentSwitchMode::Random => "random",
                    ContentSwitchMode::Sequential => "sequential",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.config.toaster.content_switch_mode,
                        ContentSwitchMode::Random,
                        "random",
                    );
                    ui.selectable_value(
                        &mut self.config.toaster.content_switch_mode,
                        ContentSwitchMode::Sequential,
                        "sequential",
                    );
                });
        });
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.separator();

        ui.horizontal(|ui| {
            let start_button = egui::Button::new("Start").fill(if self.is_running {
                egui::Color32::GRAY
            } else {
                egui::Color32::from_rgb(0, 150, 0)
            });

            if ui.add_enabled(!self.is_running, start_button).clicked() {
                self.start_service();
            }

            let stop_button = egui::Button::new("Stop").fill(if !self.is_running {
                egui::Color32::GRAY
            } else {
                egui::Color32::from_rgb(150, 0, 0)
            });

            if ui.add_enabled(self.is_running, stop_button).clicked() {
                self.stop_service();
            }

            // Test Toast button with cooldown
            let test_button_enabled = !self.is_test_toast_on_cooldown();
            let test_button_text = if let Some(remaining) = self.get_test_toast_cooldown_remaining()
            {
                format!("Test Toast ({}s)", remaining.as_secs() + 1)
            } else {
                "Test Toast".to_string()
            };

            if ui
                .add_enabled(test_button_enabled, egui::Button::new(test_button_text))
                .clicked()
            {
                self.send_test_toast();
            }

            if ui.button("Save").clicked() {
                self.save_configuration();
            }
        });

        ui.separator();

        // Status display
        ui.horizontal(|ui| {
            ui.label("Status:");
            let status_color = if self.is_running {
                egui::Color32::from_rgb(0, 150, 0)
            } else {
                egui::Color32::from_rgb(150, 0, 0)
            };
            ui.colored_label(
                status_color,
                if self.is_running {
                    "Running"
                } else {
                    "Stopped"
                },
            );

            ui.separator();

            ui.label("WebSocket:");
            let ws_color = if self.client_count > 0 {
                egui::Color32::from_rgb(0, 150, 0)
            } else {
                egui::Color32::from_rgb(150, 150, 0)
            };
            ui.colored_label(
                ws_color,
                format!("Port 7981 ({} clients)", self.client_count),
            );
        });

        // Timer countdown display
        if self.is_running {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Next Toast:");
                if let Some(ref timer) = self.timer_service {
                    if let Some(time_left) = timer.get_time_until_next_toast() {
                        let total_seconds = time_left.as_secs();
                        let minutes = total_seconds / 60;
                        let seconds = total_seconds % 60;

                        let countdown_text = if total_seconds == 0 {
                            "Sending now...".to_string()
                        } else {
                            format!("{}:{:02}", minutes, seconds)
                        };

                        let countdown_color = if total_seconds <= 10 {
                            egui::Color32::from_rgb(255, 100, 100) // Red when close
                        } else if total_seconds <= 30 {
                            egui::Color32::from_rgb(255, 200, 100) // Orange when getting close
                        } else {
                            egui::Color32::from_rgb(100, 200, 100) // Green when plenty of time
                        };

                        ui.colored_label(countdown_color, countdown_text);
                    } else {
                        ui.colored_label(egui::Color32::GRAY, "No timer active");
                    }
                } else {
                    ui.colored_label(egui::Color32::GRAY, "Timer not available");
                }
            });
        }
    }

    fn start_service(&mut self) {
        if let Err(e) = self.config.validate() {
            log::error!("Configuration validation failed: {}", e);
            return;
        }

        // Initialize WebSocket server if not already done
        self.initialize_websocket();

        self.is_running = true;

        // Update timer service with current config
        if let Some(ref mut timer) = self.timer_service {
            timer.update_config(&self.config);
            timer.start();
        }

        self.connection_status = "Running".to_string();
        log::info!("Service started");
    }

    fn stop_service(&mut self) {
        self.is_running = false;

        // Stop timer service
        if let Some(ref mut timer) = self.timer_service {
            timer.stop();
        }

        self.connection_status = "Stopped".to_string();
        log::info!("Service stopped");
    }

    fn send_test_toast(&mut self) {
        if self.config.toaster.titles.is_empty() || self.config.toaster.contents.is_empty() {
            log::warn!("Cannot send test toast: no titles or contents configured");
            return;
        }

        // Initialize WebSocket server if not already done
        self.initialize_websocket();

        let title = &self.config.toaster.titles[0];
        let content = &self.config.toaster.contents[0];

        log::info!(
            "Sending test toast - Title: {}, Content: {}",
            title,
            content
        );

        if let Some(ref sender) = self.websocket_sender {
            let message = WebSocketMessage::new_toast(
                title.clone(),
                content.clone(),
                self.config.toaster.color_1.clone(),
                self.config.toaster.color_2.clone(),
                self.config.toaster.duration,
            );

            if let Err(e) = sender.send(message) {
                log::error!("Failed to send test toast: {}", e);
            } else {
                log::info!("Test toast sent successfully");

                // Set cooldown based on duration setting + 0.3 seconds
                let cooldown_duration =
                    std::time::Duration::from_secs(self.config.toaster.duration as u64)
                        + std::time::Duration::from_millis(300);
                self.test_toast_cooldown = Some(std::time::Instant::now() + cooldown_duration);
            }
        } else {
            log::error!("WebSocket sender not available");
        }
    }

    fn send_automatic_toast(&mut self) {
        if self.config.toaster.titles.is_empty() || self.config.toaster.contents.is_empty() {
            log::warn!("Cannot send automatic toast: no titles or contents configured");
            return;
        }

        // Get next content from timer service
        let (title, content) = if let Some(ref mut timer) = self.timer_service {
            timer.get_next_content(&self.config.toaster.titles, &self.config.toaster.contents)
        } else {
            return;
        };

        log::info!(
            "Sending automatic toast - Title: {}, Content: {}",
            title,
            content
        );

        if let Some(ref sender) = self.websocket_sender {
            let message = WebSocketMessage::new_toast(
                title,
                content,
                self.config.toaster.color_1.clone(),
                self.config.toaster.color_2.clone(),
                self.config.toaster.duration,
            );

            if let Err(e) = sender.send(message) {
                log::error!("Failed to send automatic toast: {}", e);
            } else {
                log::info!("Automatic toast sent successfully");
            }
        } else {
            log::error!("WebSocket sender not available for automatic toast");
        }
    }

    fn save_configuration(&mut self) {
        match self.config.save_to_file(&self.config_path) {
            Ok(_) => {
                log::info!("Configuration saved successfully to {}", self.config_path);
            }
            Err(e) => {
                log::error!("Failed to save configuration: {}", e);
            }
        }
    }
}

impl eframe::App for ObsReminderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Clear expired test toast cooldown
        if let Some(cooldown_end) = self.test_toast_cooldown {
            if std::time::Instant::now() >= cooldown_end {
                self.test_toast_cooldown = None;
            }
        }

        // Check if it's time to send a toast
        if self.is_running {
            if let Some(ref mut timer) = self.timer_service {
                if timer.should_send_toast() {
                    self.send_automatic_toast();
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                self.render_header(ui);
                self.render_settings(ui);
                self.render_controls(ui);
            });
        });

        // Request repaint to keep UI responsive (more frequent for countdown)
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

// Helper functions for color conversion
fn hex_to_color32(hex: &str) -> egui::Color32 {
    if hex.len() != 7 || !hex.starts_with('#') {
        return egui::Color32::from_rgb(255, 107, 107); // Default fallback color
    }

    let hex = &hex[1..]; // Remove '#'
    if let Ok(r) = u8::from_str_radix(&hex[0..2], 16) {
        if let Ok(g) = u8::from_str_radix(&hex[2..4], 16) {
            if let Ok(b) = u8::from_str_radix(&hex[4..6], 16) {
                return egui::Color32::from_rgb(r, g, b);
            }
        }
    }

    egui::Color32::from_rgb(255, 107, 107) // Default fallback color
}

fn color32_to_hex(color: egui::Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b())
}
