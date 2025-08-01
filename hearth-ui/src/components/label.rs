//! Simple label component for form fields and accessibility

use dioxus::prelude::*;

/// Props for the Label component
#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    /// The text content of the label
    pub children: Element,
    
    /// Optional for attribute to associate with form control
    #[props(default = String::new())]
    pub r#for: String,
    
    /// Whether the label indicates a required field
    #[props(default = false)]
    pub required: bool,
    
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
}

/// Simple label component with basic styling and accessibility support
#[component]
pub fn Label(props: LabelProps) -> Element {
    let base_classes = "block text-sm font-medium text-gray-700 dark:text-gray-300";
    
    let combined_classes = if props.class.is_empty() {
        base_classes.to_string()
    } else {
        format!("{} {}", base_classes, props.class)
    };

    rsx! {
        label {
            class: combined_classes,
            r#for: if props.r#for.is_empty() { None } else { Some(props.r#for.as_str()) },
            {props.children}
            if props.required {
                span {
                    class: "ml-1 text-red-500",
                    "*"
                }
            }
        }
    }
}