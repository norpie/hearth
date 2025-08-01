//! Accordion component for collapsible sections with exclusive expansion

use dioxus::prelude::*;

/// Accordion item structure
#[derive(Clone, PartialEq)]
pub struct AccordionItem {
    /// Unique identifier for the item
    pub id: String,
    /// The trigger label text
    pub trigger: String,
    /// Whether this item is disabled
    pub disabled: bool,
    /// Content to display when expanded
    pub content: Element,
}

impl AccordionItem {
    /// Create a new accordion item
    pub fn new(id: impl Into<String>, trigger: impl Into<String>, content: Element) -> Self {
        Self {
            id: id.into(),
            trigger: trigger.into(),
            disabled: false,
            content,
        }
    }
    
    /// Mark this accordion item as disabled
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Props for the Accordion component
#[derive(Props, Clone, PartialEq)]
pub struct AccordionProps {
    /// List of accordion items
    pub items: Vec<AccordionItem>,
    
    /// ID of the item that should be initially open (optional)
    #[props(default)]
    pub default_open: Option<String>,
    
    /// Whether to allow all items to be closed (if false, one must always be open)
    #[props(default = true)]
    pub collapsible: bool,
    
    /// Additional CSS classes for the container
    #[props(default = String::new())]
    pub class: String,
    
    /// Additional CSS classes for individual items
    #[props(default = String::new())]
    pub item_class: String,
}

#[component]
pub fn Accordion(props: AccordionProps) -> Element {
    // Track which item is currently open
    let mut open_item = use_signal(|| props.default_open.clone());
    
    let container_classes = format!(
        "w-full space-y-1 {}",
        props.class
    );
    
    rsx! {
        div {
            class: "{container_classes}",
            
            for item in props.items.iter() {
                {
                    let item_id = item.id.clone();
                    let is_open = open_item.read().as_ref() == Some(&item.id);
                    let item_disabled = item.disabled;
                    
                    let item_classes = format!(
                        "w-full {}",
                        props.item_class
                    );
                    
                    let trigger_classes = format!(
                        "w-full flex items-center justify-between py-3 text-left font-medium text-gray-900 dark:text-gray-100 border-b border-gray-200 dark:border-gray-700 hover:text-gray-700 dark:hover:text-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800 transition-colors duration-200 {}",
                        if item_disabled { "opacity-50 cursor-not-allowed " } else { "cursor-pointer " }
                    );
                    
                    let content_classes = format!(
                        "overflow-hidden transition-all duration-300 ease-in-out {}",
                        if is_open { "max-h-screen opacity-100" } else { "max-h-0 opacity-0" }
                    );
                    
                    rsx! {
                        div {
                            key: "{item.id}",
                            class: "{item_classes}",
                            
                            // Trigger button with animated chevron
                            button {
                                r#type: "button",
                                class: "{trigger_classes}",
                                disabled: item_disabled,
                                "aria-expanded": is_open,
                                onclick: move |_| {
                                    if !item_disabled {
                                        if is_open && props.collapsible {
                                            // Close current item if collapsible is true
                                            open_item.set(None);
                                        } else if !is_open {
                                            // Open this item (closes others automatically)
                                            open_item.set(Some(item_id.clone()));
                                        }
                                    }
                                },
                                
                                span {
                                    class: "text-sm font-medium",
                                    "{item.trigger}"
                                }
                                
                                // Animated chevron
                                span {
                                    class: "ml-2 flex-shrink-0",
                                    svg {
                                        class: format!("w-5 h-5 transition-transform duration-200 {}", 
                                            if is_open { "rotate-180" } else { "rotate-0" }
                                        ),
                                        xmlns: "http://www.w3.org/2000/svg",
                                        fill: "none",
                                        "viewBox": "0 0 24 24",
                                        stroke: "currentColor",
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            "stroke-width": "2",
                                            d: "M19 9l-7 7-7-7"
                                        }
                                    }
                                }
                            }
                            
                            // Collapsible content with smooth animation
                            div {
                                class: "{content_classes}",
                                div {
                                    class: "pt-4 pb-2",
                                    {item.content.clone()}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}