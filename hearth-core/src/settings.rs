use crate::{Storage, StorageError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
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
    #[error("Backend not found: {0}")]
    BackendNotFound(String),
}

pub type BackendId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub version: u32,
    pub selected_backend: Option<BackendId>, // None = local mode
    pub local_backend: Option<LocalBackendConfig>,
    pub remote_backends: Vec<RemoteBackendConfig>,
    
    // User preferences
    pub theme: Theme,
    pub ui_preferences: UiPreferences,
    pub chat_preferences: ChatPreferences,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalBackendConfig {
    pub database_path: Option<PathBuf>, // None = default path
    pub llm_providers: Vec<LlmProviderConfig>,
    pub selected_llm_provider: Option<String>, // ID of selected provider
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoteBackendConfig {
    pub id: BackendId,
    pub name: String,
    pub url: String,
    pub auth_token: Option<String>,
    pub last_connected: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LlmProviderConfig {
    pub id: String,
    pub name: String,
    pub provider_type: LlmProviderType,
    pub config: LlmProviderSettings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LlmProviderType {
    Ollama,
    OpenAI,
    Anthropic,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LlmProviderSettings {
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub model: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiPreferences {
    pub message_timestamps: bool,
    pub typing_indicators: bool,
    pub compact_mode: bool,
    pub sidebar_collapsed: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatPreferences {
    pub auto_scroll: bool,
    pub sound_notifications: bool,
    pub message_grouping: bool,
    pub show_word_count: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    Auto, // Follow system theme
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            version: 1,
            selected_backend: None, // Local mode by default
            local_backend: Some(LocalBackendConfig::default()),
            remote_backends: Vec::new(),
            theme: Theme::Dark,
            ui_preferences: UiPreferences::default(),
            chat_preferences: ChatPreferences::default(),
        }
    }
}

impl Default for LocalBackendConfig {
    fn default() -> Self {
        Self {
            database_path: None, // Use default path
            llm_providers: vec![
                // Default Ollama provider
                LlmProviderConfig {
                    id: "ollama_default".to_string(),
                    name: "Ollama (Local)".to_string(),
                    provider_type: LlmProviderType::Ollama,
                    config: LlmProviderSettings {
                        base_url: Some("http://localhost:11434".to_string()),
                        api_key: None,
                        model: "llama3.1:8b".to_string(),
                        max_tokens: Some(2048),
                        temperature: Some(0.7),
                    },
                },
            ],
            selected_llm_provider: Some("ollama_default".to_string()),
        }
    }
}

impl Default for UiPreferences {
    fn default() -> Self {
        Self {
            message_timestamps: true,
            typing_indicators: true,
            compact_mode: false,
            sidebar_collapsed: false,
        }
    }
}

impl Default for ChatPreferences {
    fn default() -> Self {
        Self {
            auto_scroll: true,
            sound_notifications: false,
            message_grouping: true,
            show_word_count: false,
        }
    }
}

#[derive(Default)]
pub struct SettingsManager {
    settings: AppSettings,
    storage: Storage,
}

impl SettingsManager {
    pub fn new() -> Self {
        match Self::load() {
            Ok(manager) => {
                log::info!("Settings loaded successfully");
                manager
            }
            Err(e) => {
                log::warn!("Failed to load settings ({e}), using defaults");
                let manager = Self::default();
                if let Err(save_err) = manager.save() {
                    log::error!("Failed to save default settings: {save_err}");
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

    fn load_settings(storage: &Storage) -> Result<AppSettings, SettingsError> {
        #[cfg(target_arch = "wasm32")]
        {
            // Web: Use localStorage
            let window = web_sys::window().ok_or_else(|| {
                SettingsError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No window object",
                )))
            })?;

            let local_storage = window
                .local_storage()
                .map_err(|_| {
                    SettingsError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to access localStorage",
                    )))
                })?
                .ok_or_else(|| {
                    SettingsError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "localStorage not available",
                    )))
                })?;

            let storage_key = "hearth_settings";
            let content = local_storage
                .get_item(storage_key)
                .map_err(|_| {
                    SettingsError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to read from localStorage",
                    )))
                })?
                .ok_or_else(|| {
                    SettingsError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Settings not found in localStorage",
                    )))
                })?;

            toml::from_str(&content).map_err(|e| SettingsError::Deserialization(e.to_string()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Native: Use file system
            let file_path = storage.get_file_path("settings.toml")?;
            if !file_path.exists() {
                return Err(SettingsError::Storage(StorageError::Io(
                    std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Settings file does not exist",
                    ),
                )));
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
                    "No window object",
                )))
            })?;

            let local_storage = window
                .local_storage()
                .map_err(|_| {
                    SettingsError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to access localStorage",
                    )))
                })?
                .ok_or_else(|| {
                    SettingsError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "localStorage not available",
                    )))
                })?;

            let storage_key = "hearth_settings";
            let content = toml::to_string_pretty(&self.settings)
                .map_err(|e| SettingsError::Serialization(e.to_string()))?;

            local_storage.set_item(storage_key, &content).map_err(|_| {
                SettingsError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to write to localStorage",
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


    pub fn get(&self) -> &AppSettings {
        &self.settings
    }

    pub fn update(&mut self, settings: AppSettings) {
        log::debug!("Updating settings");
        self.settings = settings;
        match self.save() {
            Ok(()) => log::trace!("Settings saved successfully"),
            Err(e) => log::error!("Failed to save settings: {e}"),
        }
    }

    // Convenience methods for common operations
    pub fn get_selected_backend(&self) -> &Option<BackendId> {
        &self.settings.selected_backend
    }

    pub fn set_selected_backend(&mut self, backend_id: Option<BackendId>) {
        self.settings.selected_backend = backend_id;
        if let Err(e) = self.save() {
            log::error!("Failed to save backend selection: {e}");
        }
    }

    pub fn get_remote_backend(&self, id: &str) -> Option<&RemoteBackendConfig> {
        self.settings.remote_backends.iter().find(|b| b.id == id)
    }

    pub fn add_remote_backend(&mut self, backend: RemoteBackendConfig) {
        // Remove existing backend with same ID
        self.settings.remote_backends.retain(|b| b.id != backend.id);
        self.settings.remote_backends.push(backend);
        if let Err(e) = self.save() {
            log::error!("Failed to save remote backend: {e}");
        }
    }

    pub fn remove_remote_backend(&mut self, id: &str) {
        self.settings.remote_backends.retain(|b| b.id != id);
        // If this was the selected backend, switch to local
        if self.settings.selected_backend.as_ref() == Some(&id.to_string()) {
            self.settings.selected_backend = None;
        }
        if let Err(e) = self.save() {
            log::error!("Failed to remove remote backend: {e}");
        }
    }

    pub fn update_theme(&mut self, theme: Theme) {
        self.settings.theme = theme;
        if let Err(e) = self.save() {
            log::error!("Failed to save theme: {e}");
        }
    }
}
