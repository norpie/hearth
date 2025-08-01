//! Label component for form fields with accessibility support
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     Label {
//!         r#for: "email-input",
//!         "Email Address"
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Properties for the Label component
#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    /// Label content
    pub children: Element,
    /// HTML for attribute to associate with a form control
    #[props(default = String::new())]
    pub r#for: String,
    /// Whether to display a required field indicator
    #[props(default = false)]
    pub required: bool,
    /// Additional CSS classes to apply
    #[props(default = String::new())]
    pub class: String,
}

/// Semantic label component with accessibility and required field support
#[component]
pub fn Label(props: LabelProps) -> Element {
    let base_classes = "block text-sm font-medium text-foreground";

    let combined_classes = if props.class.is_empty() {
        base_classes.to_string()
    } else {
        format!("{} {}", base_classes, props.class)
    };

    rsx! {
        label {
            class: combined_classes,
            r#for: if props.r#for.is_empty() { None } else { Some(props.r#for.as_str()) },
            {props.children}
            if props.required {
                span { class: "ml-1 text-destructive-foreground", "*" }
            }
        }
    }
}
