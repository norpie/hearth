//! Skeleton loading components for content placeholders
//!
//! The skeleton module provides animated placeholder components that display
//! while content is loading. It includes text, heading, and avatar variants
//! with pulse animations to indicate loading states.
//!
//! # Examples
//!
//! Basic skeleton usage:
//! ```rust
//! rsx! {
//!     div { class: "space-y-3",
//!         Skeleton { variant: SkeletonVariant::H2 }
//!         Skeleton { variant: SkeletonVariant::Text }
//!         Skeleton { variant: SkeletonVariant::Avatar }
//!     }
//! }
//! ```
//!
//! Profile card skeleton with avatar:
//! ```rust
//! rsx! {
//!     div { class: "flex items-center space-x-4 p-4",
//!         Skeleton {
//!             variant: SkeletonVariant::Avatar,
//!             size: Some("w-16 h-16".to_string()),
//!         }
//!         div { class: "space-y-2 flex-1",
//!             Skeleton { variant: SkeletonVariant::H4 }
//!             Skeleton { variant: SkeletonVariant::Text, width: Some("w-3/4".to_string()) }
//!             Skeleton { variant: SkeletonVariant::Text, width: Some("w-1/2".to_string()) }
//!         }
//!     }
//! }
//! ```
//!
//! Article skeleton with structured content:
//! ```rust
//! rsx! {
//!     article { class: "max-w-2xl mx-auto space-y-6 p-6",
//!         Skeleton { variant: SkeletonVariant::H1 }
//!         div { class: "flex items-center space-x-3 mb-4",
//!             Skeleton {
//!                 variant: SkeletonVariant::Avatar,
//!                 size: Some("w-8 h-8".to_string()),
//!             }
//!             Skeleton { variant: SkeletonVariant::Text, width: Some("w-32".to_string()) }
//!         }
//!         for _ in 0..3 {
//!             div { class: "space-y-2",
//!                 Skeleton { variant: SkeletonVariant::Text }
//!                 Skeleton { variant: SkeletonVariant::Text }
//!                 Skeleton { variant: SkeletonVariant::Text, width: Some("w-2/3".to_string()) }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Custom skeleton shapes for specific layouts:
//! ```rust
//! rsx! {
//!     div { class: "grid grid-cols-3 gap-4",
//!         for _ in 0..6 {
//!             div { class: "space-y-3",
//!                 Skeleton {
//!                     variant: SkeletonVariant::Custom,
//!                     width: Some("w-full".to_string()),
//!                     height: Some("h-32".to_string()),
//!                     class: Some("rounded-xl".to_string()),
//!                 }
//!                 Skeleton { variant: SkeletonVariant::H5 }
//!                 Skeleton { variant: SkeletonVariant::Text, width: Some("w-5/6".to_string()) }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Data table skeleton with rows and columns:
//! ```rust
//! rsx! {
//!     div { class: "space-y-3",
//!         // Header skeleton
//!         div { class: "flex space-x-4",
//!             Skeleton { variant: SkeletonVariant::Text, width: Some("w-32".to_string()) }
//!             Skeleton { variant: SkeletonVariant::Text, width: Some("w-24".to_string()) }
//!             Skeleton { variant: SkeletonVariant::Text, width: Some("w-20".to_string()) }
//!             Skeleton { variant: SkeletonVariant::Text, width: Some("w-16".to_string()) }
//!         }
//!         // Row skeletons
//!         for _ in 0..5 {
//!             div { class: "flex space-x-4",
//!                 Skeleton { variant: SkeletonVariant::Text, width: Some("w-32".to_string()) }
//!                 Skeleton { variant: SkeletonVariant::Text, width: Some("w-24".to_string()) }
//!                 Skeleton { variant: SkeletonVariant::Text, width: Some("w-20".to_string()) }
//!                 Skeleton { variant: SkeletonVariant::Text, width: Some("w-16".to_string()) }
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Skeleton shape variants for different content types
///
/// Each variant provides pre-configured dimensions and styling to match
/// common content patterns and typography hierarchies:
#[derive(Clone, PartialEq)]
pub enum SkeletonVariant {
    /// Standard text line skeleton
    ///
    /// Default height (h-4) with 3/4 width for typical paragraph text.
    /// Most commonly used for body content placeholders.
    Text,

    /// Large heading skeleton (h1 equivalent)
    ///
    /// Tall height (h-10) with full width for main page titles.
    H1,

    /// Sub-heading skeleton (h2 equivalent)
    ///
    /// Height (h-8) with 5/6 width for section headers.
    H2,

    /// Section heading skeleton (h3 equivalent)
    ///
    /// Height (h-7) with 4/5 width for subsection titles.
    H3,

    /// Subsection heading skeleton (h4 equivalent)
    ///
    /// Height (h-6) with 3/4 width for card titles or smaller headings.
    H4,

    /// Minor heading skeleton (h5 equivalent)
    ///
    /// Height (h-5) with 2/3 width for item titles or labels.
    H5,

    /// Small heading skeleton (h6 equivalent)
    ///
    /// Height (h-4) with 1/2 width for captions or small labels.
    H6,

    /// Circular avatar skeleton
    ///
    /// Circular shape with customizable size for profile pictures.
    /// Uses rounded-full styling with configurable dimensions.
    Avatar,

    /// Custom skeleton with user-defined dimensions
    ///
    /// Allows complete control over width, height, and styling.
    /// Use for unique shapes or specific layout requirements.
    Custom,
}

impl Default for SkeletonVariant {
    fn default() -> Self {
        Self::Text
    }
}

