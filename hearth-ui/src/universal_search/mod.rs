//! Universal search module with Svelte-style component structure

mod search_toggle;
mod search_dropdown;
mod tag_section;
mod tag_modal;
mod tag;

pub use search_toggle::*;
pub use search_dropdown::*;
pub use tag_section::*;
pub use tag_modal::*;
pub use tag::*;

use dioxus::prelude::*;
use crate::SearchContext;

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
}

#[component]
pub fn UniversalSearch(props: UniversalSearchProps) -> Element {
    let expanded = use_signal(|| props.initially_expanded);
    let mut modal_open = use_signal(|| false);
    let mut modal_type = use_signal(|| "".to_string());
    let mut search_query = use_signal(|| "".to_string());

    let has_tags = !props.available_tags.is_empty();

    rsx! {
        div { class: "flex-shrink-0",
            div { class: "px-4 max-w-screen-2xl mx-auto",
                SearchToggle { context: props.context.clone(), expanded }
                if expanded() {
                    SearchDropdown { context: props.context.clone(),
                        // Tag filtering sections based on context
                        if matches!(props.context, SearchContext::Stories) {
                            // Show dual tag sections for stories
                            if let Some(character_tags) = &props.available_character_tags {
                                TagSection {
                                    label: "Filter by character tags".to_string(),
                                    placeholder: "Search character tags...".to_string(),
                                    tags: character_tags.clone(),
                                    on_show_all: move |_| {
                                        modal_type.set("character".to_string());
                                        modal_open.set(true);
                                        search_query.set("".to_string());
                                    },
                                }
                            }
                            if let Some(scenario_tags) = &props.available_scenario_tags {
                                TagSection {
                                    label: "Filter by scenario tags".to_string(),
                                    placeholder: "Search scenario tags...".to_string(),
                                    tags: scenario_tags.clone(),
                                    on_show_all: move |_| {
                                        modal_type.set("scenario".to_string());
                                        modal_open.set(true);
                                        search_query.set("".to_string());
                                    },
                                }
                            }
                        } else if matches!(props.context, SearchContext::Characters) && has_tags {
                            // Show only character tags
                            TagSection {
                                label: "Filter by character tags".to_string(),
                                placeholder: "Search character tags...".to_string(),
                                tags: props.available_tags.clone(),
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
                rsx! {
                    TagModal {
                        title: modal_title,
                        tags: tags_to_show,
                        is_open: modal_open,
                        search_query,
                    }
                }
            }
        }
    }
}