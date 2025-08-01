//! Radio button components for mutually exclusive selections
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     Radio {
//!         name: "demo-group",
//!         value: "option1",
//!         selected: selected_option(),
//!         label: "First Option",
//!         onchange: move |value| selected_option.set(value),
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Size variants controlling radio button dimensions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RadioSize {
    /// Small radio button size for compact layouts
    Small,
    /// Medium radio button size (default) for standard use
    Medium,
    /// Large radio button size for prominent choices
    Large,
}

impl Default for RadioSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Properties for the Radio component
#[derive(Props, Clone, PartialEq)]
pub struct RadioProps {
    /// The value this radio button represents when selected
    pub value: String,
    /// The currently selected value in the radio group
    pub selected: String,
    /// Callback function called when this radio button is selected
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
    /// Whether this radio button is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Size variant of the radio button
    #[props(default = RadioSize::default())]
    pub size: RadioSize,
    /// Additional CSS classes to apply to the radio container
    #[props(default = String::new())]
    pub class: String,
    /// HTML id attribute for accessibility and form association
    #[props(default = String::new())]
    pub id: String,
    /// ARIA label for accessibility when no visible label exists
    #[props(default = String::new())]
    pub aria_label: String,
    /// HTML name attribute for the radio group
    pub name: String,
    /// Label text to display next to the radio button
    #[props(default = String::new())]
    pub label: String,
}

/// Interactive radio button for mutually exclusive selections
#[component]
pub fn Radio(props: RadioProps) -> Element {
    let is_selected = props.selected == props.value;
    let (circle_classes, dot_classes) = get_size_classes(props.size);

    let circle_base_classes = "border-2 rounded-full transition-all duration-200 ease-in-out focus-within:outline-none focus-within:ring-2 focus-within:ring-ring focus-within:ring-offset-2 focus-within:ring-offset-background flex items-center justify-center cursor-pointer";

    let circle_state_classes = if props.disabled {
        "bg-muted border-border cursor-not-allowed opacity-50"
    } else {
        "bg-background border-input hover:border-border active:scale-95"
    };

    let combined_circle_classes = if props.class.is_empty() {
        format!("{circle_base_classes} {circle_classes} {circle_state_classes}")
    } else {
        format!(
            "{} {} {} {}",
            circle_base_classes, circle_classes, circle_state_classes, props.class
        )
    };

    let dot_visibility = if is_selected {
        "opacity-100 scale-100"
    } else {
        "opacity-0 scale-0"
    };

    let dot_color_classes = if is_selected {
        if props.disabled {
            "bg-primary/50" // Disabled selected state
        } else {
            "bg-primary"
        }
    } else {
        "bg-transparent"
    };

    let combined_dot_classes = format!("{dot_classes} {dot_color_classes} rounded-full transition-all duration-200 ease-in-out {dot_visibility}");

    let input_id = if props.id.is_empty() {
        format!("radio_{}_{}", props.name, props.value)
    } else {
        props.id
    };

    rsx! {
        div { class: "flex items-center space-x-2",
            label { class: combined_circle_classes, r#for: input_id.as_str(),
                input {
                    r#type: "radio",
                    id: input_id.as_str(),
                    name: props.name.as_str(),
                    value: props.value.as_str(),
                    checked: is_selected,
                    disabled: props.disabled,
                    class: "sr-only", // Screen reader only
                    "aria-label": if props.aria_label.is_empty() { None } else { Some(props.aria_label.as_str()) },
                    onchange: move |_| {
                        if !props.disabled {
                            if let Some(onchange) = &props.onchange {
                                onchange.call(props.value.clone());
                            }
                        }
                    },
                }
                div { class: combined_dot_classes }
            }
            if !props.label.is_empty() {
                label {
                    class: format!(
                        "text-sm font-medium cursor-pointer {}",
                        if props.disabled {
                            "text-muted-foreground cursor-not-allowed"
                        } else {
                            "text-foreground"
                        },
                    ),
                    r#for: input_id.as_str(),
                    "{props.label}"
                }
            }
        }
    }
}

/// Layout direction options for radio groups
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RadioDirection {
    /// Vertical layout with radio buttons stacked in rows
    Vertical,
    /// Horizontal layout with radio buttons arranged in columns
    Horizontal,
}

/// Properties for the RadioGroup component
#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
    /// Currently selected value in the group
    pub value: String,
    /// Callback function called when the selection changes
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
    /// Whether the entire radio group is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Size variant applied to all radio buttons in the group
    #[props(default = RadioSize::default())]
    pub size: RadioSize,
    /// HTML name attribute shared by all radio buttons in the group
    pub name: String,
    /// Additional CSS classes to apply to the group container
    #[props(default = String::new())]
    pub class: String,

    /// Layout direction for arranging radio buttons
    #[props(default = RadioDirection::default())]
    pub direction: RadioDirection,
    /// The radio button children components
    pub children: Element,
}

impl Default for RadioDirection {
    fn default() -> Self {
        Self::Vertical
    }
}

/// Container component for organizing radio buttons with consistent layout
#[component]
pub fn RadioGroup(props: RadioGroupProps) -> Element {
    let container_classes = match props.direction {
        RadioDirection::Vertical => "flex flex-col space-y-2",
        RadioDirection::Horizontal => "flex flex-row space-x-4",
    };

    let combined_classes = if props.class.is_empty() {
        container_classes.to_string()
    } else {
        format!("{} {}", container_classes, props.class)
    };

    rsx! {
        div { class: combined_classes, role: "radiogroup", {props.children} }
    }
}

/// Returns CSS classes for radio circle and dot based on size variant
/// 
/// Maps each size variant to appropriate width/height classes for both
/// the radio button circle and the selection dot. The dot is sized to
/// be clearly visible within the circle while maintaining good proportions.
/// 
/// # Returns
/// 
/// A tuple of (circle_classes, dot_classes) as static string references.
fn get_size_classes(size: RadioSize) -> (&'static str, &'static str) {
    match size {
        RadioSize::Small => (
            "w-4 h-4",
            "w-2 h-2",
        ),
        RadioSize::Medium => (
            "w-5 h-5",
            "w-2.5 h-2.5",
        ),
        RadioSize::Large => (
            "w-6 h-6",
            "w-3 h-3",
        ),
    }
}
