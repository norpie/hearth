//! Avatar component - Simple avatar for any item with name and avatar_url

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    pub name: String,
    pub avatar_url: Option<String>,
    #[props(default)]
    pub size: Option<String>,
    #[props(default)]
    pub on_hover: Option<EventHandler<bool>>,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let mut image_loaded = use_signal(|| false);
    let mut image_error = use_signal(|| false);
    
    let first_char = props.name.chars().next().unwrap_or('?').to_uppercase().to_string();
    let size_class = props.size.as_deref().unwrap_or("w-12 h-12");
    let text_size = if size_class.contains("w-6 h-6") { 
        "text-xs" 
    } else if size_class.contains("w-10 h-10") { 
        "text-sm" 
    } else { 
        "text-lg" 
    };
    let is_clickable = props.onclick.is_some();
    
    rsx! {
        div { 
            class: if is_clickable { 
                "flex-shrink-0 cursor-pointer hover:opacity-80 transition-opacity select-none touch-manipulation" 
            } else { 
                "flex-shrink-0 select-none" 
            },
            style: "user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
            onmouseenter: move |_| {
                if let Some(handler) = &props.on_hover {
                    handler.call(true);
                }
            },
            onmouseleave: move |_| {
                if let Some(handler) = &props.on_hover {
                    handler.call(false);
                }
            },
            onclick: move |event| {
                if let Some(handler) = &props.onclick {
                    handler.call(event);
                }
            },
            
            if let Some(url) = &props.avatar_url {
                // Show fallback until image loads or errors
                if !image_loaded() || image_error() {
                    div {
                        class: "{size_class} rounded-full bg-blue-600 dark:bg-blue-500 flex items-center justify-center text-white font-medium {text_size} select-none",
                        style: "user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
                        "{first_char}"
                    }
                }
                // Always render image but hide until loaded
                img {
                    class: if image_loaded() && !image_error() { 
                        "{size_class} rounded-full object-cover select-none" 
                    } else { 
                        "{size_class} rounded-full object-cover select-none opacity-0 absolute" 
                    },
                    style: "user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
                    src: "{url}",
                    alt: "{props.name}",
                    onload: move |_| {
                        image_loaded.set(true);
                        image_error.set(false);
                    },
                    onerror: move |_| {
                        image_error.set(true);
                        image_loaded.set(false);
                    }
                }
            } else {
                div {
                    class: "{size_class} rounded-full bg-blue-600 dark:bg-blue-500 flex items-center justify-center text-white font-medium {text_size} select-none",
                    style: "user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
                    "{first_char}"
                }
            }
        }
    }
}