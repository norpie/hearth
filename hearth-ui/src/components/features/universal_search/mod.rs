//! Universal search module with Svelte-style component structure

mod search_dropdown;
mod search_toggle;
mod tag;
mod tag_modal;
mod tag_section;

pub use search_dropdown::*;
pub use search_toggle::*;
pub use tag::*;
pub use tag_modal::*;
pub use tag_section::*;

use crate::{SearchContext, Collapsible, TagState, UniversalSearchQuery, UniversalSearchState};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UniversalSearchProps {
    pub context: SearchContext,
    pub available_tags: Vec<String>,
    #[props(default = None)]
    pub available_character_tags: Option<Vec<String>>,
    #[props(default = None)]
    pub available_scenario_tags: Option<Vec<String>>,
    #[props(default = None)]
    pub all_character_tags: Option<Vec<String>>,
    #[props(default = None)]
    pub all_scenario_tags: Option<Vec<String>>,
    #[props(default = Vec::new())]
    pub all_tags: Vec<String>,
    #[props(default = false)]
    pub initially_expanded: bool,
    pub search_state: Signal<UniversalSearchState>,
    pub on_query_change: EventHandler<UniversalSearchQuery>,
}

#[component]
pub fn UniversalSearch(mut props: UniversalSearchProps) -> Element {
    let mut modal_open = use_signal(|| false);
    let mut modal_type = use_signal(|| "".to_string());
    let mut search_query = use_signal(String::new);

    // Initialize expanded state from props if not already set
    use_effect(move || {
        if props.initially_expanded {
            let mut state = (props.search_state)();
            state.expanded = true;
            props.search_state.set(state);
        }
    });

    // Helper function to cycle tag state: None -> Positive -> Negative -> None
    let cycle_tag_state = |current_state: &TagState| -> TagState {
        match current_state {
            TagState::None => TagState::Positive,
            TagState::Positive => TagState::Negative,
            TagState::Negative => TagState::None,
        }
    };

    // Helper function to emit query change
    let emit_query_change = {
        let search_state = props.search_state;
        let on_query_change = props.on_query_change.clone();
        move || {
            let state = search_state();
            let query = state.to_query();
            on_query_change.call(query);
        }
    };

    let has_tags = !props.available_tags.is_empty();
    let has_character_tags = props.available_character_tags.as_ref().map_or(false, |tags| !tags.is_empty());
    let has_scenario_tags = props.available_scenario_tags.as_ref().map_or(false, |tags| !tags.is_empty());

    // Get current state values
    let current_state = (props.search_state)();

    rsx! {
        div { class: "flex-shrink-0",
            div { class: "px-4 max-w-screen-2xl mx-auto",
                Collapsible {
                    trigger: format!("Search & Filter {}", match props.context {
                        SearchContext::Stories => "Stories",
                        SearchContext::Characters => "Characters", 
                        SearchContext::Scenarios => "Scenarios",
                    }),
                    default_open: current_state.expanded,
                    class: "mb-4",
                    content_class: "pt-0",
                    
                    SearchDropdown { 
                        context: props.context.clone(),
                        current_query: current_state.to_query(),
                        on_query_change: move |new_query: UniversalSearchQuery| {
                            let mut state = (props.search_state)();
                            state.search_state.search_text = new_query.search_text;
                            state.search_state.sort_option = new_query.sort_option;
                            state.search_state.sort_ascending = new_query.sort_ascending;
                            state.search_state.favorites_only = new_query.favorites_only;
                            state.search_state.previously_used_only = new_query.previously_used_only;
                            state.search_state.multiple_characters_only = new_query.multiple_characters_only;
                            state.search_state.recently_added_only = new_query.recently_added_only;
                            state.search_state.has_images_only = new_query.has_images_only;
                            state.search_state.untagged_only = new_query.untagged_only;
                            props.search_state.set(state);
                            emit_query_change();
                        },
                        
                        // Tag filtering sections based on context
                        if matches!(props.context, SearchContext::Stories) {
                            // Show dual tag sections for stories
                            if has_character_tags {
                                if let Some(character_tags) = &props.available_character_tags {
                                    TagSection {
                                        label: "Filter by character tags".to_string(),
                                        placeholder: "Search character tags...".to_string(),
                                        tags: character_tags.clone(),
                                        tag_states: current_state.character_tag_states.clone(),
                                        on_tag_cycle: move |tag: String| {
                                            let mut state = (props.search_state)();
                                            let current_tag_state = state.character_tag_states.get(&tag).cloned().unwrap_or_default();
                                            let new_tag_state = cycle_tag_state(&current_tag_state);
                                            
                                            if new_tag_state == TagState::None {
                                                state.character_tag_states.remove(&tag);
                                            } else {
                                                state.character_tag_states.insert(tag, new_tag_state);
                                            }
                                            
                                            props.search_state.set(state);
                                            emit_query_change();
                                        },
                                        on_show_all: move |_| {
                                            modal_type.set("character".to_string());
                                            modal_open.set(true);
                                            search_query.set("".to_string());
                                        },
                                    }
                                }
                            }
                            if has_scenario_tags {
                                if let Some(scenario_tags) = &props.available_scenario_tags {
                                    TagSection {
                                        label: "Filter by scenario tags".to_string(),
                                        placeholder: "Search scenario tags...".to_string(),
                                        tags: scenario_tags.clone(),
                                        tag_states: current_state.scenario_tag_states.clone(),
                                        on_tag_cycle: move |tag: String| {
                                            let mut state = (props.search_state)();
                                            let current_tag_state = state.scenario_tag_states.get(&tag).cloned().unwrap_or_default();
                                            let new_tag_state = cycle_tag_state(&current_tag_state);
                                            
                                            if new_tag_state == TagState::None {
                                                state.scenario_tag_states.remove(&tag);
                                            } else {
                                                state.scenario_tag_states.insert(tag, new_tag_state);
                                            }
                                            
                                            props.search_state.set(state);
                                            emit_query_change();
                                        },
                                        on_show_all: move |_| {
                                            modal_type.set("scenario".to_string());
                                            modal_open.set(true);
                                            search_query.set("".to_string());
                                        },
                                    }
                                }
                            }
                        } else if matches!(props.context, SearchContext::Characters) && has_tags {
                            // Show only character tags
                            TagSection {
                                label: "Filter by character tags".to_string(),
                                placeholder: "Search character tags...".to_string(),
                                tags: props.available_tags.clone(),
                                tag_states: current_state.character_tag_states.clone(),
                                on_tag_cycle: move |tag: String| {
                                    let mut state = (props.search_state)();
                                    let current_tag_state = state.character_tag_states.get(&tag).cloned().unwrap_or_default();
                                    let new_tag_state = cycle_tag_state(&current_tag_state);
                                    
                                    if new_tag_state == TagState::None {
                                        state.character_tag_states.remove(&tag);
                                    } else {
                                        state.character_tag_states.insert(tag, new_tag_state);
                                    }
                                    
                                    props.search_state.set(state);
                                    emit_query_change();
                                },
                                on_show_all: move |_| {
                                    modal_type.set("character".to_string());
                                    modal_open.set(true);
                                    search_query.set("".to_string());
                                },
                            }
                        } else if matches!(props.context, SearchContext::Scenarios) && has_tags {
                            // Show only scenario tags
                            TagSection {
                                label: "Filter by scenario tags".to_string(),
                                placeholder: "Search scenario tags...".to_string(),
                                tags: props.available_tags.clone(),
                                tag_states: current_state.scenario_tag_states.clone(),
                                on_tag_cycle: move |tag: String| {
                                    let mut state = (props.search_state)();
                                    let current_tag_state = state.scenario_tag_states.get(&tag).cloned().unwrap_or_default();
                                    let new_tag_state = cycle_tag_state(&current_tag_state);
                                    
                                    if new_tag_state == TagState::None {
                                        state.scenario_tag_states.remove(&tag);
                                    } else {
                                        state.scenario_tag_states.insert(tag, new_tag_state);
                                    }
                                    
                                    props.search_state.set(state);
                                    emit_query_change();
                                },
                                on_show_all: move |_| {
                                    modal_type.set("scenario".to_string());
                                    modal_open.set(true);
                                    search_query.set("".to_string());
                                },
                            }
                        }
                    }
                }
            }
        }
        
        // Modal based on context
        if modal_open() {
            {
                let modal_title = if modal_type() == "character" {
                    "All Character Tags".to_string()
                } else {
                    "All Scenario Tags".to_string()
                };
                let tags_to_show = if modal_type() == "character" {
                    if let Some(all_char_tags) = &props.all_character_tags {
                        all_char_tags.clone()
                    } else if let Some(avail_char_tags) = &props.available_character_tags {
                        avail_char_tags.clone()
                    } else {
                        props.all_tags.clone()
                    }
                } else if let Some(all_scen_tags) = &props.all_scenario_tags {
                    all_scen_tags.clone()
                } else if let Some(avail_scen_tags) = &props.available_scenario_tags {
                    avail_scen_tags.clone()
                } else {
                    props.all_tags.clone()
                };
                
                let tag_states = if modal_type() == "character" {
                    current_state.character_tag_states.clone()
                } else {
                    current_state.scenario_tag_states.clone()
                };
                
                rsx! {
                    TagModal {
                        title: modal_title,
                        tags: tags_to_show,
                        is_open: modal_open,
                        search_query,
                        tag_states,
                        on_tag_cycle: move |tag: String| {
                            let mut state = (props.search_state)();
                            if modal_type() == "character" {
                                let current_tag_state = state.character_tag_states.get(&tag).cloned().unwrap_or_default();
                                let new_tag_state = cycle_tag_state(&current_tag_state);
                                
                                if new_tag_state == TagState::None {
                                    state.character_tag_states.remove(&tag);
                                } else {
                                    state.character_tag_states.insert(tag, new_tag_state);
                                }
                            } else {
                                let current_tag_state = state.scenario_tag_states.get(&tag).cloned().unwrap_or_default();
                                let new_tag_state = cycle_tag_state(&current_tag_state);
                                
                                if new_tag_state == TagState::None {
                                    state.scenario_tag_states.remove(&tag);
                                } else {
                                    state.scenario_tag_states.insert(tag, new_tag_state);
                                }
                            }
                            props.search_state.set(state);
                            emit_query_change();
                        },
                    }
                }
            }
        }
    }
}
