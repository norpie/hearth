//! Collapsible component showcase

use dioxus::prelude::*;
use crate::{Collapsible};
use super::super::{ComponentShowcase, ShowcaseVariant};

#[component]
pub fn collapsible_showcase() -> Element {
    rsx! {
        ComponentShowcase {
            name: "Collapsible".to_string(),
            description: "Expandable content sections with animated chevrons.".to_string(),
            basic_usage: r#"Collapsible {
    trigger: "Section Title"
}"#.to_string(),
            with_props_usage: r#"Collapsible {
    trigger: "Custom Section",
    default_open: true,
    disabled: false
}"#.to_string(),
            
            ShowcaseVariant {
                title: "Basic Usage".to_string(),
                
                Collapsible {
                    trigger: "Basic Collapsible",
                    div {
                        class: "space-y-2",
                        p {
                            class: "text-gray-700 dark:text-gray-300",
                            "This is collapsible content that can be expanded and collapsed. Click the trigger above to toggle visibility."
                        }
                        p {
                            class: "text-gray-700 dark:text-gray-300",
                            "The content smoothly animates in and out with CSS transitions."
                        }
                    }
                }
                
                Collapsible {
                    trigger: "Another Section",
                    div {
                        class: "space-y-2",
                        p {
                            class: "text-gray-700 dark:text-gray-300",
                            "Each collapsible section operates independently."
                        }
                        ul {
                            class: "list-disc list-inside space-y-1 text-gray-700 dark:text-gray-300 ml-4",
                            li { "List item one" }
                            li { "List item two" }
                            li { "List item three" }
                        }
                    }
                }
            }
            
            
            ShowcaseVariant {
                title: "Default Open".to_string(),
                
                Collapsible {
                    trigger: "Pre-expanded Section",
                    default_open: true,
                    div {
                        class: "space-y-3",
                        p {
                            class: "text-gray-700 dark:text-gray-300",
                            "This section starts expanded by setting the `default_open` prop to true."
                        }
                        div {
                            class: "bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-md p-3",
                            p {
                                class: "text-blue-800 dark:text-blue-200 text-sm",
                                "💡 Tip: Use default_open for sections you want users to see immediately."
                            }
                        }
                    }
                }
            }
            
            
            ShowcaseVariant {
                title: "Disabled State".to_string(),
                
                Collapsible {
                    trigger: "Disabled Collapsible",
                    disabled: true,
                    div {
                        p {
                            class: "text-gray-700 dark:text-gray-300",
                            "This content cannot be accessed because the collapsible is disabled."
                        }
                    }
                }
            }
            
            
            ShowcaseVariant {
                title: "Custom Styling".to_string(),
                
                Collapsible {
                    trigger: "Custom Styled Section",
                    trigger_class: "bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-800 text-green-800 dark:text-green-200 px-4 rounded-t-md",
                    content_class: "bg-green-25 dark:bg-green-900/10 border-x border-b border-green-200 dark:border-green-800 px-4 rounded-b-md",
                    div {
                        class: "space-y-2",
                        p {
                            class: "text-green-700 dark:text-green-300",
                            "This collapsible uses custom styling to create a themed appearance."
                        }
                        p {
                            class: "text-green-600 dark:text-green-400 text-sm",
                            "Custom classes can be applied to the trigger, content area, and container."
                        }
                    }
                }
            }
            
            
            ShowcaseVariant {
                title: "Rich Content".to_string(),
                
                Collapsible {
                    trigger: "FAQ: How does the collapsible component work?",
                    div {
                        class: "space-y-4",
                        p {
                            class: "text-gray-700 dark:text-gray-300",
                            "The collapsible component provides a clean way to organize content hierarchically:"
                        }
                        div {
                            class: "bg-gray-50 dark:bg-gray-800 rounded-md p-4 space-y-3",
                            h4 {
                                class: "font-semibold text-gray-900 dark:text-gray-100",
                                "Key Features:"
                            }
                            ul {
                                class: "list-disc list-inside space-y-1 text-gray-700 dark:text-gray-300",
                                li { "Smooth CSS animations for expand/collapse" }
                                li { "Animated chevron icon that rotates" }
                                li { "Keyboard accessible with proper ARIA attributes" }
                                li { "Customizable styling for all parts" }
                                li { "Support for any content type" }
                            }
                        }
                        div {
                            class: "border-l-4 border-blue-400 pl-4",
                            p {
                                class: "text-gray-600 dark:text-gray-400 italic",
                                "Perfect for FAQs, settings panels, navigation menus, and content organization."
                            }
                        }
                    }
                }
                
                Collapsible {
                    trigger: "Collapsible with form elements",
                    div {
                        class: "space-y-3",
                        p {
                            class: "text-gray-700 dark:text-gray-300 text-sm",
                            "Collapsibles can contain any type of content, including interactive elements:"
                        }
                        div {
                            class: "space-y-2",
                            label {
                                class: "block text-sm font-medium text-gray-700 dark:text-gray-300",
                                "Example Input:"
                            }
                            input {
                                r#type: "text",
                                placeholder: "Type something...",
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            }
                            div {
                                class: "flex items-center space-x-2",
                                input {
                                    r#type: "checkbox",
                                    id: "collapsible-checkbox",
                                    class: "rounded"
                                }
                                label {
                                    r#for: "collapsible-checkbox",
                                    class: "text-sm text-gray-700 dark:text-gray-300",
                                    "I agree to the terms"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}