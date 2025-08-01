//! Icon-only toggle button component
//!
//! The ToggleIcon component provides a minimalist toggle button that displays only an icon
//! with color changes to indicate state. Perfect for compact UIs where space is premium
//! and the icon itself communicates the action clearly.
//!
//! # Features
//!
//! - **Icon-only design**: No text, just the icon for minimal footprint
//! - **Color-based state**: Different colors for active/inactive states
//! - **Accessible**: Proper ARIA attributes and keyboard support
//! - **Flexible sizing**: Multiple size variants
//! - **Hover effects**: Visual feedback on interaction
//! - **Customizable**: Support for any FontAwesome icon and custom colors
//!
//! # Examples
//!
//! Basic favorite toggle:
//! ```rust
//! let mut is_favorite = use_signal(|| false);
//! rsx! {
//!     IconToggle {
//!         icon: "fa-star",
//!         is_active: is_favorite(),
//!         active_color: "text-yellow-400",
//!         inactive_color: "text-muted-foreground",
//!         onclick: move |_| is_favorite.set(!is_favorite()),
//!         aria_label: "Toggle favorite",
//!     }
//! }
//! ```
//!
//! Large bookmark toggle with custom colors:
//! ```rust
//! rsx! {
//!     IconToggle {
//!         icon: "fa-bookmark",
//!         is_active: bookmark_state(),
//!         active_color: "text-blue-500",
//!         inactive_color: "text-gray-400",
//!         size: IconToggleSize::Large,
//!         onclick: move |_| toggle_bookmark(),
//!         aria_label: "Toggle bookmark",
//!     }
//! }
//! ```

use dioxus::prelude::*;
use crate::{Popover, PopoverTrigger, PopoverPlacement};

/// Size variants for the toggle icon button
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToggleIconSize {
    /// Small icon toggle (16px)
    Small,
    /// Medium icon toggle (20px) - default
    Medium,
    /// Large icon toggle (24px)
    Large,
}

impl Default for ToggleIconSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Properties for the ToggleIcon component
#[derive(Props, Clone, PartialEq)]
pub struct ToggleIconProps {
    /// FontAwesome icon class (e.g., "fa-star", "fa-heart", "fa-bookmark")
    pub icon: String,
    
    /// Whether the toggle is currently active
    #[props(default = false)]
    pub is_active: bool,
    
    /// Color class when active (e.g., "text-yellow-400", "text-red-500")
    #[props(default = "text-primary".to_string())]
    pub active_color: String,
    
    /// Color class when inactive (e.g., "text-muted-foreground", "text-gray-400")
    #[props(default = "text-muted-foreground".to_string())]
    pub inactive_color: String,
    
    /// Click handler
    pub onclick: EventHandler<MouseEvent>,
    
    /// Size variant of the icon toggle
    #[props(default = ToggleIconSize::default())]
    pub size: ToggleIconSize,
    
    /// ARIA label for accessibility
    #[props(default = String::new())]
    pub aria_label: String,
    
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
    
    /// Whether the toggle is disabled
    #[props(default = false)]
    pub disabled: bool,
    
    /// Optional tooltip text to show on hover
    #[props(default = String::new())]
    pub tooltip: String,
}

/// Icon-only toggle button with color-based state indication
///
/// A minimalist toggle button that uses only an icon and color changes to communicate state.
/// Perfect for compact interfaces where space is at a premium and the icon clearly
/// communicates the action (like favorites, bookmarks, likes, etc.).
///
/// The component automatically handles:
/// - Solid FontAwesome icons for consistent appearance
/// - Color transitions between active/inactive states
/// - Dynamic background colors based on the icon's active color
/// - Hover effects and accessibility
/// - Proper button semantics with ARIA attributes
#[component]
pub fn ToggleIcon(props: ToggleIconProps) -> Element {
    let size_classes = get_size_classes(props.size);
    
    // Extract color information to generate background and border colors
    let (bg_color, border_color) = if props.is_active {
        derive_background_colors(&props.active_color)
    } else {
        ("bg-background".to_string(), "border-border".to_string())
    };
    
    // Always use solid icons for consistency
    let icon_class = format!(
        "fa-solid {} {} leading-none",
        props.icon,
        if props.is_active {
            &props.active_color
        } else {
            &props.inactive_color
        }
    );
    
    let base_classes = "relative inline-flex items-center justify-center rounded-full border transition-all duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background flex-shrink-0";
    
    let state_classes = if props.disabled {
        "cursor-not-allowed opacity-50 border-border bg-muted".to_string()
    } else if props.is_active {
        format!("cursor-pointer hover:opacity-90 active:scale-95 {} {}", border_color, bg_color)
    } else {
        "cursor-pointer hover:bg-muted/50 active:scale-95 border-border bg-background".to_string()
    };
    
    let combined_classes = if props.class.is_empty() {
        format!("{} {} {}", base_classes, size_classes, state_classes)
    } else {
        format!("{} {} {} {}", base_classes, size_classes, state_classes, props.class)
    };
    
    let button_element = rsx! {
        button {
            r#type: "button",
            class: combined_classes,
            "aria-label": if props.aria_label.is_empty() { None } else { Some(props.aria_label.as_str()) },
            "aria-pressed": props.is_active.to_string(),
            disabled: props.disabled,
            onclick: move |evt| {
                if !props.disabled {
                    props.onclick.call(evt);
                }
            },
            i {
                class: format!("{}", icon_class),
                style: "display: block; line-height: 1; vertical-align: baseline;",
            }
        }
    };

    if props.tooltip.is_empty() {
        button_element
    } else {
        rsx! {
            Popover {
                trigger: PopoverTrigger::Hover,
                placement: PopoverPlacement::BottomStart,
                content_class: Some("p-2 text-xs whitespace-nowrap z-[9999]".to_string()),
                content: rsx! {
                    p { "{props.tooltip}" }
                },
                {button_element}
            }
        }
    }
}

/// Returns CSS classes for button size based on size variant
fn get_size_classes(size: ToggleIconSize) -> &'static str {
    match size {
        ToggleIconSize::Small => "w-8 h-8 text-sm",   // 32x32px button, 14px icon
        ToggleIconSize::Medium => "w-10 h-10 text-base", // 40x40px button, 16px icon  
        ToggleIconSize::Large => "w-12 h-12 text-lg",  // 48x48px button, 18px icon
    }
}

/// Derives background and border colors from the active icon color
/// Returns (background_color, border_color) as CSS class strings
fn derive_background_colors(active_color: &str) -> (String, String) {
    // Parse text-{color}-{shade} pattern
    if let Some(color_part) = active_color.strip_prefix("text-") {
        if let Some((color, shade_str)) = color_part.split_once('-') {
            if let Ok(shade) = shade_str.parse::<u32>() {
                // Use safe, widely available shades
                let bg_shade = match shade {
                    100..=200 => 50,   // Very light colors get 50
                    201..=400 => 100,  // Light colors get 100
                    401..=600 => 200,  // Medium colors get 200
                    601..=800 => 300,  // Dark colors get 300
                    _ => 100,          // Default to 100
                };
                
                let border_shade = match shade {
                    100..=300 => 600,  // Light colors get 600 border
                    301..=500 => 700,  // Medium colors get 700 border
                    501..=700 => 800,  // Dark colors get 800 border
                    _ => 900,          // Very dark colors get 900 border
                };
                
                return (
                    format!("bg-{}-{}", color, bg_shade),
                    format!("border-{}-{}", color, border_shade)
                );
            }
        }
    }
    
    // Fallback for invalid format
    ("bg-gray-100".to_string(), "border-gray-800".to_string())
}