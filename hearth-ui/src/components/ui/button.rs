//! Button component with different variants and sizes
//!
//! The Button component is a fundamental interactive element that triggers actions
//! when clicked. It supports multiple visual variants, sizes, and states including
//! loading and disabled states. The component follows design system principles
//! with consistent styling and accessibility features.
//!
//! # Examples
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     Button {
//!         onclick: move |_| println!("Clicked!"),
//!         "Click me"
//!     }
//! }
//! ```
//!
//! Different variants and sizes:
//! ```rust
//! rsx! {
//!     Button {
//!         variant: ButtonVariant::Primary,
//!         size: ButtonSize::Large,
//!         "Primary Button"
//!     }
//!     Button {
//!         variant: ButtonVariant::Outline,
//!         size: ButtonSize::Small,
//!         "Outline Button"
//!     }
//! }
//! ```
//!
//! Loading and disabled states:
//! ```rust
//! rsx! {
//!     Button {
//!         loading: true,
//!         "Loading..."
//!     }
//!     Button {
//!         disabled: true,
//!         "Disabled"
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Visual style variants for buttons
/// 
/// Each variant represents a different visual treatment and semantic meaning:
/// - `Primary`: Main call-to-action buttons with high visual prominence
/// - `Secondary`: Secondary actions with medium visual prominence  
/// - `Outline`: Subtle actions with border styling
/// - `Ghost`: Minimal styling for low-emphasis actions
/// - `Icon`: Compact icon-only buttons with square dimensions
/// - `Destructive`: Dangerous actions like delete or remove operations
#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    /// Primary action button with high visual prominence
    Primary,
    /// Secondary action button with medium visual prominence
    Secondary,
    /// Subtle button with border styling for low-emphasis actions
    Outline,
    /// Minimal button with no background for the least emphasis
    Ghost,
    /// Compact icon-only button with square dimensions
    Icon,
    /// Button for destructive actions like delete operations
    Destructive,
}

/// Size variants for buttons
///
/// Controls the padding, text size, and overall dimensions of the button:
/// - `Small`: Compact size for tight spaces (px-3 py-1.5, text-sm)
/// - `Medium`: Standard size for most use cases (px-4 py-2, text-base) 
/// - `Large`: Prominent size for important actions (px-6 py-3, text-lg)
#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    /// Small button size for compact layouts
    Small,
    /// Medium button size (default) for standard use
    Medium,
    /// Large button size for prominent actions
    Large,
}

/// Properties for configuring the Button component
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Visual style variant of the button
    /// 
    /// Determines the button's appearance and semantic meaning.
    /// Defaults to `ButtonVariant::Primary`.
    #[props(default = ButtonVariant::Primary)]
    pub variant: ButtonVariant,
    
    /// Size of the button
    /// 
    /// Controls padding, text size, and overall dimensions.
    /// Defaults to `ButtonSize::Medium`.
    #[props(default = ButtonSize::Medium)]
    pub size: ButtonSize,
    
    /// Whether the button is disabled
    /// 
    /// When `true`, the button becomes non-interactive with reduced opacity
    /// and prevents click events. Defaults to `false`.
    #[props(default = false)]
    pub disabled: bool,
    
    /// Whether the button is in loading state
    /// 
    /// When `true`, displays a loading spinner and disables interaction.
    /// Useful for async operations. Defaults to `false`.
    #[props(default = false)]
    pub loading: bool,
    
    /// Click event handler
    /// 
    /// Called when the button is clicked, unless the button is disabled
    /// or in loading state. Optional - buttons without handlers are still
    /// focusable for accessibility.
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    
    /// Additional CSS classes to apply
    /// 
    /// Custom classes are appended to the button's base styling.
    /// Use for one-off customizations or utility classes.
    #[props(default)]
    pub class: Option<String>,
    
    /// Button content (text, icons, or other elements)
    pub children: Element,
}

