//! Gesture detection component for swipe gestures

use dioxus::prelude::*;
use crate::{ToastManager, ToastConfig, ToastType};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GestureDirection {
    Up,
    Down,
    Left,
    Right,
}

#[component]
pub fn GestureDetector(
    on_gesture: EventHandler<GestureDirection>,
    #[props(default = "fixed inset-0 pointer-events-auto z-50".to_string())] class: String,
    #[props(default = false)] debug: bool,
    children: Element,
) -> Element {
    let mut touch_start_x = use_signal(|| None::<f64>);
    let mut touch_start_y = use_signal(|| None::<f64>);
    let mut is_gesture = use_signal(|| false);
    let mut has_prevented = use_signal(|| false);
    let mut gesture_triggered = use_signal(|| false);
    
    // Minimum distance for early gesture detection (smaller threshold)
    let early_detection_distance = 15.0;
    // Minimum distance for a gesture to be recognized (in pixels)
    let min_distance = 50.0;
    
    // Toast manager for debug messages
    let toast_manager = if debug {
        Some(use_context::<ToastManager>())
    } else {
        None
    };
    
    
    let on_touch_move = move |event: TouchEvent| {
        if let Some(touch) = event.touches().first() {
            let coords = touch.page_coordinates();
            
            // If we don't have a start position yet, this is the first move - record it
            if touch_start_x().is_none() {
                touch_start_x.set(Some(coords.x));
                touch_start_y.set(Some(coords.y));
                return; // Don't process on first move
            }
            
            // Only process if we have a start position
            if let (Some(start_x), Some(start_y)) = (touch_start_x(), touch_start_y()) {
                let dx = coords.x - start_x;
                let dy = coords.y - start_y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                // Early detection: determine if this is likely a gesture (any direction)
                if !is_gesture() && distance >= early_detection_distance {
                    is_gesture.set(true);
                    // Now we know it's a gesture, prevent default to stop scrolling/clicking
                    event.prevent_default();
                    has_prevented.set(true);
                    
                    if let Some(ref toast_manager) = toast_manager {
                        let is_horizontal = dx.abs() > dy.abs();
                        let direction_hint = if is_horizontal {
                            "horizontal"
                        } else {
                            "vertical"
                        };
                        toast_manager.add_toast(ToastConfig {
                            message: format!("Detected {} gesture!", direction_hint),
                            toast_type: ToastType::Info,
                            duration: Some(std::time::Duration::from_secs(1)),
                            dismissible: true,
                        });
                    }
                }
                
                // If we've determined this is a gesture, continue processing
                if is_gesture() {
                    // Prevent default to ensure we capture the gesture
                    if !has_prevented() {
                        event.prevent_default();
                        has_prevented.set(true);
                    }
                    
                    if distance >= min_distance && !gesture_triggered() {
                        // Determine primary direction based on larger component
                        let direction = if dx.abs() > dy.abs() {
                            if dx > 0.0 { GestureDirection::Right } else { GestureDirection::Left }
                        } else {
                            if dy > 0.0 { GestureDirection::Down } else { GestureDirection::Up }
                        };
                        
                        if let Some(ref toast_manager) = toast_manager {
                            let direction_str = match direction {
                                GestureDirection::Up => "Up ⬆️",
                                GestureDirection::Down => "Down ⬇️", 
                                GestureDirection::Left => "Left ⬅️",
                                GestureDirection::Right => "Right ➡️",
                            };
                            toast_manager.add_toast(ToastConfig {
                                message: format!("Gesture: {} (distance: {:.0}px)", direction_str, distance),
                                toast_type: ToastType::Success,
                                duration: Some(std::time::Duration::from_secs(2)),
                                dismissible: true,
                            });
                        }
                        
                        on_gesture.call(direction);
                        
                        // Mark gesture as triggered to prevent multiple triggers in same touch sequence
                        gesture_triggered.set(true);
                    }
                }
            }
        }
    };
    
    let on_touch_end = move |_event: TouchEvent| {
        // Reset touch state - allows new gesture on next touch sequence
        touch_start_x.set(None);
        touch_start_y.set(None);
        is_gesture.set(false);
        has_prevented.set(false);
        gesture_triggered.set(false);
    };

    let debug_classes = if debug {
        " bg-red-500/20 border-2 border-dashed border-red-5000"
    } else {
        ""
    };

    rsx! {
        div {
            class: format!("{}{}", class, debug_classes),
            ontouchmove: on_touch_move,
            ontouchend: on_touch_end,
            {children}
        }
    }
}
