//! Toggle group component for managing multiple toggle states

use dioxus::prelude::*;
use crate::{ToggleSize, ToggleVariant};

#[derive(Clone, PartialEq)]
pub enum ToggleGroupOrientation {
    Horizontal,
    Vertical,
}

// Simple approach - no context, just direct props
#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupProps {
    /// Whether only one toggle can be active at a time (like radio buttons)
    #[props(default = false)]
    pub exclusive: bool,
    
    /// List of currently selected toggle values
    #[props(default = vec![])]
    pub value: Vec<String>,
    
    /// Callback when toggle selection changes
    #[props(default)]
    pub onchange: Option<EventHandler<Vec<String>>>,
    
    /// Whether the toggle group is disabled
    #[props(default = false)]
    pub disabled: bool,
    
    /// Size variant of the toggles
    #[props(default = ToggleSize::Medium)]
    pub size: ToggleSize,
    
    /// Visual variant of the toggles
    #[props(default = ToggleVariant::Default)]
    pub variant: ToggleVariant,
    
    /// Orientation of the toggle group
    #[props(default = ToggleGroupOrientation::Horizontal)]
    pub orientation: ToggleGroupOrientation,
    
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
    
    /// Toggle group items - expects ToggleGroupItem children
    pub children: Element,
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupItemProps {
    /// Unique value for this toggle item
    pub value: String,
    
    /// Whether this item is pressed/active (managed by parent)
    #[props(default = false)]
    pub pressed: bool,
    
    /// Whether this item is disabled
    #[props(default = false)]
    pub disabled: bool,
    
    /// Size variant
    #[props(default = ToggleSize::Medium)]
    pub size: ToggleSize,
    
    /// Visual variant
    #[props(default = ToggleVariant::Default)]
    pub variant: ToggleVariant,
    
    /// Orientation for proper grouping styles
    #[props(default = ToggleGroupOrientation::Horizontal)]
    pub orientation: ToggleGroupOrientation,
    
    /// Click handler (managed by parent)
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
    
    /// Content to display in the toggle
    pub children: Element,
}

#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
    let container_classes = match props.orientation {
        ToggleGroupOrientation::Horizontal => format!(
            "inline-flex {}",
            props.class
        ),
        ToggleGroupOrientation::Vertical => format!(
            "inline-flex flex-col {}",
            props.class
        ),
    };

    rsx! {
        div {
            class: "{container_classes}",
            role: if props.exclusive { "radiogroup" } else { "group" },
            "aria-disabled": props.disabled,
            {props.children}
        }
    }
}

#[component]
pub fn ToggleGroupItem(props: ToggleGroupItemProps) -> Element {
    let size_classes = match props.size {
        ToggleSize::Small => "px-2.5 py-1.5 text-sm",
        ToggleSize::Medium => "px-3 py-2 text-base",
        ToggleSize::Large => "px-4 py-2.5 text-lg",
    };

    let base_classes = "inline-flex items-center justify-center border font-medium transition-all duration-150 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-900 disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none active:scale-95 select-none";
    
    // Enhanced styling for proper button group appearance based on orientation
    let group_classes = match props.orientation {
        ToggleGroupOrientation::Horizontal => "!rounded-none first:!rounded-l-md last:!rounded-r-md relative focus:z-10 border-r-0 last:border-r",
        ToggleGroupOrientation::Vertical => "!rounded-none first:!rounded-t-md last:!rounded-b-md relative focus:z-10 border-b-0 last:border-b",
    };
    
    let variant_classes = match (props.variant, props.pressed) {
        (ToggleVariant::Default, false) => "bg-transparent hover:bg-gray-100 active:bg-gray-200 dark:hover:bg-gray-800 dark:active:bg-gray-700 text-gray-700 dark:text-gray-300 border-transparent",
        (ToggleVariant::Default, true) => "bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100 border-transparent",
        (ToggleVariant::Outline, false) => "bg-transparent hover:bg-gray-50 active:bg-gray-100 dark:hover:bg-gray-800 dark:active:bg-gray-700 text-gray-700 dark:text-gray-300 border-gray-300 dark:border-gray-600 hover:border-gray-400 dark:hover:border-gray-500",
        (ToggleVariant::Outline, true) => "bg-gray-50 dark:bg-gray-800 text-gray-900 dark:text-gray-100 border-gray-400 dark:border-gray-500",
    };
    
    let combined_classes = format!("{} {} {} {} {}", base_classes, variant_classes, size_classes, group_classes, props.class);

    rsx! {
        button {
            r#type: "button",
            class: "{combined_classes}",
            disabled: props.disabled,
            "aria-pressed": props.pressed,
            onclick: move |evt| {
                if !props.disabled {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                }
            },
            {props.children}
        }
    }
}