//! Conversation management components

use dioxus::prelude::*;
use super::message::*;

#[derive(Props, Clone, PartialEq)]
pub struct ConversationHeaderProps {
    pub character_name: String,
    pub character_avatar: Option<String>,
    pub conversation_title: Option<String>,
    pub scenario_name: Option<String>,
    pub is_online: bool,
    pub on_settings: EventHandler<()>,
    pub on_back: Option<EventHandler<()>>,
}

#[derive(Props, Clone, PartialEq)]
pub struct MessageListProps {
    pub messages: Vec<MessageData>,
    pub is_loading: bool,
    pub show_scroll_to_bottom: bool,
    pub on_scroll_to_bottom: EventHandler<()>,
}

#[derive(Props, Clone, PartialEq)]
pub struct ConversationSettingsProps {
    pub is_open: bool,
    pub narrator_enabled: bool,
    pub narrator_mode: NarratorMode,
    pub on_close: EventHandler<()>,
    pub on_narrator_toggle: EventHandler<bool>,
    pub on_narrator_mode_change: EventHandler<NarratorMode>,
}

#[derive(Clone, PartialEq)]
pub struct MessageData {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: Option<String>,
    pub metadata: MessageMetadata,
}

#[derive(Clone, PartialEq)]
pub enum MessageRole {
    User { guidance: Option<String>, is_edited: bool },
    Character { name: String, avatar_url: Option<String>, is_streaming: bool },
    Narrator,
    System { message_type: SystemMessageType },
}

