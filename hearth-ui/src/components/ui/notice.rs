//! Notice components for displaying important messages and alerts
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     Notice {
//!         variant: Some(NoticeVariant::Success),
//!         icon: "fas fa-check-circle".to_string(),
//!         "Your changes have been saved successfully!"
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Semantic variants for notice message types
#[derive(Clone, PartialEq, Default)]
pub enum NoticeVariant {
    /// Success state styling with green colors
    Success,
    /// Warning state styling with yellow/orange colors
    Warning,
    /// Error/destructive state styling with red colors
    Destructive,
    /// Informational styling with blue colors (default)
    #[default]
    Info,
}

/// Properties for the Notice component
#[derive(Props, Clone, PartialEq)]
pub struct NoticeProps {
    /// Semantic variant determining the visual style
    pub variant: Option<NoticeVariant>,
    /// CSS classes for the icon element
    pub icon: String,
    /// Content to display within the notice
    pub children: Element,
    /// Additional HTML attributes to apply to the notice container
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// Notice component for displaying important messages and alerts
#[component]
pub fn Notice(props: NoticeProps) -> Element {
    let variant = props.variant.unwrap_or_default();

    let (background_class, border_class, text_class) = match variant {
        NoticeVariant::Success => (
            "bg-success/10",
            "border-success/20",
            "text-success-foreground",
        ),
        NoticeVariant::Warning => ("bg-warning/10", "border-warning/20", "text-warning"),
        NoticeVariant::Destructive => (
            "bg-destructive/10",
            "border-destructive/20",
            "text-destructive-foreground",
        ),
        NoticeVariant::Info => ("bg-info/10", "border-info/20", "text-info-foreground"),
    };

    rsx! {
        div {
            class: format!("{} {} rounded-md p-3 border", background_class, border_class),
            ..props.attributes,
            div { class: "flex items-start gap-2",
                i { class: format!("{} {} text-sm flex-shrink-0 mt-0.5", props.icon, text_class) }
                div { class: format!("{} text-sm", text_class), {props.children} }
            }
        }
    }
}
