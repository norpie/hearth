//! Carousel component for displaying items with navigation

use dioxus::prelude::*;
use crate::{AspectRatio, Platform};

#[cfg(target_arch = "wasm32")]
use gloo_timers;

#[cfg(not(target_arch = "wasm32"))]
use tokio;

#[derive(Clone, PartialEq)]
pub struct CarouselItem {
    pub id: String,
    pub content: Element,
}

#[derive(Props, Clone, PartialEq)]
pub struct CarouselProps {
    /// Items to display in the carousel
    pub items: Vec<Element>,
    #[props(default = 16.0 / 9.0)]
    pub aspect_ratio: f64,
    #[props(default = true)]
    pub show_navigation: bool,
    #[props(default = true)]
    pub show_indicators: bool,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub on_change: Option<EventHandler<usize>>,
}

#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let mut current_index = use_signal(|| 0usize);
    let mut visual_index = use_signal(|| 1usize); // Start at 1 (first real item in infinite setup)
    let total_items = props.items.len();
    let platform = Platform::current();
    let is_mobile = matches!(platform, Platform::Mobile);
    
    // Touch gesture state
    let mut touch_start_x = use_signal(|| None::<f64>);
    let mut touch_start_y = use_signal(|| None::<f64>);
    let mut touch_last_x = use_signal(|| None::<f64>); // Track last known position
    let mut is_swiping = use_signal(|| false);
    let mut is_transitioning = use_signal(|| false);
    
    // Handle infinite scroll reset after transition
    use_effect(move || {
        if is_transitioning() {
            #[cfg(target_arch = "wasm32")]
            {
                let timeout_id = gloo_timers::callback::Timeout::new(300, move || {
                    is_transitioning.set(false);
                    // Reset visual position for infinite scroll
                    if visual_index() == 0 {
                        visual_index.set(total_items);
                    } else if visual_index() == total_items + 1 {
                        visual_index.set(1);
                    }
                });
                timeout_id.forget();
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            {
                let mut is_transitioning = is_transitioning.clone();
                let mut visual_index = visual_index.clone();
                spawn(async move {
                    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
                    is_transitioning.set(false);
                    // Reset visual position for infinite scroll
                    if visual_index() == 0 {
                        visual_index.set(total_items);
                    } else if visual_index() == total_items + 1 {
                        visual_index.set(1);
                    }
                });
            }
        }
    });
    
    let next = move |_: MouseEvent| {
        if is_transitioning() { return; }
        
        is_transitioning.set(true);
        visual_index.set(visual_index() + 1);
        current_index.set((current_index() + 1) % total_items);
        
        if let Some(handler) = &props.on_change {
            handler.call(current_index());
        }
    };
    
    let previous = move |_: MouseEvent| {
        if is_transitioning() { return; }
        
        is_transitioning.set(true);
        visual_index.set(visual_index() - 1);
        current_index.set((current_index() + total_items - 1) % total_items);
        
        if let Some(handler) = &props.on_change {
            handler.call(current_index());
        }
    };
    
    let mut go_to_item = move |index: usize| {
        if is_transitioning() { return; }
        
        is_transitioning.set(true);
        current_index.set(index);
        visual_index.set(index + 1); // Offset by 1 for infinite setup
        
        if let Some(handler) = &props.on_change {
            handler.call(index);
        }
    };
    
    // Touch event handlers
    let on_touch_start = move |event: TouchEvent| {
        log::info!("Touch start event triggered");
        log::info!("Number of touches: {}", event.touches().len());
        
        if let Some(touch) = event.touches().get(0) {
            let coords = touch.page_coordinates();
            log::info!("Touch start coordinates: x={}, y={}", coords.x, coords.y);
            touch_start_x.set(Some(coords.x));
            touch_start_y.set(Some(coords.y));
            touch_last_x.set(Some(coords.x)); // Initialize last position
            is_swiping.set(false);
            log::info!("Touch start state set successfully");
        } else {
            log::warn!("No touches found in touch start event");
        }
    };
    
    let on_touch_move = move |event: TouchEvent| {
        if let Some(touch) = event.touches().get(0) {
            if let (Some(start_x), Some(start_y)) = (touch_start_x(), touch_start_y()) {
                let coords = touch.page_coordinates();
                let current_x = coords.x;
                let current_y = coords.y;
                let diff_x = (current_x - start_x).abs();
                let diff_y = (current_y - start_y).abs();
                
                // Update last known position
                touch_last_x.set(Some(current_x));
                
                // Only log significant movements to reduce spam
                if diff_x > 5.0 || diff_y > 5.0 {
                    log::info!("Move: dx={:.0} dy={:.0}", diff_x, diff_y);
                    
                    // Determine if this is a horizontal swipe (not vertical scroll)
                    if diff_x > diff_y && diff_x > 10.0 {
                        if !is_swiping() {
                            log::info!("HORIZONTAL SWIPE DETECTED!");
                            is_swiping.set(true);
                        }
                        // Prevent default to avoid scrolling
                        event.prevent_default();
                    }
                }
            }
        }
    };
    
    let on_touch_end = move |event: TouchEvent| {
        let is_swiping_value = is_swiping();
        let start_x_value = touch_start_x();
        let last_x_value = touch_last_x();
        
        log::info!("=== TOUCH END ===");
        log::info!("is_swiping: {}", is_swiping_value);
        log::info!("start_x: {:?}", start_x_value);
        log::info!("last_x: {:?}", last_x_value);
        log::info!("touches count: {}", event.touches().len());
        
        if let Some(start_x) = start_x_value {
            // Use last known position instead of trying to get from empty touches
            let end_x = if let Some(last_x) = last_x_value {
                log::info!("Using last known position: {}", last_x);
                last_x
            } else if let Some(touch) = event.touches().get(0) {
                let coords = touch.page_coordinates();
                log::info!("Using current touch coords: x={}, y={}", coords.x, coords.y);
                coords.x
            } else {
                log::info!("No last position or current touch - using start position");
                start_x // Final fallback
            };
            
            let diff_x = end_x - start_x;
            let meets_threshold = diff_x.abs() > 50.0;
            
            log::info!("CALCULATION: start={:.1}, end={:.1}, diff={:.1}, threshold_met={}", 
                start_x, end_x, diff_x, meets_threshold);
            
            // Only trigger swipe if we were actually swiping horizontally
            if is_swiping_value && meets_threshold && !is_transitioning() {
                log::info!("🎯 SWIPE NAVIGATION TRIGGERED!");
                
                is_transitioning.set(true);
                
                if diff_x > 0.0 {
                    // Swipe right - go to previous (continues right infinitely)
                    visual_index.set(visual_index() - 1);
                    let new_index = (current_index() + total_items - 1) % total_items;
                    log::info!("👈 Swipe RIGHT: {} -> {}", current_index(), new_index);
                    current_index.set(new_index);
                    
                    if let Some(handler) = &props.on_change {
                        handler.call(new_index);
                    }
                } else {
                    // Swipe left - go to next (continues left infinitely)
                    visual_index.set(visual_index() + 1);
                    let new_index = (current_index() + 1) % total_items;
                    log::info!("👉 Swipe LEFT: {} -> {}", current_index(), new_index);
                    current_index.set(new_index);
                    
                    if let Some(handler) = &props.on_change {
                        handler.call(new_index);
                    }
                }
            } else {
                log::info!("❌ SWIPE NOT TRIGGERED:");
                log::info!("  - is_swiping: {}", is_swiping_value);
                log::info!("  - threshold_met: {}", meets_threshold);
                log::info!("  - diff_x: {:.1}", diff_x);
            }
        } else {
            log::warn!("❌ Touch end without start_x coordinate");
        }
        
        // Reset touch state
        log::info!("Resetting touch state");
        touch_start_x.set(None);
        touch_start_y.set(None);
        touch_last_x.set(None);
        is_swiping.set(false);
    };
    
    
    let custom_classes = props.class.as_deref().unwrap_or("");
    
    if total_items == 0 {
        return rsx! {
            div {
                class: "relative w-full {custom_classes}",
                AspectRatio {
                    ratio: props.aspect_ratio,
                    class: "bg-gray-100 dark:bg-gray-800 rounded-md",
                    div {
                        class: "flex items-center justify-center h-full text-gray-500 dark:text-gray-400",
                        "No items to display"
                    }
                }
            }
        };
    }
    
    rsx! {
        div {
            class: "relative w-full group {custom_classes}",
            
            // Main carousel content
            AspectRatio {
                ratio: props.aspect_ratio,
                class: "overflow-hidden rounded-md",
                div {
                    class: "relative h-full w-full",
                    ontouchstart: on_touch_start,
                    ontouchmove: on_touch_move,
                    ontouchend: on_touch_end,
                    
                    // Items container with infinite scroll setup
                    div {
                        class: if is_transitioning() {
                            "flex h-full transition-transform duration-300 ease-in-out"
                        } else {
                            "flex h-full"
                        },
                        style: "transform: translateX(-{visual_index() * 100}%)",
                        
                        // Last item (for infinite scroll from first to last)
                        if total_items > 1 {
                            div {
                                key: "clone-last-{total_items}",
                                class: "w-full h-full flex-shrink-0 flex items-center justify-center",
                                {&props.items[total_items - 1]}
                            }
                        }
                        
                        // Original items
                        for (index, item) in props.items.iter().enumerate() {
                            div {
                                key: "{index}",
                                class: "w-full h-full flex-shrink-0 flex items-center justify-center",
                                {item}
                            }
                        }
                        
                        // First item (for infinite scroll from last to first)
                        if total_items > 1 {
                            div {
                                key: "clone-first-{total_items}",
                                class: "w-full h-full flex-shrink-0 flex items-center justify-center",
                                {&props.items[0]}
                            }
                        }
                    }
                    
                    // Navigation buttons (hidden on mobile, use swipe instead)
                    if props.show_navigation && total_items > 1 && !is_mobile {
                        // Previous button
                        button {
                            class: "absolute left-2 top-1/2 -translate-y-1/2 w-10 h-10 rounded-full bg-white/80 dark:bg-gray-800/80 hover:bg-white dark:hover:bg-gray-800 shadow-md hover:shadow-lg transition-all duration-200 flex items-center justify-center text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-gray-100 opacity-0 group-hover:opacity-100 focus:opacity-100",
                            onclick: previous,
                            "aria-label": "Previous item",
                            
                            svg {
                                class: "w-5 h-5",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                path {
                                    d: "M15 18l-6-6 6-6"
                                }
                            }
                        }
                        
                        // Next button  
                        button {
                            class: "absolute right-2 top-1/2 -translate-y-1/2 w-10 h-10 rounded-full bg-white/80 dark:bg-gray-800/80 hover:bg-white dark:hover:bg-gray-800 shadow-md hover:shadow-lg transition-all duration-200 flex items-center justify-center text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-gray-100 opacity-0 group-hover:opacity-100 focus:opacity-100",
                            onclick: next,
                            "aria-label": "Next item",
                            
                            svg {
                                class: "w-5 h-5",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                path {
                                    d: "M9 18l6-6-6-6"
                                }
                            }
                        }
                    }
                }
            }
            
            // Indicators (always visible on mobile, show_indicators setting on desktop)
            if total_items > 1 && (is_mobile || props.show_indicators) {
                div {
                    class: "flex justify-center mt-4 space-x-2",
                    
                    for index in 0..total_items {
                        button {
                            key: "{index}",
                            class: if index == current_index() {
                                "w-2 h-2 rounded-full bg-blue-600 dark:bg-blue-400 transition-colors duration-200"
                            } else {
                                "w-2 h-2 rounded-full bg-gray-300 dark:bg-gray-600 hover:bg-gray-400 dark:hover:bg-gray-500 transition-colors duration-200"
                            },
                            onclick: move |_| go_to_item(index),
                            "aria-label": "Go to item {index + 1}"
                        }
                    }
                }
            }
        }
    }
}