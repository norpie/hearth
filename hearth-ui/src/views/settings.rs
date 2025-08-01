//! Unified responsive settings view

use dioxus::prelude::*;
use crate::{PageHeader, Route, DarkModeContext, SettingsSection, SettingsItem, DarkModeToggle, Platform, use_settings, LoggingSection};
use hearth_core::Theme;

#[component]
pub fn SettingsView(navigate_to: EventHandler<Route>) -> Element {
    let mut dark_mode_ctx = use_context::<DarkModeContext>();
    let mut settings = use_settings();
    let platform = Platform::current();
    
    rsx! {
        PageHeader { title: "Settings".to_string(), back_button: None }
        div { class: "flex-1 overflow-y-auto",
            div { class: if platform.is_mobile() { "p-4 space-y-4" } else { "p-4 max-w-4xl mx-auto space-y-6" },
                // Appearance section
                SettingsSection { title: "Appearance",
                    SettingsItem {
                        icon: "fa-solid fa-moon",
                        label: "Dark Mode",
                        description: None,
                        on_click: move |_| {
                            let old_theme = settings.read().get().theme.clone();
                            let new_theme = match old_theme {
                                Theme::Dark => Theme::Light,
                                Theme::Light => Theme::Dark,
                            };
                            
                            log::info!("Theme changed from {:?} to {:?}", old_theme, new_theme);
                            
                            // Update the settings
                            let mut new_settings = settings.read().get().clone();
                            new_settings.theme = new_theme.clone();
                            settings.write().update(new_settings);
                            
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