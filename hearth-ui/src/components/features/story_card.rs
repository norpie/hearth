//! Story card component for displaying story items

use crate::{Card, CardHeader, CardTitle, CardContent, CardFooter, Avatar, AvatarGroup, AvatarData, Badge, BadgeVariant};
use hearth_core::models::StoryItem;
use dioxus::prelude::*;

// Type alias for tooltip state - (story_id, character_index)
pub type StoryTooltipState = Signal<Option<(String, usize)>>;

#[component]
pub fn StoryCardComponent(
    story: StoryItem,
    tooltip_state: StoryTooltipState,
    on_select: EventHandler<String>,
) -> Element {
    let character_count = story.characters.len();
    let story_id = story.id.clone();

    // Check if any tooltip is open for this story
    let has_open_tooltip =
        tooltip_state().is_some_and(|(open_story_id, _)| open_story_id == story_id);

    rsx! {
        div {
            class: format!(
                "cursor-pointer transition-shadow {}",
                if has_open_tooltip { "" } else { "hover:shadow-md" },
            ),
            onclick: {
                let mut tooltip_state = tooltip_state;
                move |_| {
                    // Hide tooltip when clicking on story card
                    tooltip_state.set(None);
                    on_select.call(story.id.clone())
                }
            },
            Card {
            
            CardHeader {
                class: "pb-4".to_string(),
                div { class: "flex items-center justify-between",
                    CardTitle {
                        class: "text-lg truncate".to_string(),
                        "{story.title}"
                    }
                    span { class: "text-sm text-muted-foreground flex-shrink-0", "{story.timestamp}" }
                }
            }

            CardContent {
                class: "space-y-4".to_string(),
                
                // Participant avatars section
                div { class: "flex items-center",
                    // User character avatar (if present)
                    if let Some(user_character) = &story.user_character {
                        div {
                            class: "flex-shrink-0 mr-1 relative cursor-pointer",
                            onmouseenter: {
                                let mut tooltip_state = tooltip_state;
                                let story_id = story_id.clone();
                                move |_| {
                                    tooltip_state.set(Some((story_id.clone(), 998))); // Use 998 as special index for user
                                }
                            },
                            onmouseleave: {
                                let mut tooltip_state = tooltip_state;
                                move |_| {
                                    tooltip_state.set(None);
                                }
                            },
                            onclick: {
                                let mut tooltip_state = tooltip_state;
                                let story_id = story_id.clone();
                                move |e: Event<MouseData>| {
                                    e.stop_propagation();
                                    // Toggle tooltip for mobile
                                    match tooltip_state() {
                                        Some(
                                            (open_story_id, open_index),
                                        ) if open_story_id == story_id && open_index == 998 => {
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
                                size: "w-10 h-10".to_string(),
                            }
                        }
                    } else {
                        // Fallback for stories without user character
                        div { class: "flex-shrink-0 mr-1",
                            Avatar {
                                name: "You".to_string(),
                                avatar_url: None,
                                size: "w-10 h-10".to_string(),
                            }
                        }
                    }
                    // Separator
                    div { class: "text-muted-foreground mx-1.5", "—" }
                    // Character avatars
                    AvatarGroup {
                        avatars: story.characters.iter().map(|c| AvatarData {
                            name: c.name.clone(),
                            avatar_url: c.avatar_url.clone(),
                        }).collect(),
                        max_visible: 3,
                        size: "w-10 h-10".to_string(),
                    }
                }

            }

            CardFooter {
                class: "justify-between pt-4".to_string(),
                // Left side: scenario info
                if let Some(scenario) = &story.scenario_name {
                    div { class: "flex items-center space-x-1",
                        Badge {
                            variant: BadgeVariant::Secondary,
                            class: "text-xs".to_string(),
                            i { class: "fa-solid fa-map mr-1" }
                            "{scenario}"
                        }
                    }
                } else {
                    div {}
                }
                
                // Right side: stats
                div { class: "flex items-center space-x-3 text-xs text-muted-foreground",
                    span { class: "flex items-center space-x-1",
                        i { class: "fa-solid fa-message" }
                        span { "{story.message_count}" }
                    }
                    if character_count > 1 {
                        span { class: "flex items-center space-x-1",
                            i { class: "fa-solid fa-users" }
                            span { "{character_count}" }
                        }
                    }
                }
            }
            }
        }
    }
}