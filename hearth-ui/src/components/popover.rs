//! Popover component that displays content on click or hover

use dioxus::prelude::*;
use crate::{Platform, use_viewport};

#[derive(Clone, PartialEq)]
pub enum PopoverTrigger {
    Click,
    Hover,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PopoverPlacement {
    Top,
    TopStart,
    TopEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
    Right,
    RightStart,
    RightEnd,
    Auto,
}

#[derive(Props, Clone, PartialEq)]
pub struct PopoverProps {
    #[props(default = PopoverTrigger::Click)]
    pub trigger: PopoverTrigger,
    #[props(default = PopoverPlacement::Bottom)]
    pub placement: PopoverPlacement,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub content_class: Option<String>,
    pub trigger_element: Element,
    pub content: Element,
}

impl PopoverPlacement {
    pub fn classes(&self) -> &'static str {
        match self {
            PopoverPlacement::Top => "bottom-full left-1/2 transform -translate-x-1/2 mb-2",
            PopoverPlacement::TopStart => "bottom-full left-0 mb-2",
            PopoverPlacement::TopEnd => "bottom-full right-0 mb-2",
            PopoverPlacement::Bottom => "top-full left-1/2 transform -translate-x-1/2 mt-2",
            PopoverPlacement::BottomStart => "top-full left-0 mt-2",
            PopoverPlacement::BottomEnd => "top-full right-0 mt-2",
            PopoverPlacement::Left => "right-full top-1/2 transform -translate-y-1/2 mr-2",
            PopoverPlacement::LeftStart => "right-full top-0 mr-2",
            PopoverPlacement::LeftEnd => "right-full bottom-0 mr-2",
            PopoverPlacement::Right => "left-full top-1/2 transform -translate-y-1/2 ml-2",
            PopoverPlacement::RightStart => "left-full top-0 ml-2",
            PopoverPlacement::RightEnd => "left-full bottom-0 ml-2",
            PopoverPlacement::Auto => "top-full left-1/2 transform -translate-x-1/2 mt-2", // Default to bottom
        }
    }
}

