//! ComponentShowcase wrapper for standardized design system display

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ComponentShowcaseProps {
    /// Component name for the title
    pub name: String,
    /// Description of the component
    pub description: String,
    /// Basic usage example code
    pub basic_usage: String,
    /// Advanced usage example code with all props
    pub with_props_usage: String,
    /// Content containing the component variants
    pub children: Element,
}

/// Wrapper component that provides consistent structure for component demonstrations
#[component]
pub fn ComponentShowcase(props: ComponentShowcaseProps) -> Element {
    rsx! {
        div { class: "space-y-8",
            // Header section
            div { class: "space-y-2",
                h2 { class: "text-2xl font-bold text-foreground", "{props.name}" }
                p { class: "text-muted-foreground", "{props.description}" }
            }
            // Component variants section
            div { class: "space-y-6",
                h3 { class: "text-lg font-semibold text-foreground", "Variants" }
                div { class: "space-y-4", {props.children} }
            }
            // Usage examples section
            div { class: "space-y-6",
                h3 { class: "text-lg font-semibold text-foreground", "Usage" }
                // Basic usage
                div { class: "space-y-2",
                    h4 { class: "text-sm font-medium text-muted-foreground uppercase tracking-wide",
                        "Basic"
                    }
                    div { class: "bg-muted rounded-lg p-4",
                        pre { class: "text-sm text-foreground font-mono overflow-x-auto",
                            code { "{props.basic_usage}" }
                        }
                    }
                }
                // With Props usage
                div { class: "space-y-2",
                    h4 { class: "text-sm font-medium text-muted-foreground uppercase tracking-wide",
                        "With Props"
                    }
                    div { class: "bg-muted rounded-lg p-4",
                        pre { class: "text-sm text-foreground font-mono overflow-x-auto",
                            code { "{props.with_props_usage}" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ShowcaseVariantProps {
    /// Variant name/description
    pub title: String,
    /// The component variant to display
    pub children: Element,
}

/// Individual variant container within a ComponentShowcase
#[component]
pub fn ShowcaseVariant(props: ShowcaseVariantProps) -> Element {
    rsx! {
        div { class: "space-y-3",
            h4 { class: "text-sm font-medium text-muted-foreground uppercase tracking-wide",
                "{props.title}"
            }
            div { class: "p-6 bg-card border border-border rounded-lg", {props.children} }
        }
    }
}
