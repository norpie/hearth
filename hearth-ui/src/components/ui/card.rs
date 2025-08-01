//! Card components for displaying content in structured containers
//!
//! The card module provides a comprehensive set of components for creating
//! consistent content containers with headers, titles, descriptions, content
//! areas, and footers. Cards support multiple visual variants and sizes for
//! different design contexts and content types.
//!
//! # Features
//!
//! - **Multiple variants**: Default, Outline, Elevated, and Flat visual styles
//! - **Size options**: Small, Medium, and Large padding variants
//! - **Semantic structure**: Header, title, description, content, and footer components
//! - **Consistent styling**: Standardized spacing, typography, and color schemes
//! - **Flexible composition**: Mix and match card components as needed
//! - **Custom styling**: Support for additional CSS classes on all components
//! - **Accessibility**: Proper semantic structure with heading hierarchy
//! - **Responsive design**: Adapts well to different screen sizes
//!
//! # Examples
//!
//! Basic card with title and content:
//! ```rust
//! rsx! {
//!     Card {
//!         CardHeader {
//!             CardTitle { "Welcome" }
//!             CardDescription { "Get started with your new account" }
//!         }
//!         CardContent {
//!             p { "This is the main content area of the card." }
//!         }
//!     }
//! }
//! ```
//!
//! Elevated card with footer actions:
//! ```rust
//! rsx! {
//!     Card {
//!         variant: CardVariant::Elevated,
//!         size: CardSize::Large,
//!         CardHeader {
//!             CardTitle { "Project Settings" }
//!             CardDescription { "Manage your project configuration and preferences" }
//!         }
//!         CardContent {
//!             div { class: "space-y-4",
//!                 // Settings form content
//!             }
//!         }
//!         CardFooter {
//!             class: Some("justify-end space-x-2".to_string()),
//!             Button { variant: ButtonVariant::Outline, "Cancel" }
//!             Button { "Save Changes" }
//!         }
//!     }
//! }
//! ```
//!
//! Simple outline card for lists:
//! ```rust
//! rsx! {
//!     Card {
//!         variant: CardVariant::Outline,
//!         size: CardSize::Small,
//!         CardContent {
//!             div { class: "flex items-center space-x-3",
//!                 img { src: "{user.avatar}", class: "w-10 h-10 rounded-full" }
//!                 div {
//!                     h4 { class: "font-medium", "{user.name}" }
//!                     p { class: "text-sm text-muted-foreground", "{user.email}" }
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Card without header or footer:
//! ```rust
//! rsx! {
//!     Card {
//!         variant: CardVariant::Flat,
//!         CardContent {
//!             div { class: "text-center py-8",
//!                 h3 { class: "text-lg font-semibold mb-2", "No items found" }
//!                 p { class: "text-muted-foreground", "Try adjusting your search criteria" }
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Visual style variants for card containers
///
/// Each variant provides different visual treatment for various design contexts:
/// - `Default`: Standard card with background, border, and subtle shadow
/// - `Outline`: Emphasized border with thicker outline and no shadow
/// - `Elevated`: Enhanced shadow for prominent content and overlays
/// - `Flat`: Minimal styling with secondary background and no border
#[derive(Clone, PartialEq)]
pub enum CardVariant {
    /// Standard card with background, border, and subtle shadow
    Default,
    /// Emphasized border with thicker outline and no shadow
    Outline,
    /// Enhanced shadow for prominent content and overlays
    Elevated,
    /// Minimal styling with secondary background and no border
    Flat,
}

/// Size variants controlling card padding and spacing
///
/// Provides consistent sizing across the design system:
/// - `Small`: Compact padding (1rem) for dense layouts
/// - `Medium`: Standard padding (1.5rem) for most use cases
/// - `Large`: Generous padding (2rem) for prominent content
#[derive(Clone, PartialEq)]
pub enum CardSize {
    /// Small card size for compact layouts (16px padding)
    Small,
    /// Medium card size (default) for standard use (24px padding)
    Medium,
    /// Large card size for prominent content (32px padding)
    Large,
}

