//! Checkbox component with customizable styling

use dioxus::prelude::*;

/// Size variants for the Checkbox component
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckboxSize {
    Small,
    Medium,
    Large,
}

impl Default for CheckboxSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Props for the Checkbox component
#[derive(Props, Clone, PartialEq)]
pub struct CheckboxProps {
    /// Whether the checkbox is currently checked
    #[props(default = false)]
    pub checked: bool,
    
    /// Callback when checkbox state changes
    #[props(default)]
    pub onchange: Option<EventHandler<bool>>,
    
    /// Whether the checkbox is disabled
    #[props(default = false)]
    pub disabled: bool,
    
    /// Size variant of the checkbox
    #[props(default = CheckboxSize::default())]
    pub size: CheckboxSize,
    
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
    
    /// ID for accessibility
    #[props(default = String::new())]
    pub id: String,
    
    /// ARIA label for accessibility
    #[props(default = String::new())]
    pub aria_label: String,
    
    /// Whether the checkbox is required
    #[props(default = false)]
    pub required: bool,
}

/// Checkbox component with smooth animations and accessibility support
#[component]
pub fn Checkbox(props: CheckboxProps) -> Element {
    let (box_classes, icon_classes) = get_size_classes(props.size);
    
    let box_base_classes = "border-2 rounded transition-all duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-900 flex items-center justify-center";
    
    let box_state_classes = if props.checked {
        if props.disabled {
            "bg-blue-200 dark:bg-blue-800 border-blue-200 dark:border-blue-800 cursor-not-allowed opacity-50"
        } else {
            "bg-blue-600 dark:bg-blue-500 border-blue-600 dark:border-blue-500 cursor-pointer hover:bg-blue-700 dark:hover:bg-blue-400 active:scale-95"
        }
    } else {
        if props.disabled {
            "bg-gray-100 dark:bg-gray-800 border-gray-200 dark:border-gray-700 cursor-not-allowed opacity-50"
        } else {
            "bg-white dark:bg-gray-900 border-gray-300 dark:border-gray-600 cursor-pointer hover:border-gray-400 dark:hover:border-gray-500 active:scale-95"
        }
    };
    
    let combined_box_classes = if props.class.is_empty() {
        format!("{} {} {}", box_base_classes, box_classes, box_state_classes)
    } else {
        format!("{} {} {} {}", box_base_classes, box_classes, box_state_classes, props.class)
    };
    
    let icon_visibility = if props.checked {
        "opacity-100 scale-100"
    } else {
        "opacity-0 scale-75"
    };
    
    let combined_icon_classes = format!("{} text-white transition-all duration-200 ease-in-out {}", icon_classes, icon_visibility);

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

fn get_size_classes(size: CheckboxSize) -> (&'static str, &'static str) {
    match size {
        CheckboxSize::Small => (
            "w-4 h-4", // box
            "w-3 h-3"  // icon
        ),
        CheckboxSize::Medium => (
            "w-5 h-5", // box
            "w-4 h-4"  // icon
        ),
        CheckboxSize::Large => (
            "w-6 h-6", // box
            "w-5 h-5"  // icon
        ),
    }
}