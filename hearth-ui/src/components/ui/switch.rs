//! Toggle switch component for on/off controls
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     Switch {
//!         checked: is_enabled(),
//!         onchange: move |enabled| is_enabled.set(enabled),
//!     }
//! }

use dioxus::prelude::*;

/// Size variants for switches
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwitchSize {
    /// Small switch size for compact layouts
    Small,
    /// Medium switch size (default) for standard use
    Medium,
    /// Large switch size for prominent controls
    Large,
}

impl Default for SwitchSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Properties for configuring the Switch component
#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    /// Whether the switch is currently in the "on" (checked) state
    /// 
    /// Defaults to false (off state).
    #[props(default = false)]
    pub checked: bool,

    /// Callback function called when the switch state changes
    /// 
    /// Receives the new switch state as a boolean parameter. Optional.
    #[props(default)]
    pub onchange: Option<EventHandler<bool>>,

    /// Whether the switch is disabled
    /// 
    /// When true, prevents interaction and applies disabled styling.
    /// Defaults to false.
    #[props(default = false)]
    pub disabled: bool,

    /// Size variant of the switch
    /// 
    /// Controls the dimensions of both track and thumb elements.
    /// Defaults to `SwitchSize::Medium`.
    #[props(default = SwitchSize::default())]
    pub size: SwitchSize,

    /// Additional CSS classes to apply to the switch track
    /// 
    /// Custom classes are appended to the switch's base styling.
    #[props(default = String::new())]
    pub class: String,

    /// HTML id attribute for accessibility and form association
    /// 
    /// Used to associate labels with the switch. Defaults to empty string.
    #[props(default = String::new())]
    pub id: String,

    /// ARIA label for accessibility when no visible label exists
    /// 
    /// Provides an accessible name for screen readers when the switch
    /// doesn't have an associated visible label. Defaults to empty string.
    #[props(default = String::new())]
    pub aria_label: String,
}

/// Interactive toggle switch for binary on/off controls
/// 
/// The Switch component provides a toggle interface with smooth animations
/// and accessibility support. It's designed for settings, preferences, and
/// feature toggles where the on/off state needs clear visual communication.
/// 
/// # Accessibility
/// 
/// - Uses semantic button element with ARIA switch role
/// - Keyboard navigation support with focus indicators
/// - Screen reader compatible with proper state announcements
/// - Support for form labels via id attribute
/// 
/// # Styling
/// 
/// The component combines CSS classes for smooth transitions, hover effects,
/// and responsive scaling. The thumb slides between positions with CSS transforms
/// for performance-optimized animations.
/// 
/// # Parameters
/// 
/// - `checked`: Current on/off state (controlled component)
/// - `onchange`: State change callback function
/// - `disabled`: Prevents interaction when true
/// - `size`: Switch dimensions (Small, Medium, Large)
/// - `class`: Additional CSS classes for track styling
/// - `id`: HTML id for form association and accessibility
/// - `aria_label`: Accessible name when no visible label exists
#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let (track_classes, thumb_classes) = get_size_classes(props.size);

    let track_base_classes = "rounded-full relative transition-all duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background";
    let track_state_classes = if props.checked {
        if props.disabled {
            "bg-primary/30 cursor-not-allowed opacity-50"
        } else {
            "bg-primary cursor-pointer hover:bg-primary/90 active:scale-95"
        }
    } else if props.disabled {
        "bg-muted cursor-not-allowed opacity-50"
    } else {
        "bg-muted cursor-pointer hover:bg-muted/80 active:scale-95"
    };

    let thumb_base_classes = "bg-background rounded-full absolute top-1 left-1 transition-all duration-300 ease-in-out shadow-md";

    let thumb_position_classes = if props.checked {
        match props.size {
            SwitchSize::Small => "translate-x-3 scale-110", // 12px - should work according to Tailwind docs
            SwitchSize::Medium => "translate-x-5 scale-110", // 20px - WORKING
            SwitchSize::Large => "translate-x-6 scale-110", // 24px - should work according to Tailwind docs
        }
    } else {
        "translate-x-0 scale-100"
    };

    let combined_track_classes = if props.class.is_empty() {
        format!("{track_base_classes} {track_classes} {track_state_classes}")
    } else {
        format!(
            "{} {} {} {}",
            track_base_classes, track_classes, track_state_classes, props.class
        )
    };

    let combined_thumb_classes =
        format!("{thumb_base_classes} {thumb_classes} {thumb_position_classes}");

    rsx! {
        button {
            class: combined_track_classes,
            id: if props.id.is_empty() { None } else { Some(props.id.as_str()) },
            "aria-label": if props.aria_label.is_empty() { None } else { Some(props.aria_label.as_str()) },
            "aria-checked": props.checked.to_string(),
            role: "switch",
            disabled: props.disabled,
            onclick: move |_| {
                if !props.disabled {
                    if let Some(onchange) = &props.onchange {
                        onchange.call(!props.checked);
                    }
                }
            },
            div { class: combined_thumb_classes }
        }
    }
}

/// Returns CSS classes for switch track and thumb based on size variant
/// 
/// Maps each size variant to appropriate styling classes for both the switch
/// track container and the moveable thumb element.
fn get_size_classes(size: SwitchSize) -> (&'static str, &'static str) {
    match size {
        SwitchSize::Small => (
            "w-8 h-5", // track: 32x20px
            "w-3 h-3", // thumb: 12x12px
        ),
        SwitchSize::Medium => (
            "w-12 h-7", // track: 48x28px
            "w-5 h-5",  // thumb: 20x20px
        ),
        SwitchSize::Large => (
            "w-14 h-8", // track: 56x32px
            "w-6 h-6",  // thumb: 24x24px
        ),
    }
}
