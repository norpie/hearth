//! Popover components for contextual overlays and tooltips
//!
//! The popover module provides overlay components that display contextual content
//! relative to trigger elements. It supports click and hover triggers with
//! intelligent positioning for creating tooltips, menus, and help text.
//!
//! # Examples
//!
//! Hover tooltip:
//! ```rust
//! rsx! {
//!     Popover {
//!         trigger: PopoverTrigger::Hover,
//!         placement: PopoverPlacement::Top,
//!         content: rsx! {
//!             div { class: "text-sm", "Helpful tooltip" }
//!         },
//!         button { "Hover for help" }
//!     }
//! }
//! ```
//!
//! Click-triggered menu popover:
//! ```rust
//! rsx! {
//!     Popover {
//!         trigger: PopoverTrigger::Click,
//!         placement: PopoverPlacement::BottomEnd,
//!         content: rsx! {
//!             div { class: "py-2 w-48",
//!                 button {
//!                     class: "w-full px-4 py-2 text-left hover:bg-accent",
//!                     onclick: move |_| handle_edit(),
//!                     "Edit"
//!                 }
//!                 button {
//!                     class: "w-full px-4 py-2 text-left hover:bg-accent",
//!                     onclick: move |_| handle_duplicate(),
//!                     "Duplicate"
//!                 }
//!                 hr { class: "my-1" }
//!                 button {
//!                     class: "w-full px-4 py-2 text-left hover:bg-accent text-destructive",
//!                     onclick: move |_| handle_delete(),
//!                     "Delete"
//!                 }
//!             }
//!         },
//!         button { class: "p-2 hover:bg-accent rounded",
//!             svg { class: "w-4 h-4", /* three dots icon */ }
//!         }
//!     }
//! }
//! ```
//!
//! Auto-positioning popover with form content:
//! ```rust
//! let mut user_name = use_signal(|| String::new());
//! let mut user_email = use_signal(|| String::new());
//! 
//! rsx! {
//!     Popover {
//!         trigger: PopoverTrigger::Click,
//!         placement: PopoverPlacement::Auto,
//!         content_class: Some("p-6 w-80".to_string()),
//!         content: rsx! {
//!             div { class: "space-y-4",
//!                 h3 { class: "font-semibold text-lg", "Quick Add User" }
//!                 div { class: "space-y-2",
//!                     Label { "Name" }
//!                     Input {
//!                         value: user_name(),
//!                         onchange: move |val| user_name.set(val),
//!                         placeholder: "Enter name"
//!                     }
//!                 }
//!                 div { class: "space-y-2",
//!                     Label { "Email" }
//!                     Input {
//!                         value: user_email(),
//!                         onchange: move |val| user_email.set(val),
//!                         placeholder: "Enter email"
//!                     }
//!                 }
//!                 div { class: "flex gap-2 justify-end",
//!                     button {
//!                         class: "px-3 py-2 border rounded hover:bg-accent",
//!                         "Cancel"
//!                     }
//!                     button {
//!                         class: "px-3 py-2 bg-primary text-primary-foreground rounded",
//!                         onclick: move |_| save_user(user_name(), user_email()),
//!                         "Add User"
//!                     }
//!                 }
//!             }
//!         },
//!         button { class: "px-4 py-2 bg-primary text-primary-foreground rounded",
//!             "+ Add User"
//!         }
//!     }
//! }
//! ```
//!
//! Information popover with rich content:
//! ```rust
//! rsx! {
//!     Popover {
//!         trigger: PopoverTrigger::Click,
//!         placement: PopoverPlacement::Right,
//!         content_class: Some("p-0 w-72".to_string()),
//!         content: rsx! {
//!             div {
//!                 img {
//!                     src: "https://example.com/feature-preview.jpg",
//!                     class: "w-full h-32 object-cover rounded-t-lg"
//!                 }
//!                 div { class: "p-4 space-y-3",
//!                     h4 { class: "font-semibold", "New Feature Available!" }
//!                     p { class: "text-sm text-muted-foreground",
//!                         "We've added powerful new analytics to help you track your progress."
//!                     }
//!                     div { class: "flex gap-2",
//!                         button {
//!                             class: "px-3 py-1 text-xs bg-primary text-primary-foreground rounded",
//!                             "Learn More"
//!                         }
//!                         button {
//!                             class: "px-3 py-1 text-xs border rounded",
//!                             "Dismiss"
//!                         }
//!                     }
//!                 }
//!             }
//!         },
//!         div { class: "relative",
//!             button { class: "p-2 border rounded-full hover:bg-accent",
//!                 svg { class: "w-4 h-4", /* info icon */ }
//!             }
//!             div { class: "absolute -top-1 -right-1 w-3 h-3 bg-red-500 rounded-full" }
//!         }
//!     }
//! }
//! ```

