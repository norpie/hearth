//! Chat input components

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ChatInputProps {
    pub placeholder: Option<String>,
    pub value: String,
    pub guidance_value: Option<String>,
    pub is_guidance_expanded: bool,
    pub is_sending: bool,
    pub on_input: EventHandler<String>,
    pub on_guidance_input: EventHandler<String>,
    pub on_guidance_toggle: EventHandler<()>,
    pub on_send: EventHandler<()>,
    pub on_keypress: EventHandler<KeyboardEvent>,
}

#[derive(Props, Clone, PartialEq)]
pub struct GuidanceInputProps {
    pub value: String,
    pub placeholder: Option<String>,
    pub is_expanded: bool,
    pub on_input: EventHandler<String>,
    pub on_toggle: EventHandler<()>,
}

#[derive(Props, Clone, PartialEq)]
pub struct MessageControlsProps {
    pub is_sending: bool,
    pub can_send: bool,
    pub on_send: EventHandler<()>,
    pub show_guidance_toggle: bool,
    pub is_guidance_expanded: bool,
    pub on_guidance_toggle: EventHandler<()>,
}

#[component]
pub fn ChatInput(props: ChatInputProps) -> Element {
    rsx! {
        div { class: "border-t border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900",
            div { class: "p-4",
                // Guidance input (conditionally visible)
                if props.is_guidance_expanded {
                    GuidanceInput {
                        value: props.guidance_value.unwrap_or_default(),
                        placeholder: "Guide the story direction...",
                        is_expanded: props.is_guidance_expanded,
                        on_input: props.on_guidance_input,
                        on_toggle: props.on_guidance_toggle,
                    }
                }
                // Main input area
                div { class: "flex items-end space-x-3",
                    // Text input
                    div { class: "flex-1",
                        textarea {
                            class: "w-full resize-none rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-4 py-3 text-sm placeholder-gray-500 dark:placeholder-gray-400 focus:border-chat-user focus:outline-none focus:ring-1 focus:ring-chat-user",
                            placeholder: props.placeholder.unwrap_or_else(|| "Type your message...".to_string()),
                            value: "{props.value}",
                            rows: "1",
                            onkeypress: props.on_keypress,
                            oninput: move |evt| {
                                props.on_input.call(evt.value());
                            },
                        }
                    }
                    // Controls
                    MessageControls {
                        is_sending: props.is_sending,
                        can_send: !props.value.trim().is_empty(),
                        on_send: props.on_send,
                        show_guidance_toggle: true,
                        is_guidance_expanded: props.is_guidance_expanded,
                        on_guidance_toggle: props.on_guidance_toggle,
                    }
                }
            }
        }
    }
}

#[component]
pub fn GuidanceInput(props: GuidanceInputProps) -> Element {
    rsx! {
        div { class: "mb-3",
            // Header with toggle
            div { class: "flex items-center justify-between mb-2",
                label { class: "text-sm font-medium text-chat-guidance", "Story Guidance" }
                button {
                    class: "text-xs text-gray-500 hover:text-gray-700 dark:hover:text-gray-300",
                    onclick: move |_| props.on_toggle.call(()),
                    "✕"
                }
            }
            // Guidance text area
            textarea {
                class: "w-full resize-none rounded-lg border border-chat-guidance/30 bg-chat-guidance/5 px-3 py-2 text-sm placeholder-chat-guidance/60 focus:border-chat-guidance focus:outline-none focus:ring-1 focus:ring-chat-guidance",
                placeholder: props
                    .placeholder
                    .unwrap_or_else(|| {
                        "Provide subtle direction for how the story should develop...".to_string()
                    }),
                value: "{props.value}",
                rows: "2",
                oninput: move |evt| {
                    props.on_input.call(evt.value());
                },
            }
        }
    }
}

#[component]
pub fn MessageControls(props: MessageControlsProps) -> Element {
    rsx! {
        div { class: "flex items-center space-x-2",
            // Guidance toggle button
            if props.show_guidance_toggle {
                button {
                    class: format!(
                        "p-2 rounded-lg transition-colors {}",
                        if props.is_guidance_expanded {
                            "bg-chat-guidance text-white"
                        } else {
                            "text-chat-guidance hover:bg-chat-guidance/10"
                        },
                    ),
                    title: "Toggle story guidance",
                    onclick: move |_| props.on_guidance_toggle.call(()),
                    "💡"
                }
            }
            // Send button
            button {
                class: format!(
                    "px-4 py-2 rounded-lg font-medium transition-all {}",
                    if props.can_send && !props.is_sending {
                        "bg-chat-user text-white hover:bg-chat-user/90 shadow-sm"
                    } else {
                        "bg-gray-300 dark:bg-gray-600 text-gray-500 dark:text-gray-400 cursor-not-allowed"
                    },
                ),
                disabled: !props.can_send || props.is_sending,
                onclick: move |_| {
                    if props.can_send && !props.is_sending {
                        props.on_send.call(());
                    }
                },
                if props.is_sending {
                    div { class: "flex items-center space-x-2",
                        div { class: "w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" }
                        span { "Sending..." }
                    }
                } else {
                    "Send"
                }
            }
        }
    }
}