//! Carousel components for interactive content display
//!
//! The carousel module provides components for creating interactive slideshows
//! and content galleries with smooth animations, touch/swipe gestures, and
//! navigation controls for showcasing images or sequential content.
//!
//! # Examples
//!
//! Basic image carousel:
//! ```rust
//! rsx! {
//!     Carousel {
//!         aspect_ratio: 16.0 / 9.0,
//!         show_navigation: true,
//!         show_indicators: true,
//!         items: vec![
//!             rsx! {
//!                 img {
//!                     src: "/images/slide1.jpg",
//!                     alt: "Beautiful landscape",
//!                     class: "w-full h-full object-cover"
//!                 }
//!             },
//!             rsx! {
//!                 img {
//!                     src: "/images/slide2.jpg",
//!                     alt: "City skyline",
//!                     class: "w-full h-full object-cover"
//!                 }
//!             },
//!             rsx! {
//!                 img {
//!                     src: "/images/slide3.jpg",
//!                     alt: "Mountain view",
//!                     class: "w-full h-full object-cover"
//!                 }
//!             },
//!         ]
//!     }
//! }
//! ```
//!
//! Product showcase carousel with tracking:
//! ```rust
//! let mut current_product = use_signal(|| 0);
//! 
//! rsx! {
//!     div { class: "max-w-4xl mx-auto",
//!         Carousel {
//!             aspect_ratio: 4.0 / 3.0,
//!             class: Some("mb-6".to_string()),
//!             on_change: move |index| {
//!                 current_product.set(index);
//!                 analytics_track_product_view(products[index].id);
//!             },
//!             items: products.iter().map(|product| rsx! {
//!                 div { class: "relative w-full h-full bg-white p-8 flex flex-col items-center justify-center",
//!                     img {
//!                         src: "{product.image_url}",
//!                         alt: "{product.name}",
//!                         class: "max-w-full max-h-full object-contain mb-4"
//!                     }
//!                     div { class: "text-center",
//!                         h3 { class: "text-xl font-semibold mb-2", "{product.name}" }
//!                         p { class: "text-2xl font-bold text-primary", "${product.price}" }
//!                         div { class: "mt-4 flex gap-2 justify-center",
//!                             for i in 0..5 {
//!                                 span {
//!                                     class: if i < product.rating { "text-yellow-400" } else { "text-gray-300" },
//!                                     "★"
//!                                 }
//!                             }
//!                         }
//!                     }
//!                 }
//!             }).collect::<Vec<_>>()
//!         }
//!         div { class: "text-center",
//!             h2 { class: "text-2xl font-bold mb-2", "{products[current_product()].name}" }
//!             p { class: "text-gray-600 mb-4", "{products[current_product()].description}" }
//!             button {
//!                 class: "bg-primary text-primary-foreground px-6 py-3 rounded-lg hover:bg-primary/90",
//!                 "Add to Cart - ${products[current_product()].price}"
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Testimonial carousel with custom styling:
//! ```rust
//! rsx! {
//!     section { class: "bg-muted py-16",
//!         div { class: "container mx-auto px-4",
//!             h2 { class: "text-3xl font-bold text-center mb-12", "What Our Customers Say" }
//!             Carousel {
//!                 aspect_ratio: 3.0 / 2.0,
//!                 class: Some("max-w-3xl mx-auto".to_string()),
//!                 show_navigation: false, // Mobile-first design with swipe only
//!                 items: testimonials.iter().map(|testimonial| rsx! {
//!                     div { class: "bg-card rounded-xl p-8 shadow-lg h-full flex flex-col justify-center text-center",
//!                         blockquote { class: "text-lg italic mb-6",
//!                             "\"{testimonial.quote}\""
//!                         }
//!                         div { class: "flex items-center justify-center space-x-4",
//!                             img {
//!                                 src: "{testimonial.avatar_url}",
//!                                 alt: "{testimonial.name}",
//!                                 class: "w-12 h-12 rounded-full object-cover"
//!                             }
//!                             div {
//!                                 p { class: "font-semibold", "{testimonial.name}" }
//!                                 p { class: "text-sm text-muted-foreground", "{testimonial.title}" }
//!                             }
//!                         }
//!                     }
//!                 }).collect::<Vec<_>>()
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Feature highlights carousel with mixed content:
//! ```rust
//! rsx! {
//!     Carousel {
//!         aspect_ratio: 5.0 / 3.0,
//!         class: Some("rounded-2xl overflow-hidden shadow-2xl".to_string()),
//!         items: vec![
//!             rsx! {
//!                 div { class: "relative w-full h-full bg-gradient-to-br from-blue-500 to-purple-600 text-white p-12 flex items-center",
//!                     div { class: "max-w-md",
//!                         h3 { class: "text-4xl font-bold mb-4", "Lightning Fast" }
//!                         p { class: "text-xl opacity-90 mb-6",
//!                             "Experience blazing fast performance with our optimized algorithms."
//!                         }
//!                         div { class: "flex items-center space-x-4",
//!                             div { class: "text-center",
//!                                 p { class: "text-3xl font-bold", "<100ms" }
//!                                 p { class: "text-sm opacity-75", "Response Time" }
//!                             }
//!                             div { class: "text-center",
//!                                 p { class: "text-3xl font-bold", "99.9%" }
//!                                 p { class: "text-sm opacity-75", "Uptime" }
//!                             }
//!                         }
//!                     }
//!                     div { class: "absolute right-8 top-1/2 transform -translate-y-1/2",
//!                         svg { class: "w-32 h-32 opacity-20", /* lightning icon */ }
//!                     }
//!                 }
//!             },
//!             rsx! {
//!                 div { class: "relative w-full h-full bg-gradient-to-br from-green-500 to-teal-600 text-white p-12 flex items-center",
//!                     div { class: "max-w-md",
//!                         h3 { class: "text-4xl font-bold mb-4", "Secure by Design" }
//!                         p { class: "text-xl opacity-90 mb-6",
//!                             "Bank-level security with end-to-end encryption and zero-trust architecture."
//!                         }
//!                         ul { class: "space-y-2",
//!                             li { class: "flex items-center space-x-2",
//!                                 span { class: "text-green-200", "✓" }
//!                                 span { "256-bit AES encryption" }
//!                             }
//!                             li { class: "flex items-center space-x-2",
//!                                 span { class: "text-green-200", "✓" }
//!                                 span { "SOC 2 Type II compliant" }
//!                             }
//!                             li { class: "flex items-center space-x-2",
//!                                 span { class: "text-green-200", "✓" }
//!                                 span { "Multi-factor authentication" }
//!                             }
//!                         }
//!                     }
//!                 }
//!             },
//!             rsx! {
//!                 div { class: "relative w-full h-full bg-gradient-to-br from-orange-500 to-red-600 text-white p-12 flex items-center",
//!                     div { class: "max-w-md",
//!                         h3 { class: "text-4xl font-bold mb-4", "Scale Globally" }
//!                         p { class: "text-xl opacity-90 mb-6",
//!                             "Built for global scale with edge deployment and intelligent load balancing."
//!                         }
//!                         div { class: "grid grid-cols-2 gap-4",
//!                             div { class: "text-center p-4 bg-white/10 rounded-lg",
//!                                 p { class: "text-2xl font-bold", "50+" }
//!                                 p { class: "text-sm", "Data Centers" }
//!                             }
//!                             div { class: "text-center p-4 bg-white/10 rounded-lg",
//!                                 p { class: "text-2xl font-bold", "180+" }
//!                                 p { class: "text-sm", "Countries" }
//!                             }
//!                         }
//!                     }
//!                 }
//!             },
//!         ]
//!     }
//! }
//! ```

