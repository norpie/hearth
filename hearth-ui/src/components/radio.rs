//! Radio button component with group management

use dioxus::prelude::*;

/// Size variants for the Radio component
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RadioSize {
    Small,
    Medium,
    Large,
}

impl Default for RadioSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Props for individual Radio buttons
#[derive(Props, Clone, PartialEq)]
pub struct RadioProps {
    /// The value this radio button represents
    pub value: String,
    
    /// The currently selected value in the group
    pub selected: String,
    
    /// Callback when radio button is selected
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
    
    /// Whether the radio button is disabled
    #[props(default = false)]
    pub disabled: bool,
    
    /// Size variant of the radio button
    #[props(default = RadioSize::default())]
    pub size: RadioSize,
    
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
    
    /// ID for accessibility
    #[props(default = String::new())]
    pub id: String,
    
    /// ARIA label for accessibility
    #[props(default = String::new())]
    pub aria_label: String,
    
    /// Name attribute for the radio group
    pub name: String,
    
    /// Label text to display next to the radio button
    #[props(default = String::new())]
    pub label: String,
}

/// Individual radio button component
#[component]
pub fn Radio(props: RadioProps) -> Element {
    let is_selected = props.selected == props.value;
    let (circle_classes, dot_classes) = get_size_classes(props.size);
    
    let circle_base_classes = "border-2 rounded-full transition-all duration-200 ease-in-out focus-within:outline-none focus-within:ring-2 focus-within:ring-blue-500 focus-within:ring-offset-2 dark:focus-within:ring-offset-gray-900 flex items-center justify-center cursor-pointer";
    
    let circle_state_classes = if props.disabled {
        "bg-gray-100 dark:bg-gray-800 border-gray-200 dark:border-gray-700 cursor-not-allowed opacity-50"
    } else {
        "bg-white dark:bg-gray-900 border-gray-300 dark:border-gray-600 hover:border-gray-400 dark:hover:border-gray-500 active:scale-95"
    };
    
    let combined_circle_classes = if props.class.is_empty() {
        format!("{} {} {}", circle_base_classes, circle_classes, circle_state_classes)
    } else {
        format!("{} {} {} {}", circle_base_classes, circle_classes, circle_state_classes, props.class)
    };
    
    let dot_visibility = if is_selected {
        "opacity-100 scale-100"
    } else {
        "opacity-0 scale-0"
    };
    
    let dot_color_classes = if is_selected {
        if props.disabled {
            "bg-blue-500 dark:bg-blue-500"  // Disabled selected state - visible blue
        } else {
            "bg-blue-600 dark:bg-blue-500"
        }
    } else {
        "bg-transparent"
    };
    
    let combined_dot_classes = format!("{} {} rounded-full transition-all duration-200 ease-in-out {}", dot_classes, dot_color_classes, dot_visibility);

    let input_id = if props.id.is_empty() {
        format!("radio_{}_{}", props.name, props.value)
    } else {
        props.id
    };

    rsx! {
        div {
            class: "flex items-center space-x-2",
            label {
                class: combined_circle_classes,
                r#for: input_id.as_str(),
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
                div {
                    class: combined_dot_classes,
                }
            }
            if !props.label.is_empty() {
                label {
                    class: format!(
                        "text-sm font-medium cursor-pointer {}",
                        if props.disabled {
                            "text-gray-400 dark:text-gray-600 cursor-not-allowed"
                        } else {
                            "text-gray-700 dark:text-gray-300"
                        }
                    ),
                    r#for: input_id.as_str(),
                    "{props.label}"
                }
            }
        }
    }
}

/// Props for Radio Group container
#[derive(Props, Clone, PartialEq)]
pub struct RadioGroupProps {
    /// Currently selected value
    pub value: String,
    
    /// Callback when selection changes
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
    
    /// Whether the entire group is disabled
    #[props(default = false)]
    pub disabled: bool,
    
    /// Size variant for all radio buttons in the group
    #[props(default = RadioSize::default())]
    pub size: RadioSize,
    
    /// Name attribute for the radio group
    pub name: String,
    
    /// Additional CSS classes for the container
    #[props(default = String::new())]
    pub class: String,
    
    /// Layout direction for the group
    #[props(default = RadioDirection::default())]
    pub direction: RadioDirection,
    
    /// The radio button options
    pub children: Element,
}

/// Direction for radio group layout
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RadioDirection {
    Vertical,
    Horizontal,
}

impl Default for RadioDirection {
    fn default() -> Self {
        Self::Vertical
    }
}

/// Radio Group container component
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
        div {
            class: combined_classes,
            role: "radiogroup",
            {props.children}
        }
    }
}

fn get_size_classes(size: RadioSize) -> (&'static str, &'static str) {
    match size {
        RadioSize::Small => (
            "w-4 h-4", // circle
            "w-2 h-2"  // dot (bigger to be visible)
        ),
        RadioSize::Medium => (
            "w-5 h-5", // circle
            "w-2.5 h-2.5" // dot (bigger to reduce gap)
        ),
        RadioSize::Large => (
            "w-6 h-6", // circle
            "w-3 h-3"  // dot (bigger to reduce gap)
        ),
    }
}