//! Shared chat view

use dioxus::prelude::*;
use crate::{PageHeader, Route, Platform, sample_characters, sample_scenarios, sample_stories, CharacterItem, Avatar};

#[derive(Clone, PartialEq)]
pub struct ChatMessage {
    pub id: String,
    pub content: String,
    pub is_user: bool,
    pub timestamp: String,
}

#[component]
pub fn ChatView(navigate_to: EventHandler<Route>) -> Element {
    let platform = Platform::current();
    let mut message_input = use_signal(|| "".to_string());
    let mut details_expanded = use_signal(|| false);
    
    // Get sample data for this story (using Academy Investigation Squad for demo)
    let story = sample_stories().into_iter().find(|s| s.title == "Academy Investigation Squad").unwrap();
    let character = sample_characters().into_iter().find(|c| c.name == "Luna Blackwood").unwrap();
    let scenario = sample_scenarios().into_iter().find(|s| s.name == "Magical Academy").unwrap();
    
    // Create a fake user character for avatar consistency
    let user_character = CharacterItem {
        id: "user".to_string(),
        name: "Student".to_string(),
        description: "Aspiring magic user".to_string(),
        avatar_url: None,
        tags: vec![],
        is_favorite: false,
        story_count: 0,
        last_used: None,
    };
    
    // Sample messages for demo
    let messages = use_signal(|| vec![
        ChatMessage {
            id: "1".to_string(),
            content: "Hello! I'm excited to start our roleplay adventure. What kind of story would you like to explore today?".to_string(),
            is_user: false,
            timestamp: "2:30 PM".to_string(),
        },
        ChatMessage {
            id: "2".to_string(),
            content: "I'd love to explore a fantasy adventure! Maybe something with magic and mysterious creatures?".to_string(),
            is_user: true,
            timestamp: "2:32 PM".to_string(),
        },
        ChatMessage {
            id: "3".to_string(),
            content: "Perfect! Let me set the scene for you...\n\nYou find yourself standing at the edge of the Whispering Woods, an ancient forest where the trees seem to hum with magical energy. The air shimmers with tiny sparkles of light, and you can hear the distant sound of something moving through the underbrush. Your hand instinctively reaches for the enchanted amulet around your neck.\n\nWhat do you do?".to_string(),
            is_user: false,
            timestamp: "2:33 PM".to_string(),
        },
    ]);
    
    rsx! {
        div { class: "flex-1 flex flex-col h-full bg-gray-50 dark:bg-gray-900",
            PageHeader {
                title: "Mystical Adventure".to_string(),
                back_button: if matches!(platform, Platform::Mobile) { Some(rsx! {
                    button {
                        class: "flex items-center text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors",
                        onclick: move |_| navigate_to.call(Route::Stories),
                        span { class: "text-2xl font-bold leading-none", "‹" }
                    }
                }) } else { None },
            }
            // Story details dropdown
            div { class: "flex-shrink-0 bg-white dark:bg-gray-950 border-b border-gray-200 dark:border-gray-700",
                div { class: "max-w-4xl mx-auto",
                    // Story header
                    button {
                        class: "w-full px-6 py-4 flex items-center justify-between text-left hover:bg-gray-50 dark:hover:bg-gray-900 transition-colors",
                        onclick: move |_| details_expanded.set(!details_expanded()),
                        div { class: "flex items-center space-x-3",
                            div { class: "w-2 h-2 rounded-full bg-purple-500" }
                            span { class: "text-gray-900 dark:text-gray-100 font-medium",
                                "{story.title}"
                            }
                        }
                        svg {
                            class: format!(
                                "w-5 h-5 text-gray-400 transition-transform {}",
                                if details_expanded() { "rotate-180" } else { "" },
                            ),
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M19 9l-7 7-7-7",
                            }
                        }
                    }
                    // Expanded details
                    if details_expanded() {
                        div { class: "px-6 pb-4 space-y-4 border-t border-gray-100 dark:border-gray-800",
                            div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4 pt-4",
                                // Character info
                                div { class: "space-y-2",
                                    div { class: "text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide",
                                        "Character"
                                    }
                                    div { class: "flex items-center space-x-2",
                                        Avatar { name: character.name.clone(), avatar_url: character.avatar_url.clone() }
                                        div {
                                            div { class: "text-sm font-medium text-gray-900 dark:text-gray-100",
                                                "{character.name}"
                                            }
                                            div { class: "text-xs text-gray-500 dark:text-gray-400",
                                                "{character.description}"
                                            }
                                        }
                                    }
                                }
                                // User persona
                                div { class: "space-y-2",
                                    div { class: "text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide",
                                        "Your Role"
                                    }
                                    div { class: "flex items-center space-x-2",
                                        Avatar { name: user_character.name.clone(), avatar_url: user_character.avatar_url.clone() }
                                        div {
                                            div { class: "text-sm font-medium text-gray-900 dark:text-gray-100",
                                                "Student"
                                            }
                                            div { class: "text-xs text-gray-500 dark:text-gray-400",
                                                "{user_character.description}"
                                            }
                                        }
                                    }
                                }
                                // Scenario
                                div { class: "space-y-2",
                                    div { class: "text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide",
                                        "Scenario"
                                    }
                                    div {
                                        div { class: "text-sm font-medium text-gray-900 dark:text-gray-100",
                                            "{scenario.name}"
                                        }
                                        div { class: "text-xs text-gray-500 dark:text-gray-400",
                                            "{scenario.description}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Messages area
            div { class: "flex-1 overflow-y-auto p-6 space-y-6",
                for message in messages() {
                    ChatMessageComponent { message: message.clone() }
                }
            }
            // Input area
            div { class: "flex-shrink-0 bg-gray-50 dark:bg-gray-900 border-t border-gray-200 dark:border-gray-700 p-4 sm:p-6",
                div { class: "max-w-4xl mx-auto flex items-end space-x-3",
                    div { class: "flex-1 bg-white dark:bg-gray-800 rounded-2xl border border-gray-200 dark:border-gray-700 px-4 py-3",
                        textarea {
                            class: "w-full resize-none bg-transparent border-0 p-0 text-gray-900 dark:text-gray-100 placeholder-gray-500 dark:placeholder-gray-400 focus:ring-0 focus:outline-none text-base leading-relaxed",
                            placeholder: "Type your message...",
                            rows: "1",
                            value: message_input(),
                            oninput: move |e| message_input.set(e.value()),
                            onkeypress: move |e| {
                                if e.key() == Key::Enter {
                                    e.prevent_default();
                                    message_input.set("".to_string());
                                }
                            },
                        }
                    }
                    button {
                        class: "w-10 h-10 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-300 dark:disabled:bg-gray-600 disabled:cursor-not-allowed text-white rounded-full transition-all duration-200 flex items-center justify-center shadow-lg hover:shadow-xl disabled:shadow-none transform hover:scale-105 disabled:scale-100",
                        disabled: message_input().trim().is_empty(),
                        onclick: move |_| {
                            message_input.set("".to_string());
                        },
                        svg {
                            class: "w-5 h-5",
                            fill: "currentColor",
                            view_box: "0 0 24 24",
                            path { d: "M2.01 21L23 12 2.01 3 2 10l15 2-15 2z" }
                        }
                    }
                }
                if !platform.is_mobile() {
                    div { class: "text-xs text-gray-500 dark:text-gray-400 text-center mt-3 max-w-4xl mx-auto",
                        "Press Enter to send, Shift+Enter for new line"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ChatMessageProps {
    pub message: ChatMessage,
}

#[component]
pub fn ChatMessageComponent(props: ChatMessageProps) -> Element {
    let message = &props.message;
    
    rsx! {
        div {
            class: format!(
                "flex {} mb-4",
                if message.is_user { "justify-end" } else { "justify-start" },
            ),
            div {
                class: format!(
                    "max-w-xs sm:max-w-md lg:max-w-2xl flex space-x-3 {}",
                    if message.is_user { "flex-row-reverse space-x-reverse" } else { "flex-row" },
                ),
                // Avatar (just a dot indicator)
                div { class: "flex-shrink-0 flex items-start pt-1",
                    div {
                        class: format!(
                            "w-2 h-2 rounded-full {}",
                            if message.is_user { "bg-blue-500" } else { "bg-purple-500" },
                        ),
                    }
                }
                // Message bubble
                div { class: "flex flex-col space-y-1",
                    div {
                        class: format!(
                            "rounded-3xl px-5 py-3 shadow-sm {}",
                            if message.is_user {
                                "bg-gradient-to-br from-blue-500 to-blue-600 text-white rounded-br-lg"
                            } else {
                                "bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 border border-gray-200 dark:border-gray-700 rounded-bl-lg"
                            },
                        ),
                        div { class: "whitespace-pre-wrap text-base leading-relaxed",
                            "{message.content}"
                        }
                    }
                    div {
                        class: format!(
                            "text-xs text-gray-400 dark:text-gray-500 px-1 {}",
                            if message.is_user { "text-right" } else { "text-left" },
                        ),
                        "{message.timestamp}"
                    }
                }
            }
        }
    }
}