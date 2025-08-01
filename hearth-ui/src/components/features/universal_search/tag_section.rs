//! Tag filtering section component with 3-state selection

use super::{Tag, TagState};
use crate::{Label, Input, InputVariant, Button, ButtonVariant, ButtonSize};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Props, Clone, PartialEq)]
pub struct TagSectionProps {
    pub label: String,
    pub placeholder: String,
    pub tags: Vec<String>,
    pub tag_states: HashMap<String, TagState>,
    pub on_tag_cycle: EventHandler<String>,
    pub on_show_all: EventHandler<()>,
}

#[component]
pub fn TagSection(props: TagSectionProps) -> Element {
    let mut tag_search = use_signal(String::new);
    
    // Filter tags based on search
    let search_value = tag_search();
    let filtered_tags: Vec<String> = props.tags
        .iter()
        .filter(|tag| {
            search_value.is_empty() || 
            tag.to_lowercase().contains(&search_value.to_lowercase())
        })
        .take(8) // Show only first 8 tags
        .cloned()
        .collect();

    rsx! {
        div {
            Label { "{props.label}" }
            
            // Tag search input
            Input {
                variant: InputVariant::Default,
                placeholder: props.placeholder,
                value: tag_search(),
                oninput: move |value: String| tag_search.set(value),
                class: "mb-3".to_string(),
            }
            
            // Tag container
            div { class: "flex flex-wrap gap-2 mb-2",
                for tag in &filtered_tags {
                    Tag { 
                        text: tag.clone(),
                        state: props.tag_states.get(tag).cloned().unwrap_or_default(),
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
            
            // Show more button
            Button {
                variant: ButtonVariant::Ghost,
                size: ButtonSize::Small,
                onclick: move |_| props.on_show_all.call(()),
                "Show all tags..."
            }
        }
    }
}
