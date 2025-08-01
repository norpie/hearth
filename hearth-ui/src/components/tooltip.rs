//! Tooltip component with CSS-based arrow positioning
//!
//! Provides contextual information that appears on hover or focus with configurable arrow positioning.

use dioxus::prelude::*;
use crate::Platform;
use crate::viewport::use_viewport;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TooltipArrowSide {
    Top,
    Bottom,
}

#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// The tooltip content
    pub content: Element,
    /// The wrapped/trigger element
    pub children: Element,
    /// Optional class for styling
    #[props(default = "".to_string())]
    pub class: String,
    /// Optional width override (defaults to 200px)
    #[props(default = "200px".to_string())]
    pub width: String,
}

/// Tooltip component with CSS-based arrows
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let platform = Platform::current();
    let viewport = use_viewport();
    let mut show_tooltip = use_signal(|| false);
    let mut arrow_side = use_signal(|| TooltipArrowSide::Bottom);
    let mut arrow_offset = use_signal(|| 0.5); // Default 50% (center)
    let mut cursor_position = use_signal(|| None::<(f64, f64)>);
    
    let mut update_positioning = move |x: f64, y: f64| {
        let viewport_info = viewport.read();
        let screen_width = viewport_info.width;
        let screen_height = viewport_info.height;
        
        // Vertical positioning based on cursor/touch Y position
        if y > screen_height / 2.0 {
            // Cursor in bottom half - show tooltip above
            arrow_side.set(TooltipArrowSide::Bottom);
        } else {
            // Cursor in top half - show tooltip below
            arrow_side.set(TooltipArrowSide::Top);
        }
        
        // Horizontal offset based on cursor/touch X position
        let distance_from_center = (x - screen_width / 2.0).abs();
        let max_distance = screen_width / 2.0;
        let normalized_distance = distance_from_center / max_distance;
        
        let offset = if x < screen_width / 2.0 {
            // Cursor on left side - shift tooltip left
            0.5 - (normalized_distance * 0.3) // 0.5 to 0.2
        } else {
            // Cursor on right side - shift tooltip right
            0.5 + (normalized_distance * 0.3) // 0.5 to 0.8
        };
        
        arrow_offset.set(offset);
        cursor_position.set(Some((x, y)));
    };
    
    // Get positioning classes based on arrow side
    let (tooltip_position, arrow_class) = match arrow_side() {
        TooltipArrowSide::Top => (
            format!("top: calc(100% + 10px); left: 50%; transform: translateX(-{}%);", arrow_offset() * 100.0),
            "tooltip-arrow-top"
        ),
        TooltipArrowSide::Bottom => (
            format!("bottom: calc(100% + 10px); left: 50%; transform: translateX(-{}%);", arrow_offset() * 100.0),
            "tooltip-arrow-bottom"
        ),
    };
    
    rsx! {
        div {
            class: "relative",
            onmouseenter: move |event| {
                let coords = event.client_coordinates();
                update_positioning(coords.x, coords.y);
                show_tooltip.set(true);
            },
            onmousemove: move |event| {
                if show_tooltip() {
                    let coords = event.client_coordinates();
                    update_positioning(coords.x, coords.y);
                }
            },
            onmouseleave: move |_| show_tooltip.set(false),
            ontouchstart: move |event| {
                if let Some(touch) = event.touches().get(0) {
                    let coords = touch.client_coordinates();
                    update_positioning(coords.x, coords.y);
                }
                show_tooltip.set(true);
            },
            ontouchend: move |_| show_tooltip.set(false),
            {props.children}
            
            if show_tooltip() {
                div {
                    class: "absolute z-[99999] pointer-events-none {arrow_class} {props.class}",
                    style: "{tooltip_position} width: {props.width}; --arrow-position: {arrow_offset() * 100.0}%;",
                    
                    div {
                        class: if platform.is_mobile() {
                            "bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 p-4 rounded-lg shadow-lg border border-gray-200 dark:border-gray-600"
                        } else {
                            "bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 p-3 rounded-lg shadow-lg border border-gray-200 dark:border-gray-600"
                        },
                        
                        {props.content}
                    }
                }
            }
        }
    }
}

