//! Log viewer UI components for settings and debugging

use crate::{SettingsSection, use_toaster, Button, ButtonVariant, ButtonSize, Input, InputVariant, Badge, BadgeVariant, BadgeSize, ScrollArea, ScrollOrientation, Select, SelectVariant, SelectOption, Collapsible, Platform};
use dioxus::prelude::*;
use hearth_core::{get_logger, LogConfig, LogEntry as CoreLogEntry};
use std::collections::HashSet;

#[component]
pub fn LoggingSection() -> Element {
    let mut total_logs_count = use_signal(|| 0);
    let mut config = use_signal(LogConfig::default);

    // Load initial config and total count on first render
    use_effect(move || {
        if let Some(logger) = get_logger() {
            config.set(logger.get_config());
            total_logs_count.set(logger.get_logs_count());
        }
    });

    rsx! {
        SettingsSection { title: "Debugging",
            Collapsible {
                trigger: format!("Application Logs ({} entries)", total_logs_count()),
                default_open: false,
                class: "mb-4",
                div { class: "space-y-4",
                    // Logs Per Page setting
                    div { class: "flex items-center justify-between py-2 border-b border-border",
                        div {
                            div { class: "text-sm font-medium", "Logs Per Page" }
                            div { class: "text-xs text-muted-foreground", "Number of logs to load at once" }
                        }
                        LogPageSizeSelector { 
                            current_page_size: config().page_size,
                            on_page_size_changed: move |new_size| {
                                if let Some(logger) = get_logger() {
                                    let mut new_config = config();
                                    new_config.page_size = new_size;
                                    if let Ok(()) = logger.update_config(new_config.clone()) {
                                        config.set(new_config);
                                    }
                                }
                            }
                        }
                    }
                    
                    // Log viewer
                    LogViewer { 
                        config: config(),
                        config_signal: config,
                        on_logs_changed: move |_| {
                            if let Some(logger) = get_logger() {
                                total_logs_count.set(logger.get_logs_count());
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn LogViewer(config: LogConfig, config_signal: Signal<LogConfig>, on_logs_changed: EventHandler<()>) -> Element {
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
    let mut search_term = use_signal(String::new);
    let mut loaded_logs = use_signal(Vec::<CoreLogEntry>::new);
    let mut current_offset = use_signal(|| 0usize);
    let mut total_available_logs = use_signal(|| 0usize);
    let mut has_more_logs = use_signal(|| false);
    let mut is_loading = use_signal(|| false);

    // Load initial logs and get total count
    use_effect(move || {
        if let Some(logger) = get_logger() {
            let page_size = config.page_size;
            let logs = logger.get_logs_paginated(0, page_size);
            let total = logger.get_logs_count();
            
            loaded_logs.set(logs.clone());
            current_offset.set(logs.len());
            total_available_logs.set(total);
            has_more_logs.set(logs.len() < total);
        }
    });

    // Watch for config changes and reload logs when page size changes
    use_effect({
        move || {
            let current_config = config_signal();
            if let Some(logger) = get_logger() {
                let logs = logger.get_logs_paginated(0, current_config.page_size);
                let total = logger.get_logs_count();
                
                loaded_logs.set(logs.clone());
                current_offset.set(logs.len());
                total_available_logs.set(total);
                has_more_logs.set(logs.len() < total);
            }
        }
    });

    // Function to load more logs
    let mut load_more_logs = move || {
        if is_loading() {
            return;
        }
        
        is_loading.set(true);
        
        if let Some(logger) = get_logger() {
            let offset = current_offset();
            let page_size = config_signal().page_size;
            let new_logs = logger.get_logs_paginated(offset, page_size);
            
            if !new_logs.is_empty() {
                let mut current_logs = loaded_logs();
                current_logs.extend(new_logs.iter().cloned());
                loaded_logs.set(current_logs);
                current_offset.set(offset + new_logs.len());
            }
            
            let total = logger.get_logs_count();
            total_available_logs.set(total);
            has_more_logs.set(current_offset() < total);
        }
        
        is_loading.set(false);
    };

    // Filter logs based on enabled levels and search term
    let filtered_logs = {
        let levels = enabled_levels();
        let search = search_term().to_lowercase();

        loaded_logs()
            .iter()
            .filter(|log| {
                // Level filter - check if this log level is enabled
                let level_match = levels.contains(&log.level);

                // Search filter
                let search_match = search.is_empty()
                    || log.message.to_lowercase().contains(&search)
                    || log.target.to_lowercase().contains(&search);

                level_match && search_match
            })
            .cloned()
            .collect::<Vec<_>>()
    };

    rsx! {
        div { class: "mt-4 p-4 bg-muted rounded-lg",
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
                        },
                    }
                }
                div { class: "flex gap-2",
                    Input {
                        variant: InputVariant::Default,
                        placeholder: "Search logs...".to_string(),
                        value: search_term(),
                        oninput: move |value: String| search_term.set(value),
                        class: "w-64".to_string(),
                    }
                    LogActions { 
                        on_logs_changed: move |_| {
                            // Reset lazy loading state and reload from beginning
                            loaded_logs.set(Vec::new());
                            current_offset.set(0);
                            if let Some(logger) = get_logger() {
                                let page_size = config_signal().page_size;
                                let logs = logger.get_logs_paginated(0, page_size);
                                let total = logger.get_logs_count();
                                
                                loaded_logs.set(logs.clone());
                                current_offset.set(logs.len());
                                total_available_logs.set(total);
                                has_more_logs.set(logs.len() < total);
                            }
                            on_logs_changed.call(());
                        }
                    }
                }
            }
            
            // Stats
            div { class: "mb-2 text-sm text-muted-foreground",
                "Showing {filtered_logs.len()} of {loaded_logs().len()} loaded logs ({total_available_logs()} total) • {enabled_levels().len()}/5 filters active"
            }
            
            
            // Log entries
            ScrollArea {
                orientation: ScrollOrientation::Vertical,
                height: "400px".to_string(),
                class: "border border-border rounded-md bg-card",
                if filtered_logs.is_empty() && loaded_logs().is_empty() {
                    div { class: "p-4 text-center text-muted-foreground", "No logs found" }
                } else if filtered_logs.is_empty() {
                    div { class: "p-4 text-center text-muted-foreground", "No logs match the current filters" }
                } else {
                    div {
                        div { class: "divide-y divide-border",
                            for log in filtered_logs {
                                LogEntryComponent { log_entry: log }
                            }
                        }
                        
                        // Load more button
                        if has_more_logs() {
                            div { class: "p-4 text-center border-t border-border",
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    size: ButtonSize::Small,
                                    disabled: is_loading(),
                                    onclick: move |_| load_more_logs(),
                                    if is_loading() {
                                        "Loading..."
                                    } else {
                                        "Load More ({total_available_logs() - loaded_logs().len()} remaining)"
                                    }
                                }
                            }
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
        ("ERROR", "Error", BadgeVariant::Error, ""),
        ("WARN", "Warn", BadgeVariant::Warning, ""),
        ("INFO", "Info", BadgeVariant::Info, ""),
        ("DEBUG", "Debug", BadgeVariant::Default, "bg-purple-600 text-white"),
        ("TRACE", "Trace", BadgeVariant::Default, "bg-gray-600 text-white"),
    ];

    rsx! {
        div { class: "flex gap-1",
            for (level, label, variant, custom_class) in levels {
                Badge {
                    variant: if enabled_levels.contains(level) { variant } else { BadgeVariant::Outline },
                    size: BadgeSize::Small,
                    class: format!("cursor-pointer hover:opacity-80 transition-opacity {}", 
                        if enabled_levels.contains(level) && !custom_class.is_empty() { custom_class } else { "" }
                    ),
                    onclick: move |_| on_toggle.call(level.to_string()),
                    "{label}"
                }
            }
        }
    }
}

#[component]
pub fn LogActions(on_logs_changed: EventHandler<()>) -> Element {
    let toaster = use_toaster();
    
    rsx! {
        div { class: "flex gap-2",
            Button {
                variant: ButtonVariant::Primary,
                size: ButtonSize::Small,
                onclick: move |_| {
                    let toaster = toaster;
                    Platform::spawn(async move {
                        if let Some(logger) = get_logger() {
                            match logger.export_logs_with_dialog().await {
                                Ok(success_message) => {
                                    toaster.success(success_message);
                                },
                                Err(e) => {
                                    toaster.error(format!("Failed to export logs: {e}"));
                                },
                            }
                        }
                    });
                },
                "Export"
            }
            Button {
                variant: ButtonVariant::Destructive,
                size: ButtonSize::Small,
                onclick: move |_| {
                    let toaster = toaster;
                    if let Some(logger) = get_logger() {
                        if let Err(e) = logger.clear_logs() {
                            toaster.error(format!("Failed to clear logs: {e}"));
                        } else {
                            toaster.success("Logs cleared successfully");
                            on_logs_changed.call(());
                        }
                    }
                },
                "Clear"
            }
        }
    }
}

#[component]
pub fn LogPageSizeSelector(current_page_size: usize, on_page_size_changed: EventHandler<usize>) -> Element {
    let page_size_options = vec![25, 50, 100, 200]
        .into_iter()
        .map(|size| SelectOption::new(size.to_string(), size.to_string()))
        .collect();
    
    rsx! {
        Select {
            variant: SelectVariant::Default,
            value: current_page_size.to_string(),
            options: page_size_options,
            placeholder: "Select page size".to_string(),
            onchange: move |value: String| {
                if let Ok(new_size) = value.parse::<usize>() {
                    on_page_size_changed.call(new_size);
                }
            },
        }
    }
}

#[component]
pub fn LogEntryComponent(log_entry: CoreLogEntry) -> Element {
    let (level_variant, custom_class) = match log_entry.level.as_str() {
        "ERROR" => (BadgeVariant::Error, ""),
        "WARN" => (BadgeVariant::Warning, ""),
        "INFO" => (BadgeVariant::Info, ""),
        "DEBUG" => (BadgeVariant::Default, "bg-purple-600 text-white"),
        "TRACE" => (BadgeVariant::Default, "bg-gray-600 text-white"),
        _ => (BadgeVariant::Default, ""),
    };

    rsx! {
        div { class: "p-3 hover:bg-muted/50 transition-colors",
            div { class: "flex items-start gap-3 text-sm",
                div { class: "flex-shrink-0 min-w-0",
                    div { class: "flex items-center gap-2",
                        Badge {
                            variant: level_variant,
                            size: BadgeSize::Small,
                            class: format!("font-mono {}", custom_class),
                            "{log_entry.level}"
                        }
                        span { class: "text-xs text-muted-foreground", "{log_entry.timestamp}" }
                    }
                    div { class: "text-xs text-muted-foreground mt-1", "{log_entry.target}" }
                }
                div { class: "flex-1 min-w-0 font-mono text-xs text-foreground break-words",
                    "{log_entry.message}"
                }
            }
        }
    }
}