//! Collapsible component showcase

use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Checkbox, Collapsible, Input, Label};
use dioxus::prelude::*;

#[component]
pub fn collapsible_showcase() -> Element {
    let mut example_input = use_signal(String::new);
    let mut example_checkbox = use_signal(|| false);

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
                            class: "text-foreground",
                            "This is collapsible content that can be expanded and collapsed. Click the trigger above to toggle visibility."
                        }
                        p {
                            class: "text-foreground",
                            "The content smoothly animates in and out with CSS transitions."
                        }
                    }
                }

                Collapsible {
                    trigger: "Another Section",
                    div {
                        class: "space-y-2",
                        p {
                            class: "text-foreground",
                            "Each collapsible section operates independently."
                        }
                        ul {
                            class: "list-disc list-inside space-y-1 text-foreground ml-4",
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
                            class: "text-foreground",
                            "This section starts expanded by setting the `default_open` prop to true."
                        }
                        div {
                            class: "bg-primary/20 border border-primary rounded-md p-3",
                            p {
                                class: "text-primary-foreground text-sm",
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
                            class: "text-foreground",
                            "This content cannot be accessed because the collapsible is disabled."
                        }
                    }
                }
            }


            ShowcaseVariant {
                title: "Custom Styling".to_string(),

                Collapsible {
                    trigger: "Custom Styled Section",
                    trigger_class: "bg-success/20 border-success/20 text-success-foreground px-4 rounded-t-md",
                    content_class: "bg-success/10 border-x border-b border-success/20 px-4 rounded-b-md",
                    div {
                        class: "space-y-2",
                        p {
                            class: "text-success-foreground",
                            "This collapsible uses custom styling to create a themed appearance."
                        }
                        p {
                            class: "text-success-foreground text-sm",
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
                            class: "text-foreground",
                            "The collapsible component provides a clean way to organize content hierarchically:"
                        }
                        div {
                            class: "bg-muted rounded-md p-4 space-y-3",
                            h4 {
                                class: "font-semibold text-foreground",
                                "Key Features:"
                            }
                            ul {
                                class: "list-disc list-inside space-y-1 text-foreground",
                                li { "Smooth CSS animations for expand/collapse" }
                                li { "Animated chevron icon that rotates" }
                                li { "Keyboard accessible with proper ARIA attributes" }
                                li { "Customizable styling for all parts" }
                                li { "Support for any content type" }
                            }
                        }
                        div {
                            class: "border-l-4 border-primary pl-4",
                            p {
                                class: "text-foreground italic",
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
                            class: "text-foreground text-sm",
                            "Collapsibles can contain any type of content, including interactive elements:"
                        }
                        div {
                            class: "space-y-2",
                            Label { "Example Input:" }
                            Input {
                                placeholder: "Type something...",
                                value: example_input.read().clone(),
                                oninput: move |value: String| example_input.set(value),
                            }
                            div {
                                class: "flex items-center space-x-2",
                                Checkbox {
                                    checked: *example_checkbox.read(),
                                    onchange: move |checked| example_checkbox.set(checked),
                                }
                                Label { "I agree to the terms" }
                            }
                        }
                    }
                }
            }
        }
    }
}
