use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug)]
pub enum AudioError {
    IoError(std::io::Error),
}

impl std::fmt::Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for AudioError {}

impl From<std::io::Error> for AudioError {
    fn from(error: std::io::Error) -> Self {
        AudioError::IoError(error)
    }
}

pub struct AudioManager {
    cache_dir: PathBuf,
}

impl AudioManager {
    pub fn new() -> Result<Self, AudioError> {
        let cache_dir = PathBuf::from("audio_cache");

        // Create cache directory if it doesn't exist
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)?;
        }

        Ok(Self { cache_dir })
    }

    pub fn add_audio_file<P: AsRef<Path>>(&self, source_path: P) -> Result<String, AudioError> {
        let source_path = source_path.as_ref();

        // Generate UUID for the file
        let file_id = Uuid::new_v4().to_string();

        // Get file extension
        let extension = source_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("mp3");

        // Create destination path with UUID
        let dest_filename = format!("{}.{}", file_id, extension);
        let dest_path = self.cache_dir.join(&dest_filename);

        // Copy file to cache directory
        fs::copy(source_path, &dest_path)?;

        log::info!(
            "Audio file cached: {} -> {}",
            source_path.display(),
            dest_path.display()
        );

        Ok(file_id)
    }

    pub fn get_audio_file_path(&self, file_id: &str) -> Option<PathBuf> {
        // Try different extensions
        let extensions = ["mp3", "wav", "ogg", "m4a"];

        for ext in &extensions {
            let filename = format!("{}.{}", file_id, ext);
            let path = self.cache_dir.join(&filename);
            if path.exists() {
                return Some(path);
            }
        }

        None
    }

    pub fn remove_audio_file(&self, file_id: &str) -> Result<(), AudioError> {
        if let Some(path) = self.get_audio_file_path(file_id) {
            fs::remove_file(&path)?;
            log::info!("Audio file removed: {}", path.display());
        }
        Ok(())
    }

    pub fn _cleanup_unused_files(&self, used_ids: &[String]) -> Result<(), AudioError> {
        if !self.cache_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file()
                && let Some(filename) = path.file_stem().and_then(|s| s.to_str())
                && !used_ids.contains(&filename.to_string())
            {
                fs::remove_file(&path)?;
                log::info!("Cleaned up unused audio file: {}", path.display());
            }
        }

        Ok(())
    }
}