/// Properties for configuring the main Card container component
///
/// Provides control over the card's visual appearance, size, and custom styling.
/// The card serves as the root container for all other card-related components.
#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    /// Visual style variant of the card
    ///
    /// Determines the card's appearance including background, borders, and shadows.
    /// Defaults to `CardVariant::Default`.
    #[props(default = CardVariant::Default)]
    pub variant: CardVariant,

    /// Size variant controlling internal padding
    ///
    /// Affects the amount of padding applied to the card container.
    /// Defaults to `CardSize::Medium`.
    #[props(default = CardSize::Medium)]
    pub size: CardSize,

    /// Additional CSS classes to apply to the card
    ///
    /// Custom classes are appended to the card's base styling.
    /// Use for additional spacing, positioning, or styling overrides.
    #[props(default)]
    pub class: Option<String>,

    /// Child components to display within the card
    ///
    /// Typically contains CardHeader, CardContent, and CardFooter components
    /// but can contain any valid content.
    pub children: Element,
}

impl CardVariant {
    /// Returns CSS classes for card styling based on variant
    ///
    /// Maps each variant to appropriate background, border, and shadow classes
    /// for consistent visual treatment across the design system.
    pub fn classes(&self) -> &'static str {
        match self {
            CardVariant::Default => "bg-card border border-border shadow-sm",
            CardVariant::Outline => "bg-card border-2 border-border",
            CardVariant::Elevated => "bg-card border border-border shadow-lg",
            CardVariant::Flat => "bg-secondary border-0",
        }
    }
}

impl CardSize {
    /// Returns CSS classes for card padding based on size variant
    ///
    /// Maps each size to appropriate padding classes for consistent
    /// spacing across different card contexts.
    pub fn classes(&self) -> &'static str {
        match self {
            CardSize::Small => "p-4",
            CardSize::Medium => "p-6",
            CardSize::Large => "p-8",
        }
    }
}

/// Main card container component for structured content display
///
/// The Card component provides the foundational container for displaying content
/// in a visually distinct, bordered area. It supports multiple visual variants
/// and sizes to fit different design contexts and content hierarchies.
///
/// # Features
///
/// - **Visual variants**: Multiple styling options for different design contexts
/// - **Size variants**: Configurable padding for content density
/// - **Rounded corners**: Consistent border radius for modern appearance
/// - **Transition effects**: Smooth color transitions for theme changes
/// - **Flexible content**: Accepts any child components
/// - **Custom styling**: Support for additional CSS classes
///
/// # Implementation Details
///
/// The component combines base classes for common styling (rounded corners,
/// transitions) with variant-specific classes for visual treatment and
/// size-specific classes for padding. Custom classes are appended to
/// allow for additional styling without overriding base functionality.
///
/// # Accessibility
///
/// - Uses semantic div element for general content grouping
/// - Relies on child components for semantic structure
/// - Color contrast meets accessibility guidelines
/// - Focus management handled by interactive child components
///
/// # Parameters
///
/// - `variant`: Visual style (Default, Outline, Elevated, Flat)
/// - `size`: Padding amount (Small, Medium, Large)
/// - `class`: Additional CSS classes for customization
/// - `children`: Content to display within the card
#[component]
pub fn Card(props: CardProps) -> Element {
    let base_classes = "rounded-lg transition-colors";
    let variant_classes = props.variant.classes();
    let size_classes = props.size.classes();
    let custom_classes = props.class.as_deref().unwrap_or("");

    let combined_classes =
        format!("{base_classes} {variant_classes} {size_classes} {custom_classes}");

    rsx! {
        div { class: "{combined_classes}", {props.children} }
    }
}

/// Properties for configuring the CardHeader component
///
/// Provides control over header styling and content within card containers.
#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    /// Additional CSS classes to apply to the header
    ///
    /// Custom classes are appended to the header's base styling.
    /// Use for additional spacing, alignment, or styling customizations.
    #[props(default)]
    pub class: Option<String>,

    /// Header content to display
    ///
    /// Typically contains CardTitle and CardDescription components
    /// but can contain any header-appropriate content.
    pub children: Element,
}

