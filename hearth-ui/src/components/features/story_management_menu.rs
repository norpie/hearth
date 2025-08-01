//! Story management menu component for story options and controls

use crate::{Button, ButtonVariant, ButtonSize, ScrollArea, FadeMode};
use dioxus::prelude::*;

#[component]
pub fn StoryManagementMenu(
    on_new_story: EventHandler<()>,
    on_export: EventHandler<()>,
    on_settings: EventHandler<()>,
) -> Element {
    rsx! {
        // Content wrapper - just the menu content
        div { 
            class: "flex flex-col h-[32rem]",
            
            // Menu options
            div { 
                class: "flex-1 overflow-hidden flex flex-col",
                
                ScrollArea {
                    height: "h-full".to_string(),
                    fade_mode: FadeMode::Both,
                    fade_color: Some("from-background".to_string()),
                    class: "bg-background".to_string(),
                    
                    div { 
                        class: "p-3 space-y-2",
                        
                        // New Story
                        MenuOption {
                            icon: "fas fa-plus",
                            title: "New Story",
                            description: "Start a fresh story with the same characters",
                            on_click: move |_| on_new_story.call(()),
                        }
                        
                        // Branch History
                        MenuOption {
                            icon: "fas fa-code-branch",
                            title: "Branch History",
                            description: "View and manage story branches",
                            on_click: move |_| {
                                // TODO: Implement branch history
                            },
                        }
                        
                        // Separator
                        div { class: "border-t border-border my-2" }
                        
                        // Story Memory
                        MenuOption {
                            icon: "fas fa-brain",
                            title: "Story Memory",
                            description: "Manage characters memory and context",
                            on_click: move |_| {
                                // TODO: Implement story memory
                            },
                        }
                        
                        // Generation Settings
                        MenuOption {
                            icon: "fas fa-cog",
                            title: "Generation Settings",
                            description: "Adjust AI temperature, length, and behavior",
                            on_click: move |_| on_settings.call(()),
                        }
                        
                        // Separator
                        div { class: "border-t border-border my-2" }
                        
                        // Export Story
                        MenuOption {
                            icon: "fas fa-download",
                            title: "Export Story",
                            description: "Save story as JSON or text file",
                            on_click: move |_| on_export.call(()),
                        }
                        
                    }
                }
            }
        }
    }
}

#[component]
fn MenuOption(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    on_click: EventHandler<()>,
) -> Element {
    rsx! {
        button {
            class: "w-full p-3 rounded-lg hover:bg-muted transition-colors text-left flex items-start space-x-3",
            onclick: move |_| on_click.call(()),
            
            // Icon
            div { 
                class: "flex-shrink-0 w-10 h-10 bg-muted rounded-lg flex items-center justify-center",
                i { class: format!("{} text-muted-foreground", icon) }
            }
            
            // Content
            div { 
                class: "flex-1 min-w-0",
                div { 
                    class: "font-medium text-foreground mb-1",
                    "{title}"
                }
                div { 
                    class: "text-sm text-muted-foreground",
                    "{description}"
                }
            }
            
            // Arrow
            div { 
                class: "flex-shrink-0 text-muted-foreground",
                i { class: "fas fa-chevron-right text-xs" }
            }
        }
    }
}