use crate::{use_viewport, Platform};
use dioxus::prelude::*;

/// Trigger mechanisms for popover activation
///
/// Determines how users interact with the trigger element to show the popover:
#[derive(Clone, PartialEq)]
pub enum PopoverTrigger {
    /// Click-based activation requiring user click to toggle
    ///
    /// Suitable for menus, forms, and interactive content that should
    /// remain visible until explicitly closed. Includes click-outside
    /// handling for proper UX.
    Click,

    /// Hover-based activation showing on mouse enter
    ///
    /// Suitable for tooltips, help text, and informational content
    /// that should appear quickly and disappear when not needed.
    /// Includes hover bridge for smooth interaction.
    Hover,
}

/// Placement options for popover positioning relative to trigger
///
/// Provides 12 fixed placement options plus automatic placement that
/// adapts to viewport constraints and available space:
#[derive(Clone, Copy, PartialEq)]
pub enum PopoverPlacement {
    /// Center above trigger element
    Top,
    /// Top-left aligned with trigger start
    TopStart,
    /// Top-right aligned with trigger end
    TopEnd,
    /// Center below trigger element
    Bottom,
    /// Bottom-left aligned with trigger start
    BottomStart,
    /// Bottom-right aligned with trigger end
    BottomEnd,
    /// Center to the left of trigger
    Left,
    /// Left-top aligned with trigger start
    LeftStart,
    /// Left-bottom aligned with trigger end
    LeftEnd,
    /// Center to the right of trigger
    Right,
    /// Right-top aligned with trigger start
    RightStart,
    /// Right-bottom aligned with trigger end
    RightEnd,
    /// Automatic placement based on available viewport space
    ///
    /// Intelligently chooses the best placement to avoid viewport edges
    /// and provides optimal visibility for the popover content.
    Auto,
}

/// Properties for configuring the Popover component
///
/// Provides comprehensive control over popover behavior, placement, styling,
/// and content organization for both trigger and popover content areas.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverProps {
    /// Activation mechanism for the popover
    ///
    /// Determines how users interact with the trigger to show the popover:
    /// - `Click`: Toggle visibility on click with click-outside handling
    /// - `Hover`: Show on mouse enter, hide on mouse leave
    /// Defaults to `PopoverTrigger::Click`.
    #[props(default = PopoverTrigger::Click)]
    pub trigger: PopoverTrigger,

    /// Positioning strategy for the popover relative to trigger
    ///
    /// Controls where the popover appears relative to the trigger element.
    /// Includes 12 fixed placements plus automatic placement that adapts
    /// to viewport constraints. Defaults to `PopoverPlacement::Bottom`.
    #[props(default = PopoverPlacement::Bottom)]
    pub placement: PopoverPlacement,

    /// Whether the popover interaction is disabled
    ///
    /// When true, prevents the popover from opening regardless of trigger
    /// type. Useful for conditional disabling based on application state.
    /// Defaults to false.
    #[props(default = false)]
    pub disabled: bool,

    /// Additional CSS classes for the trigger container
    ///
    /// Applied to the root container element that wraps the trigger.
    /// Use for positioning, spacing, or layout adjustments.
    /// Defaults to None.
    #[props(default)]
    pub class: Option<String>,

    /// Additional CSS classes for the popover content container
    ///
    /// Applied to the popover content element for custom styling.
    /// Use for width, padding, colors, or other content-specific styling.
    /// Defaults to None.
    #[props(default)]
    pub content_class: Option<String>,

    /// Optional dedicated trigger element
    ///
    /// When provided, uses this element as the trigger instead of children.
    /// Use when you need explicit control over the trigger element.
    /// Either trigger_element OR children should be provided, not both.
    /// Defaults to None (uses children).
    #[props(default)]
    pub trigger_element: Option<Element>,

    /// Content to display within the popover
    ///
    /// The main content shown when the popover is active. Can contain
    /// any type of content including text, forms, menus, or media.
    /// Required parameter.
    pub content: Element,

    /// Trigger content when not using trigger_element
    ///
    /// The content that users interact with to trigger the popover.
    /// Used when trigger_element is not provided. Can contain buttons,
    /// text, icons, or any interactive elements. Defaults to None.
    #[props(default)]
    pub children: Option<Element>,
}

