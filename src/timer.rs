use crate::config::{Config, ContentSwitchMode};
use std::time::{Duration, Instant};
use rand::Rng;

pub struct TimerService {
    pub interval: Duration,
    pub is_running: bool,
    pub content_index: usize,
    pub switch_mode: ContentSwitchMode,
    pub last_toast_time: Option<Instant>,
    pub next_toast_time: Option<Instant>,
}

impl TimerService {
    pub fn new(config: &Config) -> Self {
        Self {
            interval: Duration::from_secs(config.toaster.interval_time as u64 * 60),
            is_running: false,
            content_index: 0,
            switch_mode: config.toaster.content_switch_mode.clone(),
            last_toast_time: None,
            next_toast_time: None,
        }
    }
    
    pub fn start(&mut self) {
        self.is_running = true;
        let now = Instant::now();
        self.last_toast_time = Some(now);
        self.next_toast_time = Some(now + self.interval);
        log::info!("Timer service started");
    }
    
    pub fn stop(&mut self) {
        self.is_running = false;
        self.last_toast_time = None;
        self.next_toast_time = None;
        log::info!("Timer service stopped");
    }
    
    pub fn get_time_until_next_toast(&self) -> Option<Duration> {
        if !self.is_running {
            return None;
        }
        
        if let Some(next_time) = self.next_toast_time {
            let now = Instant::now();
            if next_time > now {
                Some(next_time - now)
            } else {
                Some(Duration::from_secs(0))
            }
        } else {
            None
        }
    }
    
    pub fn should_send_toast(&mut self) -> bool {
        if !self.is_running {
            return false;
        }
        
        if let Some(next_time) = self.next_toast_time {
            let now = Instant::now();
            if now >= next_time {
                // Update times for next toast
                self.last_toast_time = Some(now);
                self.next_toast_time = Some(now + self.interval);
                return true;
            }
        }
        
        false
    }
    
    pub fn _reset_timer(&mut self) {
        if self.is_running {
            let now = Instant::now();
            self.last_toast_time = Some(now);
            self.next_toast_time = Some(now + self.interval);
        }
    }
    
    pub fn get_next_content(&mut self, titles: &[String], contents: &[String]) -> (String, String) {
        if titles.is_empty() || contents.is_empty() {
            log::warn!("No titles or contents available, using defaults");
            return ("Default Title".to_string(), "Default Content".to_string());
        }
        
        let (title, content) = match self.switch_mode {
            ContentSwitchMode::Random => {
                let mut rng = rand::thread_rng();
                let title_idx = rng.gen_range(0..titles.len());
                let content_idx = rng.gen_range(0..contents.len());
                (titles[title_idx].clone(), contents[content_idx].clone())
            }
            ContentSwitchMode::Sequential => {
                let title = titles[self.content_index % titles.len()].clone();
                let content = contents[self.content_index % contents.len()].clone();
                self.content_index = (self.content_index + 1) % titles.len().max(contents.len());
                (title, content)
            }
        };
        
        log::debug!("Selected content - Title: {}, Content: {}", title, content);
        (title, content)
    }
    
    pub fn update_config(&mut self, config: &Config) {
        let new_interval = Duration::from_secs(config.toaster.interval_time as u64 * 60);
        
        // If interval changed and timer is running, adjust next toast time
        if self.interval != new_interval && self.is_running
            && let Some(last_time) = self.last_toast_time {
                self.next_toast_time = Some(last_time + new_interval);
            }
        
        self.interval = new_interval;
        self.switch_mode = config.toaster.content_switch_mode.clone();
        log::info!("Timer service configuration updated");
    }
}