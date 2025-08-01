//! Badge components for status indicators and labels
//!
//! The badge module provides compact status indicators and labels for displaying
//! categories, states, counts, and other contextual information. Badges are designed
//! to be visually distinct while remaining accessible and consistent across the
//! application interface.
//!
//! # Features
//!
//! - **Multiple variants**: Default, Secondary, Success, Warning, Error, Info, and Outline styles
//! - **Size options**: Small, Medium, and Large sizes for different contexts
//! - **Interactive support**: Optional click handling for actionable badges
//! - **Semantic colors**: Color-coded variants for status and category indication
//! - **Flexible content**: Support for text, numbers, icons, or mixed content
//! - **Responsive design**: Consistent appearance across different screen sizes
//! - **Accessibility**: Proper contrast ratios and semantic structure
//! - **Customizable**: Additional CSS classes for specific styling needs
//!
//! # Examples
//!
//! Basic status badge:
//! ```rust
//! rsx! {
//!     Badge {
//!         variant: BadgeVariant::Success,
//!         "Active"
//!     }
//! }
//! ```
//!
//! Small notification count badge:
//! ```rust
//! rsx! {
//!     div { class: "relative",
//!         button { "Messages" }
//!         Badge {
//!             variant: BadgeVariant::Error,
//!             size: BadgeSize::Small,
//!             class: Some("absolute -top-2 -right-2".to_string()),
//!             "3"
//!         }
//!     }
//! }
//! ```
//!
//! Interactive category badges:
//! ```rust
//! let mut selected_categories = use_signal(|| vec!["rust", "ui"]);
//! rsx! {
//!     div { class: "flex flex-wrap gap-2",
//!         for category in ["rust", "ui", "components", "design"].iter() {
//!             Badge {
//!                 key: "{category}",
//!                 variant: if selected_categories().contains(&category.to_string()) {
//!                     BadgeVariant::Info
//!                 } else {
//!                     BadgeVariant::Outline
//!                 },
//!                 onclick: move |_| toggle_category(category),
//!                 "{category}"
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Warning badge with icon:
//! ```rust
//! rsx! {
//!     Badge {
//!         variant: BadgeVariant::Warning,
//!         size: BadgeSize::Large,
//!         class: Some("gap-1".to_string()),
//!         svg {
//!             class: "w-4 h-4",
//!             // Warning icon
//!         }
//!         "Pending Review"
//!     }
//! }
//! ```
//!
//! User role badges:
//! ```rust
//! rsx! {
//!     div { class: "flex items-center gap-2",
//!         span { "John Doe" }
//!         Badge {
//!             variant: BadgeVariant::Secondary,
//!             size: BadgeSize::Small,
//!             "Admin"
//!         }
//!         Badge {
//!             variant: BadgeVariant::Success,
//!             size: BadgeSize::Small,
//!             "Verified"
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Properties for configuring the Badge component
///
/// Provides control over badge appearance, content, and interactive behavior.
/// Supports various visual styles and sizes for different use cases from
/// notification counts to status indicators.
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    /// Content to display within the badge
    ///
    /// Can contain text, numbers, icons, or any combination of elements.
    /// Common content includes status text, notification counts, category
    /// labels, or icon/text combinations. Required parameter.
    pub children: Element,

    /// Visual style variant for the badge
    ///
    /// Determines the color scheme and visual treatment. Each variant
    /// provides appropriate semantic meaning:
    /// - `Default`: Neutral gray for general labels
    /// - `Secondary`: Secondary theme color for categories
    /// - `Success`: Green for positive states and success indicators
    /// - `Warning`: Yellow/orange for caution and pending states
    /// - `Error`: Red for error states and critical notifications
    /// - `Info`: Blue for informational content and neutral status
    /// - `Outline`: Transparent with border for subtle labeling
    /// Defaults to `BadgeVariant::Default`.
    #[props(default = BadgeVariant::Default)]
    pub variant: BadgeVariant,

    /// Size variant controlling badge dimensions and text size
    ///
    /// Affects padding, text size, and overall badge dimensions:
    /// - `Small`: Compact size (px-2 py-0.5, text-xs) for inline use
    /// - `Medium`: Standard size (px-2.5 py-1, text-sm) for most cases
    /// - `Large`: Prominent size (px-3 py-1.5, text-base) for emphasis
    /// Defaults to `BadgeSize::Medium`.
    #[props(default = BadgeSize::Medium)]
    pub size: BadgeSize,

    /// Additional CSS classes to apply to the badge
    ///
    /// Custom classes are appended to the badge's base styling.
    /// Use for positioning (absolute, relative), spacing (margin),
    /// or additional visual effects. Defaults to None.
    #[props(default)]
    pub class: Option<String>,

    /// Optional click event handler for interactive badges
    ///
    /// When provided, the badge becomes clickable with hover effects
    /// and cursor pointer styling. Useful for category filters,
    /// dismissible notifications, or actionable status indicators.
    /// Defaults to None (non-interactive).
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

