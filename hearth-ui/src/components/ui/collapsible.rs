//! Collapsible component for expandable content
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     Collapsible {
//!         trigger: "Click to expand".to_string(),
//!         p { "Hidden content here" }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Properties for the Collapsible component
#[derive(Props, Clone, PartialEq)]
pub struct CollapsibleProps {
    /// Text label displayed on the trigger button
    pub trigger: String,
    /// Whether the collapsible starts in expanded state
    #[props(default = false)]
    pub default_open: bool,
    /// Whether the collapsible interaction is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Additional CSS classes for the root container
    #[props(default = String::new())]
    pub class: String,
    /// Additional CSS classes for the trigger button element
    #[props(default = String::new())]
    pub trigger_class: String,
    #[props(default = String::new())]
    pub content_class: String,

    /// Content to display when expanded
    pub children: Element,
}

/// Collapsible component for expandable content
#[component]
pub fn Collapsible(props: CollapsibleProps) -> Element {
    let mut is_open = use_signal(|| props.default_open);

    let container_classes = format!("w-full {}", props.class);

    let trigger_classes = format!(
        "w-full flex items-center justify-between py-3 text-left font-medium text-foreground border-b border-border hover:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background transition-colors duration-200 {}{}",
        if props.disabled { "opacity-50 cursor-not-allowed " } else { "cursor-pointer " },
        props.trigger_class
    );

    let content_classes = format!(
        "overflow-hidden transition-all duration-300 ease-in-out {}",
        if is_open() {
            "max-h-screen opacity-100"
        } else {
            "max-h-0 opacity-0"
        },
    );

    let inner_content_classes = format!("pt-4 pb-2 {}", props.content_class);

    rsx! {
        div { class: "{container_classes}",
            button {
                r#type: "button",
                class: "{trigger_classes}",
                disabled: props.disabled,
                "aria-expanded": is_open(),
                onclick: move |_| {
                    if !props.disabled {
                        is_open.set(!is_open());
                    }
                },
                span { class: "text-sm", "{props.trigger}" }
                span { class: "ml-2 flex-shrink-0",
                    svg {
                        class: format!(
                            "w-5 h-5 transition-transform duration-200 {}",
                            if is_open() { "rotate-180" } else { "rotate-0" },
                        ),
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        stroke: "currentColor",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                            d: "M19 9l-7 7-7-7",
                        }
                    }
                }
            }
            div { class: "{content_classes}",
                div { class: "{inner_content_classes}", {props.children} }
            }
        }
    }
}
