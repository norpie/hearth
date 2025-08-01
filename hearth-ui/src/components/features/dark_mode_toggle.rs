//! Dark mode toggle component
//!
//! This module provides a toggle component for switching between light and dark themes.
//! It uses the application's dark mode context to manage theme state.

use crate::models::DarkModeContext;
use crate::Switch;
use dioxus::prelude::*;

// Dark mode toggle component
#[component]
pub fn DarkModeToggle() -> Element {
    let mut dark_mode_ctx = use_context::<DarkModeContext>();

    rsx! {
        Switch {
            checked: (dark_mode_ctx.is_dark)(),
            onchange: move |checked| {
                dark_mode_ctx.is_dark.set(checked);
            },
            aria_label: "Toggle dark mode".to_string(),
        }
    }
}