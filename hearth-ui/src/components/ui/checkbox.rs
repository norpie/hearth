//! Checkbox component with smooth animations and accessibility support
//!
//! The Checkbox component provides an interactive binary choice control with
//! comprehensive styling options, smooth animations, and full accessibility
//! compliance. It supports various sizes, states, and visual feedback for
//! an optimal user experience across different contexts.
//!
//! # Features
//!
//! - **Smooth animations**: CSS transitions for check state and hover effects
//! - **Multiple sizes**: Small, Medium, and Large size variants
//! - **State management**: Checked, unchecked, and disabled states
//! - **Accessibility**: Full ARIA support and keyboard navigation
//! - **Visual feedback**: Hover effects and active states
//! - **Customizable**: Support for custom CSS classes and styling
//! - **Form integration**: Compatible with forms and validation
//!
//! # Examples
//!
//! Basic checkbox with label:
//! ```rust
//! let mut is_checked = use_signal(|| false);
//! rsx! {
//!     div { class: "flex items-center gap-2",
//!         Checkbox {
//!             id: "terms-checkbox",
//!             checked: is_checked(),
//!             onchange: move |checked| is_checked.set(checked),
//!         }
//!         Label {
//!             r#for: "terms-checkbox",
//!             "I agree to the terms and conditions"
//!         }
//!     }
//! }
//! ```
//!
//! Large checkbox with custom styling:
//! ```rust
//! rsx! {
//!     Checkbox {
//!         size: CheckboxSize::Large,
//!         checked: settings.notifications_enabled,
//!         class: "border-blue-500",
//!         aria_label: "Enable notifications",
//!         onchange: move |checked| {
//!             settings.notifications_enabled = checked;
//!         }
//!     }
//! }
//! ```
//!
//! Disabled checkbox for display:
//! ```rust
//! rsx! {
//!     Checkbox {
//!         checked: true,
//!         disabled: true,
//!         aria_label: "Feature enabled (cannot be changed)"
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Size variants controlling checkbox dimensions and icon scaling
///
/// Provides consistent sizing across the design system:
/// - `Small`: Compact checkbox (16x16px) for dense layouts
/// - `Medium`: Standard size (20x20px) for most use cases
/// - `Large`: Prominent checkbox (24x24px) for important choices
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckboxSize {
    /// Small checkbox size for compact layouts (16x16px)
    Small,
    /// Medium checkbox size (default) for standard use (20x20px)
    Medium,
    /// Large checkbox size for prominent choices (24x24px)
    Large,
}

impl Default for CheckboxSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Properties for configuring the Checkbox component
/// 
/// Provides comprehensive control over checkbox appearance, behavior, and accessibility.
/// The component emphasizes proper form integration and semantic HTML structure.
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    /// Whether the checkbox is currently checked
    /// 
    /// Controls the visual state and ARIA checked attribute. For controlled
    /// components, this should be managed by parent state. Defaults to false.
    #[props(default = false)]
    pub checked: bool,

    /// Callback function called when checkbox state changes
    /// 
    /// Receives the new checked state as a boolean parameter. Called when
    /// the user clicks the checkbox or uses keyboard activation. Optional.
    #[props(default)]
    pub onchange: Option<EventHandler<bool>>,

    /// Whether the checkbox is disabled
    /// 
    /// When true, prevents user interaction and applies disabled styling.
    /// Disabled checkboxes are not included in form submissions and cannot
    /// be focused or activated. Defaults to false.
    #[props(default = false)]
    pub disabled: bool,

    /// Size variant of the checkbox
    /// 
    /// Controls the dimensions of both the checkbox box and the checkmark icon.
    /// Larger sizes provide better touch targets for mobile interfaces.
    /// Defaults to `CheckboxSize::Medium`.
    #[props(default = CheckboxSize::default())]
    pub size: CheckboxSize,

    /// Additional CSS classes to apply
    /// 
    /// Custom classes are appended to the checkbox's base styling.
    /// Use for one-off customizations or utility classes.
    #[props(default = String::new())]
    pub class: String,

    /// HTML id attribute for accessibility and form association
    /// 
    /// Used to associate labels with the checkbox and for programmatic access.
    /// Should be unique within the document. Essential for proper form labeling.
    /// Defaults to empty string (no id attribute).
    #[props(default = String::new())]
    pub id: String,

    /// ARIA label for accessibility when no visible label exists
    /// 
    /// Provides an accessible name for screen readers when the checkbox
    /// doesn't have an associated visible label. Use when the checkbox's
    /// purpose isn't clear from context. Defaults to empty string.
    #[props(default = String::new())]
    pub aria_label: String,

    /// Whether the checkbox is required for form submission
    /// 
    /// When true, adds ARIA required attribute for accessibility and form validation.
    /// Should be paired with appropriate validation logic. Defaults to false.
    #[props(default = false)]
    pub required: bool,
}