/// Header section component for card titles and descriptions
///
/// The CardHeader component provides a standardized container for card titles,
/// descriptions, and other header content. It applies consistent spacing and
/// layout for the top section of cards.
///
/// # Features
///
/// - **Consistent spacing**: Standardized bottom padding and vertical spacing
/// - **Flexible layout**: Column layout with spacing between header elements
/// - **Typography hierarchy**: Proper structure for titles and descriptions
/// - **Custom styling**: Support for additional CSS classes
///
/// # Implementation Details
///
/// The component uses flexbox column layout with consistent spacing between
/// child elements. Bottom padding provides separation from card content.
///
/// # Accessibility
///
/// - Provides semantic structure for card header content
/// - Maintains proper heading hierarchy when used with CardTitle
/// - Supports screen reader navigation
///
/// # Parameters
///
/// - `class`: Additional CSS classes for customization
/// - `children`: Header content (typically CardTitle and CardDescription)
#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    let base_classes = "flex flex-col space-y-1.5 pb-6";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let combined_classes = format!("{base_classes} {custom_classes}");

    rsx! {
        div { class: "{combined_classes}", {props.children} }
    }
}

/// Properties for configuring the CardTitle component
///
/// Provides control over title styling and content within card headers.
#[derive(Props, Clone, PartialEq)]
pub struct CardTitleProps {
    /// Additional CSS classes to apply to the title
    ///
    /// Custom classes are appended to the title's base styling.
    /// Use for size adjustments, color changes, or alignment modifications.
    #[props(default)]
    pub class: Option<String>,

    /// Title text or content to display
    ///
    /// The main heading content for the card. Should be descriptive
    /// and clearly identify the card's purpose or content.
    pub children: Element,
}

/// Title component for card headers with proper semantic structure
///
/// The CardTitle component provides a standardized heading element for cards
/// with consistent typography, sizing, and color treatment. It uses semantic
/// HTML heading structure for accessibility and SEO benefits.
///
/// # Features
///
/// - **Semantic structure**: Uses h3 element for proper heading hierarchy
/// - **Consistent typography**: Standardized font size, weight, and spacing
/// - **Theme colors**: Uses card-foreground color for proper contrast
/// - **Tight leading**: Optimized line height for heading text
/// - **Custom styling**: Support for additional CSS classes
///
/// # Implementation Details
///
/// The component renders as an h3 element with large text size, semibold weight,
/// and tight tracking for a professional heading appearance. The card-foreground
/// color ensures proper contrast within card containers.
///
/// # Accessibility
///
/// - Uses semantic h3 heading for proper document structure
/// - Maintains heading hierarchy within card components
/// - Provides clear content structure for screen readers
/// - Color contrast meets accessibility guidelines
///
/// # Parameters
///
/// - `class`: Additional CSS classes for styling customization
/// - `children`: Title text content or elements
#[component]
pub fn CardTitle(props: CardTitleProps) -> Element {
    let base_classes = "text-2xl font-semibold leading-none tracking-tight text-card-foreground";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let combined_classes = format!("{base_classes} {custom_classes}");

    rsx! {
        h3 { class: "{combined_classes}", {props.children} }
    }
}

/// Properties for configuring the CardDescription component
///
/// Provides control over description styling and content within card headers.
#[derive(Props, Clone, PartialEq)]
pub struct CardDescriptionProps {
    /// Additional CSS classes to apply to the description
    ///
    /// Custom classes are appended to the description's base styling.
    /// Use for spacing adjustments, color changes, or text formatting.
    #[props(default)]
    pub class: Option<String>,

    /// Description text or content to display
    ///
    /// Supporting text that provides additional context or details
    /// about the card's content or purpose.
    pub children: Element,
}

/// Description component for providing additional context in card headers
///
/// The CardDescription component provides standardized styling for descriptive
/// text that supports the card title. It uses muted colors and smaller text
/// size to create proper visual hierarchy within card headers.
///
/// # Features
///
/// - **Visual hierarchy**: Smaller text size and muted color for supporting text
/// - **Semantic structure**: Uses paragraph element for proper text flow
/// - **Consistent styling**: Standardized typography for descriptions
/// - **Theme integration**: Uses muted-foreground color for reduced emphasis
/// - **Custom styling**: Support for additional CSS classes
///
/// # Implementation Details
///
/// The component renders as a paragraph element with small text size and
/// muted foreground color to provide supporting information without
/// competing with the card title for attention.
///
/// # Accessibility
///
/// - Uses semantic paragraph element for proper text structure
/// - Provides additional context for screen readers
/// - Maintains readable contrast with muted colors
/// - Supports proper text flow and line breaking
///
/// # Parameters
///
/// - `class`: Additional CSS classes for styling customization
/// - `children`: Description text content or elements
#[component]
pub fn CardDescription(props: CardDescriptionProps) -> Element {
    let base_classes = "text-sm text-muted-foreground";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let combined_classes = format!("{base_classes} {custom_classes}");

    rsx! {
        p { class: "{combined_classes}", {props.children} }
    }
}

