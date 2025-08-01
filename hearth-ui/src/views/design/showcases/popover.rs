use dioxus::prelude::*;
use crate::{Popover, PopoverTrigger, PopoverPlacement, Button, ButtonVariant};
use super::super::{ComponentShowcase, ShowcaseVariant};

#[component]
pub fn popover_showcase() -> Element {
    rsx! {
        ComponentShowcase {
            name: "Popover".to_string(),
            description: "Display floating content that appears on click or hover, allowing mouse interaction with content.".to_string(),
            basic_usage: r#"Popover {
    trigger_element: rsx! { Button { "Click me" } },
    content: rsx! { "Popover content here" }
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
                                    div { class: "font-semibold", "Click Popover" }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400", 
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
                                    div { class: "font-semibold", "Hover Popover" }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400", 
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
                                div { "Top Start placement" }
                            }
                        }
                        
                        Popover {
                            placement: PopoverPlacement::Top,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Outline, "Top" }
                            },
                            content: rsx! {
                                div { "Top center placement" }
                            }
                        }
                        
                        Popover {
                            placement: PopoverPlacement::TopEnd,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Outline, "Top End" }
                            },
                            content: rsx! {
                                div { "Top End placement" }
                            }
                        }
                        
                        // Middle row
                        Popover {
                            placement: PopoverPlacement::Left,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Outline, "Left" }
                            },
                            content: rsx! {
                                div { "Left placement" }
                            }
                        }
                        
                        div { class: "text-center text-gray-500 dark:text-gray-400",
                            "Center"
                        }
                        
                        Popover {
                            placement: PopoverPlacement::Right,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Outline, "Right" }
                            },
                            content: rsx! {
                                div { "Right placement" }
                            }
                        }
                        
                        // Bottom row
                        Popover {
                            placement: PopoverPlacement::BottomStart,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Outline, "Bottom Start" }
                            },
                            content: rsx! {
                                div { "Bottom Start placement" }
                            }
                        }
                        
                        Popover {
                            placement: PopoverPlacement::Bottom,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Outline, "Bottom" }
                            },
                            content: rsx! {
                                div { "Bottom center placement" }
                            }
                        }
                        
                        Popover {
                            placement: PopoverPlacement::BottomEnd,
                            trigger_element: rsx! {
                                Button { variant: ButtonVariant::Outline, "Bottom End" }
                            },
                            content: rsx! {
                                div { "Bottom End placement" }
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
                                    div { class: "font-semibold", "Auto Placement" }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400", 
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
                                    div { class: "flex items-center gap-3 pb-2 border-b border-gray-200 dark:border-gray-700",
                                        div { class: "w-8 h-8 bg-blue-500 rounded-full flex items-center justify-center text-white text-sm font-semibold",
                                            "JD"
                                        }
                                        div {
                                            div { class: "font-semibold text-sm", "John Doe" }
                                            div { class: "text-xs text-gray-500 dark:text-gray-400", "john@example.com" }
                                        }
                                    }
                                    div { class: "space-y-1",
                                        button { class: "w-full text-left px-2 py-1.5 text-sm hover:bg-gray-100 dark:hover:bg-gray-700 rounded",
                                            "Profile"
                                        }
                                        button { class: "w-full text-left px-2 py-1.5 text-sm hover:bg-gray-100 dark:hover:bg-gray-700 rounded",
                                            "Settings"
                                        }
                                        button { class: "w-full text-left px-2 py-1.5 text-sm hover:bg-gray-100 dark:hover:bg-gray-700 rounded text-red-600 dark:text-red-400",
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
                                    div { class: "font-semibold", "Information" }
                                    div { class: "text-sm text-gray-600 dark:text-gray-400",
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
                                div { "Normal popover content" }
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
                                div { "This won't show" }
                            }
                        }
                        
                        div { class: "text-sm text-gray-500 dark:text-gray-400",
                            "Disabled popover won't open"
                        }
                    }
                }
            }
        }
    }
}