/// Interactive checkbox control with animations and full accessibility support
/// 
/// The Checkbox component provides a visually appealing and fully accessible
/// binary choice control. It features smooth CSS animations, comprehensive
/// keyboard support, and proper ARIA attributes for screen reader compatibility.
/// 
/// # Features
/// 
/// - **Smooth animations**: CSS transitions for state changes and interactions
/// - **Size variants**: Multiple size options for different design contexts
/// - **State management**: Proper handling of checked, unchecked, and disabled states
/// - **Accessibility**: Full ARIA support with role, checked, and required attributes
/// - **Keyboard navigation**: Space bar activation and tab navigation support
/// - **Visual feedback**: Hover effects, active states, and focus indicators
/// - **Form integration**: Compatible with HTML forms and validation systems
/// - **Custom styling**: Support for additional CSS classes and themes
/// 
/// # Implementation Details
/// 
/// The component renders as a semantic button with ARIA role="checkbox" for
/// proper accessibility. The checkmark icon uses SVG with smooth scale and
/// opacity transitions. CSS classes are dynamically generated based on the
/// current state (checked/unchecked, enabled/disabled) and size variant.
/// 
/// State changes are handled through the optional onchange callback, following
/// controlled component patterns where the parent manages the checked state.
/// 
/// # Accessibility
/// 
/// - Uses semantic button element with checkbox ARIA role
/// - Proper ARIA attributes: checked, required, label
/// - Keyboard activation with Space bar
/// - Focus indicators with ring styling
/// - Screen reader compatible state announcements
/// - Support for form labels via id attribute
/// 
/// # Styling
/// 
/// The component combines multiple CSS class layers:
/// - Base classes for layout, transitions, and focus indicators
/// - Size-specific classes for dimensions and icon scaling
/// - State-specific classes for visual feedback and interactions
/// - Custom classes for additional styling needs
/// 
/// # Parameters
/// 
/// - `checked`: Current checked state (controlled component)
/// - `onchange`: State change callback function
/// - `disabled`: Prevents interaction when true
/// - `size`: Checkbox dimensions (Small, Medium, Large)
/// - `class`: Additional CSS classes
/// - `id`: HTML id for form association and accessibility
/// - `aria_label`: Accessible name when no visible label exists
/// - `required`: Form validation requirement indicator
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let (box_classes, icon_classes) = get_size_classes(props.size);

    let box_base_classes = "border-2 rounded transition-all duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background flex items-center justify-center";

    let box_state_classes = if props.checked {
        if props.disabled {
            "bg-primary/30 border-primary/30 cursor-not-allowed opacity-50"
        } else {
            "bg-primary border-primary cursor-pointer hover:bg-primary/90 active:scale-95"
        }
    } else if props.disabled {
        "bg-muted border-border cursor-not-allowed opacity-50"
    } else {
        "bg-background border-input cursor-pointer hover:border-border active:scale-95"
    };

    let combined_box_classes = if props.class.is_empty() {
        format!("{box_base_classes} {box_classes} {box_state_classes}")
    } else {
        format!(
            "{} {} {} {}",
            box_base_classes, box_classes, box_state_classes, props.class
        )
    };

    let icon_visibility = if props.checked {
        "opacity-100 scale-100"
    } else {
        "opacity-0 scale-75"
    };

    let combined_icon_classes = format!("{icon_classes} text-primary-foreground transition-all duration-200 ease-in-out {icon_visibility}");

    rsx! {
        button {
            r#type: "button",
            class: combined_box_classes,
            id: if props.id.is_empty() { None } else { Some(props.id.as_str()) },
            "aria-label": if props.aria_label.is_empty() { None } else { Some(props.aria_label.as_str()) },
            "aria-checked": props.checked.to_string(),
            role: "checkbox",
            disabled: props.disabled,
            "aria-required": if props.required { Some("true") } else { None },
            onclick: move |_| {
                if !props.disabled {
                    if let Some(onchange) = &props.onchange {
                        onchange.call(!props.checked);
                    }
                }
            },
            svg {
                class: combined_icon_classes,
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 20 20",
                fill: "currentColor",
                path {
                    fill_rule: "evenodd",
                    d: "M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z",
                    clip_rule: "evenodd",
                }
            }
        }
    }
}

/// Returns CSS classes for checkbox box and icon based on size variant
/// 
/// Maps each size variant to appropriate width/height classes for both
/// the checkbox container and the checkmark icon. The icon is always
/// slightly smaller than the container for proper visual proportions.
/// 
/// # Returns
/// 
/// A tuple of (box_classes, icon_classes) as static string references.
fn get_size_classes(size: CheckboxSize) -> (&'static str, &'static str) {
    match size {
        CheckboxSize::Small => (
            "w-4 h-4",
            "w-3 h-3",
        ),
        CheckboxSize::Medium => (
            "w-5 h-5",
            "w-4 h-4",
        ),
        CheckboxSize::Large => (
            "w-6 h-6",
            "w-5 h-5",
        ),
    }
}
