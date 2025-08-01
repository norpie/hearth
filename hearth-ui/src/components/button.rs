//! Button component with different variants and sizes

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Destructive,
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default = ButtonVariant::Primary)]
    pub variant: ButtonVariant,
    #[props(default = ButtonSize::Medium)]
    pub size: ButtonSize,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = false)]
    pub loading: bool,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

impl ButtonVariant {
    pub fn classes(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "bg-blue-600 hover:bg-blue-700 active:bg-blue-800 dark:bg-blue-500 dark:hover:bg-blue-600 dark:active:bg-blue-700 text-white border-transparent shadow-sm hover:shadow active:shadow-none",
            ButtonVariant::Secondary => "bg-gray-100 hover:bg-gray-200 active:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 dark:active:bg-gray-500 text-gray-900 dark:text-gray-100 border-transparent shadow-sm hover:shadow active:shadow-none",
            ButtonVariant::Outline => "bg-transparent hover:bg-gray-50 active:bg-gray-100 dark:hover:bg-gray-800 dark:active:bg-gray-700 text-gray-700 dark:text-gray-300 border-gray-300 dark:border-gray-600 hover:border-gray-400 dark:hover:border-gray-500 active:border-gray-500 dark:active:border-gray-400",
            ButtonVariant::Ghost => "bg-transparent hover:bg-gray-100 active:bg-gray-200 dark:hover:bg-gray-800 dark:active:bg-gray-700 text-gray-700 dark:text-gray-300 border-transparent",
            ButtonVariant::Destructive => "bg-red-600 hover:bg-red-700 active:bg-red-800 dark:bg-red-500 dark:hover:bg-red-600 dark:active:bg-red-700 text-white border-transparent shadow-sm hover:shadow active:shadow-none",
        }
    }
}

impl ButtonSize {
    pub fn classes(&self) -> &'static str {
        match self {
            ButtonSize::Small => "px-3 py-1.5 text-sm",
            ButtonSize::Medium => "px-4 py-2 text-base",
            ButtonSize::Large => "px-6 py-3 text-lg",
        }
    }
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let base_classes = "inline-flex items-center justify-center rounded-md border font-medium transition-all duration-150 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-900 disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none active:scale-95 select-none";
    
    let variant_classes = props.variant.classes();
    let size_classes = props.size.classes();
    
    let custom_classes = props.class.as_deref().unwrap_or("");
    
    let combined_classes = format!("{} {} {} {}", base_classes, variant_classes, size_classes, custom_classes);

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
                // Loading spinner
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