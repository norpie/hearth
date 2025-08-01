//! Cross-platform logging system
//! 
//! Implements the standard Rust `log` crate with persistent storage and UI access.

use log::{Level, Record, Metadata};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, OnceLock};
use thiserror::Error;
use crate::{Storage, StorageError};

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
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
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
        }
    }
}

/// Cross-platform logger that implements the `log::Log` trait
pub struct HearthLogger {
    entries: Arc<Mutex<VecDeque<LogEntry>>>,
    config: Arc<Mutex<LogConfig>>,
    storage: Storage,
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
                    "No window object"
                )))
            })?;
            
            let local_storage = window.local_storage().map_err(|_| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to access localStorage"
                )))
            })?.ok_or_else(|| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "localStorage not available"
                )))
            })?;
            
            let storage_key = "hearth_log_config";
            let content = local_storage.get_item(storage_key).map_err(|_| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to read from localStorage"
                )))
            })?.ok_or_else(|| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Log config not found in localStorage"
                )))
            })?;
            
            toml::from_str(&content).map_err(|e| LoggingError::Serialization(e.to_string()))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Native: Use file system
            let file_path = storage.get_file_path("log_config.toml")?;
            if !file_path.exists() {
                return Err(LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Log config file does not exist"
                ))));
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
                    "No window object"
                )))
            })?;
            
            let local_storage = window.local_storage().map_err(|_| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to access localStorage"
                )))
            })?.ok_or_else(|| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "localStorage not available"
                )))
            })?;
            
            let storage_key = "hearth_log_config";
            let content = toml::to_string_pretty(config)
                .map_err(|e| LoggingError::Serialization(e.to_string()))?;
                
            local_storage.set_item(storage_key, &content).map_err(|_| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to write to localStorage"
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
                    "No window object"
                )))
            })?;
            
            let local_storage = window.local_storage().map_err(|_| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to access localStorage"
                )))
            })?.ok_or_else(|| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "localStorage not available"
                )))
            })?;
            
            let storage_key = "hearth_logs";
            let content = local_storage.get_item(storage_key).map_err(|_| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to read from localStorage"
                )))
            })?.unwrap_or_else(|| "[]".to_string());
            
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
        let config = self.config.lock().map_err(|e| LoggingError::Lock(e.to_string()))?;
        if !config.persist_logs {
            return Ok(());
        }
        drop(config);

        let entries = self.entries.lock().map_err(|e| LoggingError::Lock(e.to_string()))?;
        let entries_vec: Vec<LogEntry> = entries.iter().cloned().collect();
        drop(entries);

        #[cfg(target_arch = "wasm32")]
        {
            // Web: Use localStorage
            let window = web_sys::window().ok_or_else(|| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No window object"
                )))
            })?;
            
            let local_storage = window.local_storage().map_err(|_| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to access localStorage"
                )))
            })?.ok_or_else(|| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "localStorage not available"
                )))
            })?;
            
            let storage_key = "hearth_logs";
            let content = serde_json::to_string(&entries_vec)
                .map_err(|e| LoggingError::Serialization(e.to_string()))?;
                
            local_storage.set_item(storage_key, &content).map_err(|_| {
                LoggingError::Storage(StorageError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to write to localStorage"
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
        self.entries.lock()
            .map(|entries| entries.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn clear_logs(&self) -> Result<(), LoggingError> {
        if let Ok(mut entries) = self.entries.lock() {
            entries.clear();
        }
        self.persist_logs()
    }

    pub fn get_config(&self) -> LogConfig {
        self.config.lock()
            .map(|config| config.clone())
            .unwrap_or_default()
    }

    pub fn update_config(&self, new_config: LogConfig) -> Result<(), LoggingError> {
        {
            let mut config = self.config.lock().map_err(|e| LoggingError::Lock(e.to_string()))?;
            *config = new_config.clone();
        }
        
        self.save_config(&new_config)?;
        Ok(())
    }

    pub fn export_logs(&self) -> Result<String, LoggingError> {
        let entries = self.get_logs();
        let mut output = String::new();
        
        output.push_str("# Hearth Application Logs\n");
        output.push_str(&format!("Generated: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        
        for entry in entries {
            output.push_str(&format!("[{}] {} {} - {}\n", 
                entry.timestamp, entry.level, entry.target, entry.message));
        }
        
        Ok(output)
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
            let max_entries = self.config.lock()
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