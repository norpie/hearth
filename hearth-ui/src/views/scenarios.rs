//! Unified responsive scenarios view

use dioxus::prelude::*;
use crate::{ItemCard, PageHeader, Route, UniversalSearch, SearchContext, sample_scenarios, sample_scenario_tags_sorted, Platform, models::CardMetadata};

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

    rsx! {
        PageHeader { title: "Scenarios".to_string(), back_button: None }
        // Universal Search/Filter Section
        UniversalSearch {
            context: SearchContext::Scenarios,
            available_tags,
            all_tags,
        }
        // Scenarios list with responsive design
        div { class: "flex-1 overflow-y-auto",
            div { class: if platform.is_mobile() { "px-4 pb-4 pt-3" } else { "px-4 pb-4 max-w-screen-2xl mx-auto" },
                div { class: if platform.is_mobile() { "space-y-3" } else { "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4" },
                    for scenario in &scenarios() {
                        {
                            let scenario_id = scenario.id.clone();
                            let scenario_id_2 = scenario.id.clone();
                            rsx! {
                                ItemCard {
                                    item: scenario.clone(),
                                    metadata: vec![CardMetadata { icon: "fa-solid fa-book", count: scenario.story_count, label: "stories" }],
                                    on_select: move |_| println!("Selected scenario: {scenario_id}"),
                                    on_favorite: move |_| {
                                        let mut scenarios_vec = scenarios();
                                        if let Some(scenario) = scenarios_vec.iter_mut().find(|s| s.id == scenario_id_2)
                                        {
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