//! Logging UI components for settings and debugging

use dioxus::prelude::*;
use hearth_core::{get_logger, LogConfig, LogEntry as CoreLogEntry};
use crate::{SettingsSection, SettingsItem};
use std::collections::HashSet;

#[component]
pub fn LoggingSection() -> Element {
    let mut show_logs = use_signal(|| false);
    let mut logs = use_signal(|| Vec::<CoreLogEntry>::new());
    let mut config = use_signal(|| LogConfig::default());
    
    // Load initial logs and config on first render
    use_effect(move || {
        if let Some(logger) = get_logger() {
            logs.set(logger.get_logs());
            config.set(logger.get_config());
        }
    });

    rsx! {
        SettingsSection { title: "Debugging",
            SettingsItem {
                icon: "fa-solid fa-file-lines",
                label: "Application Logs",
                description: Some("View and export application logs"),
                on_click: move |_| {
                    show_logs.set(!show_logs());
                    
                    // Refresh logs when showing
                    if show_logs() {
                        if let Some(logger) = get_logger() {
                            logs.set(logger.get_logs());
                        }
                    }
                },
                trailing: rsx! {
                    span { 
                        class: "text-sm text-gray-500 dark:text-gray-400",
                        "{logs.read().len()} entries"
                    }
                },
            }

            if show_logs() {
                LogViewer { logs: logs(), config: config() }
            }
        }
    }
}

#[component]
pub fn LogViewer(logs: Vec<CoreLogEntry>, config: LogConfig) -> Element {
    let mut enabled_levels = use_signal(|| {
        let mut levels = HashSet::new();
        // Enable all levels by default
        levels.insert("ERROR".to_string());
        levels.insert("WARN".to_string());
        levels.insert("INFO".to_string());
        levels.insert("DEBUG".to_string());
        levels.insert("TRACE".to_string());
        levels
    });
    let mut search_term = use_signal(|| String::new());
    
    // Filter logs based on enabled levels and search term
    let filtered_logs = use_memo({
        let logs = logs.clone();
        move || {
            let levels = enabled_levels();
            let search = search_term().to_lowercase();
            
            logs.iter()
                .rev() // Reverse to show newest logs first
                .filter(|log| {
                    // Level filter - check if this log level is enabled
                    let level_match = levels.contains(&log.level);
                    
                    // Search filter  
                    let search_match = search.is_empty() || 
                        log.message.to_lowercase().contains(&search) ||
                        log.target.to_lowercase().contains(&search);
                    
                    level_match && search_match
                })
                .cloned()
                .collect::<Vec<_>>()
        }
    });

    rsx! {
        div { class: "mt-4 p-4 bg-gray-50 dark:bg-gray-800 rounded-lg",
            // Controls
            div { class: "mb-4 flex flex-col sm:flex-row gap-4 items-start sm:items-center justify-between",
                div { class: "flex flex-wrap gap-2",
                    LogLevelFilter { 
                        enabled_levels: enabled_levels(),
                        on_toggle: move |level| {
                            let mut levels = enabled_levels();
                            if levels.contains(&level) {
                                levels.remove(&level);
                            } else {
                                levels.insert(level);
                            }
                            enabled_levels.set(levels);
                        }
                    }
                }
                
                div { class: "flex gap-2",
                    input {
                        class: "px-3 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100",
                        placeholder: "Search logs...",
                        value: search_term(),
                        oninput: move |evt| search_term.set(evt.value()),
                    }
                    
                    LogActions { logs: logs.clone() }
                }
            }
            
            // Log entries
            div { class: "max-h-96 overflow-y-auto border border-gray-200 dark:border-gray-700 rounded-md bg-white dark:bg-gray-900",
                if filtered_logs().is_empty() {
                    div { class: "p-4 text-center text-gray-500 dark:text-gray-400",
                        "No logs found"
                    }
                } else {
                    div { class: "divide-y divide-gray-200 dark:divide-gray-700",
                        for log in filtered_logs() {
                            LogEntryComponent { log_entry: log }
                        }
                    }
                }
            }
        }
    }
}

#[component] 
pub fn LogLevelFilter(enabled_levels: HashSet<String>, on_toggle: EventHandler<String>) -> Element {
    let levels = vec![
        ("ERROR", "Error"),
        ("WARN", "Warn"), 
        ("INFO", "Info"),
        ("DEBUG", "Debug"),
        ("TRACE", "Trace"),
    ];

    rsx! {
        div { class: "flex gap-1",
            for (level, label) in levels {
                button {
                    class: format!("px-2 py-1 text-xs rounded transition-colors {}",
                        if enabled_levels.contains(level) {
                            "bg-blue-100 dark:bg-blue-900/50 text-blue-900 dark:text-blue-300"
                        } else {
                            "text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700"
                        }
                    ),
                    onclick: move |_| on_toggle.call(level.to_string()),
                    "{label}"
                }
            }
        }
    }
}

#[component]
pub fn LogActions(logs: Vec<CoreLogEntry>) -> Element {
    rsx! {
        div { class: "flex gap-2",
            button {
                class: "px-2 py-1 text-xs bg-blue-500 hover:bg-blue-600 text-white rounded transition-colors",
                onclick: move |_| {
                    if let Some(logger) = get_logger() {
                        match logger.export_logs() {
                            Ok(content) => {
                                // For web, create a download link
                                #[cfg(target_arch = "wasm32")]
                                {
                                    if let Some(window) = web_sys::window() {
                                        let _ = window.open_with_url_and_target(&format!("data:text/plain;charset=utf-8,{}", urlencoding::encode(&content)), "_blank");
                                    }
                                }
                                
                                log::info!("Logs exported successfully");
                            },
                            Err(e) => log::error!("Failed to export logs: {}", e),
                        }
                    }
                },
                "Export"
            }
            
            button {
                class: "px-2 py-1 text-xs bg-red-500 hover:bg-red-600 text-white rounded transition-colors",
                onclick: move |_| {
                    if let Some(logger) = get_logger() {
                        if let Err(e) = logger.clear_logs() {
                            log::error!("Failed to clear logs: {}", e);
                        } else {
                            log::info!("Logs cleared successfully");
                        }
                    }
                },
                "Clear"
            }
        }
    }
}

#[component]
pub fn LogEntryComponent(log_entry: CoreLogEntry) -> Element {
    let level_color = match log_entry.level.as_str() {
        "ERROR" => "text-red-600 dark:text-red-400",
        "WARN" => "text-yellow-600 dark:text-yellow-400", 
        "INFO" => "text-blue-600 dark:text-blue-400",
        "DEBUG" => "text-green-600 dark:text-green-400",
        "TRACE" => "text-purple-600 dark:text-purple-400",
        _ => "text-gray-600 dark:text-gray-400",
    };

    rsx! {
        div { class: "p-3 hover:bg-gray-50 dark:hover:bg-gray-800",
            div { class: "flex items-start gap-3 text-sm",
                div { class: "flex-shrink-0 min-w-0",
                    div { class: "flex items-center gap-2",
                        span { class: format!("font-mono text-xs px-1.5 py-0.5 rounded {}", level_color),
                            "{log_entry.level}"
                        }
                        span { class: "text-xs text-gray-500 dark:text-gray-400",
                            "{log_entry.timestamp}"
                        }
                    }
                    div { class: "text-xs text-gray-500 dark:text-gray-400 mt-1",
                        "{log_entry.target}"
                    }
                }
                div { class: "flex-1 min-w-0 font-mono text-xs text-gray-900 dark:text-gray-100 break-words",
                    "{log_entry.message}"
                }
            }
        }
    }
}