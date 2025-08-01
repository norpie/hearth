//! Scroll Area component with custom scrollbars and fade effects

use dioxus::prelude::*;

/// Scroll orientation options
#[derive(Clone, PartialEq, Debug)]
pub enum ScrollOrientation {
    Vertical,
    Horizontal,
    Both,
}

/// Fade mode for scroll boundaries
#[derive(Clone, PartialEq, Debug)]
pub enum FadeMode {
    None,
    Top,
    Bottom,
    Both,
}

/// Scrollbar visibility options
#[derive(Clone, PartialEq, Debug)]
pub enum ScrollbarVisibility {
    Auto,
    Always,
    Never,
}

impl Default for ScrollOrientation {
    fn default() -> Self {
        Self::Vertical
    }
}

impl Default for FadeMode {
    fn default() -> Self {
        Self::None
    }
}

impl Default for ScrollbarVisibility {
    fn default() -> Self {
        Self::Auto
    }
}

/// Props for the Scroll Area component
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaProps {
    /// Content to be scrolled
    pub children: Element,
    
    /// Scroll orientation
    #[props(default)]
    pub orientation: ScrollOrientation,
    
    /// Fade mode for boundaries
    #[props(default)]
    pub fade_mode: FadeMode,
    
    /// Scrollbar visibility
    #[props(default)]
    pub scrollbar_visibility: ScrollbarVisibility,
    
    /// Height of the scroll area (required for vertical scrolling)
    #[props(default = String::new())]
    pub height: String,
    
    /// Width of the scroll area (required for horizontal scrolling)
    #[props(default = String::new())]
    pub width: String,
    
    /// Additional CSS classes for the root container
    #[props(default = String::new())]
    pub class: String,
    
    /// Additional CSS classes for the viewport
    #[props(default = String::new())]
    pub viewport_class: String,
    
    /// Scroll event handler
    #[props(default)]
    pub onscroll: Option<EventHandler<ScrollEvent>>,
}

