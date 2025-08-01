//! Character avatar group component for displaying multiple character avatars with tooltips

use dioxus::prelude::*;
use crate::{Avatar, Platform};
use hearth_core::StoryParticipant;

#[derive(Props, Clone, PartialEq)]
pub struct CharacterAvatarGroupProps {
    pub characters: Vec<StoryParticipant>,
    pub story_id: String,
    pub tooltip_state: Signal<Option<(String, usize)>>,
}

#[component]
pub fn CharacterAvatarGroup(props: CharacterAvatarGroupProps) -> Element {
    let character_count = props.characters.len();
    let platform = Platform::current();
    
    rsx! {
        div { class: "flex items-center -space-x-2",
            // Show first 3 characters
            for (index, character) in props.characters.iter().take(3).enumerate() {
                div { 
                    class: format!("relative cursor-pointer {}", match index {
                        0 => "z-10",
                        1 => "z-20", 
                        _ => "z-30"
                    }),
                    style: if index > 0 { format!("margin-left: -8px;") } else { String::new() },
                    onmouseenter: {
                        let mut tooltip_state = props.tooltip_state.clone();
                        let story_id = props.story_id.clone();
                        move |_| {
                            if !platform.is_mobile() {
                                tooltip_state.set(Some((story_id.clone(), index)));
                            }
                        }
                    },
                    onmouseleave: {
                        let mut tooltip_state = props.tooltip_state.clone();
                        move |_| {
                            if !platform.is_mobile() {
                                tooltip_state.set(None);
                            }
                        }
                    },
                    onclick: {
                        let mut tooltip_state = props.tooltip_state.clone();
                        let story_id = props.story_id.clone();
                        move |e: Event<MouseData>| {
                            e.stop_propagation();
                            if platform.is_mobile() {
                                // Use click event for mobile - toggle tooltip
                                match tooltip_state() {
                                    Some((open_story_id, open_index)) if open_story_id == story_id && open_index == index => {
                                        tooltip_state.set(None);
                                    }
                                    _ => {
                                        tooltip_state.set(Some((story_id.clone(), index)));
                                    }
                                }
                            }
                        }
                    },
                    if let Some(url) = &character.avatar_url {
                        img {
                            class: "w-10 h-10 rounded-full object-cover border-2 border-white dark:border-gray-800 cursor-pointer select-none touch-manipulation",
                            style: "user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
                            src: "{url}",
                            alt: "{character.name}",
                        }
                    } else {
                        div {
                            class: "w-10 h-10 rounded-full bg-blue-600 dark:bg-blue-500 border-2 border-white dark:border-gray-800 flex items-center justify-center text-white font-medium text-sm cursor-pointer select-none touch-manipulation",
                            style: "user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
                            "{character.name.chars().next().unwrap_or('?').to_uppercase()}"
                        }
                    }
                    // Show tooltip for this character
                    if let Some((open_story_id, open_index)) = (props.tooltip_state)() {
                        if open_story_id == props.story_id && open_index == index {
                            div {
                                class: "absolute z-[99999] pointer-events-none",
                                style: "top: calc(100% + 12px); left: 50%; transform: translateX(-50%);",
                                
                                // Container that holds both arrow and content
                                div {
                                    class: "relative flex flex-col items-center",
                                    
                                    // Arrow
                                    div {
                                        class: "absolute -top-[6px] z-10",
                                        
                                        // Single arrow that matches tooltip background exactly
                                        div {
                                            class: "w-0 h-0 border-l-[6px] border-r-[6px] border-b-[6px] border-l-transparent border-r-transparent border-b-white dark:border-b-gray-800",
                                        }
                                    }
                                    
                                    // Tooltip content box
                                    div {
                                        class: if platform.is_mobile() {
                                            "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-600 rounded-lg shadow-lg p-4"
                                        } else {
                                            "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-600 rounded-lg shadow-lg p-3"
                                        },
                                        style: "width: 280px;",
                                        
                                        // Character info
                                        div { class: "flex items-start space-x-3",
                                            // Use existing Avatar component
                                            Avatar {
                                                name: character.name.clone(),
                                                avatar_url: character.avatar_url.clone(),
                                                size: "w-12 h-12".to_string()
                                            }
                                            
                                            // Character details
                                            div { class: "flex-1 min-w-0",
                                                h3 { class: "font-medium text-gray-900 dark:text-gray-100 text-sm truncate",
                                                    "{character.name}"
                                                }
                                                p { class: "text-xs text-gray-500 dark:text-gray-400 mt-1",
                                                    "AI Character"
                                                }
                                                if platform.is_mobile() {
                                                    p { class: "text-xs text-gray-400 dark:text-gray-500 mt-2",
                                                        "Tap avatar for details"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Show count if more than 3
            if character_count > 3 {
                div { 
                    class: "relative z-40 cursor-pointer",
                    style: "margin-left: -8px;",
                    onmouseenter: {
                        let mut tooltip_state = props.tooltip_state.clone();
                        let story_id = props.story_id.clone();
                        move |_| {
                            if !platform.is_mobile() {
                                // Use a special index (999) to indicate the count circle tooltip
                                tooltip_state.set(Some((story_id.clone(), 999)));
                            }
                        }
                    },
                    onmouseleave: {
                        let mut tooltip_state = props.tooltip_state.clone();
                        move |_| {
                            if !platform.is_mobile() {
                                tooltip_state.set(None);
                            }
                        }
                    },
                    onclick: {
                        let mut tooltip_state = props.tooltip_state.clone();
                        let story_id = props.story_id.clone();
                        move |e: Event<MouseData>| {
                            e.stop_propagation();
                            if platform.is_mobile() {
                                // Use click event for mobile - toggle tooltip
                                match tooltip_state() {
                                    Some((open_story_id, open_index)) if open_story_id == story_id && open_index == 999 => {
                                        tooltip_state.set(None);
                                    }
                                    _ => {
                                        tooltip_state.set(Some((story_id.clone(), 999)));
                                    }
                                }
                            }
                        }
                    },
                    div { class: "w-10 h-10 rounded-full bg-blue-600 dark:bg-blue-500 border-2 border-white dark:border-gray-800 flex items-center justify-center text-white text-xs font-medium select-none touch-manipulation",
                        style: "user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
                        "+{character_count - 3}"
                    }
                }
            }
        }
    }
}