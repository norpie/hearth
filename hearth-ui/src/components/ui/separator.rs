//! Separator components for visual content division and spacing
//!
//! The separator module provides components for creating visual divisions between
//! content sections. It supports both horizontal and vertical orientations with
//! multiple visual styles and accessibility features for proper content organization.
//!
//! # Features
//!
//! - **Multiple orientations**: Horizontal and vertical separator layouts
//! - **Visual variants**: Default, Subtle, Bold, Dashed, and Dotted styles
//! - **Size options**: Small, Medium, and Large thickness variants
//! - **Accessibility**: Proper ARIA roles and orientation attributes
//! - **Decorative mode**: Option to mark separators as purely decorative
//! - **Flexible styling**: Custom CSS class support for additional styling
//! - **Responsive design**: Adapts to container dimensions
//! - **Theme integration**: Uses design system colors and spacing
//!
//! # Examples
//!
//! Basic horizontal separator:
//! ```rust
//! rsx! {
//!     div { class: "space-y-4",
//!         p { "First section content" }
//!         Separator {}
//!         p { "Second section content" }
//!     }
//! }
//! ```
//!
//! Vertical separator in a flex layout:
//! ```rust
//! rsx! {
//!     div { class: "flex items-center space-x-4 h-20",
//!         div { "Left content" }
//!         Separator {
//!             orientation: SeparatorOrientation::Vertical,
//!             size: SeparatorSize::Large
//!         }
//!         div { "Right content" }
//!     }
//! }
//! ```
//!
//! Dashed separator with semantic meaning:
//! ```rust
//! rsx! {
//!     Separator {
//!         variant: SeparatorVariant::Dashed,
//!         decorative: false,
//!         role: "separator".to_string(),
//!         class: "my-6".to_string()
//!     }
//! }
//! ```
//!
//! Subtle separator for grouped content:
//! ```rust
//! rsx! {
//!     div { class: "space-y-2",
//!         h3 { "Settings Group" }
//!         Separator {
//!             variant: SeparatorVariant::Subtle,
//!             size: SeparatorSize::Small
//!         }
//!         // Settings items
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Orientation options controlling separator direction and layout
///
/// Determines how the separator is displayed and which dimension it extends across:
/// - `Horizontal`: Full width separator for dividing vertical content
/// - `Vertical`: Full height separator for dividing horizontal content
#[derive(Clone, PartialEq, Debug)]
pub enum SeparatorOrientation {
    /// Horizontal separator spanning full width for vertical content division
    Horizontal,
    /// Vertical separator spanning full height for horizontal content division
    Vertical,
}

/// Size variants controlling separator thickness
///
/// Provides different visual weights for separators based on content hierarchy:
/// - `Small`: Thin separator (1px) for subtle divisions
/// - `Medium`: Standard separator (2px) for most use cases
/// - `Large`: Thick separator (4px) for prominent divisions
#[derive(Clone, PartialEq, Debug)]
pub enum SeparatorSize {
    /// Small separator thickness (1px) for subtle content divisions
    Small,
    /// Medium separator thickness (2px) for standard use
    Medium,
    /// Large separator thickness (4px) for prominent divisions
    Large,
}

/// Visual style variants for separator appearance
///
/// Each variant provides different visual treatment for various design contexts:
/// - `Default`: Standard solid separator with muted background
/// - `Subtle`: Lighter separator for minimal visual impact
/// - `Bold`: Emphasized separator for strong content divisions
/// - `Dashed`: Dashed border style for informal or temporary divisions
/// - `Dotted`: Dotted border style for soft or decorative divisions
#[derive(Clone, PartialEq, Debug)]
pub enum SeparatorVariant {
    /// Standard solid separator with muted background
    Default,
    /// Lighter separator for minimal visual impact
    Subtle,
    /// Emphasized separator for strong content divisions
    Bold,
    /// Dashed border style for informal or temporary divisions
    Dashed,
    /// Dotted border style for soft or decorative divisions
    Dotted,
}

