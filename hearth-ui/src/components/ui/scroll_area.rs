//! Scroll area components with custom scrollbars and fade effects
//!
//! The scroll area module provides enhanced scrolling containers with customizable
//! scrollbars, fade effects, and smooth scrolling behavior. It supports vertical,
//! horizontal, or both-direction scrolling with optional fade indicators.
//!
//! # Examples
//!
//! Basic vertical scroll area:
//! ```rust
//! rsx! {
//!     ScrollArea {
//!         height: "300px".to_string(),
//!         fade_mode: FadeMode::Both,
//!         div { class: "space-y-4 p-4",
//!             // Content here
//!         }
//!     }
//! }
//! ```
//!
//! Horizontal scroll area:
//! ```rust
//! rsx! {
//!     ScrollArea {
//!         orientation: ScrollOrientation::Horizontal,
//!         width: "100%".to_string(),
//!         height: "200px".to_string(),
//!         div { class: "flex space-x-4 p-4",
//!             // Horizontal content
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use crate::Platform;
use std::rc::Rc;
use uuid;


/// Scroll orientation options controlling scrolling directions
///
/// Determines which directions the scroll area allows scrolling:
/// - `Vertical`: Only vertical scrolling (overflow-y: auto, overflow-x: hidden)
/// - `Horizontal`: Only horizontal scrolling (overflow-x: auto, overflow-y: hidden)
/// - `Both`: Both vertical and horizontal scrolling (overflow: auto)
#[derive(Clone, PartialEq, Debug)]
pub enum ScrollOrientation {
    /// Vertical scrolling only (up/down)
    Vertical,
    /// Horizontal scrolling only (left/right)
    Horizontal,
    /// Both vertical and horizontal scrolling
    Both,
}

/// Fade effect options for scroll boundaries
///
/// Controls gradient overlay placement to indicate scrollable content:
/// - `None`: No fade effects
/// - `Top`: Fade effect at the top edge when content is scrolled down
/// - `Bottom`: Fade effect at the bottom edge when content can be scrolled down
/// - `Both`: Fade effects at both top and bottom edges based on scroll position
#[derive(Clone, PartialEq, Debug)]
pub enum FadeMode {
    /// No fade effects
    None,
    /// Fade effect at top edge only
    Top,
    /// Fade effect at bottom edge only
    Bottom,
    /// Fade effects at both top and bottom edges
    Both,
}

/// Scrollbar visibility control options
///
/// Determines when scrollbars are displayed to the user:
/// - `Auto`: Browser default behavior (show when content overflows)
/// - `Always`: Scrollbars always visible regardless of content size
/// - `Never`: Scrollbars hidden (content still scrollable via touch/wheel)
#[derive(Clone, PartialEq, Debug)]
pub enum ScrollbarVisibility {
    /// Auto-show scrollbars when content overflows (browser default)
    Auto,
    /// Always show scrollbars regardless of content size
    Always,
    /// Never show scrollbars (content still scrollable)
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

/// Scroll position control for programmatic scrolling
#[derive(Clone, PartialEq, Debug)]
pub struct ScrollPosition {
    /// Vertical scroll position in pixels
    pub top: i32,
    /// Horizontal scroll position in pixels  
    pub left: i32,
    /// Whether to animate the scroll transition
    pub smooth: bool,
}

impl ScrollPosition {
    /// Create a new scroll position
    pub fn new(top: i32, left: i32) -> Self {
        Self { top, left, smooth: false }
    }

    /// Create a new scroll position with smooth animation
    pub fn smooth(top: i32, left: i32) -> Self {
        Self { top, left, smooth: true }
    }

    /// Create a scroll position for bottom of content (use a large value)
    pub fn bottom() -> Self {
        Self { top: i32::MAX, left: 0, smooth: false }
    }

    /// Create a scroll position for bottom of content with smooth animation
    pub fn bottom_smooth() -> Self {
        Self { top: i32::MAX, left: 0, smooth: true }
    }