#[component]
pub fn Popover(props: PopoverProps) -> Element {
    let mut is_open = use_signal(|| false);
    let mut actual_placement = use_signal(|| props.placement.clone());
    let mut horizontal_offset = use_signal(|| 0.0);
    let platform = Platform::current();
    let placement = props.placement.clone(); // Clone once to avoid move issues

    // Handler functions for trigger events
    let click_handler = move |_| {
        if !props.disabled {
            is_open.set(!is_open());
        }
    };
    
    let hover_enter = move |_| {
        if !props.disabled {
            is_open.set(true);
        }
    };
    
    let hover_leave = move |_| {
        if !props.disabled {
            is_open.set(false);
        }
    };

    // Close popover when clicking outside (simplified approach)
    let close_handler = move |_| {
        is_open.set(false);
    };

    let container_class = if let Some(class) = &props.class {
        format!("relative inline-block {}", class)
    } else {
        "relative inline-block".to_string()
    };

    let content_base_classes = "absolute z-50 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg p-4 min-w-max";
    let content_class = if let Some(class) = &props.content_class {
        format!("{} {} {}", content_base_classes, actual_placement().classes(), class)
    } else {
        format!("{} {}", content_base_classes, actual_placement().classes())
    };
    
    let content_style = if placement == PopoverPlacement::Auto && horizontal_offset() != 0.0 {
        format!("transform: translateX({}px);", horizontal_offset())
    } else {
        String::new()
    };

    // Auto placement logic - only for Auto placement
    let auto_placement_handler = move |event: MountedEvent| {
        if placement == PopoverPlacement::Auto {
            let mounted_data = event.data.clone();
            let viewport = use_viewport();
            
            spawn(async move {
                // Add error handling to prevent index out of bounds
                match mounted_data.get_client_rect().await {
                    Ok(popover_rect) => {
                        let viewport_info = viewport.read();
                        let viewport_width = viewport_info.width;
                        let viewport_height = viewport_info.height;
                        
                        // Ensure we have valid viewport dimensions
                        if viewport_width <= 0.0 || viewport_height <= 0.0 {
                            return;
                        }
                        
                        // Calculate trigger position approximation
                        let trigger_center_y = popover_rect.min_y() + (popover_rect.height() / 2.0);
                        let trigger_center_x = popover_rect.min_x() + (popover_rect.width() / 2.0);
                        let trigger_bottom = popover_rect.min_y() - 10.0;
                        let trigger_top = popover_rect.max_y() + 10.0;
                        
                        let viewport_center_y = viewport_height / 2.0;
                        
                        // Primary placement: top half shows bottom, bottom half shows top
                        let primary_placement = if trigger_center_y < viewport_center_y {
                            PopoverPlacement::Bottom
                        } else {
                            PopoverPlacement::Top
                        };
                        
                        // Calculate where popover would appear with primary placement
                        let popover_height = popover_rect.height();
                        let popover_width = popover_rect.width();
                        
                        let (predicted_top, predicted_bottom) = if primary_placement == PopoverPlacement::Bottom {
                            let top = trigger_bottom + 8.0; // 8px gap (mt-2)
                            (top, top + popover_height)
                        } else {
                            let bottom = trigger_top - 8.0; // 8px gap (mb-2)
                            (bottom - popover_height, bottom)
                        };
                        
                        // Check if primary placement would go off screen vertically
                        let final_placement = if primary_placement == PopoverPlacement::Bottom {
                            if predicted_bottom > viewport_height - 16.0 {
                                PopoverPlacement::Top
                            } else {
                                PopoverPlacement::Bottom
                            }
                        } else {
                            if predicted_top < 80.0 {
                                PopoverPlacement::Bottom
                            } else {
                                PopoverPlacement::Top
                            }
                        };
                        
                        // Calculate horizontal placement - check if popover would go off screen horizontally
                        let predicted_left = trigger_center_x - (popover_width / 2.0);
                        let predicted_right = predicted_left + popover_width;
                        
                        let left_margin = if platform.is_mobile() { 16.0 } else { 280.0 }; // Account for sidebar
                        let right_margin = 16.0;
                        
                        let mut h_offset = 0.0;
                        
                        // If popover would go off left edge (including sidebar)
                        if predicted_left < left_margin {
                            h_offset = left_margin - predicted_left;
                        }
                        
                        // If popover would go off right edge
                        if predicted_right > viewport_width - right_margin {
                            let right_overflow = predicted_right - (viewport_width - right_margin);
                            h_offset = h_offset.min(-right_overflow);
                        }
                        
                        actual_placement.set(final_placement);
                        horizontal_offset.set(h_offset);
                    }
                    Err(_) => {
                        // Handle error gracefully - just use default placement
                    }
                }
            });
        }
    };

    // Render different trigger based on trigger type
    match props.trigger {
        PopoverTrigger::Click => rsx! {
            div {
                class: "{container_class}",
                div {
                    onclick: click_handler,
                    {props.trigger_element}
                }
                if is_open() {
                    if placement == PopoverPlacement::Auto {
                        div {
                            class: "{content_class}",
                            style: "{content_style}",
                            onclick: move |e| {
                                // Prevent event from bubbling up to close the popover
                                e.stop_propagation();
                            },
                            onmounted: auto_placement_handler,
                            {props.content}
                        }
                    } else {
                        div {
                            class: "{content_class}",
                            style: "{content_style}",
                            onclick: move |e| {
                                // Prevent event from bubbling up to close the popover
                                e.stop_propagation();
                            },
                            {props.content}
                        }
                    }
                }
                // Overlay to catch outside clicks for click trigger
                if is_open() {
                    div {
                        class: "fixed inset-0 z-40",
                        onclick: close_handler,
                    }
                }
            }
        },
        PopoverTrigger::Hover => rsx! {
            div {
                class: "{container_class}",
                div {
                    onmouseenter: hover_enter,
                    onmouseleave: hover_leave,
                    {props.trigger_element}
                }
                if is_open() {
                    if placement == PopoverPlacement::Auto {
                        div {
                            class: "{content_class}",
                            style: "{content_style}",
                            onmouseenter: move |_| {
                                is_open.set(true);
                            },
                            onmouseleave: move |_| {
                                is_open.set(false);
                            },
                            onmounted: auto_placement_handler,
                            {props.content}
                        }
                    } else {
                        div {
                            class: "{content_class}",
                            style: "{content_style}",
                            onmouseenter: move |_| {
                                is_open.set(true);
                            },
                            onmouseleave: move |_| {
                                is_open.set(false);
                            },
                            {props.content}
                        }
                    }
                }
            }
        }
    }
}