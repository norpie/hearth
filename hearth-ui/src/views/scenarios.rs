//! Unified responsive scenarios view

use crate::{
    sample_scenario_tags_sorted, sample_scenarios, PageHeader, Platform, Route, SearchContext, 
    UniversalSearch, UniversalSearchState, UniversalSearchQuery, ToastManager, ToastType, ToastConfig,
    Card, CardHeader, CardTitle, CardDescription, CardContent, Avatar, Badge, BadgeVariant,
    Button, ButtonVariant, ScrollArea, ScrollOrientation, FadeMode,
};
use hearth_core::models::ScenarioItem;
use std::time::Duration;
use dioxus::prelude::*;

#[component]
pub fn ScenariosView(navigate_to: EventHandler<Route>) -> Element {
    let available_tags: Vec<String> = sample_scenario_tags_sorted()
        .into_iter()
        .take(12)
        .map(|(tag, count)| format!("{tag} ({count})"))
        .collect();

    let all_tags: Vec<String> = sample_scenario_tags_sorted()
        .into_iter()
        .map(|(tag, count)| format!("{tag} ({count})"))
        .collect();
    let mut scenarios = use_signal(sample_scenarios);
    let platform = Platform::current();
    
    // Universal search state
    let search_state = use_signal(UniversalSearchState::default);
    
    // Toast manager for showing search query changes
    let toast_manager = use_context::<ToastManager>();

    rsx! {
        div { class: "flex-1 flex flex-col min-h-0",
            // PageHeader inside the flex container
            PageHeader { title: "Scenarios".to_string(), back_button: None }
            
            // Universal Search/Filter Section
            UniversalSearch {
                context: SearchContext::Scenarios,
                available_tags,
                all_tags,
                search_state,
                on_query_change: move |query: UniversalSearchQuery| {
                    // Show toast with complete query information
                    let query_text = format!(
                        "Scenarios Search Query:\n\
                        Text: '{}'\n\
                        Sort: {} ({})\n\
                        Favorites Only: {}\n\
                        Wanted Character Tags: {:?}\n\
                        Unwanted Character Tags: {:?}\n\
                        Wanted Scenario Tags: {:?}\n\
                        Unwanted Scenario Tags: {:?}",
                        query.search_text,
                        query.sort_option,
                        if query.sort_ascending { "Ascending" } else { "Descending" },
                        query.favorites_only,
                        query.wanted_character_tags,
                        query.unwanted_character_tags,
                        query.wanted_scenario_tags,
                        query.unwanted_scenario_tags
                    );
                    
                    toast_manager.add_toast(ToastConfig {
                        message: query_text,
                        toast_type: ToastType::Info,
                        duration: Some(Duration::from_millis(8000)), // Show for 8 seconds to read the data
                        dismissible: true,
                    });
                },
            }
            
            // Scrollable Scenarios list
            div { class: "flex-1 min-h-0",
                ScrollArea {
                    height: "h-full".to_string(),
                    fade_mode: FadeMode::Both,
                    div { class: if platform.is_mobile() { "px-4 pb-4 pt-3" } else { "px-4 pb-4 max-w-screen-2xl mx-auto" },
                        div { class: if platform.is_mobile() { "space-y-3" } else { "grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4" },
                            for scenario in &scenarios() {
                                ScenarioCard {
                                    scenario: scenario.clone(),
                                    on_select: move |id| println!("Selected scenario: {id}"),
                                    on_favorite: move |id| {
                                        let mut scenarios_vec = scenarios();
                                        if let Some(scenario) = scenarios_vec.iter_mut().find(|s| s.id == id) {
                                            scenario.is_favorite = !scenario.is_favorite;
                                        }
                                        scenarios.set(scenarios_vec);
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ScenarioCard(
    scenario: ScenarioItem,
    on_select: EventHandler<String>,
    on_favorite: EventHandler<String>,
) -> Element {
    let scenario_id = scenario.id.clone();
    let scenario_id_fav = scenario.id.clone();

    rsx! {
        div {
            class: "cursor-pointer hover:shadow-md transition-shadow",
            onclick: move |_| on_select.call(scenario_id.clone()),
            Card {
            
            CardContent {
                class: "space-y-4".to_string(),
                
                // Line 1: Avatar, Name, and Description  
                div { class: "flex items-start space-x-3",
                    Avatar {
                        name: scenario.name.clone(),
                        avatar_url: scenario.avatar_url.clone(),
                        size: "w-12 h-12".to_string(),
                    }
                    div { class: "flex-1 min-w-0",
                        div { class: "flex items-center justify-between mb-1",
                            CardTitle {
                                class: "text-lg truncate".to_string(),
                                "{scenario.name}"
                            }
                            Button {
                                variant: ButtonVariant::Icon,
                                onclick: move |e: MouseEvent| {
                                    e.stop_propagation();
                                    on_favorite.call(scenario_id_fav.clone());
                                },
                                class: "flex-shrink-0".to_string(),
                                if scenario.is_favorite {
                                    i { class: "fas fa-heart text-red-500" }
                                } else {
                                    i { class: "far fa-heart" }
                                }
                            }
                        }
                        CardDescription {
                            class: "text-sm line-clamp-2".to_string(),
                            "{scenario.description}"
                        }
                    }
                }

                // Line 2: Tags
                if !scenario.tags.is_empty() {
                    ScrollArea {
                        orientation: ScrollOrientation::Horizontal,
                            height: "40px".to_string(),
                            width: "100%".to_string(),
                            fade_mode: FadeMode::Both,
                            fade_color: Some("from-card".to_string()),
                            boundary_tolerance: 17,
                            class: "bg-card",
                            div { class: "flex space-x-1 py-1 w-max",
                                for tag in &scenario.tags {
                                    Badge {
                                        variant: BadgeVariant::Secondary,
                                        class: "text-xs flex-shrink-0".to_string(),
                                        "{tag}"
                                    }
                                }
                            }
                        }
                    }
                }

                // Line 3: Other info (metadata)
                div { class: "flex items-center justify-between text-sm text-muted-foreground",
                    if let Some(last_used) = &scenario.last_used {
                        div { class: "flex items-center space-x-1",
                            i { class: "fas fa-clock text-xs" }
                            span { "Last used: {last_used}" }
                        }
                    }
                    div { class: "flex items-center space-x-1",
                        i { class: "fas fa-book text-xs" }
                        span { 
                            "{scenario.story_count} "
                            if scenario.story_count == 1 { "story" } else { "stories" }
                        }
                    }
                }
            }
        }
    }
}