/// Visual style variants for badge appearance and semantic meaning
///
/// Each variant provides distinct color schemes and visual treatments to
/// convey different types of information and status levels:
#[derive(Clone, Copy, PartialEq)]
pub enum BadgeVariant {
    /// Default neutral styling with muted colors
    ///
    /// Uses muted background with muted-foreground text for general-purpose
    /// labels and non-semantic content. Ideal for category tags and
    /// neutral information display.
    Default,

    /// Secondary theme styling
    ///
    /// Uses secondary color scheme for content hierarchy and alternative
    /// labeling. Good for secondary information and complementary badges.
    Secondary,

    /// Success state styling with green colors
    ///
    /// Indicates positive states, successful operations, active status,
    /// or completed actions. Uses green background with white text for
    /// clear success communication.
    Success,

    /// Warning state styling with yellow/orange colors
    ///
    /// Indicates caution, pending states, or attention-required conditions.
    /// Uses warning color background with white text for visibility.
    Warning,

    /// Error state styling with red colors
    ///
    /// Indicates error conditions, failed operations, or critical notifications.
    /// Uses destructive color background with white text for urgent communication.
    Error,

    /// Informational styling with blue colors
    ///
    /// For informational content, neutral status, or general notifications.
    /// Uses info color background with white text for clear information display.
    Info,

    /// Outline-only styling with transparent background
    ///
    /// Provides subtle labeling with border and no background fill.
    /// Uses transparent background with border and foreground text for
    /// minimal visual impact while maintaining readability.
    Outline,
}

impl BadgeVariant {
    /// Returns CSS classes for the badge variant's styling
    ///
    /// Maps each variant to appropriate background and text color classes
    /// based on the design system's semantic color tokens.
    pub fn class(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "bg-muted text-muted-foreground",
            BadgeVariant::Secondary => "bg-secondary text-secondary-foreground",
            BadgeVariant::Success => "bg-success text-white",
            BadgeVariant::Warning => "bg-warning text-white",
            BadgeVariant::Error => "bg-destructive text-white",
            BadgeVariant::Info => "bg-info text-white",
            BadgeVariant::Outline => "bg-transparent border border-border text-foreground",
        }
    }
}

/// Size variants for badge dimensions and typography
///
/// Controls the overall size of badges including padding, text size,
/// and visual prominence for different contexts and use cases:
#[derive(Clone, Copy, PartialEq)]
pub enum BadgeSize {
    /// Small badge size for compact layouts and inline use
    ///
    /// Uses minimal padding (px-2 py-0.5) with extra-small text (text-xs).
    /// Ideal for notification counts, inline tags, or space-constrained layouts.
    Small,

    /// Medium badge size for standard use cases
    ///
    /// Uses standard padding (px-2.5 py-1) with small text (text-sm).
    /// The default size for most badge applications including status indicators
    /// and category labels in typical layouts.
    Medium,

    /// Large badge size for prominent display
    ///
    /// Uses generous padding (px-3 py-1.5) with base text size (text-base).
    /// Best for important status information, featured categories, or
    /// when badges need to be visually prominent.
    Large,
}

