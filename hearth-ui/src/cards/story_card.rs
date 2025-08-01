//! Story card component for group conversations

use dioxus::prelude::*;
use crate::{models::StoryItem, Avatar, CharacterAvatarGroup};

// Type alias for tooltip state - (story_id, character_index)
pub type TooltipState = Signal<Option<(String, usize)>>;

#[component]
pub fn StoryCard(
    story: StoryItem,
    tooltip_state: Signal<Option<(String, usize)>>,
    on_select: EventHandler<String>,
) -> Element {
    let character_count = story.characters.len();
    
    // Clone values needed in closures
    let story_id = story.id.clone();
    
    // Check if any tooltip is open for this story
    let has_open_tooltip = tooltip_state().map_or(false, |(open_story_id, _)| open_story_id == story_id);
    
    rsx! {
        button {
            class: format!(
                "w-full p-5 bg-gray-100 dark:bg-gray-900 transition-colors text-left rounded-xl {}",
                if has_open_tooltip { "" } else { "hover:bg-gray-200 dark:hover:bg-gray-800" }
            ),
            onclick: {
                let mut tooltip_state = tooltip_state.clone();
                move |_| {
                    // Hide tooltip when clicking on story card 
                    tooltip_state.set(None);
                    on_select.call(story.id.clone())
                }
            },
            
            // Header with title and timestamp
            div { class: "flex items-center justify-between mb-3",
                h3 { class: "font-medium text-gray-900 dark:text-gray-100 truncate text-lg",
                    "{story.title}"
                }
                span { class: "text-sm text-gray-500 flex-shrink-0", "{story.timestamp}" }
            }
            
            // Participant avatars section
            div { class: "flex items-center mb-3",
                // User character avatar (if present)
                if let Some(user_character) = &story.user_character {
                    div { 
                        class: "flex-shrink-0 mr-1 relative cursor-pointer",
                        onmouseenter: {
                            let mut tooltip_state = tooltip_state.clone();
                            let story_id = story_id.clone();
                            move |_| {
                                tooltip_state.set(Some((story_id.clone(), 998))); // Use 998 as special index for user
                            }
                        },
                        onmouseleave: {
                            let mut tooltip_state = tooltip_state.clone();
                            move |_| {
                                tooltip_state.set(None);
                            }
                        },
                        onclick: {
                            let mut tooltip_state = tooltip_state.clone();
                            let story_id = story_id.clone();
                            move |e: Event<MouseData>| {
                                e.stop_propagation();
                                // Toggle tooltip for mobile
                                match tooltip_state() {
                                    Some((open_story_id, open_index)) if open_story_id == story_id && open_index == 998 => {
                                        tooltip_state.set(None);
                                    }
                                    _ => {
                                        tooltip_state.set(Some((story_id.clone(), 998)));
                                    }
                                }
                            }
                        },
                        Avatar { 
                            name: user_character.name.clone(), 
                            avatar_url: user_character.avatar_url.clone(),
                            size: "w-12 h-12".to_string()
                        }
                    }
                } else {
                    // Fallback for stories without user character
                    div { class: "flex-shrink-0 mr-1",
                        Avatar { 
                            name: "You".to_string(), 
                            avatar_url: None,
                            size: "w-12 h-12".to_string()
                        }
                    }
                }
                
                // Separator
                div { class: "text-gray-400 mx-1.5", "—" }
                
                // Character avatars
                div { class: "flex items-center relative",
                    CharacterAvatarGroup {
                        characters: story.characters.clone(),
                        story_id: story_id.clone(),
                        tooltip_state: tooltip_state
                    }
                }
            }
            
            // Last message
            div { class: "mb-3",
                div { class: "flex items-start space-x-2",
                    span { class: "font-medium text-sm text-gray-700 dark:text-gray-300 flex-shrink-0",
                        "{story.last_speaker}:"
                    }
                    p { class: "text-gray-600 dark:text-gray-400 text-sm line-clamp-2 flex-1",
                        "{story.last_message}"
                    }
                }
            }
            
            // Footer with scenario and message count
            div { class: "flex items-center justify-between text-xs text-gray-500",
                if let Some(scenario) = &story.scenario_name {
                    div { class: "flex items-center space-x-1",
                        i { class: "fa-solid fa-map text-gray-500" }
                        span { class: "truncate", "{scenario}" }
                    }
                } else {
                    div {}
                }
                
                div { class: "flex items-center space-x-3",
                    span { class: "flex items-center space-x-1",
                        i { class: "fa-solid fa-message text-gray-500" }
                        span { "{story.message_count}" }
                    }
                    if character_count > 1 {
                        span { class: "flex items-center space-x-1",
                            i { class: "fa-solid fa-users text-gray-500" }
                            span { "{character_count}" }
                        }
                    }
                }
            }
        }
    }
}