    /// Create a scroll position for top of content
    pub fn top() -> Self {
        Self { top: 0, left: 0, smooth: false }
    }

    /// Create a scroll position for top of content with smooth animation
    pub fn top_smooth() -> Self {
        Self { top: 0, left: 0, smooth: true }
    }
}

impl Default for ScrollPosition {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

/// Scroll delta for relative scrolling operations
#[derive(Clone, PartialEq, Debug)]
pub struct ScrollDelta {
    /// Horizontal scroll delta in pixels (positive = scroll right)
    pub x: i32,
    /// Vertical scroll delta in pixels (positive = scroll down)
    pub y: i32,
    /// Whether to animate the scroll transition
    pub smooth: bool,
}

impl ScrollDelta {
    /// Create a new scroll delta
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y, smooth: false }
    }

    /// Create a new scroll delta with smooth animation
    pub fn smooth(x: i32, y: i32) -> Self {
        Self { x, y, smooth: true }
    }

    /// Create a delta for scrolling down
    pub fn down(pixels: i32) -> Self {
        Self { x: 0, y: pixels, smooth: false }
    }

    /// Create a delta for scrolling up
    pub fn up(pixels: i32) -> Self {
        Self { x: 0, y: -pixels, smooth: false }
    }

    /// Create a delta for scrolling right
    pub fn right(pixels: i32) -> Self {
        Self { x: pixels, y: 0, smooth: false }
    }

    /// Create a delta for scrolling left
    pub fn left(pixels: i32) -> Self {
        Self { x: -pixels, y: 0, smooth: false }
    }
}

/// Scroll action that can be sent to a ScrollController
#[derive(Clone, PartialEq, Debug)]
pub enum ScrollAction {
    /// Scroll to a specific position
    ScrollTo(ScrollPosition),
    /// Scroll by a relative amount
    ScrollBy(ScrollDelta),
}

/// Scroll control functions for programmatic scrolling
pub struct ScrollControl;

impl ScrollControl {
    /// Scroll to a specific position
    pub fn scroll_to(mut action_signal: Signal<Option<ScrollAction>>, position: ScrollPosition) {
        action_signal.set(Some(ScrollAction::ScrollTo(position)));
    }

    /// Scroll to the top of the content
    pub fn scroll_to_top(action_signal: Signal<Option<ScrollAction>>) {
        Self::scroll_to(action_signal, ScrollPosition::top());
    }

    /// Scroll to the top of the content with smooth animation
    pub fn scroll_to_top_smooth(action_signal: Signal<Option<ScrollAction>>) {
        Self::scroll_to(action_signal, ScrollPosition::top_smooth());
    }

    /// Scroll to the bottom of the content
    pub fn scroll_to_bottom(action_signal: Signal<Option<ScrollAction>>) {
        Self::scroll_to(action_signal, ScrollPosition::bottom());
    }

    /// Scroll to the bottom of the content with smooth animation
    pub fn scroll_to_bottom_smooth(action_signal: Signal<Option<ScrollAction>>) {
        Self::scroll_to(action_signal, ScrollPosition::bottom_smooth());
    }

    /// Scroll by a relative amount
    pub fn scroll_by(mut action_signal: Signal<Option<ScrollAction>>, delta: ScrollDelta) {
        action_signal.set(Some(ScrollAction::ScrollBy(delta)));
    }
}

/// Platform-independent scroll operations
pub struct ScrollOperations;

/// Scroll dimensions for initial measurement
#[derive(Clone, PartialEq, Debug)]
pub struct ScrollDimensions {
    pub scroll_top: i32,
    pub scroll_left: i32,
    pub scroll_width: i32,
    pub scroll_height: i32,
    pub client_width: i32,
    pub client_height: i32,
}

impl ScrollOperations {
    /// Execute a scroll action on an element by ID
    pub async fn execute_action_by_id(element_id: &str, action: ScrollAction) {
        match action {
            ScrollAction::ScrollTo(position) => {
                Self::scroll_to_by_id(element_id, position).await;
            }
            ScrollAction::ScrollBy(delta) => {
                Self::scroll_by_by_id(element_id, delta).await;
            }
        }
    }

