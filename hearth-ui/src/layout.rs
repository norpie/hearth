//! Core platform-adaptive layout components
//!
//! This module provides the main application layout orchestration components that
//! handle platform detection and render the appropriate layout structure for each
//! platform (mobile, desktop, web). These components focus purely on layout
//! structure and platform adaptation without containing specific UI features.

use crate::models::*;
use crate::routes::Route;
use crate::{GestureDetector, GestureDirection, ToastManager};
use crate::components::features::navigation::{AppSidebar, BottomNavigation};
use dioxus::prelude::*;

// Unified adaptive layout that switches between sidebar and bottom nav
#[derive(Props, Clone, PartialEq)]
pub struct AdaptiveLayoutProps {
    pub current_route: Route,
    pub navigate_to: EventHandler<Route>,
    pub children: Element,
    #[props(default = "Hearth")]
    pub app_name: &'static str,
    #[props(default = "v1.0.0")]
    pub app_version: &'static str,
}

#[component]
pub fn AdaptiveLayout(props: AdaptiveLayoutProps) -> Element {
    let platform = Platform::current();

    match platform {
        Platform::Mobile => rsx! {
            MobileLayout {
                current_route: props.current_route,
                navigate_to: props.navigate_to,
                {props.children}
            }
        },
        _ => rsx! {
            DesktopLayout {
                current_route: props.current_route,
                navigate_to: props.navigate_to,
                app_name: props.app_name,
                {props.children}
            }
        },
    }
}

// Mobile layout with bottom navigation
#[component]
pub fn MobileLayout(
    current_route: Route,
    navigate_to: EventHandler<Route>,
    children: Element,
) -> Element {
    // State for controlling bottom navbar visibility
    let mut navbar_visible = use_signal(|| true);
    let show_bottom_nav = true;
    
    // Provide mobile navbar context so Story view can access it
    use_context_provider(|| MobileNavbarContext {
        is_visible: navbar_visible,
    });
    
    // Check if we're in Story view - only enable navbar gestures in Story view
    let is_story_view = matches!(current_route, Route::Story { .. });
    
    // Reset navbar visibility when leaving Story view
    use_effect(use_reactive((&current_route,), move |_| {
        if !is_story_view {
            navbar_visible.set(true);
        }
    }));
    
    // Toast manager for navbar state changes
    let _toast_manager = use_context::<ToastManager>();

    // Handle gesture events for navbar (only in Story view)
    let handle_navbar_gesture = move |direction: GestureDirection| {
        if !is_story_view {
            return; // Don't handle gestures outside Story view
        }
        
        match direction {
            GestureDirection::Right => {
                // Swipe right opens nav
                if !navbar_visible() {
                    navbar_visible.set(true);
                }
            },
            GestureDirection::Left => {
                // Swipe left closes nav (only when navbar is visible)
                if navbar_visible() {
                    navbar_visible.set(false);
                }
            },
            _ => {
                // Ignore up/down swipes for navbar control
            }
        }
    };
    
    // Input bar gesture handling is done in the Story view itself

    rsx! {
        div { class: "h-screen w-full flex flex-col bg-background overflow-hidden relative",
            // Main content area - fills remaining space
            div { 
                class: format!(
                    "flex-1 flex flex-col min-h-0 transition-all duration-300 ease-in-out {}",
                    if navbar_visible() && show_bottom_nav { "pb-20" } else { "pb-0" }
                ),
                {children} 
            }
            
            // Bottom navigation with slide animation AND gesture detection (only in Story view when visible)
            if show_bottom_nav {
                div {
                    class: format!(
                        "fixed bottom-0 left-0 right-0 z-50 transition-transform duration-300 ease-in-out {}",
                        if navbar_visible() { "translate-x-0" } else { "-translate-x-full" }
                    ),
                    // Only wrap with gesture detector in Story view
                    if is_story_view {
                        GestureDetector {
                            class: "w-full h-full",
                            debug: false,
                            on_gesture: handle_navbar_gesture,
                            BottomNavigation { current_route, navigate_to }
                        }
                    } else {
                        BottomNavigation { current_route, navigate_to }
                    }
                }
            }
        }
    }
}

// Desktop layout with sidebar navigation
#[component]
pub fn DesktopLayout(
    current_route: Route,
    navigate_to: EventHandler<Route>,
    app_name: &'static str,
    children: Element,
) -> Element {
    let sidebar_visible = use_signal(|| true);

    // Provide sidebar context - always provide it so PageHeader can access it
    use_context_provider(|| SidebarContext {
        is_visible: sidebar_visible,
    });

    rsx! {
        div { class: "h-screen w-full flex bg-background overflow-hidden",
            // Sidebar navigation with slide animation
            div {
                class: format!(
                    "flex-shrink-0 overflow-hidden transition-all duration-300 ease-in-out bg-sidebar border-r border-sidebar-border {}",
                    if sidebar_visible() { "w-64" } else { "w-0" },
                ),
                AppSidebar {
                    current_route: current_route.clone(),
                    navigate_to,
                    app_name,
                    routes: vec![
                        SidebarRoute::new(Route::Stories, Route::Stories.icon(), Route::Stories.label()),
                        SidebarRoute::new(
                            Route::Characters,
                            Route::Characters.icon(),
                            Route::Characters.label(),
                        ),
                        SidebarRoute::new(
                            Route::Scenarios,
                            Route::Scenarios.icon(),
                            Route::Scenarios.label(),
                        ),
                        SidebarRoute::new(Route::Design, Route::Design.icon(), Route::Design.label()),
                    ],
                    bottom_routes: vec![
                        SidebarRoute::new(
                            Route::Settings,
                            Route::Settings.icon(),
                            Route::Settings.label(),
                        ),
                    ],
                }
            }
            // Main content area
            div { class: "flex-1 flex flex-col min-w-0", {children} }
        }
    }
}