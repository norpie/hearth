//! Aspect ratio container component for maintaining proportional content
//!
//! The AspectRatio component provides a container that maintains a specific width-to-height
//! ratio regardless of the actual content size. This is essential for creating consistent
//! layouts with media content, placeholders, and responsive design elements that need to
//! preserve their proportions across different screen sizes.
//!
//! # Features
//!
//! - **Flexible ratios**: Support for any numeric aspect ratio (width/height)
//! - **Responsive design**: Maintains proportions across all screen sizes
//! - **Content preservation**: Child content fills the available space
//! - **CSS-based solution**: Uses padding-bottom percentage technique
//! - **Performance optimized**: Minimal JavaScript, pure CSS approach
//! - **Custom styling**: Support for additional CSS classes
//! - **Absolute positioning**: Child content positioned absolutely for perfect fit
//! - **Full coverage**: Child content covers the entire aspect ratio container
//!
//! # Examples
//!
//! Standard 16:9 video aspect ratio:
//! ```rust
//! rsx! {
//!     AspectRatio {
//!         ratio: 16.0 / 9.0,
//!         class: Some("bg-muted rounded-lg overflow-hidden".to_string()),
//!         video {
//!             class: "w-full h-full object-cover",
//!             src: "video.mp4",
//!             controls: true
//!         }
//!     }
//! }
//! ```
//!
//! Square aspect ratio for profile images:
//! ```rust
//! rsx! {
//!     AspectRatio {
//!         ratio: 1.0,
//!         class: Some("rounded-full overflow-hidden shadow-lg".to_string()),
//!         img {
//!             class: "w-full h-full object-cover",
//!             src: "profile.jpg",
//!             alt: "Profile picture"
//!         }
//!     }
//! }
//! ```
//!
//! Wide banner aspect ratio:
//! ```rust
//! rsx! {
//!     AspectRatio {
//!         ratio: 21.0 / 9.0,
//!         class: Some("bg-gradient-to-r from-blue-500 to-purple-600 rounded-lg".to_string()),
//!         div {
//!             class: "flex items-center justify-center text-white text-2xl font-bold",
//!             "Hero Banner Content"
//!         }
//!     }
//! }
//! ```
//!
//! Placeholder with specific ratio:
//! ```rust
//! rsx! {
//!     AspectRatio {
//!         ratio: 4.0 / 3.0,
//!         class: Some("border-2 border-dashed border-border bg-muted".to_string()),
//!         div {
//!             class: "flex items-center justify-center text-muted-foreground",
//!             span { "Image placeholder 4:3" }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Properties for configuring the AspectRatio component
///
/// Provides control over the aspect ratio constraint and styling of the container.
/// The component uses mathematical ratios to maintain precise proportional relationships
/// between width and height regardless of container size.
#[derive(Props, Clone, PartialEq)]
pub struct AspectRatioProps {
    /// The desired aspect ratio as a floating-point number (width/height)
    ///
    /// Calculated as width divided by height. Common examples:
    /// - `16.0 / 9.0` = 1.777... for widescreen video (16:9)
    /// - `4.0 / 3.0` = 1.333... for traditional video/photos (4:3) 
    /// - `1.0` for perfect squares (1:1)
    /// - `21.0 / 9.0` = 2.333... for ultra-wide banners (21:9)
    /// - `3.0 / 2.0` = 1.5 for standard photo prints (3:2)
    /// Required parameter.
    pub ratio: f64,

    /// Additional CSS classes to apply to the container
    ///
    /// Custom classes are applied to the outer container element.
    /// Use for styling like backgrounds, borders, shadows, or spacing.
    /// The inner content area will automatically fill the aspect ratio.
    /// Defaults to None.
    #[props(default)]
    pub class: Option<String>,

    /// Content to display within the aspect ratio container
    ///
    /// Child content is positioned absolutely to fill the entire aspect ratio area.
    /// Common content includes images, videos, iframes, or placeholder content.
    /// Content should use `w-full h-full` classes to fill the available space.
    pub children: Element,
}

/// Container component for maintaining specific aspect ratios
///
/// The AspectRatio component creates a responsive container that maintains a precise
/// width-to-height ratio regardless of the container's actual size. This is essential
/// for media content, placeholders, and any UI elements that need consistent proportions
/// across different screen sizes and layouts.
///
/// # Features
///
/// - **Mathematical precision**: Uses exact ratio calculations for consistent proportions
/// - **CSS-based solution**: Leverages padding-bottom percentage technique for performance
/// - **Responsive behavior**: Automatically adapts to parent container width
/// - **Absolute positioning**: Child content positioned to fill entire ratio area
/// - **Content flexibility**: Supports any type of child content (images, videos, divs)
/// - **Custom styling**: Container styling through additional CSS classes
/// - **Cross-browser compatible**: Works reliably across all modern browsers
/// - **Performance optimized**: Pure CSS solution with minimal overhead
///
/// # Implementation Details
///
/// The component uses the "padding-bottom percentage" technique where:
/// 1. The outer container has `width: 100%` to fill available space
/// 2. `padding-bottom` is set to `(100 / ratio)%` to create the height
/// 3. Child content uses `position: absolute` with `inset: 0` to fill the area
/// 4. This creates a responsive box that maintains the exact aspect ratio
///
/// The padding-bottom percentage is relative to the element's width, making this
/// technique reliable for creating proportional containers.
///
/// # Accessibility
///
/// - Uses semantic container structure with proper nesting
/// - Preserves accessibility of child content
/// - Maintains proper focus and tab order for interactive content
/// - Does not interfere with screen reader navigation
/// - Child content accessibility depends on the specific content provided
///
/// # Common Use Cases
///
/// - Video players and embedded media (16:9, 4:3)
/// - Image galleries and thumbnails (1:1, 3:2, 4:3)
/// - Card layouts with consistent proportions
/// - Hero sections and banners (21:9, 2:1)
/// - Placeholder content during loading
/// - Responsive iframe containers
///
/// # Parameters
///
/// - `ratio`: Width/height ratio as floating-point number
/// - `class`: Additional CSS classes for container styling
/// - `children`: Content to display within the aspect ratio bounds
#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
    let padding_bottom = (100.0 / props.ratio).to_string();
    let custom_classes = props.class.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "relative w-full {custom_classes}",
            style: "padding-bottom: {padding_bottom}%",
            div { class: "absolute inset-0", {props.children} }
        }
    }
}
