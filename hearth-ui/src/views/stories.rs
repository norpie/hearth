//! Unified responsive stories view

use dioxus::prelude::*;
use crate::{StoryCard, PageHeader, Route, UniversalSearch, SearchContext, sample_stories, sample_character_tags_sorted, Platform};

#[component]
pub fn StoriesView(navigate_to: EventHandler<Route>) -> Element {
    let available_character_tags: Vec<String> = sample_character_tags_sorted()
        .into_iter()
        .take(12)
        .map(|(tag, count)| format!("{tag} ({count})"))
        .collect();
    
    let all_character_tags: Vec<String> = sample_character_tags_sorted()
        .into_iter()
        .map(|(tag, count)| format!("{tag} ({count})"))
        .collect();
    
    // Extract scenario names from sample data for search tags  
    let available_scenario_tags: Vec<String> = crate::sample_scenario_tags_sorted()
        .into_iter()
        .take(12)
        .map(|(tag, count)| format!("{tag} ({count})"))
        .collect();
        
    let all_scenario_tags: Vec<String> = crate::sample_scenario_tags_sorted()
        .into_iter()
        .map(|(tag, count)| format!("{tag} ({count})"))
        .collect();
    
    let stories = use_signal(sample_stories);
    let platform = Platform::current();
    
    // Global tooltip state for all story cards
    let tooltip_state = use_signal(|| None::<(String, usize)>);

    rsx! {
        PageHeader { title: "Stories".to_string(), back_button: None }
        // Universal Search/Filter Section
        UniversalSearch {
            context: SearchContext::Stories,
            available_tags: vec![], // Not used for stories
            available_character_tags: Some(available_character_tags),
            available_scenario_tags: Some(available_scenario_tags),
            all_character_tags: Some(all_character_tags),
            all_scenario_tags: Some(all_scenario_tags),
        }
        // Stories list with responsive design
        div { class: "flex-1 overflow-y-auto",
            div { class: if platform.is_mobile() { "px-4 pb-4 pt-3" } else { "px-4 pb-4 max-w-screen-2xl mx-auto" },
                if !stories().is_empty() {
                    div { class: if platform.is_mobile() { "space-y-3" } else { "grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4" },
                        for story in stories() {
                            StoryCard {
                                story,
                                tooltip_state,
                                on_select: move |id| {
                                    navigate_to.call(Route::Chat { story_id: id });
                                },
                            }
                        }
                    }
                } else {
                    // Empty state
                    div { class: if platform.is_mobile() { "flex-1 flex items-center justify-center p-8" } else { "flex items-center justify-center py-16" },
                        div { class: "text-center",
                            div { class: "text-6xl mb-4",
                                i { class: "fa-solid fa-book text-gray-400" }
                            }
                            h3 { class: "text-lg font-medium text-gray-900 dark:text-gray-100 mb-2",
                                "No stories yet"
                            }
                            p { class: if platform.is_mobile() { "text-gray-500 mb-6" } else { "text-gray-500 mb-6 max-w-md" },
                                "Start a new story with a character to begin your adventure. Choose from our library or create your own."
                            }
                            button {
                                class: "px-6 py-3 bg-purple-600 text-white rounded-lg font-medium hover:bg-purple-700 dark:bg-purple-500 dark:hover:bg-purple-600",
                                onclick: move |_| navigate_to.call(Route::Characters),
                                "Browse Characters"
                            }
                        }
                    }
                }
            }
        }
    }
}