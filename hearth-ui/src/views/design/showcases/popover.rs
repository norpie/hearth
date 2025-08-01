use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Avatar, Button, ButtonVariant, Popover, PopoverPlacement, PopoverTrigger};
use dioxus::prelude::*;

#[component]
pub fn popover_showcase() -> Element {
    rsx! {
        ComponentShowcase {
            name: "Popover".to_string(),
            description: "Display floating content that appears on click or hover, allowing mouse interaction with content.".to_string(),
            basic_usage: r#"// Traditional popover with trigger_element
Popover {
    trigger_element: rsx! { Button { "Click me" } },
    content: rsx! { "Popover content here" }
}

// Tooltip-style with content trigger
Popover {
    trigger: PopoverTrigger::Hover,
    content: rsx! { "Tooltip content" },
    Button { "Hover me" }
}"#.to_string(),
            with_props_usage: r#"Popover {
    trigger: PopoverTrigger::Click,
    placement: PopoverPlacement::Bottom,
    trigger_element: rsx! { Button { "Trigger" } },
    content: rsx! { "Custom content" }
}"#.to_string(),

            ShowcaseVariant {
                title: "Trigger Types".to_string(),
                children: rsx! {
                    div { class: "flex gap-4 flex-wrap",
                        Popover {
                            trigger: PopoverTrigger::Click,
                            placement: PopoverPlacement::Auto,
                            trigger_element: rsx! {
                                Button {
                                    variant: ButtonVariant::Primary,
                                    "Click Trigger"
                                }
                            },
                            content: rsx! {
                                div { class: "space-y-2",
                                    h4 { "Click Popover" }
                                    p { class: "text-sm",
                                        "Click outside to close"
                                    }
                                    Button {
                                        variant: ButtonVariant::Secondary,
                                        "Action"
                                    }
                                }
                            }
                        }

                        Popover {
                            trigger: PopoverTrigger::Hover,
                            placement: PopoverPlacement::Auto,
                            trigger_element: rsx! {
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    "Hover Trigger"
                                }
                            },
                            content: rsx! {
                                div { class: "space-y-2",
                                    h4 { "Hover Popover" }
                                    p { class: "text-sm",
                                        "Mouse can enter content"
                                    }
                                    Button {
                                        variant: ButtonVariant::Outline,
                                        "Clickable"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Tooltip Style (Content Trigger)".to_string(),
                children: rsx! {
                    div { class: "flex gap-6 flex-wrap items-center",
                        Popover {
                            trigger: PopoverTrigger::Hover,
                            placement: PopoverPlacement::Auto,
                            content: rsx! {
                                "This is a basic tooltip with helpful information"
                            },
                            Button {
                                variant: ButtonVariant::Primary,
                                "Hover for tooltip"
                            }
                        }

                        Popover {
                            trigger: PopoverTrigger::Hover,
                            placement: PopoverPlacement::Auto,
                            content: rsx! {
                                "John Doe - Software Engineer"
                            },
                            Avatar {
                                name: "John Doe".to_string(),
                                avatar_url: None,
                            }
                        }

                        Popover {
                            trigger: PopoverTrigger::Hover,
                            placement: PopoverPlacement::Auto,
                            content: rsx! {
                                div { class: "space-y-2",
                                    h4 { "Advanced Feature" }
                                    p { class: "text-sm",
                                        "This feature provides enhanced functionality with multiple options and settings."
                                    }
                                    p { class: "text-xs pt-1",
                                        "Press Ctrl+Shift+F to access"
                                    }
                                }
                            },
                            Button {
                                variant: ButtonVariant::Secondary,
                                "Rich tooltip"
                            }
                        }

                        Popover {
                            trigger: PopoverTrigger::Hover,
                            placement: PopoverPlacement::Auto,
                            content: rsx! {
                                div { class: "space-y-2 max-w-xs",
                                    div { class: "flex items-center gap-2",
                                        div { class: "w-3 h-3 bg-primary rounded-full" }
                                        h4 { "Status: Active" }
                                    }
                                    p { class: "text-sm",
                                        "Service is running normally. Last checked 2 minutes ago."
                                    }
                                    div { class: "flex gap-2 pt-1",
                                        div { class: "px-2 py-1 bg-success text-success-foreground text-xs rounded",
                                            "Healthy"
                                        }
                                        div { class: "px-2 py-1 bg-primary text-primary-foreground text-xs rounded",
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
                title: "Placements".to_string(),
                children: rsx! {
                    div { class: "grid grid-cols-3 gap-8 place-items-center max-w-2xl mx-auto",
                        // Top row
                        Popover {
                            placement: PopoverPlacement::TopStart,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Outline, "Top Start" }
                            },
                            content: rsx! {
                                p { "Top Start placement" }
                            }
                        }

                        Popover {
                            placement: PopoverPlacement::Top,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Outline, "Top" }
                            },
                            content: rsx! {
                                p { "Top center placement" }
                            }
                        }

                        Popover {
                            placement: PopoverPlacement::TopEnd,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Outline, "Top End" }
                            },
                            content: rsx! {
                                p { "Top End placement" }
                            }
                        }

                        // Middle row
                        Popover {
                            placement: PopoverPlacement::Left,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Outline, "Left" }
                            },
                            content: rsx! {
                                p { "Left placement" }
                            }
                        }

                        p { class: "text-center",
                            "Center"
                        }

                        Popover {
                            placement: PopoverPlacement::Right,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Outline, "Right" }
                            },
                            content: rsx! {
                                p { "Right placement" }
                            }
                        }

                        // Bottom row
                        Popover {
                            placement: PopoverPlacement::BottomStart,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Outline, "Bottom Start" }
                            },
                            content: rsx! {
                                p { "Bottom Start placement" }
                            }
                        }

                        Popover {
                            placement: PopoverPlacement::Bottom,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Outline, "Bottom" }
                            },
                            content: rsx! {
                                p { "Bottom center placement" }
                            }
                        }

                        Popover {
                            placement: PopoverPlacement::BottomEnd,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Outline, "Bottom End" }
                            },
                            content: rsx! {
                                p { "Bottom End placement" }
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Auto Placement".to_string(),
                children: rsx! {
                    div { class: "flex gap-4 flex-wrap",
                        Popover {
                            placement: PopoverPlacement::Auto,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Primary, "Auto Placement" }
                            },
                            content: rsx! {
                                div { class: "space-y-2",
                                    h4 { "Auto Placement" }
                                    p { class: "text-sm",
                                        "Automatically adjusts to stay in viewport"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Rich Content".to_string(),
                children: rsx! {
                    div { class: "flex gap-4 flex-wrap",
                        Popover {
                            placement: PopoverPlacement::Auto,
                            trigger_element: rsx! {
                                Button {
                                    variant: ButtonVariant::Primary,
                                    "User Menu"
                                }
                            },
                            content: rsx! {
                                div { class: "space-y-3 min-w-48",
                                    div { class: "flex items-center gap-3 pb-2 border-b border-border",
                                        div { class: "w-8 h-8 bg-primary rounded-full flex items-center justify-center text-white text-sm font-semibold",
                                            "JD"
                                        }
                                        div {
                                            p { class: "font-semibold text-sm", "John Doe" }
                                            p { class: "text-xs", "john@example.com" }
                                        }
                                    }
                                    div { class: "space-y-1",
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            class: "w-full justify-start h-auto py-1.5 px-2 text-sm",
                                            "Profile"
                                        }
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            class: "w-full justify-start h-auto py-1.5 px-2 text-sm",
                                            "Settings"
                                        }
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            class: "w-full justify-start h-auto py-1.5 px-2 text-sm text-destructive-foreground",
                                            "Sign out"
                                        }
                                    }
                                }
                            }
                        }

                        Popover {
                            placement: PopoverPlacement::Auto,
                            trigger_element: rsx! {
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    "Info Panel"
                                }
                            },
                            content: rsx! {
                                div { class: "space-y-3 max-w-xs",
                                    h4 { "Information" }
                                    p { class: "text-sm",
                                        "This is a rich content popover that can contain multiple elements, formatted text, and interactive components."
                                    }
                                    div { class: "flex gap-2 pt-2",
                                        Button {
                                            variant: ButtonVariant::Primary,
                                            "Learn More"
                                        }
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            "Dismiss"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "States".to_string(),
                children: rsx! {
                    div { class: "flex gap-4 flex-wrap items-center",
                        Popover {
                            placement: PopoverPlacement::Auto,
                            trigger_element: rsx! {
                                Button {
                                    variant: ButtonVariant::Primary,
                                    "Normal"
                                }
                            },
                            content: rsx! {
                                p { "Normal popover content" }
                            }
                        }

                        Popover {
                            disabled: true,
                            placement: PopoverPlacement::Auto,
                            trigger_element: rsx! {
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    disabled: true,
                                    "Disabled"
                                }
                            },
                            content: rsx! {
                                p { "This won't show" }
                            }
                        }

                        p { class: "text-sm",
                            "Disabled popover won't open"
                        }
                    }
                }
            }
        }
    }
}
