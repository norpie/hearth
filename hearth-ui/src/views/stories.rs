//! Unified responsive stories view

use crate::{
    PageHeader, Platform, Route, SearchContext, UniversalSearch, UniversalSearchState, 
    UniversalSearchQuery, ToastManager, ScrollArea, FadeMode,
    StoryCardComponent, StoryTooltipState,
};
use hearth_core::sample::{sample_character_tags_sorted, sample_scenario_tags_sorted, sample_stories};
use dioxus::prelude::*;

#[component]
pub fn StoriesView(navigate_to: EventHandler<Route>) -> Element {
    let character_tags = sample_character_tags_sorted();
    let available_character_tags: Vec<String> = character_tags
        .iter()
        .take(12)
        .map(|(name, count)| format!("{} ({})", name, count))
        .collect();

    let all_character_tags: Vec<String> = character_tags
        .iter()
        .map(|(name, count)| format!("{} ({})", name, count))
        .collect();

    // Extract scenario names from sample data for search tags
    let scenario_tags = sample_scenario_tags_sorted();
    let available_scenario_tags: Vec<String> = scenario_tags
        .iter()
        .take(12)
        .map(|(name, count)| format!("{} ({})", name, count))
        .collect();

    let all_scenario_tags: Vec<String> = scenario_tags
        .iter()
        .map(|(name, count)| format!("{} ({})", name, count))
        .collect();

    let stories = use_signal(sample_stories);
    let platform = Platform::current();
    
    // Universal search state
    let search_state = use_signal(UniversalSearchState::default);

    // Global tooltip state for all story cards
    let tooltip_state: StoryTooltipState = use_signal(|| None::<(String, usize)>);
    
    // Toast manager for showing search query changes
    let _toast_manager = use_context::<ToastManager>();
    
    // Helper function to convert formatted tag display names back to tag IDs
    let _display_to_id = move |formatted_name: &str| -> String {
        // Find matching tag by display format
        for (name, count) in &character_tags {
            if format!("{} ({})", name, count) == formatted_name {
                return name.to_lowercase()
                    .replace(" ", "_")
                    .replace("-", "_")
                    .chars()
                    .filter(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
            }
        }
        for (name, count) in &scenario_tags {
            if format!("{} ({})", name, count) == formatted_name {
                return name.to_lowercase()
                    .replace(" ", "_")
                    .replace("-", "_")
                    .chars()
                    .filter(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
            }
        }
        // Fallback: extract name from "Name (count)" format and generate ID
        if let Some(name_end) = formatted_name.rfind(" (") {
            let name = &formatted_name[..name_end];
            return name.to_lowercase()
                .replace(" ", "_")
                .replace("-", "_")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '_')
                .collect();
        }
        formatted_name.to_string()
    };

    rsx! {
        div { class: "flex-1 flex flex-col min-h-0",
            // PageHeader inside the flex container
            PageHeader { title: "Stories".to_string(), back_button: None }
            
            // Universal Search/Filter Section
            UniversalSearch {
                context: SearchContext::Stories,
                available_tags: vec![], // Not used for stories
                available_character_tags: Some(available_character_tags),
                available_scenario_tags: Some(available_scenario_tags),
                all_character_tags: Some(all_character_tags),
                all_scenario_tags: Some(all_scenario_tags),
                search_state,
                on_query_change: move |_query: UniversalSearchQuery| {
                    // Handle query changes here if needed
                },
            }
            
            // Scrollable Stories list
            div { class: "flex-1 min-h-0",
                ScrollArea {
                    height: "h-full".to_string(),
                    fade_mode: FadeMode::Both,
                    div { class: if platform.is_mobile() { "px-4 pb-4 pt-3" } else { "px-4 pb-4 max-w-screen-2xl mx-auto" },
                        if !stories().is_empty() {
                            div { class: if platform.is_mobile() { "space-y-3" } else { "grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4" },
                                for story in stories() {
                                    StoryCardComponent {
                                        story,
                                        tooltip_state,
                                        on_select: move |id| {
                                            navigate_to.call(Route::Story { story_id: id });
                                        },
                                    }
                                }
                            }
                        } else {
                            // Empty state
                            div { class: if platform.is_mobile() { "flex-1 flex items-center justify-center p-8" } else { "flex items-center justify-center py-16" },
                                div { class: "text-center",
                                    div { class: "text-6xl mb-4",
                                        i { class: "fa-solid fa-book text-muted-foreground" }
                                    }
                                    h3 { class: "text-lg font-medium text-foreground mb-2",
                                        "No stories yet"
                                    }
                                    p { class: if platform.is_mobile() { "text-muted-foreground mb-6" } else { "text-muted-foreground mb-6 max-w-md" },
                                        "Start a new story with a character to begin your adventure. Choose from our library or create your own."
                                    }
                                    button {
                                        class: "px-6 py-3 bg-primary text-primary-foreground rounded-lg font-medium hover:bg-primary/90",
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
    }
}

