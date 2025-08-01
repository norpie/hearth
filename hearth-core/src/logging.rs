//! Cross-platform logging system
//!
//! Implements the standard Rust `log` crate with persistent storage and UI access.

use crate::{Storage, StorageError};
use log::{Level, Metadata, Record};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, OnceLock};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoggingError {
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Lock error: {0}")]
    Lock(String),
    #[error("Log error: {0}")]
    Log(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub target: String,
    pub message: String,
}

impl LogEntry {
    pub fn from_record(record: &Record) -> Self {
        let timestamp = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string();
        Self {
            timestamp,
            level: record.level().to_string(),
            target: record.target().to_string(),
            message: record.args().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogConfig {
    pub max_entries: usize,
    pub min_level: String, // Store as string for serialization
    pub persist_logs: bool,
    pub page_size: usize, // Number of logs to load per page in UI
}

impl LogConfig {
    pub fn min_level(&self) -> Level {
        match self.min_level.as_str() {
            "ERROR" => Level::Error,
            "WARN" => Level::Warn,
            "INFO" => Level::Info,
            "DEBUG" => Level::Debug,
            "TRACE" => Level::Trace,
            _ => Level::Info,
        }
    }

    pub fn set_min_level(&mut self, level: Level) {
        self.min_level = level.to_string();
    }
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            max_entries: 1000,
            min_level: "INFO".to_string(),
            persist_logs: true,
            page_size: 50, // Default to loading 50 logs at a time
        }
    }
}

/// Cross-platform logger that implements the `log::Log` trait
pub struct HearthLogger {
    entries: Arc<Mutex<VecDeque<LogEntry>>>,
    config: Arc<Mutex<LogConfig>>,
    storage: Storage,
}

impl Default for HearthLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl HearthLogger {
    pub fn new() -> Self {
        let storage = Storage::new();
        let config = Self::load_config(&storage).unwrap_or_default();
        let max_entries = config.max_entries;
        let entries = Arc::new(Mutex::new(VecDeque::with_capacity(max_entries)));

        let logger = Self {
            entries,
            config: Arc::new(Mutex::new(config)),
            storage,
        };

        // Load persisted logs (keep only the most recent ones)
        if let Ok(persisted_entries) = logger.load_persisted_logs() {
            if let Ok(mut entries) = logger.entries.lock() {
                // If we have more persisted entries than max, take only the last ones
                let entries_to_load = if persisted_entries.len() > max_entries {
                    &persisted_entries[persisted_entries.len() - max_entries..]
                } else {
                    &persisted_entries[..]
                };

                for entry in entries_to_load {
                    entries.push_back(entry.clone());
                }
            }
        }

        logger
    }

    fn load_config(storage: &Storage) -> Result<LogConfig, LoggingError> {
        #[cfg(target_arch = "wasm32")]
        {
            // Web: Use localStorage
            let window = web_sys::window().ok_or_else(|| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No window object",
                )))
            })?;

            let local_storage = window
                .local_storage()
                .map_err(|_| {
                    LoggingError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to access localStorage",
                    )))
                })?
                .ok_or_else(|| {
                    LoggingError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "localStorage not available",
                    )))
                })?;

            let storage_key = "hearth_log_config";
            let content = local_storage
                .get_item(storage_key)
                .map_err(|_| {
                    LoggingError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to read from localStorage",
                    )))
                })?
                .ok_or_else(|| {
                    LoggingError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Log config not found in localStorage",
                    )))
                })?;

            toml::from_str(&content).map_err(|e| LoggingError::Serialization(e.to_string()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Native: Use file system
            let file_path = storage.get_file_path("log_config.toml")?;
            if !file_path.exists() {
                return Err(LoggingError::Storage(StorageError::Io(
                    std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Log config file does not exist",
                    ),
                )));
            }

            let content = std::fs::read_to_string(file_path)?;
            toml::from_str(&content).map_err(|e| LoggingError::Serialization(e.to_string()))
        }
    }

    fn save_config(&self, config: &LogConfig) -> Result<(), LoggingError> {
        #[cfg(target_arch = "wasm32")]
        {
            // Web: Use localStorage
            let window = web_sys::window().ok_or_else(|| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No window object",
                )))
            })?;

            let local_storage = window
                .local_storage()
                .map_err(|_| {
                    LoggingError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to access localStorage",
                    )))
                })?
                .ok_or_else(|| {
                    LoggingError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "localStorage not available",
                    )))
                })?;

            let storage_key = "hearth_log_config";
            let content = toml::to_string_pretty(config)
                .map_err(|e| LoggingError::Serialization(e.to_string()))?;

            local_storage.set_item(storage_key, &content).map_err(|_| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to write to localStorage",
                )))
            })?;

            Ok(())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Native: Use file system
            let file_path = self.storage.get_file_path("log_config.toml")?;
            let content = toml::to_string_pretty(config)
                .map_err(|e| LoggingError::Serialization(e.to_string()))?;
            std::fs::write(file_path, content)?;
            Ok(())
        }
    }

    fn load_persisted_logs(&self) -> Result<Vec<LogEntry>, LoggingError> {
        #[cfg(target_arch = "wasm32")]
        {
            // Web: Use localStorage
            let window = web_sys::window().ok_or_else(|| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No window object",
                )))
            })?;

            let local_storage = window
                .local_storage()
                .map_err(|_| {
                    LoggingError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to access localStorage",
                    )))
                })?
                .ok_or_else(|| {
                    LoggingError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "localStorage not available",
                    )))
                })?;

            let storage_key = "hearth_logs";
            let content = local_storage
                .get_item(storage_key)
                .map_err(|_| {
                    LoggingError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to read from localStorage",
                    )))
                })?
                .unwrap_or_else(|| "[]".to_string());

            serde_json::from_str(&content).map_err(|e| LoggingError::Serialization(e.to_string()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Native: Use file system
            let file_path = self.storage.get_file_path("logs.json")?;
            if !file_path.exists() {
                return Ok(Vec::new());
            }

            let content = std::fs::read_to_string(file_path)?;
            serde_json::from_str(&content).map_err(|e| LoggingError::Serialization(e.to_string()))
        }
    }

    fn persist_logs(&self) -> Result<(), LoggingError> {
        let config = self
            .config
            .lock()
            .map_err(|e| LoggingError::Lock(e.to_string()))?;
        if !config.persist_logs {
            return Ok(());
        }
        drop(config);

        let entries = self
            .entries
            .lock()
            .map_err(|e| LoggingError::Lock(e.to_string()))?;
        let entries_vec: Vec<LogEntry> = entries.iter().cloned().collect();
        drop(entries);

        #[cfg(target_arch = "wasm32")]
        {
            // Web: Use localStorage
            let window = web_sys::window().ok_or_else(|| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No window object",
                )))
            })?;

            let local_storage = window
                .local_storage()
                .map_err(|_| {
                    LoggingError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to access localStorage",
                    )))
                })?
                .ok_or_else(|| {
                    LoggingError::Storage(StorageError::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "localStorage not available",
                    )))
                })?;

            let storage_key = "hearth_logs";
            let content = serde_json::to_string(&entries_vec)
                .map_err(|e| LoggingError::Serialization(e.to_string()))?;

            local_storage.set_item(storage_key, &content).map_err(|_| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to write to localStorage",
                )))
            })?;

            Ok(())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Native: Use file system
            let file_path = self.storage.get_file_path("logs.json")?;
            let content = serde_json::to_string_pretty(&entries_vec)
                .map_err(|e| LoggingError::Serialization(e.to_string()))?;
            std::fs::write(file_path, content)?;
            Ok(())
        }
    }

    pub fn get_logs(&self) -> Vec<LogEntry> {
        self.entries
            .lock()
            .map(|entries| entries.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get the total number of logs
    pub fn get_logs_count(&self) -> usize {
        self.entries
            .lock()
            .map(|entries| entries.len())
            .unwrap_or_default()
    }

    /// Get logs with pagination - returns the most recent logs first
    /// `offset`: Number of logs to skip from the end (0 = most recent)
    /// `limit`: Maximum number of logs to return
    pub fn get_logs_paginated(&self, offset: usize, limit: usize) -> Vec<LogEntry> {
        self.entries
            .lock()
            .map(|entries| {
                let total = entries.len();
                if total == 0 || offset >= total {
                    return Vec::new();
                }

                // Calculate range - we want most recent first, so work backwards
                let start_from_end = offset;
                let end_from_end = (offset + limit).min(total);
                
                // Convert to actual indices (VecDeque indices)
                let start_idx = total - end_from_end;
                let end_idx = total - start_from_end;

                entries
                    .range(start_idx..end_idx)
                    .rev() // Reverse to get most recent first
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn clear_logs(&self) -> Result<(), LoggingError> {
        if let Ok(mut entries) = self.entries.lock() {
            entries.clear();
        }
        self.persist_logs()
    }

    pub fn get_config(&self) -> LogConfig {
        self.config
            .lock()
            .map(|config| config.clone())
            .unwrap_or_default()
    }

    pub fn update_config(&self, new_config: LogConfig) -> Result<(), LoggingError> {
        {
            let mut config = self
                .config
                .lock()
                .map_err(|e| LoggingError::Lock(e.to_string()))?;
            *config = new_config.clone();
        }

        self.save_config(&new_config)?;
        Ok(())
    }

    pub fn export_logs(&self) -> Result<String, LoggingError> {
        let entries = self.get_logs();
        let mut output = String::new();

        output.push_str("# Hearth Application Logs\n");
        output.push_str(&format!(
            "Generated: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        for entry in entries {
            output.push_str(&format!(
                "[{}] {} {} - {}\n",
                entry.timestamp, entry.level, entry.target, entry.message
            ));
        }

        Ok(output)
    }

    /// Export logs with a save dialog for all platforms
    pub async fn export_logs_with_dialog(&self) -> Result<String, LoggingError> {
        let content = self.export_logs()?;
        
        #[cfg(target_arch = "wasm32")]
        {
            self.export_logs_web(content).await
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.export_logs_native(content).await
        }
    }

    #[cfg(target_arch = "wasm32")]
    async fn export_logs_web(&self, content: String) -> Result<String, LoggingError> {
        use web_sys::{window, Blob, BlobPropertyBag, Url};
        use wasm_bindgen::{JsValue, JsCast};
        
        let window = window().ok_or_else(|| {
            LoggingError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No window available"
            ))
        })?;
        let document = window.document().ok_or_else(|| {
            LoggingError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No document available"
            ))
        })?;
        
        // Create a blob with the log content
        let blob_parts = js_sys::Array::new();
        blob_parts.push(&JsValue::from_str(&content));
        
        let blob_options = BlobPropertyBag::new();
        blob_options.set_type("text/plain");
        
        let blob = Blob::new_with_str_sequence_and_options(&blob_parts, &blob_options)
            .map_err(|_| LoggingError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to create blob"
            )))?;
        
        // Create object URL
        let url = Url::create_object_url_with_blob(&blob)
            .map_err(|_| LoggingError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to create object URL"
            )))?;
        
        // Create a download link
        let a = document.create_element("a")
            .map_err(|_| LoggingError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to create anchor element"
            )))?;
        a.set_attribute("href", &url)
            .map_err(|_| LoggingError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to set href"
            )))?;
        
        // Generate filename with timestamp
        let now = js_sys::Date::new_0();
        let timestamp = now.to_iso_string();
        let filename = format!("hearth-logs-{}.txt", timestamp.as_string().unwrap_or_else(|| "unknown".to_string()));
        a.set_attribute("download", &filename)
            .map_err(|_| LoggingError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to set download attribute"
            )))?;
        
        // Trigger download
        let html_a = a.dyn_into::<web_sys::HtmlElement>()
            .map_err(|_| LoggingError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to cast to HtmlElement"
            )))?;
        html_a.click();
        
        // Clean up object URL
        Url::revoke_object_url(&url)
            .map_err(|_| LoggingError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to revoke object URL"
            )))?;
        
        Ok(format!("Log file '{}' downloaded successfully", filename))
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn export_logs_native(&self, content: String) -> Result<String, LoggingError> {
        // Generate default filename with timestamp
        let now = chrono::Utc::now();
        let timestamp = now.format("%Y%m%d_%H%M%S");
        let default_filename = format!("hearth-logs-{}.txt", timestamp);
        
        // Use file dialog for desktop, save to documents directory for mobile
        #[cfg(all(not(feature = "mobile"), not(target_os = "android")))]
        {
            use rfd::AsyncFileDialog;
            
            let file_path = AsyncFileDialog::new()
                .set_title("Save Hearth Logs")
                .set_file_name(&default_filename)
                .add_filter("Text Files", &["txt"])
                .add_filter("All Files", &["*"])
                .save_file()
                .await;
            
            if let Some(path) = file_path {
                let path = path.path();
                let mut file = std::fs::File::create(&path)?;
                
                std::io::Write::write_all(&mut file, content.as_bytes())?;
                
                log::info!("Logs exported to: {}", path.display());
                Ok(format!("Logs saved to: {}", path.display()))
            } else {
                Err(LoggingError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Save dialog was cancelled"
                )))
            }
        }
        
        // For mobile platforms and Android, use Android DownloadManager (like web download)
        #[cfg(any(feature = "mobile", target_os = "android"))]
        {
            self.export_logs_android_download(content, default_filename).await
        }
    }

    #[cfg(any(feature = "mobile", target_os = "android"))]
    async fn export_logs_android_download(&self, content: String, default_filename: String) -> Result<String, LoggingError> {
        // Use web-style download approach for mobile - create a downloadable link
        // This provides a proper "download" experience like web browsers
        
        #[cfg(target_arch = "wasm32")]
        {
            // If we're in a mobile web environment, use the same web API
            self.export_logs_web(content).await
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // For native mobile (Android), try to use the system's sharing/download capability
            // Fall back to a user-accessible location if available
            
            // Try multiple common Android storage locations
            let potential_paths = vec![
                // Android external storage paths that are typically accessible
                std::env::var("EXTERNAL_STORAGE").ok().map(|p| std::path::PathBuf::from(p).join("Download")),
                // Standard Android downloads
                Some(std::path::PathBuf::from("/storage/emulated/0/Download")),
                // Fallback to app's external files directory
                dirs::document_dir().map(|p| p.join("Downloads")),
                // Last resort - current directory
                Some(std::path::PathBuf::from(".")),
            ];
            
            let mut last_error = "No accessible storage location found".to_string();
            
            for potential_path in potential_paths.into_iter().flatten() {
                if let Err(_) = std::fs::create_dir_all(&potential_path) {
                    continue;
                }
                
                let file_path = potential_path.join(&default_filename);
                match std::fs::File::create(&file_path) {
                    Ok(mut file) => {
                        match std::io::Write::write_all(&mut file, content.as_bytes()) {
                            Ok(_) => {
                                log::info!("Logs saved to: {}", file_path.display());
                                log::info!("File downloaded successfully");
                                return Ok(format!("Logs saved to: {}", file_path.display()));
                            }
                            Err(e) => {
                                last_error = format!("Failed to write to file: {e}");
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        last_error = format!("Failed to create file: {e}");
                        continue;
                    }
                }
            }
            
            Err(LoggingError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                last_error
            )))
        }
    }
}

