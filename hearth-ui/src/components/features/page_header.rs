//! Page header component with expandable content support
//!
//! This module provides a flexible page header component that can display a title,
//! back button, and optional expandable content. It supports both mobile gesture
//! controls and desktop click interactions.

use crate::models::*;
use crate::{GestureDetector, GestureDirection};
use dioxus::prelude::*;

// Generic page header component
#[derive(Props, Clone)]
pub struct PageHeaderProps {
    pub title: String,
    #[props(default = None)]
    pub back_button: Option<Element>,
    #[props(default = None)]
    pub expandable_content: Option<Element>,
    #[props(default = None)]
    pub expanded: Option<bool>,
    #[props(default = None)]
    pub on_expanded_change: Option<EventHandler<bool>>,
    #[props(default = false)]
    pub enable_desktop_click: bool,
}

impl PartialEq for PageHeaderProps {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && 
        self.enable_desktop_click == other.enable_desktop_click &&
        self.expanded == other.expanded
        // Skip comparison of EventHandler and Element as they don't implement PartialEq
    }
}

#[component]
pub fn PageHeader(props: PageHeaderProps) -> Element {
    // Try to get sidebar context - it might not exist on mobile
    let sidebar_ctx = try_use_context::<SidebarContext>();
    
    // Internal expansion state (used when not controlled)
    let mut internal_expanded = use_signal(|| false);
    
    // Check if we have expandable content
    let has_expandable_content = props.expandable_content.is_some();
    
    // Handle gesture events for swipe down (toggle) and swipe up (close)
    let handle_gesture = {
        let expanded_change_handler = props.on_expanded_change.clone();
        let mut internal_expanded = internal_expanded.clone();
        let props_expanded = props.expanded.clone();
        move |direction: GestureDirection| {
            if has_expandable_content {
                match direction {
                    GestureDirection::Down => {
                        // Toggle menu with down swipe
                        let current_expanded = props_expanded.unwrap_or_else(|| internal_expanded());
                        let new_state = !current_expanded;
                        if let Some(handler) = &expanded_change_handler {
                            handler.call(new_state);
                        } else {
                            internal_expanded.set(new_state);
                        }
                    },
                    GestureDirection::Up => {
                        // Close menu with up swipe (when expanded)
                        let current_expanded = props_expanded.unwrap_or_else(|| internal_expanded());
                        if current_expanded {
                            if let Some(handler) = &expanded_change_handler {
                                handler.call(false);
                            } else {
                                internal_expanded.set(false);
                            }
                        }
                    },
                    _ => {
                        // Ignore other gestures
                    }
                }
            }
        }
    };
    
    // Click handler for desktop
    let handle_click = {
        let expanded_change_handler = props.on_expanded_change.clone();
        let mut internal_expanded = internal_expanded.clone();
        let props_expanded = props.expanded.clone();
        move |_| {
            if props.enable_desktop_click && has_expandable_content {
                // Compute current state dynamically
                let current_expanded = props_expanded.unwrap_or_else(|| internal_expanded());
                let new_state = !current_expanded;
                if let Some(handler) = &expanded_change_handler {
                    handler.call(new_state);
                } else {
                    internal_expanded.set(new_state);
                }
            }
        }
    };

    // Compute current expansion state
    let is_expanded = props.expanded.unwrap_or_else(|| internal_expanded());
    
    if has_expandable_content {
        // Expandable header structure
        let transform_class = if is_expanded {
            "translate-y-0"  // Show content when expanded
        } else {
            "translate-y-[-32rem]"  // Hide content when collapsed
        };
        
        rsx! {
            // Container for expandable header - no height constraint
            div { 
                class: "relative z-50",
                
                // Sliding content container - absolute positioned
                div { 
                    class: format!(
                        "absolute top-0 left-0 right-0 transform transition-transform duration-300 ease-in-out {}",
                        transform_class
                    ),
                    
                    // Content wrapper
                    div { 
                        class: "bg-background shadow-lg",
                        style: "max-height: 50vh; display: flex; flex-direction: column;",
                        
                        // Expandable content at top
                        if let Some(content) = props.expandable_content {
                            div { 
                                class: "border-b border-border bg-background",
                                {content}
                            }
                        }
                        
                        // Header - always at bottom of sliding container
                        GestureDetector {
                            debug: false,
                            class: "w-full".to_string(),
                            on_gesture: handle_gesture,
                            
                            div { 
                                class: "flex items-center space-x-3 p-4 border-b border-border bg-background relative",
                                
                                // Gesture zone at bottom edge (like character menu but at bottom)
                                if props.enable_desktop_click {
                                    div {
                                        class: "absolute bottom-0 left-0 right-0 h-3 cursor-ns-resize",
                                        onclick: handle_click,
                                    }
                                }
                                
                                // Header content inline
                                if let Some(back_btn) = props.back_button {
                                    {back_btn}
                                } else if let Some(mut ctx) = sidebar_ctx {
                                    if !(ctx.is_visible)() {
                                        button {
                                            class: "flex items-center text-muted-foreground hover:text-foreground transition-colors",
                                            onclick: move |_| {
                                                ctx.is_visible.set(true);
                                            },
                                            span { class: "text-2xl font-bold leading-none", "»" }
                                        }
                                    }
                                }
                                
                                div { class: "flex-1",
                                    h1 { class: "text-xl font-semibold text-foreground", "{props.title}" }
                                }
                                
                                button {
                                    class: "flex items-center text-muted-foreground hover:text-foreground transition-colors",
                                    onclick: move |_| {
                                        let current_expanded = props.expanded.unwrap_or_else(|| internal_expanded());
                                        let new_state = !current_expanded;
                                        if let Some(handler) = &props.on_expanded_change {
                                            handler.call(new_state);
                                        } else {
                                            internal_expanded.set(new_state);
                                        }
                                    },
                                    if is_expanded {
                                        span { class: "text-2xl font-bold leading-none", "×" }
                                    } else {
                                        i { class: "fas fa-chevron-down text-lg" }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Reserve space for header when collapsed
                div { 
                    class: "h-16", // 4rem height to reserve space
                }
            }
        }
    } else {
        // Simple static header for non-expandable content
        rsx! {
            div { 
                class: "flex items-center space-x-3 p-4 border-b border-border bg-background",
                
                // Show back button if provided (for mobile) or sidebar toggle (for desktop)
                if let Some(back_btn) = props.back_button {
                    {back_btn}
                } else if let Some(mut ctx) = sidebar_ctx {
                    // Show open button only when sidebar is closed
                    if !(ctx.is_visible)() {
                        button {
                            class: "flex items-center text-muted-foreground hover:text-foreground transition-colors",
                            onclick: move |_| {
                                ctx.is_visible.set(true);
                            },
                            span { class: "text-2xl font-bold leading-none", "»" }
                        }
                    }
                }
                
                // Title section
                div { class: "flex-1",
                    h1 { class: "text-xl font-semibold text-foreground", "{props.title}" }
                }
            }
        }
    }
}