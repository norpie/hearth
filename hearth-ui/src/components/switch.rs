//! Toggle switch component with customizable styling

use dioxus::prelude::*;

/// Size variants for the Switch component
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwitchSize {
    Small,
    Medium,
    Large,
}

impl Default for SwitchSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Props for the Switch component
#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    /// Whether the switch is currently on/checked
    #[props(default = false)]
    pub checked: bool,
    
    /// Callback when switch state changes
    #[props(default)]
    pub onchange: Option<EventHandler<bool>>,
    
    /// Whether the switch is disabled
    #[props(default = false)]
    pub disabled: bool,
    
    /// Size variant of the switch
    #[props(default = SwitchSize::default())]
    pub size: SwitchSize,
    
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
    
    /// ID for accessibility
    #[props(default = String::new())]
    pub id: String,
    
    /// ARIA label for accessibility
    #[props(default = String::new())]
    pub aria_label: String,
}

/// Toggle switch component with smooth animations and accessibility support
#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let (track_classes, thumb_classes) = get_size_classes(props.size);
    
    let track_base_classes = "rounded-full relative transition-all duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-900";
    let track_state_classes = if props.checked {
        if props.disabled {
            "bg-blue-200 dark:bg-blue-800 cursor-not-allowed opacity-50"
        } else {
            "bg-blue-600 dark:bg-blue-500 cursor-pointer hover:bg-blue-700 dark:hover:bg-blue-400 active:scale-95"
        }
    } else {
        if props.disabled {
            "bg-gray-100 dark:bg-gray-800 cursor-not-allowed opacity-50"
        } else {
            "bg-gray-300 dark:bg-gray-600 cursor-pointer hover:bg-gray-400 dark:hover:bg-gray-500 active:scale-95"
        }
    };
    
    let thumb_base_classes = "bg-white rounded-full absolute top-1 left-1 transition-all duration-300 ease-in-out shadow-md";
    
    let thumb_position_classes = if props.checked {
        match props.size {
            SwitchSize::Small => "translate-x-3 scale-110",     // 12px - should work according to Tailwind docs
            SwitchSize::Medium => "translate-x-5 scale-110",    // 20px - WORKING
            SwitchSize::Large => "translate-x-6 scale-110",     // 24px - should work according to Tailwind docs
        }
    } else {
        "translate-x-0 scale-100"
    };
    
    let combined_track_classes = if props.class.is_empty() {
        format!("{} {} {}", track_base_classes, track_classes, track_state_classes)
    } else {
        format!("{} {} {} {}", track_base_classes, track_classes, track_state_classes, props.class)
    };
    
    let combined_thumb_classes = format!("{} {} {}", thumb_base_classes, thumb_classes, thumb_position_classes);

    rsx! {
        button {
            class: combined_track_classes,
            id: if props.id.is_empty() { None } else { Some(props.id.as_str()) },
            "aria-label": if props.aria_label.is_empty() { None } else { Some(props.aria_label.as_str()) },
            "aria-checked": props.checked.to_string(),
            role: "switch",
            disabled: props.disabled,
            onclick: move |_| {
                if !props.disabled {
                    if let Some(onchange) = &props.onchange {
                        onchange.call(!props.checked);
                    }
                }
            },
            div {
                class: combined_thumb_classes,
            }
        }
    }
}

fn get_size_classes(size: SwitchSize) -> (&'static str, &'static str) {
    match size {
        SwitchSize::Small => (
            "w-8 h-5", // track
            "w-3 h-3"  // thumb
        ),
        SwitchSize::Medium => (
            "w-12 h-7", // track  
            "w-5 h-5"   // thumb
        ),
        SwitchSize::Large => (
            "w-14 h-8", // track
            "w-6 h-6"   // thumb
        ),
    }
}