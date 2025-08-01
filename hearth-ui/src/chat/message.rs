//! Message display components

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MessageProps {
    pub content: String,
    pub timestamp: Option<String>,
    pub avatar_url: Option<String>,
    pub sender_name: Option<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct UserMessageProps {
    pub content: String,
    pub timestamp: Option<String>,
    pub guidance: Option<String>,
    pub is_edited: Option<bool>,
}

#[derive(Props, Clone, PartialEq)]
pub struct CharacterMessageProps {
    pub content: String,
    pub character_name: String,
    pub avatar_url: Option<String>,
    pub timestamp: Option<String>,
    pub is_streaming: Option<bool>,
}

#[derive(Props, Clone, PartialEq)]
pub struct NarratorMessageProps {
    pub content: String,
    pub timestamp: Option<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct SystemMessageProps {
    pub content: String,
    pub message_type: SystemMessageType,
}

#[derive(Clone, PartialEq)]
pub enum SystemMessageType {
    Info,
    Warning,
    Error,
    Success,
}

#[component]
pub fn UserMessage(props: UserMessageProps) -> Element {
    rsx! {
        div { class: "flex justify-end mb-4 group",
            div { class: "max-w-xs lg:max-w-md",
                // Message bubble
                div { class: "bg-chat-user text-white rounded-lg rounded-br-none px-4 py-2 shadow-sm",
                    "{props.content}"
                }
                // Metadata row
                div { class: "flex items-center justify-end mt-1 space-x-2 text-xs text-gray-500",
                    if props.is_edited.unwrap_or(false) {
                        span { class: "text-gray-400", "edited" }
                    }
                    if let Some(timestamp) = props.timestamp {
                        span { "{timestamp}" }
                    }
                }
                // Guidance indicator (only visible if present)
                if let Some(guidance) = props.guidance {
                    div { class: "mt-1 text-xs text-chat-guidance opacity-75 italic flex items-center space-x-1",
                        i { class: "fa-solid fa-lightbulb" }
                        span { "{guidance}" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CharacterMessage(props: CharacterMessageProps) -> Element {
    rsx! {
        div { class: "flex justify-start mb-4 group",
            // Avatar
            if let Some(avatar_url) = props.avatar_url {
                img {
                    class: "w-8 h-8 rounded-full mr-3 mt-1 flex-shrink-0",
                    src: "{avatar_url}",
                    alt: "{props.character_name}",
                }
            } else {
                div { class: "w-8 h-8 rounded-full mr-3 mt-1 flex-shrink-0 bg-chat-character flex items-center justify-center text-white text-sm font-medium",
                    "{props.character_name.chars().next().unwrap_or('?').to_uppercase()}"
                }
            }
            div { class: "max-w-xs lg:max-w-md",
                // Character name
                div { class: "text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                    "{props.character_name}"
                }
                // Message bubble
                div { class: "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg rounded-tl-none px-4 py-2 shadow-sm",
                    if props.is_streaming.unwrap_or(false) {
                        div { class: "flex items-center",
                            span { "{props.content}" }
                            span { class: "ml-1 animate-pulse", "▌" }
                        }
                    } else {
                        "{props.content}"
                    }
                }
                // Timestamp
                if let Some(timestamp) = props.timestamp {
                    div { class: "text-xs text-gray-500 mt-1", "{timestamp}" }
                }
            }
        }
    }
}

#[component]
pub fn NarratorMessage(props: NarratorMessageProps) -> Element {
    rsx! {
        div { class: "flex justify-center mb-4",
            div { class: "max-w-2xl mx-4",
                div { class: "bg-chat-narrator/10 border border-chat-narrator/20 rounded-lg px-4 py-3 text-chat-narrator italic text-center",
                    "{props.content}"
                }
                if let Some(timestamp) = props.timestamp {
                    div { class: "text-xs text-gray-500 text-center mt-1", "{timestamp}" }
                }
            }
        }
    }
}

#[component]
pub fn SystemMessage(props: SystemMessageProps) -> Element {
    let (bg_class, border_class, text_class, icon) = match props.message_type {
        SystemMessageType::Info => ("bg-blue-50 dark:bg-blue-900/20", "border-blue-200 dark:border-blue-800", "text-blue-800 dark:text-blue-200", "fa-solid fa-info-circle"),
        SystemMessageType::Warning => ("bg-yellow-50 dark:bg-yellow-900/20", "border-yellow-200 dark:border-yellow-800", "text-yellow-800 dark:text-yellow-200", "fa-solid fa-exclamation-triangle"),
        SystemMessageType::Error => ("bg-red-50 dark:bg-red-900/20", "border-red-200 dark:border-red-800", "text-red-800 dark:text-red-200", "fa-solid fa-times-circle"),
        SystemMessageType::Success => ("bg-green-50 dark:bg-green-900/20", "border-green-200 dark:border-green-800", "text-green-800 dark:text-green-200", "fa-solid fa-check-circle"),
    };

    rsx! {
        div { class: "flex justify-center mb-4",
            div { class: "max-w-md mx-4",
                div { class: "rounded-lg border px-4 py-2 text-sm {bg_class} {border_class} {text_class}",
                    div { class: "flex items-center space-x-2",
                        i { class: "{icon}" }
                        span { "{props.content}" }
                    }
                }
            }
        }
    }
}