use dioxus::prelude::*;
use crate::{Button, ButtonVariant, Avatar, Tooltip};
use super::super::{ComponentShowcase, ShowcaseVariant};

#[component]
pub fn tooltip_showcase() -> Element {

    rsx! {
        ComponentShowcase {
            name: "Tooltip".to_string(),
            description: "Display informative text when hovering over elements. Tooltips use smart auto-positioning to always stay within the viewport.".to_string(),
            basic_usage: r#"Tooltip {
    content: rsx! { "Tooltip content" },
    Button { "Hover me" }
}"#.to_string(),
            with_props_usage: r#"Tooltip {
    content: rsx! {
        div {
            div { class: "font-semibold", "Rich Content" }
            div { class: "text-sm", "Detailed information here" }
        }
    },
    Button { "Trigger" }
}"#.to_string(),
            
            ShowcaseVariant {
                title: "Basic Tooltips".to_string(),
                children: rsx! {
                    div { class: "flex gap-6 flex-wrap items-center",
                        Tooltip {
                            content: rsx! {
                                "This is a basic tooltip with helpful information"
                            },
                            Button {
                                variant: ButtonVariant::Primary,
                                "Hover for tooltip"
                            }
                        }
                        
                        Tooltip {
                            content: rsx! {
                                "John Doe - Software Engineer"
                            },
                            Avatar {
                                name: "John Doe".to_string(),
                                avatar_url: None,
                            }
                        }
                    }
                }
            }
            
            ShowcaseVariant {
                title: "Rich Content".to_string(),
                children: rsx! {
                    div { class: "flex gap-6 flex-wrap items-center",
                        Tooltip {
                            content: rsx! {
                                div { class: "space-y-2",
                                    div { class: "font-semibold text-sm", "Advanced Feature" }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400",
                                        "This feature provides enhanced functionality with multiple options and settings."
                                    }
                                    div { class: "text-xs text-gray-500 dark:text-gray-500 pt-1",
                                        "Press Ctrl+Shift+F to access"
                                    }
                                }
                            },
                            Button {
                                variant: ButtonVariant::Secondary,
                                "Rich tooltip"
                            }
                        }
                        
                        Tooltip {
                            content: rsx! {
                                div { class: "space-y-2 max-w-xs",
                                    div { class: "flex items-center gap-2",
                                        div { class: "w-3 h-3 bg-blue-500 rounded-full" }
                                        div { class: "font-semibold text-sm", "Status: Active" }
                                    }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400",
                                        "Service is running normally. Last checked 2 minutes ago."
                                    }
                                    div { class: "flex gap-2 pt-1",
                                        div { class: "px-2 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 text-xs rounded",
                                            "Healthy"
                                        }
                                        div { class: "px-2 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 text-xs rounded",
                                            "Auto-scaling"
                                        }
                                    }
                                }
                            },
                            Button {
                                variant: ButtonVariant::Outline,
                                "Info tooltip"
                            }
                        }
                    }
                }
            }
                       
            ShowcaseVariant {
                title: "Interactive Elements".to_string(),
                children: rsx! {
                    div { class: "flex gap-6 flex-wrap items-center",
                        TooltipWrapper {
                            tooltip_content: rsx! {
                                "Click to perform the primary action for this workflow"
                            },
                            children: rsx! {
                                Button {
                                    variant: ButtonVariant::Primary,
                                    "Primary Action"
                                }
                            }
                        }
                        
                        TooltipWrapper {
                            tooltip_content: rsx! {
                                div { class: "space-y-1",
                                    div { class: "font-semibold text-sm", "Dangerous Action" }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400",
                                        "This action cannot be undone. Please confirm before proceeding."
                                    }
                                }
                            },
                            children: rsx! {
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    class: "text-red-600 dark:text-red-400",
                                    "Delete"
                                }
                            }
                        }
                        
                        TooltipWrapper {
                            tooltip_content: rsx! {
                                "Feature coming soon in the next update"
                            },
                            children: rsx! {
                                Button {
                                    variant: ButtonVariant::Outline,
                                    disabled: true,
                                    "Coming Soon"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TooltipExampleProps {
    text: String,
    position: String, // Keep for now to avoid breaking the existing calls, but not used
}

#[component]
fn TooltipExample(props: TooltipExampleProps) -> Element {
    rsx! {
        Tooltip {
            content: rsx! {
                "Tooltip for {props.text} - automatically positioned to stay in viewport"
            },
            Button {
                variant: ButtonVariant::Ghost,
                "{props.text}"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TooltipWrapperProps {
    tooltip_content: Element,
    children: Element,
}

#[component]
fn TooltipWrapper(props: TooltipWrapperProps) -> Element {
    rsx! {
        Tooltip {
            content: props.tooltip_content,
            {props.children}
        }
    }
}