impl SeparatorOrientation {
    /// Returns base CSS classes for separator orientation
    ///
    /// Maps orientation to appropriate width and height classes for proper
    /// spanning in the intended direction.
    pub fn classes(&self) -> &'static str {
        match self {
            SeparatorOrientation::Horizontal => "w-full h-px",
            SeparatorOrientation::Vertical => "h-full w-px",
        }
    }
}

impl SeparatorSize {
    /// Returns CSS classes for separator thickness based on size and orientation
    ///
    /// Maps size variants to appropriate thickness classes for the given orientation.
    /// Horizontal separators use height classes, vertical separators use width classes.
    ///
    /// # Parameters
    ///
    /// - `orientation`: The separator orientation to determine which dimension to size
    pub fn classes(&self, orientation: &SeparatorOrientation) -> &'static str {
        match (self, orientation) {
            (SeparatorSize::Small, SeparatorOrientation::Horizontal) => "h-px",
            (SeparatorSize::Medium, SeparatorOrientation::Horizontal) => "h-0.5",
            (SeparatorSize::Large, SeparatorOrientation::Horizontal) => "h-1",
            (SeparatorSize::Small, SeparatorOrientation::Vertical) => "w-px",
            (SeparatorSize::Medium, SeparatorOrientation::Vertical) => "w-0.5",
            (SeparatorSize::Large, SeparatorOrientation::Vertical) => "w-1",
        }
    }
}

impl SeparatorVariant {
    /// Returns CSS classes for separator visual styling based on variant
    ///
    /// Maps each variant to appropriate background and border classes for
    /// different visual treatments. Dashed and dotted variants use borders
    /// instead of background colors for their patterns.
    pub fn classes(&self) -> &'static str {
        match self {
            SeparatorVariant::Default => "bg-muted",
            SeparatorVariant::Subtle => "bg-muted",
            SeparatorVariant::Bold => "bg-muted",
            SeparatorVariant::Dashed => {
                "border-0 border-t border-dashed border-border bg-transparent"
            }
            SeparatorVariant::Dotted => {
                "border-0 border-t border-dotted border-border bg-transparent"
            }
        }
    }
}

impl Default for SeparatorOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

impl Default for SeparatorSize {
    fn default() -> Self {
        Self::Medium
    }
}

impl Default for SeparatorVariant {
    fn default() -> Self {
        Self::Default
    }
}

/// Properties for configuring the Separator component
///
/// Provides comprehensive control over separator appearance, behavior, and accessibility.
/// The component emphasizes proper semantic structure and visual hierarchy for content
/// organization while maintaining accessibility compliance.
#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    /// Orientation determining separator direction and layout
    ///
    /// Controls whether the separator spans horizontally or vertically.
    /// Affects both visual appearance and accessibility attributes.
    /// Defaults to `SeparatorOrientation::Horizontal`.
    #[props(default)]
    pub orientation: SeparatorOrientation,

    /// Size variant controlling separator thickness
    ///
    /// Determines the visual weight of the separator for content hierarchy.
    /// Defaults to `SeparatorSize::Medium`.
    #[props(default)]
    pub size: SeparatorSize,

    /// Visual style variant of the separator
    ///
    /// Controls the visual treatment including color, pattern, and emphasis.
    /// Defaults to `SeparatorVariant::Default`.
    #[props(default)]
    pub variant: SeparatorVariant,

    /// Additional CSS classes to apply to the separator
    ///
    /// Custom classes are appended to the separator's base styling.
    /// Use for spacing, positioning, or additional visual customizations.
    #[props(default = String::new())]
    pub class: String,

    /// ARIA role attribute for accessibility
    ///
    /// Defines the semantic role of the separator for assistive technologies.
    /// Use "separator" for meaningful divisions or "none" for decorative separators.
    /// Defaults to "separator".
    #[props(default = "separator".to_string())]
    pub role: String,

    /// ARIA orientation attribute override
    ///
    /// Optional override for the orientation communicated to assistive technologies.
    /// When None, the orientation is automatically derived from the orientation prop.
    /// Defaults to None (auto-derived).
    #[props(default)]
    pub aria_orientation: Option<String>,

    /// Whether the separator is purely decorative
    ///
    /// When true, the separator is marked as decorative (role="none") and
    /// excludes ARIA attributes for assistive technologies. When false,
    /// proper semantic roles and attributes are applied. Defaults to true.
    #[props(default = true)]
    pub decorative: bool,
}

