//! Cross-platform storage directory abstraction
//!
//! Provides consistent storage directory management across platforms.

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[cfg(target_os = "android")]
    #[error("JNI error: {0}")]
    Jni(#[from] jni::errors::Error),
}

/// Cross-platform storage directory abstraction
pub struct Storage;

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage {
    pub fn new() -> Self {
        Self
    }

    /// Get the storage directory path (desktop/mobile only)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_storage_dir(&self) -> Result<PathBuf, StorageError> {
        // On Android, use app's internal files directory
        #[cfg(target_os = "android")]
        let base_dir = {
            Self::get_android_files_dir().unwrap_or_else(|e| {
                log::warn!(
                    "Failed to get Android files directory ({}), using fallback",
                    e
                );
                // Fallback to a safe location
                PathBuf::from("/storage/emulated/0/Android/data/com.hearth.app/files")
            })
        };

        #[cfg(not(target_os = "android"))]
        let base_dir = dirs::config_dir().ok_or_else(|| {
            StorageError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find config directory",
            ))
        })?;

        let app_dir = base_dir.join("hearth");
        if !app_dir.exists() {
            log::debug!("Creating storage directory: {app_dir:?}");
            std::fs::create_dir_all(&app_dir)?;
            log::info!("Storage directory created at: {app_dir:?}");
        } else {
            log::trace!("Using existing storage directory: {app_dir:?}");
        }
        Ok(app_dir)
    }

    /// Get Android internal files directory using JNI
    #[cfg(target_os = "android")]
    fn get_android_files_dir() -> Result<PathBuf, StorageError> {
        use jni::objects::JObject;

        // Get the Android app context through dioxus_mobile
        let vm = unsafe { jni::JavaVM::from_raw(ndk_context::android_context().vm().cast())? };
        let mut env = vm.attach_current_thread()?;

        // Get the application context
        let context = unsafe { JObject::from_raw(ndk_context::android_context().context().cast()) };

        // Call getFilesDir() on the context
        let files_dir = env.call_method(context, "getFilesDir", "()Ljava/io/File;", &[])?;

        // Get the absolute path from the File object
        let path_string = env.call_method(
            files_dir.l()?,
            "getAbsolutePath",
            "()Ljava/lang/String;",
            &[],
        )?;

        // Convert Java string to Rust string - fix lifetime issue
        let path_obj = path_string.l()?;
        let binding = path_obj.into();
        let java_string = env.get_string(&binding)?;
        let path_str: String = java_string.into();

        Ok(PathBuf::from(path_str))
    }

    /// Get a file path within the storage directory (desktop/mobile only)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_file_path(&self, filename: &str) -> Result<PathBuf, StorageError> {
        let storage_dir = self.get_storage_dir()?;
        Ok(storage_dir.join(filename))
    }
}