/// Properties for configuring the Skeleton component
///
/// Provides control over skeleton appearance, dimensions, and shape variants
/// for creating appropriate placeholder content during loading states.
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonProps {
    /// Shape and size variant for the skeleton
    ///
    /// Determines the pre-configured dimensions and styling:
    /// - Text variants (Text, H1-H6) use appropriate heights and widths
    /// - Avatar variant creates circular shapes with configurable size
    /// - Custom variant allows full control over dimensions
    /// Defaults to `SkeletonVariant::Text`.
    #[props(default)]
    pub variant: SkeletonVariant,

    /// Additional CSS classes to apply to the skeleton
    ///
    /// Custom classes for styling, margins, or layout adjustments.
    /// Applied in addition to variant-specific classes.
    /// Defaults to None.
    #[props(default)]
    pub class: Option<String>,

    /// Custom width override using CSS classes
    ///
    /// Overrides the default width from the variant. Use Tailwind
    /// width classes like "w-1/2", "w-32", "w-full", etc.
    /// Only applied when variant is Custom or as an override.
    /// Defaults to None (use variant default).
    #[props(default)]
    pub width: Option<String>,

    /// Custom height override using CSS classes
    ///
    /// Overrides the default height from the variant. Use Tailwind
    /// height classes like "h-4", "h-12", "h-32", etc.
    /// Only applied when variant is Custom or as an override.
    /// Defaults to None (use variant default).
    #[props(default)]
    pub height: Option<String>,

    /// Size specification for avatar variant
    ///
    /// Defines both width and height for circular avatar skeletons.
    /// Use Tailwind size classes like "w-12 h-12", "w-16 h-16", etc.
    /// Only used when variant is Avatar. Defaults to "w-12 h-12".
    #[props(default)]
    pub size: Option<String>,
}

/// Skeleton loading component for content placeholders
///
/// The Skeleton component provides animated placeholder elements that display
/// while content is loading. It helps create smooth loading experiences by
/// showing the approximate shape and structure of content before it becomes
/// available, reducing perceived loading times and preventing layout shifts.
///
/// # Features
///
/// - **Typography variants**: Pre-configured shapes matching text and heading sizes
/// - **Avatar support**: Circular skeletons for profile pictures and user avatars
/// - **Custom shapes**: Flexible dimensions for unique layout requirements
/// - **Pulse animation**: Smooth CSS animations indicating active loading state
/// - **Responsive design**: Adapts to container constraints and screen sizes
/// - **Consistent theming**: Uses design system colors and border radius
/// - **Performance optimized**: Lightweight CSS animations with minimal overhead
///
/// # Implementation Details
///
/// The component uses CSS pulse animations with muted background colors to
/// create a loading effect. Different variants provide pre-configured dimensions
/// that match common content patterns:
///
/// - Text variants use heights and widths appropriate for typography
/// - Avatar variant uses circular shapes with customizable sizing
/// - Custom variant allows complete dimensional control
///
/// The pulse animation is implemented using Tailwind's `animate-pulse` class
/// for consistent performance across different browsers and devices.
///
/// # Accessibility
///
/// - Visual loading indicator that doesn't interfere with screen readers
/// - Maintains proper layout structure during content loading
/// - Uses semantic color patterns from the design system
/// - Non-interactive elements that don't capture focus
/// - Consistent with loading state expectations
///
/// # Performance Considerations
///
/// - Lightweight CSS animations using GPU acceleration
/// - Minimal DOM structure with efficient class combinations
/// - Responsive sizing that adapts without JavaScript
/// - Optimized for rendering performance during loading states
///
/// # Common Use Cases
///
/// - **Article loading**: Headers, paragraphs, and content structure
/// - **Profile cards**: Avatar, name, and description placeholders
/// - **Data tables**: Row and column structure during data fetching
/// - **Image galleries**: Custom shapes matching image aspect ratios
/// - **Navigation**: Menu items and link placeholders
/// - **Forms**: Field labels and input placeholders
///
/// # Parameters
///
/// - `variant`: Shape and size preset (Text, H1-H6, Avatar, Custom)
/// - `class`: Additional CSS classes for styling customization
/// - `width`: Custom width override for flexible sizing
/// - `height`: Custom height override for specific dimensions
/// - `size`: Avatar-specific size specification (width and height)
#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    let base_classes = "animate-pulse bg-muted rounded-lg";

    let variant_classes = match props.variant {
        SkeletonVariant::Text => "h-4 w-3/4",
        SkeletonVariant::H1 => "h-10 w-full",
        SkeletonVariant::H2 => "h-8 w-5/6",
        SkeletonVariant::H3 => "h-7 w-4/5",
        SkeletonVariant::H4 => "h-6 w-3/4",
        SkeletonVariant::H5 => "h-5 w-2/3",
        SkeletonVariant::H6 => "h-4 w-1/2",
        SkeletonVariant::Avatar => {
            let size = props.size.as_deref().unwrap_or("w-12 h-12");
            return rsx! {
                div { class: "{base_classes} {size} rounded-full {props.class.as_deref().unwrap_or(\"\")}" }
            };
        }
        SkeletonVariant::Custom => "",
    };

    let width = props.width.as_deref().unwrap_or("");
    let height = props.height.as_deref().unwrap_or("");
    let custom_class = props.class.as_deref().unwrap_or("");

    let final_classes = if props.variant == SkeletonVariant::Custom {
        format!("{base_classes} {width} {height} {custom_class}")
    } else {
        format!("{base_classes} {variant_classes} {custom_class}")
    };

    rsx! {
        div { class: "{final_classes}" }
    }
}