/// Scroll Area component with custom scrollbars and fade effects
#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    let mut scroll_top = use_signal(|| 0i32);
    let mut scroll_left = use_signal(|| 0i32);
    let mut can_scroll_up = use_signal(|| false);
    let mut can_scroll_down = use_signal(|| false);
    let mut can_scroll_left = use_signal(|| false);
    let mut can_scroll_right = use_signal(|| false);
    
    // Generate a unique ID without using rand crate
    let viewport_id = use_signal(|| {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        std::thread::current().id().hash(&mut hasher);
        format!("scroll-viewport-{}", hasher.finish() % 10000)
    });
    
    // For initial state, assume content might be scrollable based on orientation
    // The scroll handler will update these values once scrolling starts
    use_effect({
        let orientation = props.orientation.clone();
        let mut can_scroll_up = can_scroll_up.clone();
        let mut can_scroll_down = can_scroll_down.clone();
        let mut can_scroll_left = can_scroll_left.clone();
        let mut can_scroll_right = can_scroll_right.clone();
        
        move || {
            match orientation {
                ScrollOrientation::Vertical => {
                    can_scroll_down.set(true); // Assume we can scroll down initially
                },
                ScrollOrientation::Horizontal => {
                    can_scroll_right.set(true); // Assume we can scroll right initially
                },
                ScrollOrientation::Both => {
                    can_scroll_down.set(true); // Assume we can scroll both directions initially
                    can_scroll_right.set(true);
                },
            }
        }
    });
    
    // Base container classes - ensure it can be a flex child
    // When no explicit height is set, assume this is in a flex layout and needs to fill available space
    let container_classes = format!(
        "relative overflow-hidden min-h-0 {}",
        if !props.class.is_empty() { &props.class } else { "" }
    );
    
    // Viewport classes based on orientation and scrollbar visibility
    let viewport_classes = {
        // Only use h-full if an explicit height is provided, otherwise let it size naturally
        let base = if props.height.is_empty() { "w-full" } else { "h-full w-full" };
        let overflow = match props.orientation {
            ScrollOrientation::Vertical => "overflow-y-auto overflow-x-hidden",
            ScrollOrientation::Horizontal => "overflow-x-auto overflow-y-hidden", 
            ScrollOrientation::Both => "overflow-auto",
        };
        
        let scrollbar = match props.scrollbar_visibility {
            ScrollbarVisibility::Never => "scrollbar-hide",
            ScrollbarVisibility::Always => "scrollbar-show",
            ScrollbarVisibility::Auto => "",
        };
        
        format!("{} {} {} {}", 
            base, 
            overflow, 
            scrollbar,
            if !props.viewport_class.is_empty() { &props.viewport_class } else { "" }
        )
    };
    
    // Container style with explicit dimensions
    let container_style = {
        let mut style = String::new();
        if !props.height.is_empty() {
            style.push_str(&format!("height: {}; ", props.height));
        }
        if !props.width.is_empty() {
            style.push_str(&format!("width: {}; ", props.width));
        }
        style
    };
    
    // Update scroll state with proper boundary detection
    let handle_scroll = {
        let onscroll = props.onscroll.clone();
        move |evt: ScrollEvent| {
            let current_scroll_top = evt.data().scroll_top();
            let current_scroll_left = evt.data().scroll_left();
            let scroll_width = evt.data().scroll_width();
            let scroll_height = evt.data().scroll_height();
            let client_width = evt.data().client_width();
            let client_height = evt.data().client_height();
            
            // Update scroll position
            scroll_top.set(current_scroll_top);
            scroll_left.set(current_scroll_left);
            
            // Calculate scroll boundaries - fade only shows when there's content to scroll to
            let can_scroll_up_val = current_scroll_top > 0;
            let can_scroll_down_val = (current_scroll_top + client_height) < scroll_height;
            let can_scroll_left_val = current_scroll_left > 0;
            let can_scroll_right_val = (current_scroll_left + client_width) < scroll_width;
            
            can_scroll_up.set(can_scroll_up_val);
            can_scroll_down.set(can_scroll_down_val);
            can_scroll_left.set(can_scroll_left_val);
            can_scroll_right.set(can_scroll_right_val);
            
            if let Some(handler) = &onscroll {
                handler.call(evt);
            }
        }
    };
    
    rsx! {
        div {
            class: container_classes,
            style: container_style,
            
            // Top fade overlay
            if matches!(props.fade_mode, FadeMode::Top | FadeMode::Both) && can_scroll_up() {
                div {
                    class: "absolute top-0 left-0 right-0 h-4 bg-gradient-to-b from-white to-transparent dark:from-gray-900 pointer-events-none z-10",
                }
            }
            
            // Bottom fade overlay  
            if matches!(props.fade_mode, FadeMode::Bottom | FadeMode::Both) && can_scroll_down() {
                div {
                    class: "absolute bottom-0 left-0 right-0 h-4 bg-gradient-to-t from-white to-transparent dark:from-gray-900 pointer-events-none z-10",
                }
            }
            
            // Left fade overlay
            if matches!(props.orientation, ScrollOrientation::Horizontal | ScrollOrientation::Both) && can_scroll_left() {
                div {
                    class: "absolute top-0 bottom-0 left-0 w-4 bg-gradient-to-r from-white to-transparent dark:from-gray-900 pointer-events-none z-10",
                }
            }
            
            // Right fade overlay
            if matches!(props.orientation, ScrollOrientation::Horizontal | ScrollOrientation::Both) && can_scroll_right() {
                div {
                    class: "absolute top-0 bottom-0 right-0 w-4 bg-gradient-to-l from-white to-transparent dark:from-gray-900 pointer-events-none z-10",
                }
            }
            
            // Viewport
            div {
                id: viewport_id(),
                class: viewport_classes,
                onscroll: handle_scroll,
                
                {props.children}
            }
        }
    }
}