//! Desktop sidebar navigation components
//!
//! This module provides sidebar navigation components for desktop and web platforms.
//! The sidebar includes the app logo, navigation items, and collapsible functionality.

use crate::models::*;
use dioxus::prelude::*;

// Shared sidebar layout component
#[component]
pub fn AppSidebar<T: AppRoute + Clone + PartialEq + 'static>(
    current_route: T,
    navigate_to: EventHandler<T>,
    app_name: &'static str,
    routes: Vec<SidebarRoute<T>>,
    #[props(default = vec![])] bottom_routes: Vec<SidebarRoute<T>>,
) -> Element {
    let mut sidebar_ctx = use_context::<SidebarContext>();

    rsx! {
        div { class: "w-64 bg-sidebar flex flex-col justify-between border-r border-sidebar-border relative z-10 transition-transform duration-300 ease-in-out",
            // Top section
            div { class: "flex flex-col",
                // Logo/Brand header with close button to the left
                div { class: "pt-4 px-4",
                    div { class: "flex items-center justify-between",
                        button {
                            class: "flex items-center text-muted-foreground hover:text-foreground transition-colors",
                            onclick: move |_| {
                                let current = (sidebar_ctx.is_visible)();
                                sidebar_ctx.is_visible.set(!current);
                            },
                            span { class: "text-2xl font-bold leading-none", "«" }
                        }
                        div { class: "flex items-center space-x-3",
                            crate::Logo { width: 28, height: 28 }
                            span { class: "text-xl font-semibold text-sidebar-foreground",
                                "{app_name}"
                            }
                        }
                    }
                }

                // Navigation
                nav { class: "px-4 pt-4 space-y-1",
                    for route in &routes {
                        if route.is_separator {
                            div { class: "my-4 border-t border-sidebar-border" }
                        } else if let Some(route_value) = &route.route {
                            SidebarItem {
                                route: route_value.clone(),
                                current_route: current_route.clone(),
                                icon: route.icon,
                                label: route.label,
                                navigate_to,
                            }
                        }
                    }
                }
            }

            // Bottom section
            div { class: "flex flex-col",
                // Bottom navigation
                if !bottom_routes.is_empty() {
                    nav { class: "px-4 pb-4 space-y-1",
                        for route in &bottom_routes {
                            if let Some(route_value) = &route.route {
                                SidebarItem {
                                    route: route_value.clone(),
                                    current_route: current_route.clone(),
                                    icon: route.icon,
                                    label: route.label,
                                    navigate_to,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Generic sidebar item component
#[component]
pub fn SidebarItem<T: AppRoute + Clone + PartialEq + 'static>(
    route: T,
    current_route: T,
    icon: &'static str,
    label: &'static str,
    navigate_to: EventHandler<T>,
) -> Element {
    let is_active = std::mem::discriminant(&route) == std::mem::discriminant(&current_route);

    rsx! {
        button {
            class: format!(
                "w-full flex items-center space-x-3 px-3 py-3 rounded-lg text-left transition-colors {}",
                if is_active {
                    "bg-sidebar-accent text-sidebar-primary"
                } else {
                    "text-sidebar-foreground hover:bg-sidebar-accent"
                },
            ),
            onclick: move |_| navigate_to.call(route.clone()),
            span { class: "text-xl",
                i { class: "{icon}" }
            }
            span { class: "font-medium", "{label}" }
        }
    }
}