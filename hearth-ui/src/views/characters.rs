//! Unified responsive characters view

use dioxus::prelude::*;
use crate::{ItemCard, PageHeader, Route, UniversalSearch, SearchContext, sample_characters, sample_character_tags_sorted, Platform, models::CardMetadata};

#[component]
pub fn CharactersView(navigate_to: EventHandler<Route>) -> Element {
    let available_tags: Vec<String> = sample_character_tags_sorted()
        .into_iter()
        .take(12)
        .map(|(tag, count)| format!("{tag} ({count})"))
        .collect();
        
    let all_tags: Vec<String> = sample_character_tags_sorted()
        .into_iter()
        .map(|(tag, count)| format!("{tag} ({count})"))
        .collect();
    let mut characters = use_signal(sample_characters);
    let platform = Platform::current();

    rsx! {
        PageHeader { title: "Characters".to_string(), back_button: None }
        // Universal Search/Filter Section
        UniversalSearch {
            context: SearchContext::Characters,
            available_tags,
            all_tags,
        }
        // Characters display with responsive design
        div { class: "flex-1 overflow-y-auto",
            div { class: if platform.is_mobile() { "px-4 pb-4 pt-3" } else { "px-4 pb-4 max-w-screen-2xl mx-auto" },
                div { class: if platform.is_mobile() { "space-y-3" } else { "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4" },
                    for character in &characters() {
                        ItemCard {
                            item: character.clone(),
                            metadata: vec![CardMetadata { icon: "fa-solid fa-book", count: character.story_count, label: "stories" }],
                            on_select: move |id| println!("Selected character: {id}"),
                            on_favorite: move |id| {
                                let mut chars = characters();
                                if let Some(char) = chars.iter_mut().find(|c| c.id == id) {
                                    char.is_favorite = !char.is_favorite;
                                }
                                characters.set(chars);
                            },
                        }
                    }
                }
            }
        }
    }
}