#[derive(Clone, PartialEq)]
pub struct MessageMetadata {
    pub character_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum NarratorMode {
    Automatic,
    UserControlled,
    Disabled,
    Guided,
}

#[component]
pub fn ConversationHeader(props: ConversationHeaderProps) -> Element {
    rsx! {
        div { class: "flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900",
            div { class: "flex items-center space-x-3",
                // Back button (mobile)
                if let Some(on_back) = props.on_back {
                    button {
                        class: "lg:hidden p-2 text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-100",
                        onclick: move |_| on_back.call(()),
                        "←"
                    }
                }
                // Character avatar
                if let Some(avatar_url) = props.character_avatar {
                    img {
                        class: "w-10 h-10 rounded-full",
                        src: "{avatar_url}",
                        alt: "{props.character_name}",
                    }
                } else {
                    div { class: "w-10 h-10 rounded-full bg-chat-character flex items-center justify-center text-white font-medium",
                        "{props.character_name.chars().next().unwrap_or('?').to_uppercase()}"
                    }
                }
                // Character info
                div {
                    div { class: "font-medium text-gray-900 dark:text-gray-100",
                        "{props.character_name}"
                    }
                    div { class: "flex items-center space-x-2 text-sm text-gray-500",
                        // Online status
                        div { class: "flex items-center space-x-1",
                            div {
                                class: format!(
                                    "w-2 h-2 rounded-full {}",
                                    if props.is_online { "bg-green-500" } else { "bg-gray-400" },
                                ),
                            }
                            span {
                                if props.is_online {
                                    "Online"
                                } else {
                                    "Offline"
                                }
                            }
                        }
                        // Scenario info
                        if let Some(scenario) = props.scenario_name {
                            span { "•" }
                            span { "{scenario}" }
                        }
                    }
                }
            }
            // Settings button
            button {
                class: "p-2 text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-100 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg",
                onclick: move |_| props.on_settings.call(()),
                i { class: "fas fa-gear" }
            }
        }
    }
}

#[component]
pub fn MessageList(props: MessageListProps) -> Element {
    rsx! {
        div { class: "flex-1 overflow-y-auto p-4 space-y-1",
            // Messages
            for message in props.messages.iter() {
                {render_message(message)}
            }
            // Loading indicator
            if props.is_loading {
                div { class: "flex justify-center py-4",
                    div { class: "flex items-center space-x-2 text-gray-500",
                        div { class: "w-4 h-4 border-2 border-gray-400 border-t-transparent rounded-full animate-spin" }
                        span { "Loading..." }
                    }
                }
            }
            // Scroll to bottom button
            if props.show_scroll_to_bottom {
                div { class: "fixed bottom-20 right-4 z-10",
                    button {
                        class: "p-3 bg-chat-user text-white rounded-full shadow-lg hover:bg-chat-user/90 transition-all",
                        onclick: move |_| props.on_scroll_to_bottom.call(()),
                        "↓"
                    }
                }
            }
        }
    }
}

fn render_message(message: &MessageData) -> Element {
    match &message.role {
        MessageRole::User { guidance, is_edited } => rsx! {
            UserMessage {
                content: message.content.clone(),
                timestamp: message.timestamp.clone(),
                guidance: guidance.clone(),
                is_edited: Some(*is_edited),
            }
        },
        MessageRole::Character { name, avatar_url, is_streaming } => rsx! {
            CharacterMessage {
                content: message.content.clone(),
                character_name: name.clone(),
                avatar_url: avatar_url.clone(),
                timestamp: message.timestamp.clone(),
                is_streaming: Some(*is_streaming),
            }
        },
        MessageRole::Narrator => rsx! {
            NarratorMessage {
                content: message.content.clone(),
                timestamp: message.timestamp.clone(),
            }
        },
        MessageRole::System { message_type } => rsx! {
            SystemMessage {
                content: message.content.clone(),
                message_type: message_type.clone(),
            }
        },
    }
}

#[component]
pub fn ConversationSettings(props: ConversationSettingsProps) -> Element {
    if !props.is_open {
        return rsx! {
            div {}
        };
    }

    rsx! {
        div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50",
            div { class: "bg-white dark:bg-gray-900 rounded-lg shadow-xl max-w-md w-full mx-4 p-4",
                // Header
                div { class: "flex items-center justify-between mb-6",
                    h3 { class: "text-lg font-semibold text-gray-900 dark:text-gray-100",
                        "Conversation Settings"
                    }
                    button {
                        class: "text-gray-500 hover:text-gray-700 dark:hover:text-gray-300",
                        onclick: move |_| props.on_close.call(()),
                        "✕"
                    }
                }
                // Narrator settings
                div { class: "space-y-4",
                    div {
                        label { class: "flex items-center space-x-3",
                            input {
                                r#type: "checkbox",
                                class: "w-4 h-4 text-chat-narrator border-gray-300 rounded focus:ring-chat-narrator",
                                checked: props.narrator_enabled,
                                onchange: move |evt| {
                                    props.on_narrator_toggle.call(evt.checked());
                                },
                            }
                            span { class: "text-sm font-medium text-gray-700 dark:text-gray-300",
                                "Enable Narrator"
                            }
                        }
                    }
                    if props.narrator_enabled {
                        div {
                            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                                "Narrator Mode"
                            }
                            select {
                                class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100",
                                onchange: move |evt| {
                                    let mode = match evt.value().as_str() {
                                        "automatic" => NarratorMode::Automatic,
                                        "user_controlled" => NarratorMode::UserControlled,
                                        "guided" => NarratorMode::Guided,
                                        _ => NarratorMode::Disabled,
                                    };
                                    props.on_narrator_mode_change.call(mode);
                                },
                                option {
                                    value: "automatic",
                                    selected: matches!(props.narrator_mode, NarratorMode::Automatic),
                                    "Automatic"
                                }
                                option {
                                    value: "user_controlled",
                                    selected: matches!(props.narrator_mode, NarratorMode::UserControlled),
                                    "User Controlled"
                                }
                                option {
                                    value: "guided",
                                    selected: matches!(props.narrator_mode, NarratorMode::Guided),
                                    "Guided by Input"
                                }
                            }
                        }
                    }
                }
                // Footer
                div { class: "flex justify-end mt-6 pt-6 border-t border-gray-200 dark:border-gray-700",
                    button {
                        class: "px-4 py-2 bg-chat-user text-white rounded-lg hover:bg-chat-user/90",
                        onclick: move |_| props.on_close.call(()),
                        "Done"
                    }
                }
            }
        }
    }
}