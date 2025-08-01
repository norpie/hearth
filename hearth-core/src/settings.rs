use serde::{Deserialize, Serialize};
use crate::{Storage, StorageError};
use thiserror::Error;

#[cfg(target_arch = "wasm32")]
use web_sys;

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub version: u32,
    pub theme: Theme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: 1,
            theme: Theme::Dark,
        }
    }
}

pub struct SettingsManager {
    settings: Settings,
    storage: Storage,
}

impl SettingsManager {
    pub fn new() -> Self {
        match Self::load() {
            Ok(manager) => {
                log::info!("Settings loaded successfully");
                manager
            },
            Err(e) => {
                log::warn!("Failed to load settings ({}), using defaults", e);
                let manager = Self::default();
                if let Err(save_err) = manager.save() {
                    log::error!("Failed to save default settings: {}", save_err);
                } else {
                    log::info!("Default settings saved successfully");
                }
                manager
            }
        }
    }

    pub fn load() -> Result<Self, SettingsError> {
        let storage = Storage::new();
        let settings = Self::load_settings(&storage)?;
        Ok(Self { settings, storage })
    }

    fn load_settings(storage: &Storage) -> Result<Settings, SettingsError> {
        #[cfg(target_arch = "wasm32")]
        {
            // Web: Use localStorage
            let window = web_sys::window().ok_or_else(|| {
                SettingsError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No window object"
                )))
            })?;
            
            let local_storage = window.local_storage().map_err(|_| {
                SettingsError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to access localStorage"
                )))
            })?.ok_or_else(|| {
                SettingsError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "localStorage not available"
                )))
            })?;
            
            let storage_key = "hearth_settings";
            let content = local_storage.get_item(storage_key).map_err(|_| {
                SettingsError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to read from localStorage"
                )))
            })?.ok_or_else(|| {
                SettingsError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Settings not found in localStorage"
                )))
            })?;
            
            toml::from_str(&content).map_err(|e| SettingsError::Deserialization(e.to_string()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Native: Use file system
            let file_path = storage.get_file_path("settings.toml")?;
            if !file_path.exists() {
                return Err(SettingsError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Settings file does not exist"
                ))));
            }
            
            let content = std::fs::read_to_string(file_path)?;
            toml::from_str(&content).map_err(|e| SettingsError::Deserialization(e.to_string()))
        }
    }

    pub fn save(&self) -> Result<(), SettingsError> {
        #[cfg(target_arch = "wasm32")]
        {
            // Web: Use localStorage
            let window = web_sys::window().ok_or_else(|| {
                SettingsError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No window object"
                )))
            })?;
            
            let local_storage = window.local_storage().map_err(|_| {
                SettingsError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to access localStorage"
                )))
            })?.ok_or_else(|| {
                SettingsError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "localStorage not available"
                )))
            })?;
            
            let storage_key = "hearth_settings";
            let content = toml::to_string_pretty(&self.settings)
                .map_err(|e| SettingsError::Serialization(e.to_string()))?;
                
            local_storage.set_item(storage_key, &content).map_err(|_| {
                SettingsError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to write to localStorage"
                )))
            })?;
            
            Ok(())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Native: Use file system
            let file_path = self.storage.get_file_path("settings.toml")?;
            let content = toml::to_string_pretty(&self.settings)
                .map_err(|e| SettingsError::Serialization(e.to_string()))?;
            std::fs::write(file_path, content)?;
            Ok(())
        }
    }

    pub fn get(&self) -> &Settings {
        &self.settings
    }

    pub fn update(&mut self, settings: Settings) {
        log::debug!("Updating settings: {:?}", settings);
        self.settings = settings;
        match self.save() {
            Ok(()) => log::trace!("Settings saved successfully"),
            Err(e) => log::error!("Failed to save settings: {}", e),
        }
    }
}

impl Default for SettingsManager {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            storage: Storage::new(),
        }
    }
}