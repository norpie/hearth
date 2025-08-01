//! Unified responsive settings view

use crate::{
    use_settings, use_theme, use_backend_selection, use_remote_backends, 
    DarkModeContext, DarkModeToggle, LoggingSection, PageHeader, Platform, Route,
    SettingsItem, SettingsSection, Select, SelectOption,
};
use dioxus::prelude::*;
use hearth_core::{Theme, BackendId, RemoteBackendConfig};

#[component]
pub fn SettingsView(navigate_to: EventHandler<Route>) -> Element {
    let mut dark_mode_ctx = use_context::<DarkModeContext>();
    let (theme, set_theme) = use_theme();
    let (selected_backend, set_selected_backend) = use_backend_selection();
    let (remote_backends, add_remote_backend, remove_remote_backend) = use_remote_backends();
    let platform = Platform::current();

    rsx! {
        PageHeader { title: "Settings".to_string(), back_button: None }
        div { class: "flex-1 overflow-y-auto",
            div { class: if platform.is_mobile() { "p-4 space-y-4" } else { "p-4 max-w-4xl mx-auto space-y-6" },
                
                // Backend Configuration section (always show, read-only on web)
                BackendSection { 
                    selected_backend: selected_backend.clone(),
                    remote_backends: remote_backends.clone(),
                    read_only: !platform.can_edit_backend_settings(),
                    on_backend_select: move |backend_id| set_selected_backend(backend_id),
                    on_add_remote: move |backend| add_remote_backend(backend),
                    on_remove_remote: move |id| remove_remote_backend(id),
                }

                // Appearance section
                SettingsSection { title: "Appearance",
                    SettingsItem {
                        icon: "fa-solid fa-moon",
                        label: "Dark Mode",
                        description: None,
                        on_click: move |_| {
                            let new_theme = match theme {
                                Theme::Dark => Theme::Light,
                                Theme::Light => Theme::Dark,
                                Theme::Auto => Theme::Light, // Default to light when cycling from auto
                            };

                            log::info!("Theme changed from {theme:?} to {new_theme:?}");
                            set_theme(new_theme.clone());

                            // Update the UI context
                            dark_mode_ctx.is_dark.set(matches!(new_theme, Theme::Dark));
                        },
                        trailing: rsx! {
                            DarkModeToggle {}
                        },
                    }
                }

                // Logging section
                LoggingSection {}
            }
        }
    }
}

#[component]
fn BackendSection(
    selected_backend: Option<BackendId>,
    remote_backends: Vec<RemoteBackendConfig>,
    read_only: bool,
    on_backend_select: EventHandler<Option<BackendId>>,
    on_add_remote: EventHandler<RemoteBackendConfig>,
    on_remove_remote: EventHandler<String>,
) -> Element {
    let title = if read_only { 
        "Backend Configuration (Read-only)" 
    } else { 
        "Backend Configuration" 
    };
    
    rsx! {
        SettingsSection { 
            title: title,
            SettingsItem {
                icon: "fa-solid fa-server",
                label: "Active Backend",
                description: None,
                on_click: |_| {},
                trailing: rsx! {
                    div { class: "flex items-center space-x-2",
                        if read_only {
                            // Web platform: Always remote, show info
                            div { class: "flex items-center space-x-2",
                                span { class: "text-sm text-muted-foreground", "Remote (Current Server)" }
                                button {
                                    class: "flex items-center justify-center w-4 h-4 text-muted-foreground hover:text-foreground transition-colors",
                                    title: "Web apps can only connect to the server hosting this page. Desktop and mobile apps can choose between local and remote backends.",
                                    i { class: "fa-solid fa-info-circle text-xs" }
                                }
                            }
                        } else {
                            // Desktop/Mobile: Show select dropdown
                            div { class: "w-48",
                                Select {
                                    value: selected_backend.clone().unwrap_or_else(|| "local".to_string()),
                                    options: {
                                        let mut options = vec![
                                            SelectOption::new("local", "Local (SQLite)")
                                        ];
                                        for backend in remote_backends.iter() {
                                            options.push(SelectOption::new(&backend.id, &backend.name));
                                        }
                                        options
                                    },
                                    placeholder: "Select backend...",
                                    searchable: true,
                                    disabled: read_only,
                                    onchange: move |value| {
                                        let backend_id = if value == "local" { 
                                            None 
                                        } else { 
                                            Some(value) 
                                        };
                                        on_backend_select.call(backend_id);
                                    },
                                }
                            }
                        }
                    }
                },
            }

            // Remote backend management (only show if there are remote backends)
            for backend in remote_backends.iter() {
                RemoteBackendConfigItem {
                    backend: backend.clone(),
                    read_only: read_only,
                    on_remove: move |id| on_remove_remote.call(id),
                }
            }

            // Add remote backend button (only show if not read-only)
            if !read_only {
                SettingsItem {
                    icon: "fa-solid fa-plus",
                    label: "Add Remote Backend",
                    description: Some("Connect to a Hearth server"),
                    on_click: |_| {
                        // TODO: Show add backend modal
                    },
                    trailing: rsx! {},
                }
            }
        }
    }
}

#[component]
fn RemoteBackendConfigItem(
    backend: RemoteBackendConfig,
    read_only: bool,
    on_remove: EventHandler<String>,
) -> Element {
    let backend_id = backend.id.clone();
    
    rsx! {
        div {
            class: "w-full py-4 px-4 flex items-center space-x-4 hover:bg-muted transition-colors text-left rounded-lg",
            
            i { class: "fa-solid fa-cloud text-xl text-foreground" }
            div { class: "flex-1 min-w-0",
                div { class: "font-medium text-foreground", "{backend.name}" }
                div { class: "text-sm text-muted-foreground mt-0.5", "{backend.url}" }
            }

            if !read_only {
                button {
                    class: "text-red-500 hover:text-red-700 text-sm px-2 py-1 rounded hover:bg-red-50",
                    onclick: move |_| on_remove.call(backend_id.clone()),
                    "Remove"
                }
            } else {
                div { class: "text-xs text-muted-foreground", 
                    if let Some(connected) = &backend.last_connected {
                        "Connected: {connected:?}"
                    } else {
                        "Never connected"
                    }
                }
            }
        }
    }
}