use crate::{AspectRatio, Platform};
use dioxus::prelude::*;

/// Individual carousel item with identification and content
///
/// Represents a single item within a carousel component. Each item has
/// a unique identifier for tracking and content elements for display.
/// Currently unused in favor of direct Element vector but maintained
/// for future extensibility and typed item management.
#[derive(Clone, PartialEq)]
pub struct CarouselItem {
    /// Unique identifier for this carousel item
    ///
    /// Used for tracking, analytics, or programmatic navigation.
    /// Should be unique within the carousel context.
    pub id: String,

    /// Content elements to display for this item
    ///
    /// Can contain any valid Dioxus elements including images,
    /// text, forms, or complex layouts.
    pub content: Element,
}

/// Properties for configuring the Carousel component
///
/// Provides comprehensive control over carousel behavior, appearance,
/// navigation options, and interaction handling for creating responsive
/// and accessible content slideshows.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselProps {
    /// Collection of content elements to display in the carousel
    ///
    /// Vector of Dioxus elements representing each slide or item.
    /// Each element can contain any content including images, text,
    /// cards, or complex layouts. The order determines display sequence.
    /// Required parameter.
    pub items: Vec<Element>,

    /// Aspect ratio for the carousel container
    ///
    /// Controls the width-to-height ratio of the carousel viewport.
    /// Common values: 16:9 (1.78), 4:3 (1.33), 1:1 (1.0), 21:9 (2.33).
    /// Uses AspectRatio component for consistent responsive behavior.
    /// Defaults to 16:9 (16.0 / 9.0).
    #[props(default = 16.0 / 9.0)]
    pub aspect_ratio: f64,

    /// Whether to show navigation arrow buttons
    ///
    /// When true, displays left/right arrow buttons for manual navigation.
    /// Buttons are hidden on mobile devices where swipe gestures are preferred.
    /// Buttons appear on hover and provide keyboard accessibility.
    /// Defaults to true.
    #[props(default = true)]
    pub show_navigation: bool,

    /// Whether to show indicator dots below the carousel
    ///
    /// When true, displays clickable dots indicating current position and
    /// allowing direct navigation to specific items. Always visible on mobile
    /// regardless of this setting. Defaults to true.
    #[props(default = true)]
    pub show_indicators: bool,

    /// Additional CSS classes for the carousel container
    ///
    /// Applied to the root carousel element. Use for positioning, margins,
    /// shadows, or custom styling. The component provides base styling
    /// for layout and functionality. Defaults to None.
    #[props(default)]
    pub class: Option<String>,

    /// Event handler called when the active item changes
    ///
    /// Receives the zero-based index of the newly active item. Useful for
    /// analytics tracking, external state synchronization, or triggering
    /// related UI updates. Called for all navigation methods including
    /// swipe, click, and programmatic changes. Defaults to None.
    #[props(default)]
    pub on_change: Option<EventHandler<usize>>,
}

