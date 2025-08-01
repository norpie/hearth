//! Toggle component for on/off state with ghost button styling

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ToggleSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, PartialEq)]
pub enum ToggleVariant {
    Default,
    Outline,
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    /// Whether the toggle is currently pressed/active
    #[props(default = false)]
    pub pressed: bool,
    
    /// Whether the toggle is disabled
    #[props(default = false)]
    pub disabled: bool,
    
    /// Size variant of the toggle
    #[props(default = ToggleSize::Medium)]
    pub size: ToggleSize,
    
    /// Visual variant of the toggle
    #[props(default = ToggleVariant::Default)]
    pub variant: ToggleVariant,
    
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
    
    /// Click event handler
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    
    /// Content to display in the toggle
    pub children: Element,
}

impl ToggleSize {
    pub fn classes(&self) -> &'static str {
        match self {
            ToggleSize::Small => "px-2.5 py-1.5 text-sm",
            ToggleSize::Medium => "px-3 py-2 text-base", 
            ToggleSize::Large => "px-4 py-2.5 text-lg",
        }
    }
}

impl ToggleVariant {
    pub fn classes(&self, pressed: bool) -> &'static str {
        match (self, pressed) {
            (ToggleVariant::Default, false) => "bg-transparent hover:bg-gray-100 active:bg-gray-200 dark:hover:bg-gray-800 dark:active:bg-gray-700 text-gray-700 dark:text-gray-300 border-transparent",
            (ToggleVariant::Default, true) => "bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100 border-transparent",
            (ToggleVariant::Outline, false) => "bg-transparent hover:bg-gray-50 active:bg-gray-100 dark:hover:bg-gray-800 dark:active:bg-gray-700 text-gray-700 dark:text-gray-300 border-gray-300 dark:border-gray-600 hover:border-gray-400 dark:hover:border-gray-500",
            (ToggleVariant::Outline, true) => "bg-gray-50 dark:bg-gray-800 text-gray-900 dark:text-gray-100 border-gray-400 dark:border-gray-500",
        }
    }
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let base_classes = "inline-flex items-center justify-center rounded-md border font-medium transition-all duration-150 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-900 disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none active:scale-95 select-none";
    
    let variant_classes = props.variant.classes(props.pressed);
    let size_classes = props.size.classes();
    
    let combined_classes = format!("{} {} {} {}", base_classes, variant_classes, size_classes, props.class);

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