//! Tag modal component for showing all available tags with 3-state selection

use super::{Tag, TagState};
use crate::{Modal, Input, InputVariant, ScrollArea, ScrollOrientation, FadeMode};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Props, Clone, PartialEq)]
pub struct TagModalProps {
    pub title: String,
    pub tags: Vec<String>,
    pub is_open: Signal<bool>,
    pub search_query: Signal<String>,
    pub tag_states: HashMap<String, TagState>,
    pub on_tag_cycle: EventHandler<String>,
}

#[component]
pub fn TagModal(mut props: TagModalProps) -> Element {
    if !(props.is_open)() {
        return rsx! { div {} };
    }

    let search_query_value = (props.search_query)();
    let filtered_tags: Vec<String> = if search_query_value.is_empty() {
        props.tags.clone()
    } else {
        props
            .tags
            .iter()
            .filter(|tag| {
                tag.to_lowercase()
                    .contains(&search_query_value.to_lowercase())
            })
            .cloned()
            .collect()
    };

    rsx! {
        Modal { 
            title: props.title.clone(), 
            is_open: props.is_open,
            
            div { class: "flex flex-col h-full",
                // Search input
                div { class: "flex-shrink-0 p-4 border-b border-border",
                    Input {
                        variant: InputVariant::Default,
                        placeholder: "Search tags...".to_string(),
                        value: (props.search_query)(),
                        oninput: move |value: String| props.search_query.set(value),
                    }
                }
                
                // Tags list
                div { class: "flex-1 min-h-0",
                    ScrollArea {
                        orientation: ScrollOrientation::Vertical,
                        height: "h-[50vh]".to_string(),
                        fade_mode: FadeMode::Both,
                        fade_color: Some("from-card".to_string()),
                        class: "p-4",
                        if filtered_tags.is_empty() {
                            div { class: "flex items-center justify-center h-32 text-center text-muted-foreground",
                                "No tags found matching your search."
                            }
                        } else {
                            div { class: "flex flex-wrap gap-2 justify-evenly",
                                for tag in filtered_tags {
                                    Tag {
                                        text: tag.clone(),
                                        state: props.tag_states.get(&tag).cloned().unwrap_or_default(),
                                        onclick: {
                                            let tag = tag.clone();
                                            let on_tag_cycle = props.on_tag_cycle.clone();
                                            EventHandler::new(move |_| {
                                                on_tag_cycle.call(tag.clone());
                                            })
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