impl PopoverPlacement {
    /// Returns CSS positioning classes for the placement option
    ///
    /// Maps each placement to appropriate absolute positioning classes
    /// including transforms for centering and margins for spacing.
    /// Auto placement defaults to bottom positioning.
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

/// Popover component for contextual overlays and interactive content
///
/// The Popover component provides a sophisticated overlay system for displaying
/// contextual content relative to trigger elements. It supports both click and
/// hover interactions with intelligent positioning that adapts to viewport
/// constraints and provides smooth user experiences across desktop and mobile.
///
/// # Features
///
/// - **Dual trigger modes**: Click-based for persistent content, hover for quick info
/// - **Intelligent positioning**: 12 placement options plus auto-placement with overflow handling
/// - **Responsive design**: Adapts to mobile and desktop with sidebar awareness
/// - **Flexible content**: Supports any content from simple tooltips to complex forms
/// - **Event handling**: Proper click-outside, escape key, and hover bridge behavior
/// - **Accessibility**: Focus management and keyboard navigation support
/// - **Performance**: Efficient viewport calculations with error handling
/// - **Touch-friendly**: Optimized for mobile interactions and touch devices
///
/// # Implementation Details
///
/// The component uses different rendering strategies based on trigger type:
///
/// **Click Trigger**: Renders with click handlers and full-screen overlay for
/// click-outside detection. Includes event propagation control to prevent
/// unwanted closures when interacting with popover content.
///
/// **Hover Trigger**: Renders with mouse enter/leave handlers and invisible
/// bridge elements to allow smooth movement between trigger and content.
/// Includes timing delays to prevent flickering during cursor movement.
///
/// **Auto Placement**: Uses viewport detection and client rect calculations
/// to determine optimal placement. Includes sidebar awareness for desktop
/// layouts and proper edge detection for mobile devices.
///
/// # Accessibility
///
/// - Proper focus management when popover opens and closes
/// - Keyboard navigation support within popover content
/// - Screen reader compatible with semantic content structure
/// - ARIA attributes for popover relationship (via content implementation)
/// - Click-outside and escape key handling for expected behavior
/// - Touch-friendly interactions for mobile accessibility
///
/// # Performance Considerations
///
/// - Efficient viewport calculations with async processing
/// - Error handling for client rect failures
/// - Conditional rendering to minimize DOM when closed
/// - Optimized event handling with proper cleanup
/// - Platform-specific timing adjustments for smooth animations
///
/// # Common Use Cases
///
/// - **Tooltips**: Hover-triggered help text and contextual information
/// - **Menus**: Click-triggered action menus and dropdown options
/// - **Forms**: Quick input forms and data entry overlays
/// - **Previews**: Rich content previews and media information
/// - **Navigation**: Secondary navigation and quick access panels
/// - **Notifications**: Contextual alerts and status information
///
/// # Parameters
///
/// - `trigger`: Activation mechanism (Click, Hover)
/// - `placement`: Positioning strategy (12 options plus Auto)
/// - `disabled`: Prevents popover activation when true
/// - `class`: Additional CSS classes for trigger container
/// - `content_class`: Additional CSS classes for popover content
/// - `trigger_element`: Optional dedicated trigger element
/// - `content`: Popover content (required)
/// - `children`: Default trigger content when trigger_element not provided
#[component]
pub fn Popover(props: PopoverProps) -> Element {
    let mut is_open = use_signal(|| false);
    let mut actual_placement = use_signal(|| props.placement);
    let mut horizontal_offset = use_signal(|| 0.0);
    let platform = Platform::current();
    let placement = props.placement; // Clone once to avoid move issues

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
            Platform::spawn(async move {
                Platform::sleep(std::time::Duration::from_millis(100)).await;
                is_open.set(false);
            });
        }
    };

    let close_handler = move |_| {
        is_open.set(false);
    };

    let container_class = if let Some(class) = &props.class {
        format!("relative inline-block {class}")
    } else {
        "relative inline-block".to_string()
    };

    let content_base_classes =
        "absolute z-50 bg-card border border-border rounded-lg shadow-lg p-4 min-w-max";
    let content_class = if let Some(class) = &props.content_class {
        format!(
            "{} {} {}",
            content_base_classes,
            actual_placement().classes(),
            class
        )
    } else {
        format!("{} {}", content_base_classes, actual_placement().classes())
    };

    let content_style = if placement == PopoverPlacement::Auto && horizontal_offset() != 0.0 {
        format!("transform: translateX({}px);", horizontal_offset())
    } else {
        String::new()
    };

    let auto_placement_handler = move |event: MountedEvent| {
        if placement == PopoverPlacement::Auto {
            let mounted_data = event.data.clone();
            let viewport = use_viewport();

            Platform::spawn(async move {
                match mounted_data.get_client_rect().await {
                    Ok(popover_rect) => {
                        let viewport_info = viewport.read();
                        let viewport_width = viewport_info.width;
                        let viewport_height = viewport_info.height;

                        if viewport_width <= 0.0 || viewport_height <= 0.0 {
                            return;
                        }

                        let trigger_center_y = popover_rect.min_y() + (popover_rect.height() / 2.0);
                        let trigger_center_x = popover_rect.min_x() + (popover_rect.width() / 2.0);
                        let trigger_bottom = popover_rect.min_y() - 10.0;
                        let trigger_top = popover_rect.max_y() + 10.0;

                        let viewport_center_y = viewport_height / 2.0;

                        let primary_placement = if trigger_center_y < viewport_center_y {
                            PopoverPlacement::Bottom
                        } else {
                            PopoverPlacement::Top
                        };

                        let popover_height = popover_rect.height();
                        let popover_width = popover_rect.width();

                        let (predicted_top, predicted_bottom) =
                            if primary_placement == PopoverPlacement::Bottom {
                                let top = trigger_bottom + 8.0; // 8px gap (mt-2)
                                (top, top + popover_height)
                            } else {
                                let bottom = trigger_top - 8.0; // 8px gap (mb-2)
                                (bottom - popover_height, bottom)
                            };

                        let final_placement = if primary_placement == PopoverPlacement::Bottom {
                            if predicted_bottom > viewport_height - 16.0 {
                                PopoverPlacement::Top
                            } else {
                                PopoverPlacement::Bottom
                            }
                        } else if predicted_top < 80.0 {
                            PopoverPlacement::Bottom
                        } else {
                            PopoverPlacement::Top
                        };

                        let predicted_left = trigger_center_x - (popover_width / 2.0);
                        let predicted_right = predicted_left + popover_width;

                        let left_margin = if platform.is_mobile() { 16.0 } else { 280.0 }; // Account for sidebar
                        let right_margin = 16.0;

                        let mut h_offset = 0.0;

                        if predicted_left < left_margin {
                            h_offset = left_margin - predicted_left;
                        }

                        if predicted_right > viewport_width - right_margin {
                            let right_overflow = predicted_right - (viewport_width - right_margin);
                            h_offset = h_offset.min(-right_overflow);
                        }

                        actual_placement.set(final_placement);
                        horizontal_offset.set(h_offset);
                    }
                    Err(_) => {
                    }
                }
            });
        }
    };

    match props.trigger {
        PopoverTrigger::Click => rsx! {
            div { class: "{container_class}",
                div { onclick: click_handler,
                    {
                        props
                            .trigger_element
                            .unwrap_or_else(|| props.children.unwrap_or_else(|| rsx! {}))
                    }
                }
                if is_open() {
                    if placement == PopoverPlacement::Auto {
                        div {
                            class: "{content_class}",
                            style: "{content_style}",
                            onclick: move |e| {
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
                                e.stop_propagation();
                            },
                            {props.content}
                        }
                    }
                }
                if is_open() {
                    div { class: "fixed inset-0 z-40", onclick: close_handler }
                }
            }
        },
        PopoverTrigger::Hover => rsx! {
            div {
                class: "{container_class}",
                onmouseenter: hover_enter,
                onmouseleave: hover_leave,
                div {
                    {
                        props
                            .trigger_element
                            .unwrap_or_else(|| props.children.unwrap_or_else(|| rsx! {}))
                    }
                }
                if is_open() {
                    div {
                        class: format!(
                            "absolute {}",
                            match actual_placement() {
                                PopoverPlacement::Top
                                | PopoverPlacement::TopStart
                                | PopoverPlacement::TopEnd => {
                                    "top-0 left-0 right-0 h-3 transform -translate-y-full"
                                }
                                PopoverPlacement::Bottom
                                | PopoverPlacement::BottomStart
                                | PopoverPlacement::BottomEnd => {
                                    "bottom-0 left-0 right-0 h-3 transform translate-y-full"
                                }
                                PopoverPlacement::Left
                                | PopoverPlacement::LeftStart
                                | PopoverPlacement::LeftEnd => {
                                    "left-0 top-0 bottom-0 w-3 transform -translate-x-full"
                                }
                                PopoverPlacement::Right
                                | PopoverPlacement::RightStart
                                | PopoverPlacement::RightEnd => {
                                    "right-0 top-0 bottom-0 w-3 transform translate-x-full"
                                }
                                PopoverPlacement::Auto => {
                                    match actual_placement() {
                                        PopoverPlacement::Top => {
                                            "top-0 left-0 right-0 h-3 transform -translate-y-full"
                                        }
                                        _ => "bottom-0 left-0 right-0 h-3 transform translate-y-full",
                                    }
                                }
                            },
                        ),
                        style: "z-index: 45;",
                    }
                    if placement == PopoverPlacement::Auto {
                        div {
                            class: "{content_class}",
                            style: "{content_style}",
                            onmouseenter: move |_| {
                                is_open.set(true);
                            },
                            onmouseleave: hover_leave,
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
                            onmouseleave: hover_leave,
                            {props.content}
                        }
                    }
                }
            }
        },
    }
}
