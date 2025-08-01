//! Gesture detector component showcase

use crate::{ComponentShowcase, ShowcaseVariant, GestureDetector, GestureDirection, ToastManager, ToastConfig, ToastType};
use dioxus::prelude::*;

#[component]
pub fn GestureDetectorShowcase() -> Element {
    let toast_manager = use_context::<ToastManager>();
    
    let handle_gesture = move |direction: GestureDirection| {
        let message = match direction {
            GestureDirection::Up => "🔼 Swipe Up detected!",
            GestureDirection::Down => "🔽 Swipe Down detected!",
            GestureDirection::Left => "◀️ Swipe Left detected!",
            GestureDirection::Right => "▶️ Swipe Right detected!",
        };
        
        toast_manager.add_toast(ToastConfig {
            message: message.to_string(),
            toast_type: ToastType::Success,
            duration: Some(std::time::Duration::from_secs(3)),
            dismissible: true,
        });
    };

    rsx! {
        ComponentShowcase {
            name: "Gesture Detector".to_string(),
            description: "An invisible overlay component that detects swipe gestures and triggers callbacks. Works with touch events on mobile.".to_string(),
            basic_usage: r#"GestureDetector {
    on_gesture: move |direction| {
        // Handle gesture
    },
    div { "Swipe me!" }
}"#.to_string(),
            with_props_usage: r#"GestureDetector {
    class: "absolute top-0 left-0 w-1/2 h-1/2 z-10",
    on_gesture: move |direction| {
        match direction {
            GestureDirection::Up => println!("Swiped up!"),
            GestureDirection::Down => println!("Swiped down!"),
            GestureDirection::Left => println!("Swiped left!"),
            GestureDirection::Right => println!("Swiped right!"),
        }
    },
    div { "Content here" }
}"#.to_string(),

            ShowcaseVariant {
                title: "Interactive Demo",
                div { class: "space-y-3",
                    p { class: "text-sm text-muted-foreground", 
                        "Try swiping in any direction within the demo area below. Works with touch swipe on mobile." 
                    }
                    div { class: "relative",
                        GestureDetector {
                            class: "absolute inset-0 z-10",
                            debug: true,
                            on_gesture: handle_gesture,
                        }
                        div { 
                            class: "w-full h-64 bg-gradient-to-br from-blue-100 to-purple-100 dark:from-blue-900 dark:to-purple-900 border-2 border-dashed border-blue-300 dark:border-blue-600 rounded-lg flex flex-col items-center justify-center text-center p-6 relative",
                            div { class: "text-lg font-medium text-foreground mb-2",
                                "👆 Swipe or Drag Here"
                            }
                            div { class: "text-sm text-muted-foreground mb-4",
                                "Try swiping up, down, left, or right"
                            }
                            div { class: "grid grid-cols-3 gap-2 text-xs text-muted-foreground",
                                div {}
                                div { class: "flex justify-center", "🔼" }
                                div {}
                                div { class: "flex justify-center", "◀️" }
                                div { class: "flex justify-center", "⭕" }
                                div { class: "flex justify-center", "▶️" }
                                div {}
                                div { class: "flex justify-center", "🔽" }
                                div {}
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Direction Types",
                div { class: "space-y-4",
                    p { class: "text-sm text-muted-foreground mb-4", 
                        "The GestureDirection enum provides four directional values that can be matched in the callback." 
                    }
                    div { class: "space-y-4",
                        div { class: "grid grid-cols-2 gap-4",
                            div { class: "p-3 bg-muted rounded border",
                                div { class: "font-medium text-sm mb-1", "GestureDirection::Up" }
                                div { class: "text-xs text-muted-foreground", "Detected when swiping upward" }
                            }
                            div { class: "p-3 bg-muted rounded border",
                                div { class: "font-medium text-sm mb-1", "GestureDirection::Down" }
                                div { class: "text-xs text-muted-foreground", "Detected when swiping downward" }
                            }
                            div { class: "p-3 bg-muted rounded border",
                                div { class: "font-medium text-sm mb-1", "GestureDirection::Left" }
                                div { class: "text-xs text-muted-foreground", "Detected when swiping leftward" }
                            }
                            div { class: "p-3 bg-muted rounded border",
                                div { class: "font-medium text-sm mb-1", "GestureDirection::Right" }
                                div { class: "text-xs text-muted-foreground", "Detected when swiping rightward" }
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Properties",
                div { class: "space-y-4",
                    p { class: "text-sm text-muted-foreground mb-4", 
                        "Key properties and behaviors of the GestureDetector component." 
                    }
                    div { class: "space-y-3",
                        div { class: "flex items-start space-x-3",
                            div { class: "w-2 h-2 bg-blue-500 rounded-full mt-2 flex-shrink-0" }
                            div {
                                div { class: "font-medium text-sm", "Invisible & Non-blocking" }
                                div { class: "text-xs text-muted-foreground", "Component is transparent and doesn't interfere with underlying content" }
                            }
                        }
                        div { class: "flex items-start space-x-3",
                            div { class: "w-2 h-2 bg-green-500 rounded-full mt-2 flex-shrink-0" }
                            div {
                                div { class: "font-medium text-sm", "Cross-platform Support" }
                                div { class: "text-xs text-muted-foreground", "Works with touch events on mobile and mouse events on desktop" }
                            }
                        }
                        div { class: "flex items-start space-x-3",
                            div { class: "w-2 h-2 bg-purple-500 rounded-full mt-2 flex-shrink-0" }
                            div {
                                div { class: "font-medium text-sm", "Minimum Distance Threshold" }
                                div { class: "text-xs text-muted-foreground", "Requires 50px minimum movement to trigger a gesture (prevents accidental activation)" }
                            }
                        }
                        div { class: "flex items-start space-x-3",
                            div { class: "w-2 h-2 bg-orange-500 rounded-full mt-2 flex-shrink-0" }
                            div {
                                div { class: "font-medium text-sm", "Directional Priority" }
                                div { class: "text-xs text-muted-foreground", "Determines direction based on the axis with larger movement (X or Y)" }
                            }
                        }
                        div { class: "flex items-start space-x-3",
                            div { class: "w-2 h-2 bg-red-500 rounded-full mt-2 flex-shrink-0" }
                            div {
                                div { class: "font-medium text-sm", "Customizable Positioning" }
                                div { class: "text-xs text-muted-foreground", "Use the 'class' prop to position the detector anywhere on screen" }
                            }
                        }
                    }
                }
            }
        }
    }
}