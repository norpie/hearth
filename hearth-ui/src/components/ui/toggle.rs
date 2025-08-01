//! Toggle button component for on/off states
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     Toggle {
//!         pressed: is_bold(),
//!         onclick: move |_| toggle_bold(),
//!         "Bold"
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Size variants for toggle buttons
#[derive(Clone, PartialEq)]
pub enum ToggleSize {
    /// Small size for compact layouts
    Small,
    /// Medium size for standard use
    Medium,
    /// Large size for prominent display
    Large,
}

/// Visual style variants for toggle buttons
#[derive(Clone, PartialEq)]
pub enum ToggleVariant {
    /// Default ghost button style
    Default,
    /// Outlined button style
    Outline,
}

/// Properties for the Toggle component
#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    /// Whether the toggle is currently pressed
    #[props(default = false)]
    pub pressed: bool,

    /// Whether the toggle is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Size variant of the toggle
    #[props(default = ToggleSize::Medium)]
    pub size: ToggleSize,

    /// Visual style variant
    #[props(default = ToggleVariant::Default)]
    pub variant: ToggleVariant,

    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,

    /// Click event handler
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// Content to display inside the button
    pub children: Element,
}

impl ToggleSize {
    /// Returns CSS classes for toggle sizing
    pub fn classes(&self) -> &'static str {
        match self {
            ToggleSize::Small => "px-2.5 py-1.5 text-sm",
            ToggleSize::Medium => "px-3 py-2 text-base",
            ToggleSize::Large => "px-4 py-2.5 text-lg",
        }
    }
}

impl ToggleVariant {
    /// Returns CSS classes for toggle styling
    pub fn classes(&self, pressed: bool) -> &'static str {
        match (self, pressed) {
            (ToggleVariant::Default, false) => "bg-transparent hover:bg-accent active:bg-accent/80 text-foreground border-transparent",
            (ToggleVariant::Default, true) => "bg-accent text-accent-foreground border-transparent",
            (ToggleVariant::Outline, false) => "bg-transparent hover:bg-accent active:bg-accent/80 text-foreground border-border hover:border-border/80",
            (ToggleVariant::Outline, true) => "bg-accent text-accent-foreground border-border",
        }
    }
}

/// Interactive toggle button for on/off states
#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let base_classes = "inline-flex items-center justify-center rounded-md border font-medium transition-all duration-150 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none active:scale-95 select-none";

    let variant_classes = props.variant.classes(props.pressed);
    let size_classes = props.size.classes();

    let combined_classes = format!(
        "{} {} {} {}",
        base_classes, variant_classes, size_classes, props.class
    );

    rsx! {
        button {
            r#type: "button",
            class: "{combined_classes}",
            disabled: props.disabled,
            "aria-pressed": props.pressed,
            onclick: move |evt| {
                if !props.disabled {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                }
            },
            {props.children}
        }
    }
}