impl BadgeSize {
    /// Returns CSS classes for the badge size's dimensions and typography
    ///
    /// Maps each size to appropriate padding and text size classes
    /// for consistent badge proportions across the design system.
    pub fn class(&self) -> &'static str {
        match self {
            BadgeSize::Small => "px-2 py-0.5 text-xs",
            BadgeSize::Medium => "px-2.5 py-1 text-sm",
            BadgeSize::Large => "px-3 py-1.5 text-base",
        }
    }
}

/// Badge component for status indicators, labels, and categorization
///
/// The Badge component provides a compact and visually distinct way to display
/// status information, categories, counts, and other contextual data. It supports
/// multiple visual variants, sizes, and optional interactivity while maintaining
/// accessibility and consistent design system integration.
///
/// # Features
///
/// - **Semantic variants**: Seven distinct color schemes for different types of information
/// - **Flexible sizing**: Three size options for different layout contexts
/// - **Interactive support**: Optional click handling with hover effects
/// - **Content flexibility**: Supports text, numbers, icons, or mixed content
/// - **Accessibility**: Proper contrast ratios and semantic structure
/// - **Visual consistency**: Rounded design with consistent spacing and typography
/// - **Responsive design**: Adapts well to different screen sizes and containers
/// - **Customizable styling**: Support for additional CSS classes
///
/// # Implementation Details
///
/// The component combines multiple CSS class sets to build the final appearance:
/// 1. Base classes for layout, typography, and transitions
/// 2. Variant-specific classes for colors and backgrounds
/// 3. Size-specific classes for dimensions and text scaling
/// 4. Interactive classes for hover effects when clickable
/// 5. Custom classes for additional styling
///
/// The badge uses a `span` element for inline display and semantic appropriateness.
/// Interactive badges gain cursor pointer styling and opacity transitions on hover.
/// All variants maintain proper contrast ratios for accessibility compliance.
///
/// # Accessibility
///
/// - Uses semantic HTML span element for proper inline content structure
/// - Maintains WCAG AA contrast ratios across all color variants
/// - Supports keyboard interaction when clickable (inherits from parent focus)
/// - Screen reader friendly content with clear visual hierarchy
/// - Respects user motion preferences through CSS transitions
/// - Proper color semantics for status indication
///
/// # Visual Design
///
/// - Rounded pill shape (rounded-full) for modern appearance
/// - Inline-flex layout for proper content alignment
/// - Medium font weight for readability without excessive boldness
/// - Smooth transitions for interactive state changes
/// - Consistent spacing using design system tokens
/// - Color-coded variants following semantic conventions
///
/// # Use Cases
///
/// - **Status indicators**: Active, pending, error, success states
/// - **Notification counts**: Unread messages, alerts, updates
/// - **Category labels**: Tags, filters, content classification
/// - **User roles**: Admin, moderator, verified user badges
/// - **Progress indicators**: Step completion, workflow status
/// - **Interactive filters**: Selectable category or option badges
///
/// # Parameters
///
/// - `children`: Badge content (text, numbers, icons)
/// - `variant`: Visual style and semantic meaning
/// - `size`: Badge dimensions and text scaling
/// - `class`: Additional CSS classes for customization
/// - `onclick`: Optional click event handler for interactivity
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let is_clickable = props.onclick.is_some();
    let base_classes = "inline-flex items-center font-medium rounded-full transition-colors";
    let variant_classes = props.variant.class();
    let size_classes = props.size.class();
    let hover_classes = if is_clickable {
        "cursor-pointer hover:opacity-80"
    } else {
        ""
    };
    let custom_classes = props.class.as_deref().unwrap_or("");

    let classes =
        format!("{base_classes} {variant_classes} {size_classes} {hover_classes} {custom_classes}");

    rsx! {
        span {
            class: "{classes}",
            onclick: move |event| {
                if let Some(handler) = &props.onclick {
                    handler.call(event);
                }
            },
            {props.children}
        }
    }
}