impl log::Log for HearthLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if let Ok(config) = self.config.lock() {
            metadata.level() <= config.min_level()
        } else {
            true // Default to allowing all logs if we can't read config
        }
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let entry = LogEntry::from_record(record);

        if let Ok(mut entries) = self.entries.lock() {
            let max_entries = self
                .config
                .lock()
                .map(|config| config.max_entries)
                .unwrap_or(1000);

            // Remove old entries if we're at capacity
            while entries.len() >= max_entries {
                entries.pop_front();
            }

            entries.push_back(entry);
        }

        // Persist logs (fire and forget)
        let _ = self.persist_logs();
    }

    fn flush(&self) {
        let _ = self.persist_logs();
    }
}

static GLOBAL_LOGGER: OnceLock<HearthLogger> = OnceLock::new();

/// Initialize the Hearth logging system
pub fn init_logging() -> Result<&'static HearthLogger, LoggingError> {
    let logger = HearthLogger::new();

    let logger_ref = GLOBAL_LOGGER.get_or_init(|| logger);

    log::set_logger(logger_ref).map_err(|e| LoggingError::Log(e.to_string()))?;
    log::set_max_level(log::LevelFilter::Trace);

    Ok(logger_ref)
}

/// Get a reference to the global logger
pub fn get_logger() -> Option<&'static HearthLogger> {
    GLOBAL_LOGGER.get()
}
