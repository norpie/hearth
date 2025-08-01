//! Platform-agnostic layout components

use crate::models::*;
use crate::routes::Route;
use dioxus::prelude::*;

// Generic page header component
#[derive(Props, Clone, PartialEq)]
pub struct PageHeaderProps {
    pub title: String,
    #[props(default = None)]
    pub back_button: Option<Element>,
}

#[component]
pub fn PageHeader(props: PageHeaderProps) -> Element {
    // Try to get sidebar context - it might not exist on mobile
    let sidebar_ctx = try_use_context::<SidebarContext>();

    rsx! {
        div { class: "flex-shrink-0 bg-white dark:bg-gray-950",
            div { class: "flex items-center space-x-3 m-4",
                // Show back button if provided (for mobile)
                if let Some(back_btn) = props.back_button {
                    {back_btn}
                } else if let Some(mut ctx) = sidebar_ctx {
                    // Show open button only when sidebar is closed
                    if !(ctx.is_visible)() {
                        button {
                            class: "flex items-center text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors",
                            onclick: move |_| {
                                ctx.is_visible.set(true);
                            },
                            span { class: "text-2xl font-bold leading-none", "»" }
                        }
                    }
                }
                div {
                    h1 { class: "text-xl font-semibold text-gray-900 dark:text-white",
                        "{props.title}"
                    }
                }
            }
        }
    }
}




// Dark mode toggle component
#[component]
pub fn DarkModeToggle() -> Element {
    let mut dark_mode_ctx = use_context::<DarkModeContext>();

    rsx! {
        button {
            class: if (dark_mode_ctx.is_dark)() { "w-12 h-7 rounded-full relative transition-colors bg-blue-600 dark:bg-blue-500" } else { "w-12 h-7 rounded-full relative transition-colors bg-gray-300" },
            onclick: move |_| {
                let current = (dark_mode_ctx.is_dark)();
                dark_mode_ctx.is_dark.set(!current);
            },
            div { class: if (dark_mode_ctx.is_dark)() { "w-5 h-5 bg-white rounded-full absolute top-1 transition-transform right-1" } else { "w-5 h-5 bg-white rounded-full absolute top-1 transition-transform left-1" } }
        }
    }
}

// Shared sidebar layout component
#[component]
pub fn AppSidebar<T: AppRoute + Clone + PartialEq + 'static>(
    current_route: T,
    navigate_to: EventHandler<T>,
    app_name: &'static str,
    routes: Vec<SidebarRoute<T>>,
    #[props(default = vec![])]
    bottom_routes: Vec<SidebarRoute<T>>,
) -> Element {
    let mut sidebar_ctx = use_context::<SidebarContext>();

    rsx! {
        div { class: "w-64 bg-white dark:bg-gray-950 flex flex-col justify-between border-r border-gray-200 dark:border-gray-700 relative z-10",
            // Top section
            div { class: "flex flex-col",
                // Logo/Brand header with close button to the left
                div { class: "pt-4 px-4",
                    div { class: "flex items-center justify-between",
                        button {
                            class: "flex items-center text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors",
                            onclick: move |_| {
                                let current = (sidebar_ctx.is_visible)();
                                sidebar_ctx.is_visible.set(!current);
                            },
                            span { class: "text-2xl font-bold leading-none", "«" }
                        }
                        div { class: "flex items-center space-x-3",
                            crate::Logo { width: 28, height: 28 }
                            span { class: "text-xl font-semibold text-gray-900 dark:text-white",
                                "{app_name}"
                            }
                        }
                    }
                }

                // Navigation
                nav { class: "px-4 pt-4 space-y-1",
                    for route in &routes {
                        if route.is_separator {
                            div { class: "my-4 border-t border-gray-200 dark:border-gray-700" }
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
                    "bg-blue-50 dark:bg-blue-950/50 text-blue-600 dark:text-blue-400"
                } else {
                    "text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-900/50"
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
        }
    }
}

// Mobile layout with bottom navigation
#[component]
pub fn MobileLayout(
    current_route: Route,
    navigate_to: EventHandler<Route>,
    children: Element,
) -> Element {
    let show_bottom_nav = !current_route.is_chat_route();

    rsx! {
        div { class: "h-screen w-full flex flex-col bg-white dark:bg-gray-950 overflow-hidden",
            // Main content area - fills remaining space
            div { class: "flex-1 flex flex-col min-h-0", {children} }
            // Bottom navigation (hidden in chat)
            if show_bottom_nav {
                BottomNavigation { current_route, navigate_to }
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
    let sidebar_visible = use_signal(|| !current_route.is_chat_route());
    
    // Provide sidebar context
    use_context_provider(|| SidebarContext { is_visible: sidebar_visible });
    
    rsx! {
        div { class: "h-screen w-full flex bg-white dark:bg-gray-950 overflow-hidden",
            // Sidebar navigation (hidden in chat)
            if sidebar_visible() {
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
                        SidebarRoute::new(
                            Route::Design,
                            Route::Design.icon(),
                            Route::Design.label(),
                        ),
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
        div { class: "flex-shrink-0 bg-white dark:bg-gray-950 safe-area-inset-bottom",
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
    navigate_to: EventHandler<Route>
) -> Element {
    let is_active = std::mem::discriminant(&route) == std::mem::discriminant(&current_route);

    rsx! {
        button {
            class: format!(
                "flex flex-col items-center space-y-1 px-3 py-2 rounded-lg transition-colors min-w-0 flex-1 {}",
                if is_active {
                    "text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-950/50"
                } else {
                    "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white"
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