/// Visual separator component for content division and spacing
///
/// The Separator component provides a flexible visual divider for organizing content
/// into logical sections. It supports both horizontal and vertical orientations with
/// multiple visual styles and proper accessibility features for semantic content
/// organization.
///
/// # Features
///
/// - **Flexible orientation**: Horizontal and vertical layout support
/// - **Visual variety**: Multiple styles from subtle to bold, including patterns
/// - **Size options**: Different thickness options for visual hierarchy
/// - **Accessibility**: Proper ARIA roles and orientation attributes
/// - **Decorative mode**: Option to exclude semantic meaning for pure decoration
/// - **Theme integration**: Uses design system colors and spacing tokens
/// - **Responsive design**: Adapts to container dimensions automatically
/// - **Custom styling**: Support for additional CSS classes
///
/// # Implementation Details
///
/// The component dynamically generates CSS classes based on orientation, size, and
/// variant properties. Special handling for dashed and dotted variants uses border
/// properties instead of background colors to achieve the desired patterns.
///
/// ARIA attributes are conditionally applied based on the decorative flag to ensure
/// proper accessibility without cluttering the accessibility tree with decorative elements.
///
/// # Accessibility
///
/// - Uses appropriate ARIA roles (separator or none for decorative)
/// - Provides orientation information for assistive technologies
/// - Supports both semantic and decorative use cases
/// - Maintains proper content structure and navigation
/// - Follows WAI-ARIA separator guidelines
///
/// # Parameters
///
/// - `orientation`: Direction and layout (Horizontal/Vertical)
/// - `size`: Thickness variant (Small, Medium, Large)
/// - `variant`: Visual style (Default, Subtle, Bold, Dashed, Dotted)
/// - `class`: Additional CSS classes for customization
/// - `role`: ARIA role for accessibility
/// - `aria_orientation`: Optional orientation override
/// - `decorative`: Whether separator has semantic meaning
#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let orientation_classes = props.orientation.classes();
    let size_classes = props.size.classes(&props.orientation);
    let variant_classes = props.variant.classes();

    // For dashed and dotted variants, we need different base classes
    let base_classes = match props.variant {
        SeparatorVariant::Dashed | SeparatorVariant::Dotted => match props.orientation {
            SeparatorOrientation::Horizontal => "w-full",
            SeparatorOrientation::Vertical => "h-full border-l border-t-0",
        },
        _ => orientation_classes,
    };

    let combined_classes = if props.class.is_empty() {
        format!("{base_classes} {size_classes} {variant_classes}")
    } else {
        format!(
            "{} {} {} {}",
            base_classes, size_classes, variant_classes, props.class
        )
    };

    // Determine ARIA orientation
    let aria_orientation = props
        .aria_orientation
        .unwrap_or_else(|| match props.orientation {
            SeparatorOrientation::Horizontal => "horizontal".to_string(),
            SeparatorOrientation::Vertical => "vertical".to_string(),
        });

    rsx! {
        div {
            class: combined_classes,
            role: if props.decorative { "none" } else { props.role.as_str() },
            "aria-orientation": if !props.decorative { Some(aria_orientation.as_str()) } else { None },
            "data-orientation": match props.orientation {
                SeparatorOrientation::Horizontal => "horizontal",
                SeparatorOrientation::Vertical => "vertical",
            },
        }
    }
}