    /// Get scroll dimensions for initial measurement
    pub async fn get_scroll_dimensions(element_id: &str) -> Option<ScrollDimensions> {
        Self::get_dimensions_by_id(element_id).await
    }

    #[cfg(target_arch = "wasm32")]
    async fn scroll_to_by_id(element_id: &str, position: ScrollPosition) {
        use web_sys::{window, ScrollBehavior, ScrollToOptions};
        
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(element) = document.get_element_by_id(element_id) {
                    let mut options = ScrollToOptions::new();
                    
                    let top = if position.top == i32::MAX {
                        element.scroll_height() as f64
                    } else {
                        position.top as f64
                    };
                    
                    options.top(top);
                    options.left(position.left as f64);
                    
                    if position.smooth {
                        options.behavior(ScrollBehavior::Smooth);
                    } else {
                        options.behavior(ScrollBehavior::Instant);
                    }
                    
                    element.scroll_to_with_scroll_to_options(&options);
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn scroll_to_by_id(element_id: &str, position: ScrollPosition) {
        use dioxus_document::eval;
        
        let behavior = if position.smooth { "smooth" } else { "instant" };
        let top = if position.top == i32::MAX { "element.scrollHeight" } else { &position.top.to_string() };
        
        let script = format!(
            r#"
            (() => {{
                const element = document.getElementById('{}');
                if (element) {{
                    element.scrollTo({{
                        top: {},
                        left: {},
                        behavior: '{}'
                    }});
                }}
            }})();
            "#,
            element_id, top, position.left, behavior
        );
        
        eval(&script);
    }

    #[cfg(target_arch = "wasm32")]
    async fn scroll_by_by_id(element_id: &str, delta: ScrollDelta) {
        use web_sys::{window, ScrollBehavior, ScrollToOptions};
        
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(element) = document.get_element_by_id(element_id) {
                    let mut options = ScrollToOptions::new();
                    options.top((element.scroll_top() + delta.y) as f64);
                    options.left((element.scroll_left() + delta.x) as f64);
                    
                    if delta.smooth {
                        options.behavior(ScrollBehavior::Smooth);
                    } else {
                        options.behavior(ScrollBehavior::Instant);
                    }
                    
                    element.scroll_to_with_scroll_to_options(&options);
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn scroll_by_by_id(element_id: &str, delta: ScrollDelta) {
        use dioxus_document::eval;
        
        let behavior = if delta.smooth { "smooth" } else { "instant" };
        
        let script = format!(
            r#"
            (() => {{
                const element = document.getElementById('{}');
                if (element) {{
                    element.scrollBy({{
                        top: {},
                        left: {},
                        behavior: '{}'
                    }});
                }}
            }})();
            "#,
            element_id, delta.y, delta.x, behavior
        );
        
        eval(&script);
    }

    #[cfg(target_arch = "wasm32")]
    async fn get_dimensions_by_id(element_id: &str) -> Option<ScrollDimensions> {
        use web_sys::window;
        
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(element) = document.get_element_by_id(element_id) {
                    return Some(ScrollDimensions {
                        scroll_top: element.scroll_top(),
                        scroll_left: element.scroll_left(),
                        scroll_width: element.scroll_width(),
                        scroll_height: element.scroll_height(),
                        client_width: element.client_width(),
                        client_height: element.client_height(),
                    });
                }
            }
        }
        None
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn get_dimensions_by_id(element_id: &str) -> Option<ScrollDimensions> {
        use dioxus_document::eval;
        
        let script = format!(
            r#"
            (() => {{
                const element = document.getElementById('{}');
                if (element) {{
                    return {{
                        scrollTop: element.scrollTop,
                        scrollLeft: element.scrollLeft,
                        scrollWidth: element.scrollWidth,
                        scrollHeight: element.scrollHeight,
                        clientWidth: element.clientWidth,
                        clientHeight: element.clientHeight
                    }};
                }}
                return null;
            }})();
            "#,
            element_id
        );
        
        let result = eval(&script);
        if let Ok(json_value) = result.await {
            if let Some(obj) = json_value.as_object() {
                if let (Some(scroll_top), Some(scroll_left), Some(scroll_width), 
                       Some(scroll_height), Some(client_width), Some(client_height)) = (
                    obj.get("scrollTop").and_then(|v| v.as_f64()),
                    obj.get("scrollLeft").and_then(|v| v.as_f64()),
                    obj.get("scrollWidth").and_then(|v| v.as_f64()),
                    obj.get("scrollHeight").and_then(|v| v.as_f64()),
                    obj.get("clientWidth").and_then(|v| v.as_f64()),
                    obj.get("clientHeight").and_then(|v| v.as_f64()),
                ) {
                    return Some(ScrollDimensions {
                        scroll_top: scroll_top as i32,
                        scroll_left: scroll_left as i32,
                        scroll_width: scroll_width as i32,
                        scroll_height: scroll_height as i32,
                        client_width: client_width as i32,
                        client_height: client_height as i32,
                    });
                }
            }
        }
        None
    }
}

/// Properties for configuring the ScrollArea component
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaProps {
    /// Content to be displayed within the scrollable area
    ///
    /// The content that will be rendered inside the scroll viewport.
    /// Should contain the scrollable content elements.
    pub children: Element,

    /// Scroll orientation determining allowed scroll directions
    ///
    /// Controls which directions scrolling is enabled. Affects overflow
    /// properties and fade effect positioning.
    /// Defaults to `ScrollOrientation::Vertical`.
    #[props(default)]
    pub orientation: ScrollOrientation,

    /// Fade effect mode for scroll boundaries
    ///
    /// Controls gradient overlay placement to visually indicate scrollable
    /// content beyond the visible area. Fade effects automatically show/hide
    /// based on scroll position and content overflow.
    /// Defaults to `FadeMode::None`.
    #[props(default)]
    pub fade_mode: FadeMode,

    /// Fade color class for gradient overlays
    ///
    /// Tailwind gradient from-color class for the fade effect. If not provided,
    /// defaults to from-background. Use this when the ScrollArea is on a 
    /// different colored background (e.g., cards, panels).
    /// Example values: "from-card", "from-background", "from-white", "from-slate-100"
    /// Defaults to None (uses from-background).
    #[props(default = None)]
    pub fade_color: Option<String>,

    /// Scrollbar visibility control
    ///
    /// Determines when scrollbars are displayed to users. Affects the
    /// scrollbar CSS classes applied to the viewport.
    /// Defaults to `ScrollbarVisibility::Auto`.
    #[props(default)]
    pub scrollbar_visibility: ScrollbarVisibility,

    /// Height constraint for the scroll area container
    ///
    /// CSS height value for the scroll container. Required for vertical
    /// scrolling when the container needs a fixed height. Can be any valid
    /// CSS height value (e.g., "300px", "50vh", "100%").
    /// Defaults to empty string (flexible height).
    #[props(default = String::new())]
    pub height: String,

    /// Width constraint for the scroll area container
    ///
    /// CSS width value for the scroll container. Required for horizontal
    /// scrolling when the container needs a fixed width. Can be any valid
    /// CSS width value (e.g., "500px", "50vw", "100%").
    /// Defaults to empty string (flexible width).
    #[props(default = String::new())]
    pub width: String,

    /// Additional CSS classes for the root container element
    ///
    /// Custom classes applied to the outermost scroll area container.
    /// Use for positioning, spacing, borders, or other container styling.
    /// Defaults to empty string.
    #[props(default = String::new())]
    pub class: String,

    /// Additional CSS classes for the scrollable viewport
    ///
    /// Custom classes applied to the inner scrollable viewport element.
    /// Use for content-specific styling or scroll behavior customization.
    /// Defaults to empty string.
    #[props(default = String::new())]
    pub viewport_class: String,

    /// Custom scroll event handler callback
    ///
    /// Called when the scroll position changes. Receives a ScrollEvent
    /// with scroll position, dimensions, and boundary information.
    /// Use for implementing scroll-based features like infinite loading.
    /// Defaults to None (no custom handling).
    #[props(default)]
    pub onscroll: Option<EventHandler<ScrollEvent>>,

    /// Tolerance for platform-specific precision issues in boundary detection
    ///
    /// Adds pixel tolerance to scroll boundary calculations to handle platform-specific
    /// floating-point precision issues. Useful for compact ScrollAreas where fade effects
    /// may not disappear properly at scroll boundaries on certain platforms.
    /// Set to 0 for precise boundary detection (default).
    /// Values between 10-20 pixels work well for most cases requiring tolerance.
    #[props(default = 0)]
    pub boundary_tolerance: i32,

    /// Optional scroll controller signal for programmatic scrolling
    ///
    /// Signal containing scroll actions that should be executed on the viewport.
    /// Create using use_signal(|| None) and pass to this prop. Use ScrollControl
    /// static methods to send scroll commands. Defaults to None (no programmatic scrolling control).
    #[props(default)]
    pub scroll_controller_signal: Option<Signal<Option<ScrollAction>>>,

    /// Callback fired when the viewport element is mounted
    ///
    /// Called with the MountedData of the scrollable viewport element.
    /// Use this to perform setup operations that require access to the DOM element.
    /// Defaults to None (no callback).
    #[props(default)]
    pub on_viewport_mounted: Option<EventHandler<Rc<MountedData>>>,

}

/// Scroll area component with custom scrollbars and fade effects
///
/// The ScrollArea component provides enhanced scrolling containers with custom
/// styling and fade effects. It supports different scroll orientations and
/// automatic fade indicators to show scrollable content boundaries.
#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    let mut scroll_top = use_signal(|| 0i32);
    let mut scroll_left = use_signal(|| 0i32);
    let mut can_scroll_up = use_signal(|| false);
    let mut can_scroll_down = use_signal(|| false);
    let mut can_scroll_left = use_signal(|| false);
    let mut can_scroll_right = use_signal(|| false);

    let viewport_id = use_signal(|| {
        format!("scroll-viewport-{}", uuid::Uuid::new_v4())
    });

    // Handle scroll controller actions
    if let Some(action_signal) = &props.scroll_controller_signal {
        let viewport_id = viewport_id();
        use_effect({
            let mut action_signal = *action_signal;
            move || {
                if let Some(action) = action_signal() {
                    let viewport_id = viewport_id.clone();
                    Platform::spawn(async move {
                        ScrollOperations::execute_action_by_id(&viewport_id, action).await;
                    });
                    // Clear the action after processing
                    action_signal.set(None);
                }
            }
        });
    }


    let container_classes = {
        let mut classes = vec!["relative", "overflow-hidden", "min-h-0"];
        
        // Add height classes - height prop should always be Tailwind classes
        if !props.height.is_empty() {
            classes.push(&props.height);
        }
        
        if !props.class.is_empty() {
            classes.push(&props.class);
        }
        
        classes.join(" ")
    };

    let viewport_classes = {
        let base = if props.height.is_empty() {
            "w-full"
        } else {
            "h-full w-full"
        };
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

        format!(
            "{} {} {} {}",
            base,
            overflow,
            scrollbar,
            if !props.viewport_class.is_empty() {
                &props.viewport_class
            } else {
                ""
            }
        )
    };

    let container_style = {
        let mut style = String::new();
        // Only width goes in style now - height is always Tailwind classes
        if !props.width.is_empty() {
            style.push_str(&format!("width: {}; ", props.width));
        }
        style
    };

    let handle_scroll = {
        let onscroll = props.onscroll;
        move |evt: ScrollEvent| {
            let current_scroll_top = evt.data().scroll_top();
            let current_scroll_left = evt.data().scroll_left();
            let scroll_width = evt.data().scroll_width();
            let scroll_height = evt.data().scroll_height();
            let client_width = evt.data().client_width();
            let client_height = evt.data().client_height();

            scroll_top.set(current_scroll_top);
            scroll_left.set(current_scroll_left);

            // Use configurable tolerance for platform-specific precision issues
            let tolerance = props.boundary_tolerance;
            let can_scroll_up_val = current_scroll_top > tolerance;
            let can_scroll_down_val = (current_scroll_top + client_height + tolerance) < scroll_height;
            let can_scroll_left_val = current_scroll_left > tolerance;
            let can_scroll_right_val = (current_scroll_left + client_width + tolerance) < scroll_width;

            can_scroll_up.set(can_scroll_up_val);
            can_scroll_down.set(can_scroll_down_val);
            can_scroll_left.set(can_scroll_left_val);
            can_scroll_right.set(can_scroll_right_val);

            if let Some(handler) = &onscroll {
                handler.call(evt);
            }
        }
    };

    // Determine fade color class
    let fade_from_class = props.fade_color.as_deref().unwrap_or("from-background");

    rsx! {
        div { class: container_classes, style: container_style,
            if matches!(props.fade_mode, FadeMode::Top | FadeMode::Both) && can_scroll_up() {
                div { class: format!("absolute top-0 left-0 right-0 h-8 bg-gradient-to-b {} to-transparent pointer-events-none z-10", fade_from_class) }
            }
            if matches!(props.fade_mode, FadeMode::Bottom | FadeMode::Both) && can_scroll_down() {
                div { class: format!("absolute bottom-0 left-0 right-0 h-8 bg-gradient-to-t {} to-transparent pointer-events-none z-10", fade_from_class) }
            }
            if matches!(props.orientation, ScrollOrientation::Horizontal | ScrollOrientation::Both)
                && can_scroll_left()
            {
                div { class: format!("absolute top-0 bottom-0 left-0 w-8 bg-gradient-to-r {} to-transparent pointer-events-none z-10", fade_from_class) }
            }
            if matches!(props.orientation, ScrollOrientation::Horizontal | ScrollOrientation::Both)
                && can_scroll_right()
            {
                div { class: format!("absolute top-0 bottom-0 right-0 w-8 bg-gradient-to-l {} to-transparent pointer-events-none z-10", fade_from_class) }
            }
            div {
                id: viewport_id(),
                class: viewport_classes,
                onscroll: handle_scroll,
                onmounted: move |mounted_event: MountedEvent| {
                    // Measure initial scroll state
                    let viewport_id = viewport_id();
                    let _orientation = props.orientation.clone();
                    let boundary_tolerance = props.boundary_tolerance;
                    Platform::spawn(async move {
                        if let Some(dimensions) = ScrollOperations::get_scroll_dimensions(&viewport_id).await {
                            let tolerance = boundary_tolerance;
                            let can_scroll_up_val = dimensions.scroll_top > tolerance;
                            let can_scroll_down_val = (dimensions.scroll_top + dimensions.client_height + tolerance) < dimensions.scroll_height;
                            let can_scroll_left_val = dimensions.scroll_left > tolerance;
                            let can_scroll_right_val = (dimensions.scroll_left + dimensions.client_width + tolerance) < dimensions.scroll_width;

                            can_scroll_up.set(can_scroll_up_val);
                            can_scroll_down.set(can_scroll_down_val);
                            can_scroll_left.set(can_scroll_left_val);
                            can_scroll_right.set(can_scroll_right_val);
                            
                            scroll_top.set(dimensions.scroll_top);
                            scroll_left.set(dimensions.scroll_left);
                        }
                    });
                    
                    // Call user-provided mounted handler if present
                    if let Some(handler) = &props.on_viewport_mounted {
                        handler.call(mounted_event.data());
                    }
                },
                {props.children}
            }
        }
    }
}