/// Properties for configuring the CardContent component
///
/// Provides control over content area styling and layout within cards.
#[derive(Props, Clone, PartialEq)]
pub struct CardContentProps {
    /// Additional CSS classes to apply to the content area
    ///
    /// Custom classes are appended to the content's base styling.
    /// Use for spacing, layout adjustments, or content-specific styling.
    #[props(default)]
    pub class: Option<String>,

    /// Main content to display within the card
    ///
    /// The primary content area of the card. Can contain any type
    /// of content including text, forms, lists, or other components.
    pub children: Element,
}

/// Content area component for the main body of cards
///
/// The CardContent component provides the primary content container within
/// cards, positioned between the header and footer. It removes top padding
/// to flow naturally from the header while maintaining other card spacing.
///
/// # Features
///
/// - **Natural flow**: No top padding for seamless header integration
/// - **Flexible content**: Accepts any type of content or components
/// - **Consistent spacing**: Maintains card's internal spacing patterns
/// - **Custom styling**: Support for additional CSS classes
///
/// # Implementation Details
///
/// The component uses a simple div container with top padding removed
/// (pt-0) to ensure proper spacing flow from card headers. Other padding
/// is inherited from the parent card component.
///
/// # Accessibility
///
/// - Uses semantic div for general content grouping
/// - Relies on child components for specific semantic structure
/// - Maintains proper content flow and readability
/// - Supports all standard accessibility features of child content
///
/// # Parameters
///
/// - `class`: Additional CSS classes for styling customization
/// - `children`: Main content elements or components
#[component]
pub fn CardContent(props: CardContentProps) -> Element {
    let base_classes = "pt-0";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let combined_classes = format!("{base_classes} {custom_classes}");

    rsx! {
        div { class: "{combined_classes}", {props.children} }
    }
}

/// Properties for configuring the CardFooter component
///
/// Provides control over footer styling and content within cards.
#[derive(Props, Clone, PartialEq)]
pub struct CardFooterProps {
    /// Additional CSS classes to apply to the footer
    ///
    /// Custom classes are appended to the footer's base styling.
    /// Use for alignment, spacing, or footer-specific styling.
    #[props(default)]
    pub class: Option<String>,

    /// Footer content to display
    ///
    /// Typically contains action buttons, links, or status information.
    /// Content is arranged horizontally with center alignment.
    pub children: Element,
}

/// Footer section component for card actions and additional information
///
/// The CardFooter component provides a standardized container for card actions,
/// buttons, links, or other footer content. It uses horizontal layout with
/// center alignment and consistent top spacing from the card content.
///
/// # Features
///
/// - **Horizontal layout**: Flexbox layout for arranging footer elements
/// - **Center alignment**: Items are vertically centered for consistent appearance
/// - **Consistent spacing**: Standardized top padding for separation from content
/// - **Action-oriented**: Designed for buttons, links, and interactive elements
/// - **Custom styling**: Support for additional CSS classes
///
/// # Implementation Details
///
/// The component uses flexbox with items-center for vertical alignment and
/// consistent top padding to separate footer content from the main card body.
/// Additional alignment and spacing can be controlled through custom classes.
///
/// # Accessibility
///
/// - Uses semantic div for footer content grouping
/// - Supports proper tab order for interactive footer elements
/// - Maintains accessible spacing and visual hierarchy
/// - Provides clear separation between content and actions
///
/// # Parameters
///
/// - `class`: Additional CSS classes (commonly used for justify-* alignment)
/// - `children`: Footer content (buttons, links, status indicators, etc.)
#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    let base_classes = "flex items-center pt-6";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let combined_classes = format!("{base_classes} {custom_classes}");

    rsx! {
        div { class: "{combined_classes}", {props.children} }
    }
}
