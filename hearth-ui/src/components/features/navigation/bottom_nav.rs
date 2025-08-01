//! Mobile bottom navigation components
//!
//! This module provides bottom navigation components for mobile platforms.
//! The bottom navigation provides tab-based navigation optimized for touch interaction.

use crate::routes::Route;
use dioxus::prelude::*;

// Bottom navigation for mobile
#[component]
pub fn BottomNavigation(current_route: Route, navigate_to: EventHandler<Route>) -> Element {
    let available_routes = vec![
        Route::Stories,
        Route::Characters,
        Route::Scenarios,
        Route::Design,
        Route::Settings,
    ];

    rsx! {
        div { class: "flex-shrink-0 bg-sidebar border-t border-sidebar-border",
            div { class: "flex items-center justify-around py-3 px-4",
                for route in available_routes {
                    MobileNavItem {
                        route: route.clone(),
                        current_route: current_route.clone(),
                        icon: route.icon(),
                        label: route.label(),
                        navigate_to,
                    }
                }
            }
        }
    }
}

// Mobile navigation item
#[component]
pub fn MobileNavItem(
    route: Route,
    current_route: Route,
    icon: &'static str,
    label: &'static str,
    navigate_to: EventHandler<Route>,
) -> Element {
    let is_active = std::mem::discriminant(&route) == std::mem::discriminant(&current_route);

    rsx! {
        button {
            class: format!(
                "flex flex-col items-center space-y-1 px-3 py-2 rounded-lg transition-colors min-w-0 flex-1 {}",
                if is_active {
                    "text-sidebar-primary bg-sidebar-accent"
                } else {
                    "text-sidebar-foreground hover:bg-sidebar-accent"
                },
            ),
            onclick: move |_| navigate_to.call(route.clone()),
            div { class: "text-xl",
                i { class: "{icon}" }
            }
            div { class: "text-xs font-medium truncate", "{label}" }
        }
    }
}