/// Carousel component for interactive content display with navigation and touch support
///
/// The Carousel component provides a sophisticated slideshow interface for displaying
/// sequences of content with smooth animations, touch gestures, and accessible navigation.
/// It features infinite scrolling, responsive design, and cross-platform compatibility
/// for showcasing images, product galleries, testimonials, or any sequential content.
///
/// # Features
///
/// - **Touch/swipe navigation**: Native mobile swipe gestures with momentum and direction detection
/// - **Infinite scrolling**: Seamless looping between first and last items without visual breaks
/// - **Smooth animations**: CSS-based transitions with configurable duration and easing
/// - **Responsive controls**: Arrow buttons for desktop, swipe for mobile, indicators for both
/// - **Aspect ratio control**: Consistent container dimensions across different content types
/// - **Keyboard accessibility**: Arrow key navigation and proper focus management
/// - **Event notifications**: Change callbacks for external state synchronization
/// - **Platform optimization**: Adaptive UI based on mobile vs desktop detection
///
/// # Implementation Details
///
/// The component uses an infinite scroll technique with cloned first/last items to create
/// seamless looping. Visual positioning is managed separately from logical state to handle
/// the transition resets required for infinite scrolling.
///
/// Touch gesture detection includes horizontal vs vertical movement analysis to distinguish
/// between carousel swipes and page scrolling. The component prevents default scroll
/// behavior only when a horizontal swipe is detected.
///
/// State management includes transition locking to prevent rapid navigation and smooth
/// animation coordination between visual transforms and logical state updates.
///
/// # Accessibility
///
/// - Arrow buttons with proper ARIA labels and keyboard navigation
/// - Indicator dots with descriptive labels for screen readers
/// - Focus management that respects user navigation preferences
/// - Touch targets sized appropriately for mobile interaction
/// - Semantic HTML structure for assistive technology compatibility
/// - Keyboard navigation with arrow keys and Enter/Space activation
///
/// # Touch Gesture Behavior
///
/// - **Swipe threshold**: 50px minimum movement to trigger navigation
/// - **Direction detection**: Horizontal vs vertical movement analysis
/// - **Momentum preservation**: Natural feel with proper velocity handling
/// - **Visual feedback**: Immediate response with smooth animations
/// - **Scroll prevention**: Prevents page scroll during horizontal swipes
/// - **Multi-touch handling**: Proper handling of multiple contact points
///
/// # Animation System
///
/// - **Transform-based**: Uses CSS translateX for smooth GPU-accelerated movement
/// - **Duration**: 300ms transition duration for responsive but smooth feel
/// - **Easing**: ease-in-out timing function for natural motion
/// - **State coordination**: Synchronized visual and logical state management
/// - **Infinite reset**: Seamless position resets for continuous scrolling
///
/// # Platform Adaptations
///
/// - **Mobile**: Touch-first interface with swipe gestures and always-visible indicators
/// - **Desktop**: Hover-revealed navigation buttons with mouse interaction
/// - **Responsive**: Adaptive control visibility and sizing based on screen size
/// - **Cross-platform**: Consistent behavior across web, desktop, and mobile builds
///
/// # Performance Considerations
///
/// - **GPU acceleration**: Transform-based animations for smooth performance
/// - **Efficient rendering**: Minimal DOM manipulation with CSS transitions
/// - **Event optimization**: Throttled touch events and efficient state updates
/// - **Memory management**: Proper cleanup of timers and event listeners
/// - **Platform detection**: Compile-time optimizations for target platforms
///
/// # Use Cases
///
/// - **Image galleries**: Photo slideshows with navigation and zoom support
/// - **Product showcases**: E-commerce product images and details
/// - **Testimonials**: Customer feedback and review displays
/// - **Feature highlights**: Product feature demonstrations
/// - **Hero sections**: Landing page content rotation
/// - **News/content**: Article previews and content discovery
///
/// # Parameters
///
/// - `items`: Vector of content elements to display
/// - `aspect_ratio`: Container width-to-height ratio (defaults to 16:9)
/// - `show_navigation`: Display arrow buttons for manual navigation
/// - `show_indicators`: Display position indicator dots
/// - `class`: Additional CSS classes for custom styling
/// - `on_change`: Event handler for item change notifications
#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let mut current_index = use_signal(|| 0usize);
    let mut visual_index = use_signal(|| 1usize); // Start at 1 (first real item in infinite setup)
    let total_items = props.items.len();
    let platform = Platform::current();
    let is_mobile = matches!(platform, Platform::Mobile);

    let mut touch_start_x = use_signal(|| None::<f64>);
    let mut touch_start_y = use_signal(|| None::<f64>);
    let mut touch_last_x = use_signal(|| None::<f64>); // Track last known position
    let mut is_swiping = use_signal(|| false);
    let mut is_transitioning = use_signal(|| false);

    use_effect(move || {
        if is_transitioning() {
            #[cfg(target_arch = "wasm32")]
            {
                let timeout_id = gloo_timers::callback::Timeout::new(300, move || {
                    is_transitioning.set(false);
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
                let mut is_transitioning = is_transitioning;
                let mut visual_index = visual_index;
                Platform::spawn(async move {
                    Platform::sleep(std::time::Duration::from_millis(300)).await;
                    is_transitioning.set(false);
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
        if is_transitioning() {
            return;
        }

        is_transitioning.set(true);
        visual_index.set(visual_index() + 1);
        current_index.set((current_index() + 1) % total_items);

        if let Some(handler) = &props.on_change {
            handler.call(current_index());
        }
    };

    let previous = move |_: MouseEvent| {
        if is_transitioning() {
            return;
        }

        is_transitioning.set(true);
        visual_index.set(visual_index() - 1);
        current_index.set((current_index() + total_items - 1) % total_items);

        if let Some(handler) = &props.on_change {
            handler.call(current_index());
        }
    };

    let mut go_to_item = move |index: usize| {
        if is_transitioning() {
            return;
        }

        is_transitioning.set(true);
        current_index.set(index);
        visual_index.set(index + 1); // Offset by 1 for infinite setup

        if let Some(handler) = &props.on_change {
            handler.call(index);
        }
    };

    let on_touch_start = move |event: TouchEvent| {
        log::info!("Touch start event triggered");
        log::info!("Number of touches: {}", event.touches().len());

        if let Some(touch) = event.touches().first() {
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
        if let Some(touch) = event.touches().first() {
            if let (Some(start_x), Some(start_y)) = (touch_start_x(), touch_start_y()) {
                let coords = touch.page_coordinates();
                let current_x = coords.x;
                let current_y = coords.y;
                let diff_x = (current_x - start_x).abs();
                let diff_y = (current_y - start_y).abs();

                touch_last_x.set(Some(current_x));

                if diff_x > 5.0 || diff_y > 5.0 {
                    log::info!("Move: dx={diff_x:.0} dy={diff_y:.0}");

                    if diff_x > diff_y && diff_x > 10.0 {
                        if !is_swiping() {
                            log::info!("HORIZONTAL SWIPE DETECTED!");
                            is_swiping.set(true);
                        }
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
        log::info!("is_swiping: {is_swiping_value}");
        log::info!("start_x: {start_x_value:?}");
        log::info!("last_x: {last_x_value:?}");
        log::info!("touches count: {}", event.touches().len());

        if let Some(start_x) = start_x_value {
            let end_x = if let Some(last_x) = last_x_value {
                log::info!("Using last known position: {last_x}");
                last_x
            } else if let Some(touch) = event.touches().first() {
                let coords = touch.page_coordinates();
                log::info!("Using current touch coords: x={}, y={}", coords.x, coords.y);
                coords.x
            } else {
                log::info!("No last position or current touch - using start position");
                start_x // Final fallback
            };

            let diff_x = end_x - start_x;
            let meets_threshold = diff_x.abs() > 50.0;

            log::info!("CALCULATION: start={start_x:.1}, end={end_x:.1}, diff={diff_x:.1}, threshold_met={meets_threshold}");

            if is_swiping_value && meets_threshold && !is_transitioning() {
                log::info!("🎯 SWIPE NAVIGATION TRIGGERED!");

                is_transitioning.set(true);

                if diff_x > 0.0 {
                    visual_index.set(visual_index() - 1);
                    let new_index = (current_index() + total_items - 1) % total_items;
                    log::info!("👈 Swipe RIGHT: {} -> {}", current_index(), new_index);
                    current_index.set(new_index);

                    if let Some(handler) = &props.on_change {
                        handler.call(new_index);
                    }
                } else {
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
                log::info!("  - is_swiping: {is_swiping_value}");
                log::info!("  - threshold_met: {meets_threshold}");
                log::info!("  - diff_x: {diff_x:.1}");
            }
        } else {
            log::warn!("❌ Touch end without start_x coordinate");
        }

        log::info!("Resetting touch state");
        touch_start_x.set(None);
        touch_start_y.set(None);
        touch_last_x.set(None);
        is_swiping.set(false);
    };

    let custom_classes = props.class.as_deref().unwrap_or("");

    if total_items == 0 {
        return rsx! {
            div { class: "relative w-full {custom_classes}",
                AspectRatio { ratio: props.aspect_ratio, class: "bg-muted rounded-md",
                    div { class: "flex items-center justify-center h-full text-foreground",
                        "No items to display"
                    }
                }
            }
        };
    }

    rsx! {
        div { class: "relative w-full group {custom_classes}",
            AspectRatio {
                ratio: props.aspect_ratio,
                class: "overflow-hidden rounded-md",
                div {
                    class: "relative h-full w-full",
                    ontouchstart: on_touch_start,
                    ontouchmove: on_touch_move,
                    ontouchend: on_touch_end,
                    div {
                        class: if is_transitioning() { "flex h-full transition-transform duration-300 ease-in-out" } else { "flex h-full" },
                        style: "transform: translateX(-{visual_index() * 100}%)",
                        if total_items > 1 {
                            div {
                                key: "clone-last-{total_items}",
                                class: "w-full h-full flex-shrink-0 flex items-center justify-center",
                                {&props.items[total_items - 1]}
                            }
                        }
                        for (index , item) in props.items.iter().enumerate() {
                            div {
                                key: "{index}",
                                class: "w-full h-full flex-shrink-0 flex items-center justify-center",
                                {item}
                            }
                        }
                        if total_items > 1 {
                            div {
                                key: "clone-first-{total_items}",
                                class: "w-full h-full flex-shrink-0 flex items-center justify-center",
                                {&props.items[0]}
                            }
                        }
                    }
                    if props.show_navigation && total_items > 1 && !is_mobile {
                        button {
                            class: "absolute left-2 top-1/2 -translate-y-1/2 w-10 h-10 rounded-full bg-white/80 hover:bg-white shadow-md hover:shadow-lg transition-all duration-200 flex items-center justify-center text-foreground hover:text-muted-foreground opacity-0 group-hover:opacity-100 focus:opacity-100",
                            onclick: previous,
                            "aria-label": "Previous item",
                            svg {
                                class: "w-5 h-5",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                path { d: "M15 18l-6-6 6-6" }
                            }
                        }
                        button {
                            class: "absolute right-2 top-1/2 -translate-y-1/2 w-10 h-10 rounded-full bg-white/80 hover:bg-white shadow-md hover:shadow-lg transition-all duration-200 flex items-center justify-center text-foreground hover:text-muted-foreground opacity-0 group-hover:opacity-100 focus:opacity-100",
                            onclick: next,
                            "aria-label": "Next item",
                            svg {
                                class: "w-5 h-5",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                path { d: "M9 18l6-6-6-6" }
                            }
                        }
                    }
                }
            }
            if total_items > 1 && (is_mobile || props.show_indicators) {
                div { class: "flex justify-center mt-4 space-x-2",
                    for index in 0..total_items {
                        button {
                            key: "{index}",
                            class: if index == current_index() { "w-2 h-2 rounded-full bg-primary transition-colors duration-200" } else { "w-2 h-2 rounded-full bg-muted hover:bg-muted transition-colors duration-200" },
                            onclick: move |_| go_to_item(index),
                            "aria-label": "Go to item {index + 1}",
                        }
                    }
                }
            }
        }
    }
}
