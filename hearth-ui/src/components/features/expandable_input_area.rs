//! Character selection menu component for story input

use crate::{Avatar, AvatarVariant, Input, InputVariant, Button, ButtonVariant, ButtonSize, ScrollArea, ScrollOrientation, FadeMode, CharacterOption};
use dioxus::prelude::*;

#[component]
pub fn ExpandableInputArea(
    // Input props
    current_message: String,
    on_input_change: EventHandler<String>,
    on_send: EventHandler<()>,
    send_disabled: bool,
    // Character selection props
    is_expanded: bool,
    characters: Vec<CharacterOption>,
    selected_character_id: Option<String>,
    character_goals: std::collections::HashMap<String, String>,
    on_character_select: EventHandler<String>,
    on_goal_change: EventHandler<(String, String)>,
    on_toggle: EventHandler<()>,
    // Platform detection for desktop click handler
    #[props(default = false)]
    enable_desktop_click: bool,
    // Mobile navbar visibility for positioning
    #[props(default = false)]
    navbar_visible: bool,
) -> Element {
    let selected_character = characters.iter()
        .find(|c| Some(c.id.clone()) == selected_character_id)
        .cloned();

    let current_goal = selected_character_id.as_ref()
        .and_then(|id| character_goals.get(id))
        .cloned()
        .unwrap_or_default();

    // Calculate transform based on expanded state
    // Parent container already handles navbar offset with pb-20 padding
    // When closed: show only input area (4rem height)
    // When open: show full container
    let transform_class = if is_expanded {
        "translate-y-0"
    } else {
        "translate-y-[calc(100%-4rem)]"   // Always just show 4rem of input
    };

    rsx! {
        // Main sliding container - absolute positioned to respect container boundaries
        div { 
            class: format!(
                "absolute bottom-0 left-0 right-0 z-40 transform transition-transform duration-300 ease-in-out {}",
                transform_class
            ),
            
            // Content wrapper with max height constraint
            div { 
                class: "bg-background border-t border-border shadow-lg",
                style: "max-height: 50vh; display: flex; flex-direction: column;",
                
                // Input area - always at top, always visible
                div { 
                    class: "p-3 border-b border-border bg-background",
                    
                    // Desktop click area for toggling
                    if enable_desktop_click {
                        div {
                            class: "absolute top-0 left-0 right-0 h-3 cursor-ns-resize",
                            onclick: move |_| on_toggle.call(()),
                        }
                    }
                    
                    div { 
                        class: "flex items-end space-x-2",
                        
                        // Image attachment button
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Small,
                            onclick: move |_| {
                                // TODO: Implement image attachment
                            },
                            class: "flex-shrink-0",
                            i { class: "fas fa-image text-muted-foreground" }
                        }
                        
                        // Input field
                        div { 
                            class: "flex-1",
                            Input {
                                placeholder: "What happens next?",
                                value: current_message,
                                oninput: move |val| on_input_change.call(val),
                                variant: InputVariant::Ghost,
                                class: "border-0 bg-transparent resize-none focus:ring-0 p-2",
                            }
                        }
                        
                        // Send button
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Small,
                            onclick: move |_| on_send.call(()),
                            disabled: send_disabled,
                            class: "flex-shrink-0",
                            i { 
                                class: if send_disabled { 
                                    "fas fa-paper-plane text-muted-foreground" 
                                } else { 
                                    "fas fa-paper-plane text-primary" 
                                }
                            }
                        }
                    }
                }
                
                // Character selection area - below input, scrollable
                div { 
                    class: "flex-1 overflow-hidden flex flex-col",
                    
                    // Character selection section
                    div { 
                        class: "p-3 pb-2",
                        
                        // Horizontal scroll for characters
                        ScrollArea {
                            orientation: ScrollOrientation::Horizontal,
                            height: "80px".to_string(),
                            width: "100%".to_string(),
                            fade_mode: FadeMode::Both,
                            fade_color: Some("from-background".to_string()),
                            boundary_tolerance: 10,
                            
                            div { 
                                class: "flex space-x-3 pb-2 w-max",
                                
                                for character in characters {
                                    div {
                                        class: format!(
                                            "flex-shrink-0 flex flex-col items-center space-y-2 p-2 rounded-lg cursor-pointer transition-colors {}",
                                            if Some(character.id.clone()) == selected_character_id {
                                                "bg-primary/10 border border-primary/20"
                                            } else {
                                                "hover:bg-muted"
                                            }
                                        ),
                                        onclick: {
                                            let character_id = character.id.clone();
                                            move |_| on_character_select.call(character_id.clone())
                                        },
                                        
                                        // Avatar - treat narrator as normal character
                                        Avatar {
                                            name: character.name.clone(),
                                            avatar_url: character.avatar_url.clone(),
                                            variant: AvatarVariant::Portrait,
                                            size: Some("w-12 h-12".to_string())
                                        }
                                        
                                        // Name
                                        span { 
                                            class: format!(
                                                "text-xs text-center max-w-16 truncate {}",
                                                if Some(character.id.clone()) == selected_character_id {
                                                    "text-primary font-medium"
                                                } else {
                                                    "text-foreground"
                                                }
                                            ),
                                            "{character.name}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // Goal input for selected character
                    if let Some(selected) = selected_character {
                        div { 
                            class: "px-3 pb-6",
                            Input {
                                placeholder: format!("What does {} want to achieve?", selected.name),
                                value: current_goal,
                                oninput: {
                                    let character_id = selected.id.clone();
                                    move |val| on_goal_change.call((character_id.clone(), val))
                                },
                                variant: InputVariant::Default,
                                class: "text-sm",
                            }
                        }
                    }
                }
            }
        }
    }
}