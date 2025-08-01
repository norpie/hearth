//! Accordion component for collapsible sections
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     Accordion {
//!         items: vec![
//!             AccordionItem::new("id1", "Section 1", rsx! { p { "Content 1" } }),
//!             AccordionItem::new("id2", "Section 2", rsx! { p { "Content 2" } }),
//!         ],
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Individual accordion section
#[derive(Clone, PartialEq)]
pub struct AccordionItem {
    /// Unique identifier for this section
    pub id: String,
    /// Text displayed on the trigger button
    pub trigger: String,
    /// Whether this item is disabled
    pub disabled: bool,
    /// Content to display when expanded
    pub content: Element,
}

impl AccordionItem {
    /// Creates a new accordion item
    pub fn new(id: impl Into<String>, trigger: impl Into<String>, content: Element) -> Self {
        Self {
            id: id.into(),
            trigger: trigger.into(),
            disabled: false,
            content,
        }
    }

    /// Marks this item as disabled
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Properties for the Accordion component
#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    /// Collection of accordion sections to display
    pub items: Vec<AccordionItem>,
    /// ID of the section that should be initially expanded
    #[props(default)]
    pub default_open: Option<String>,
    /// Whether all sections can be closed simultaneously
    #[props(default = true)]
    pub collapsible: bool,
    /// Additional CSS classes for the accordion container
    #[props(default = String::new())]
    pub class: String,
    /// Additional CSS classes for individual accordion items
    #[props(default = String::new())]
    pub item_class: String,
}

/// Accordion component for collapsible sections
#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    let mut open_item = use_signal(|| props.default_open.clone());

    let container_classes = format!("w-full space-y-1 {}", props.class);

    rsx! {
        div { class: "{container_classes}",
            for item in props.items.iter() {
                {
                    let item_id = item.id.clone();
                    let is_open = open_item.read().as_ref() == Some(&item.id);
                    let item_disabled = item.disabled;

                    let item_classes = format!("w-full {}", props.item_class);
                    let trigger_classes = format!(
                        "w-full flex items-center justify-between py-3 text-left font-medium text-foreground border-b border-border hover:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background transition-colors duration-200 {}",
                        if item_disabled {
                            "opacity-50 cursor-not-allowed "
                        } else {
                            "cursor-pointer "
                        },
                    );
                    let content_classes = format!(
                        "overflow-hidden transition-all duration-300 ease-in-out {}",
                        if is_open { "max-h-screen opacity-100" } else { "max-h-0 opacity-0" },
                    );
                    rsx! {
                        div { key: "{item.id}", class: "{item_classes}",
                            button {
                                r#type: "button",
                                class: "{trigger_classes}",
                                disabled: item_disabled,
                                "aria-expanded": is_open,
                                onclick: move |_| {
                                    if !item_disabled {
                                        if is_open && props.collapsible {
                                            open_item.set(None);
                                        } else if !is_open {
                                            open_item.set(Some(item_id.clone()));
                                        }
                                    }
                                },
                                span { class: "text-sm font-medium", "{item.trigger}" }
                                span { class: "ml-2 flex-shrink-0",
                                    svg {
                                        class: format!(
                                            "w-5 h-5 transition-transform duration-200 {}",
                                            if is_open { "rotate-180" } else { "rotate-0" },
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
                                div { class: "pt-4 pb-2", {item.content.clone()} }
                            }
                        }
                    }
                }
            }
        }
    }
}
