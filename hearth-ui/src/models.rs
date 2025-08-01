//! Shared data models and types

use dioxus::prelude::*;
pub use hearth_core::{models::*, sample::*, settings::*, storage::*};

// Route trait for generic navigation
pub trait AppRoute: Clone + PartialEq {
    fn is_chat_route(&self) -> bool;
    fn get_route_name(&self) -> &'static str;
}

// Dark mode context for platform-agnostic theming
#[derive(Clone, Copy)]
pub struct DarkModeContext {
    pub is_dark: Signal<bool>,
}

// Platform detection utilities
#[derive(Clone, Copy, PartialEq)]
pub enum Platform {
    Mobile,
    Desktop,
    Web,
}

impl Platform {
    pub fn current() -> Self {
        if cfg!(feature = "mobile") {
            Platform::Mobile
        } else if cfg!(target_arch = "wasm32") {
            Platform::Web
        } else {
            Platform::Desktop
        }
    }

    pub fn is_mobile(&self) -> bool {
        matches!(self, Platform::Mobile)
    }

    pub fn uses_bottom_navigation(&self) -> bool {
        matches!(self, Platform::Mobile)
    }

    pub fn uses_sidebar(&self) -> bool {
        !self.uses_bottom_navigation()
    }
}

// Sidebar visibility context for web/desktop platforms
#[derive(Clone, Copy)]
pub struct SidebarContext {
    pub is_visible: Signal<bool>,
}

// Sidebar route configuration
#[derive(Clone, PartialEq)]
pub struct SidebarRoute<T> {
    pub route: Option<T>,
    pub icon: &'static str,
    pub label: &'static str,
    pub is_separator: bool,
}

impl<T> SidebarRoute<T> {
    pub fn new(route: T, icon: &'static str, label: &'static str) -> Self {
        Self {
            route: Some(route),
            icon,
            label,
            is_separator: false,
        }
    }
    
    pub fn separator() -> Self {
        Self {
            route: None,
            icon: "",
            label: "",
            is_separator: true,
        }
    }
}