impl ButtonVariant {
    /// Returns the CSS classes for this button variant
    /// 
    /// Provides the complete styling including background, hover states,
    /// active states, text color, border, and shadow effects.
    pub fn classes(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "bg-primary hover:bg-primary/90 active:bg-primary/80 text-primary-foreground border-transparent shadow-sm hover:shadow active:shadow-none",
            ButtonVariant::Secondary => "bg-secondary hover:bg-secondary/80 active:bg-secondary/70 text-secondary-foreground border-transparent shadow-sm hover:shadow active:shadow-none",
            ButtonVariant::Outline => "bg-transparent hover:bg-accent active:bg-accent/80 text-foreground border-border hover:border-border/80 active:border-border/60",
            ButtonVariant::Ghost => "bg-transparent hover:bg-accent active:bg-accent/80 text-foreground border-transparent",
            ButtonVariant::Icon => "bg-transparent hover:bg-accent active:bg-accent/80 text-foreground border-transparent",
            ButtonVariant::Destructive => "bg-destructive hover:bg-destructive/90 active:bg-destructive/80 text-white border-transparent shadow-sm hover:shadow active:shadow-none",
        }
    }
}

impl ButtonSize {
    /// Returns the CSS classes for this button size
    /// 
    /// Provides padding and text size classes that determine the button's dimensions.
    pub fn classes(&self) -> &'static str {
        match self {
            ButtonSize::Small => "px-3 py-1.5 text-sm",
            ButtonSize::Medium => "px-4 py-2 text-base",
            ButtonSize::Large => "px-6 py-3 text-lg",
        }
    }

    /// Returns the CSS classes for icon button size (square dimensions)
    /// 
    /// Provides equal padding for square icon buttons.
    pub fn icon_classes(&self) -> &'static str {
        match self {
            ButtonSize::Small => "p-1.5 text-sm",
            ButtonSize::Medium => "p-2 text-base", 
            ButtonSize::Large => "p-3 text-lg",
        }
    }
}

/// Interactive button component with variants, sizes, and states
/// 
/// The Button component provides a consistent, accessible way to trigger actions
/// in the user interface. It supports different visual styles through variants,
/// multiple sizes, and special states like loading and disabled.
/// 
/// # Accessibility
/// 
/// - Keyboard navigation support with focus indicators
/// - Screen reader compatible with proper semantic structure
/// - Disabled state prevents interaction and indicates unavailability
/// - Loading state provides visual feedback for async operations
/// 
/// # Styling
/// 
/// The component combines multiple CSS class layers:
/// - Base classes for layout, transitions, and accessibility
/// - Variant classes for visual styling and color schemes  
/// - Size classes for dimensions and typography
/// - Custom classes for additional styling needs
/// 
/// # Parameters
/// 
/// - `variant`: Visual style (Primary, Secondary, Outline, Ghost, Destructive)
/// - `size`: Button dimensions (Small, Medium, Large)
/// - `disabled`: Prevents interaction when true
/// - `loading`: Shows spinner and prevents interaction when true
/// - `onclick`: Optional click event handler
/// - `class`: Additional CSS classes to apply
/// - `children`: Button content (text, icons, etc.)
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let base_classes = "inline-flex items-center justify-center rounded-md border font-medium transition-all duration-150 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none active:scale-95 select-none";

    let variant_classes = props.variant.classes();
    let size_classes = if props.variant == ButtonVariant::Icon {
        props.size.icon_classes()
    } else {
        props.size.classes()
    };
    let custom_classes = props.class.as_deref().unwrap_or("");

    let combined_classes =
        format!("{base_classes} {variant_classes} {size_classes} {custom_classes}");

    rsx! {
        button {
            class: "{combined_classes}",
            disabled: props.disabled || props.loading,
            onclick: move |evt| {
                if !props.disabled && !props.loading {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                }
            },
            if props.loading {
                svg {
                    class: "animate-spin -ml-1 mr-2 h-4 w-4",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    circle {
                        class: "opacity-25",
                        cx: "12",
                        cy: "12",
                        r: "10",
                        stroke: "currentColor",
                        stroke_width: "4",
                    }
                    path {
                        class: "opacity-75",
                        fill: "currentColor",
                        d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
                    }
                }
            }
            {props.children}
        }
    }
}
