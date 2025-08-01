//! Story message components for interactive storytelling interface

use crate::{Avatar, AvatarVariant, MarkdownContent, Badge, BadgeVariant, Button, ButtonVariant, ButtonSize, StoryMessage, StoryRole};
use dioxus::prelude::*;

#[component]
pub fn StoryMessageComponent(message: StoryMessage) -> Element {
    match message.role {
        StoryRole::User { name } => rsx! {
            div { class: "mb-4 flex justify-center",
                div { class: "bg-user-message text-user-message-foreground p-4 rounded-lg border-r-4 border-user-message-foreground/20 max-w-2xl w-full relative overflow-visible",
                    // Drag handle positioned on the avatar side (right) middle, halfway between edge and outside
                    div { 
                        class: "absolute right-0 top-1/2 transform -translate-y-1/2 cursor-grab active:cursor-grabbing w-5 h-5 flex items-center justify-center text-muted-foreground hover:text-foreground transition-colors",
                        onclick: move |_| {
                            // TODO: Implement drag/reorder action
                        },
                        i { class: "fas fa-grip-vertical text-xs" }
                    }
                    // Main content area with avatar
                    div { class: "flex items-start space-x-3 mb-3",
                        div { class: "flex-1 min-w-0",
                            MarkdownContent {
                                content: message.content.clone(),
                                class: Some("prose prose-sm prose-invert max-w-none".to_string()),
                                italic_class: Some("text-gray-400".to_string()),
                                quote_class: Some("text-orange-400".to_string()),
                            }
                        }
                        div { class: "flex-shrink-0",
                            Avatar {
                                name: name.clone(),
                                variant: AvatarVariant::Portrait,
                                size: Some("w-16".to_string())
                            }
                        }
                    }
                    // Bottom row with name badge and quick actions
                    div { class: "flex items-center justify-between",
                        // Name badge on left (opposite side of avatar)
                        Badge {
                            variant: BadgeVariant::Secondary,
                            class: "text-xs".to_string(),
                            "{name}"
                        }
                        // Quick actions on right (same side as avatar)
                        div { class: "flex items-center space-x-1",
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Small,
                                class: "w-6 h-6 p-0".to_string(),
                                onclick: move |_| {
                                    // TODO: Implement edit action
                                },
                                i { class: "fas fa-edit text-xs" }
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Small,
                                class: "w-6 h-6 p-0".to_string(),
                                onclick: move |_| {
                                    // TODO: Implement delete action
                                },
                                i { class: "fas fa-trash text-xs" }
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Small,
                                class: "w-6 h-6 p-0".to_string(),
                                onclick: move |_| {
                                    // TODO: Implement regenerate action
                                },
                                i { class: "fas fa-redo text-xs" }
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Small,
                                class: "w-6 h-6 p-0".to_string(),
                                onclick: move |_| {
                                    // TODO: Implement branch action
                                },
                                i { class: "fas fa-code-branch text-xs" }
                            }
                        }
                    }
                }
            }
        },
        StoryRole::Narrator => rsx! {
            div { class: "flex justify-center mb-2",
                div { class: "max-w-2xl text-center",
                    div { class: "text-foreground",
                        MarkdownContent {
                            content: message.content.clone(),
                            class: Some("prose prose-sm max-w-none text-center".to_string()),
                            italic_class: Some("text-gray-500".to_string()),
                            quote_class: Some("text-orange-500".to_string()),
                        }
                    }
                }
            }
        },
        StoryRole::Character { name } => rsx! {
            div { class: "mb-4 flex justify-center",
                div { class: "bg-bot-message text-bot-message-foreground p-4 rounded-lg border-l-4 border-bot-message-foreground/20 max-w-2xl w-full relative overflow-visible",
                    // Drag handle positioned on the avatar side (left) middle, tiny movement inward
                    div { 
                        class: "absolute left-0 top-1/2 transform -translate-y-1/2 cursor-grab active:cursor-grabbing w-5 h-5 flex items-center justify-center text-muted-foreground hover:text-foreground transition-colors",
                        onclick: move |_| {
                            // TODO: Implement drag/reorder action
                        },
                        i { class: "fas fa-grip-vertical text-xs" }
                    }
                    // Main content area with avatar
                    div { class: "flex items-start space-x-3 mb-3",
                        div { class: "flex-shrink-0",
                            Avatar {
                                name: name.clone(),
                                variant: AvatarVariant::Portrait,
                                size: Some("w-16".to_string())
                            }
                        }
                        div { class: "flex-1 min-w-0",
                            MarkdownContent {
                                content: message.content.clone(),
                                class: Some("prose prose-sm max-w-none".to_string()),
                                italic_class: Some("text-gray-500".to_string()),
                                quote_class: Some("text-orange-500".to_string()),
                            }
                        }
                    }
                    // Bottom row with name badge and quick actions
                    div { class: "flex items-center justify-between",
                        // Quick actions on left (same side as avatar)
                        div { class: "flex items-center space-x-1",
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Small,
                                class: "w-6 h-6 p-0".to_string(),
                                onclick: move |_| {
                                    // TODO: Implement branch action
                                },
                                i { class: "fas fa-code-branch text-xs" }
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Small,
                                class: "w-6 h-6 p-0".to_string(),
                                onclick: move |_| {
                                    // TODO: Implement regenerate action
                                },
                                i { class: "fas fa-redo text-xs" }
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Small,
                                class: "w-6 h-6 p-0".to_string(),
                                onclick: move |_| {
                                    // TODO: Implement delete action
                                },
                                i { class: "fas fa-trash text-xs" }
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Small,
                                class: "w-6 h-6 p-0".to_string(),
                                onclick: move |_| {
                                    // TODO: Implement edit action
                                },
                                i { class: "fas fa-edit text-xs" }
                            }
                        }
                        // Name badge on right (opposite side of avatar)
                        Badge {
                            variant: BadgeVariant::Secondary,
                            class: "text-xs".to_string(),
                            "{name}"
                        }
                    }
                }
            }
        },
    }
}