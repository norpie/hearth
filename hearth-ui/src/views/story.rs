//! Story view - Interactive storytelling interface

use crate::{PageHeader, Platform, Route, components::*, ScrollArea, ScrollControl, ScrollAction, FadeMode, GestureDetector, GestureDirection, ToastManager, MobileNavbarContext, StoryMessage, StoryRole, StoryMessageComponent, ExpandableInputArea, CharacterOption, StoryManagementMenu};
use hearth_core::sample::sample_stories;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn StoryView(story_id: String, navigate_to: EventHandler<Route>) -> Element {
    let mut current_message = use_signal(String::new);
    let mut story_messages = use_signal(Vec::<StoryMessage>::new);
    let mut is_typing = use_signal(|| false);
    let mut show_character_menu = use_signal(|| false);
    let mut show_story_menu = use_signal(|| false);
    let mut selected_character_id = use_signal(|| Some("narrator".to_string()));
    let mut character_goals = use_signal(HashMap::<String, String>::new);
    let scroll_controller = use_signal(|| None::<ScrollAction>);
    let platform = Platform::current();
    
    // Load story data to get user character info
    let story_data = sample_stories().into_iter().find(|s| s.id == story_id);
    let user_name = story_data
        .as_ref()
        .and_then(|s| s.user_character.as_ref())
        .map(|uc| uc.name.clone())
        .unwrap_or_else(|| "You".to_string());
    
    // Get the story title from the story data
    let story_title = story_data
        .as_ref()
        .map(|s| s.title.clone())
        .unwrap_or_else(|| story_id.clone());
    
    // Toast manager for input bar gesture feedback
    let _toast_manager = use_context::<ToastManager>();
    
    // Try to get mobile navbar context - only available on mobile
    let mobile_navbar_ctx = try_use_context::<MobileNavbarContext>();
    
    // Get navbar visibility state for input positioning
    let navbar_visible = mobile_navbar_ctx
        .as_ref()
        .map(|ctx| (ctx.is_visible)())
        .unwrap_or(false);
    
    // Sample character options for the selection menu
    let character_options = vec![
        CharacterOption {
            id: "narrator".to_string(),
            name: "Narrator".to_string(),
            avatar_url: None,
            is_narrator: true,
        },
        CharacterOption {
            id: "user".to_string(),
            name: user_name.clone(),
            avatar_url: None,
            is_narrator: false,
        },
        CharacterOption {
            id: "forest_guide".to_string(),
            name: "Forest Guide".to_string(),
            avatar_url: None,
            is_narrator: false,
        },
        CharacterOption {
            id: "mysterious_stranger".to_string(),
            name: "Mysterious Stranger".to_string(),
            avatar_url: None,
            is_narrator: false,
        },
        CharacterOption {
            id: "village_elder".to_string(),
            name: "Village Elder".to_string(),
            avatar_url: None,
            is_narrator: false,
        },
    ];
    
    // Handle gesture events for input bar
    let handle_input_gesture = move |direction: GestureDirection| {
        match direction {
            GestureDirection::Up => {
                // Toggle character selection menu
                let was_open = show_character_menu();
                show_character_menu.set(!was_open);
                
                // Scroll down to reveal character menu when opening
                if !was_open {
                    use crate::ScrollDelta;
                    ScrollControl::scroll_by(scroll_controller, ScrollDelta::smooth(0, 268)); // ~268px is character menu height with smooth animation
                }
                
            },
            GestureDirection::Down => {
                // Close character selection menu with down swipe
                if show_character_menu() {
                    show_character_menu.set(false);
                }
            },
            GestureDirection::Right => {
                // Show navbar if we're on mobile and have the context
                if let Some(mut ctx) = mobile_navbar_ctx {
                    if !(ctx.is_visible)() {
                        ctx.is_visible.set(true);
                    }
                } else {
                    // Fallback for non-mobile or when context is not available
                }
            },
            _ => {
                // Ignore other gestures on input bar
            }
        }
    };
    
    // Add some sample messages on first render
    use_effect({
        let user_name = user_name.clone();
        move || {
            let sample_messages = vec![
                StoryMessage {
                    id: "1".to_string(),
                    role: StoryRole::Narrator,
                    content: "You find yourself standing at the edge of an ancient forest. The towering trees whisper secrets in the wind, and a narrow path winds deeper into the shadows.".to_string(),
                },
                StoryMessage {
                    id: "2".to_string(),
                    role: StoryRole::User { name: user_name.clone() },
                    content: "*I step carefully onto the forest path, scanning the ground for tracks while keeping my hand near my weapon* This place feels alive... I need to stay alert.".to_string(),
                },
                StoryMessage {
                    id: "3".to_string(),
                    role: StoryRole::Character { name: "Forest Guide".to_string() },
                    content: "*An elderly woman emerges from the bushes, her walking stick tapping against the ground as she approaches* Wait, traveler! That path leads to the Heart of the Wilds. Are you certain you're prepared for such a journey?".to_string(),
                },
                StoryMessage {
                    id: "4".to_string(),
                    role: StoryRole::User { name: user_name.clone() },
                    content: "*I think to myself \"Should I trust this stranger?\" before responding carefully* What dangers should I be aware of? Do you have any advice for a traveler like myself?".to_string(),
                },
                StoryMessage {
                    id: "5".to_string(),
                    role: StoryRole::Character { name: "Forest Guide".to_string() },
                    content: "*She leans heavily on her gnarled staff and points toward the dark path ahead* Many have ventured into those depths, young one. The forest itself is alive, and it does not welcome intruders. Trust the silver moonlight, and beware the whispering stones.".to_string(),
                },
                StoryMessage {
                    id: "6".to_string(),
                    role: StoryRole::Narrator,
                    content: "As the old woman's words fade into the forest air, a sudden chill runs down your spine. The wind picks up, rustling the leaves overhead, and somewhere in the distance you hear the haunting call of an unknown creature.".to_string(),
                },
                StoryMessage {
                    id: "7".to_string(),
                    role: StoryRole::User { name: user_name.clone() },
                    content: "*I remember what my mentor always said \"Knowledge is the best weapon\" and decide to heed her advice* Thank you for the warning. I'll be careful and watch for the silver moonlight.".to_string(),
                },
            ];
            story_messages.set(sample_messages);
            // Auto-scroll to bottom when messages are loaded
            ScrollControl::scroll_to_bottom(scroll_controller);
        }
    });
    
    // Auto-scroll to bottom when new messages are added
    use_effect({
        let story_messages = story_messages();
        move || {
            if !story_messages.is_empty() {
                ScrollControl::scroll_to_bottom(scroll_controller);
            }
        }
    });
    
    rsx! {
        div { class: "flex-1 flex flex-col min-h-0 relative",
            // PageHeader with expandable story management menu
            PageHeader { 
                title: story_title.clone(),
                back_button: if platform.is_mobile() {
                    Some(rsx! {
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Small,
                            onclick: move |_| navigate_to.call(Route::Stories),
                            i { class: "fas fa-arrow-left" }
                        }
                    })
                } else {
                    None
                },
                expandable_content: Some(rsx! {
                    StoryManagementMenu {
                        on_new_story: move |_| {
                            // TODO: Implement new story
                        },
                        on_export: move |_| {
                            // TODO: Implement export
                        },
                        on_settings: move |_| {
                            // TODO: Implement settings
                        },
                    }
                }),
                expanded: Some(show_story_menu()),
                on_expanded_change: Some(EventHandler::new(move |expanded: bool| {
                    show_story_menu.set(expanded);
                })),
                enable_desktop_click: !platform.is_mobile(),
            }
            
            // Scrollable Story content
            div { class: "flex-1 min-h-0",
                ScrollArea {
                    height: "h-full".to_string(),
                    fade_mode: FadeMode::Both,
                    scroll_controller_signal: Some(scroll_controller),
                    div { 
                        class: format!(
                            "p-3 space-y-1 {}",
                            if show_character_menu() {
                                "mb-72"      // Account for expanded character menu (~268px content)
                            } else {
                                "mb-16"      // Account for closed input area (~64px)
                            }
                        ),
                        for message in story_messages().iter() {
                            StoryMessageComponent { message: message.clone() }
                        }
                        if is_typing() {
                            div { class: "flex items-center space-x-2 text-muted-foreground",
                                div { class: "w-4 h-4 border-2 border-muted-foreground border-t-transparent rounded-full animate-spin" }
                                span { "Story is continuing..." }
                            }
                        }
                    }
                }
            }
            
            // Container for input area with relative positioning to contain absolute positioned elements
            div { class: "relative",
                // Expandable input area with character selection
                GestureDetector {
                    debug: false,
                    class: "relative".to_string(),
                    on_gesture: handle_input_gesture,
                    
                    ExpandableInputArea {
                        // Input props
                        current_message: current_message(),
                        on_input_change: move |val| current_message.set(val),
                        on_send: {
                            let user_name = user_name.clone();
                            move |_| {
                                if !current_message().trim().is_empty() {
                                    let user_msg = StoryMessage {
                                        id: format!("msg_{}", story_messages().len()),
                                        role: StoryRole::User { name: user_name.clone() },
                                        content: current_message(),
                                    };
                                    story_messages.with_mut(|msgs| msgs.push(user_msg));
                                    current_message.set(String::new());
                                    is_typing.set(true);
                                    
                                    // Auto-scroll after user message
                                    ScrollControl::scroll_to_bottom(scroll_controller);
                                    
                                    // Simulate immediate story response for now
                                    let story_msg = StoryMessage {
                                        id: format!("msg_{}", story_messages().len()),
                                        role: StoryRole::Narrator,
                                        content: "The story continues with your choice, weaving new possibilities into the narrative thread...".to_string(),
                                    };
                                    story_messages.with_mut(|msgs| msgs.push(story_msg));
                                    is_typing.set(false);
                                    
                                    // Auto-scroll after story response
                                    ScrollControl::scroll_to_bottom(scroll_controller);
                                }
                            }
                        },
                        send_disabled: current_message().trim().is_empty(),
                        // Character selection props
                        is_expanded: show_character_menu(),
                        characters: character_options.clone(),
                        selected_character_id: selected_character_id(),
                        character_goals: character_goals(),
                        on_character_select: move |id| {
                            selected_character_id.set(Some(id));
                        },
                        on_goal_change: move |(char_id, goal)| {
                            character_goals.with_mut(|goals| {
                                goals.insert(char_id, goal);
                            });
                        },
                        on_toggle: move |_| {
                            let was_open = show_character_menu();
                            show_character_menu.set(!was_open);
                            
                            // Scroll down to reveal character menu when opening
                            if !was_open {
                                use crate::ScrollDelta;
                                ScrollControl::scroll_by(scroll_controller, ScrollDelta::smooth(0, 268)); // ~268px is character menu height with smooth animation
                            }
                        },
                        enable_desktop_click: !platform.is_mobile(),
                        navbar_visible,
                    }
                }
            }
        }
    }
}
