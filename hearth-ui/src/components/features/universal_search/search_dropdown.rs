//! Main search dropdown component

use crate::{
    SearchContext, Card, Input, InputVariant, Label, Select, SelectVariant, SelectOption,
    Button, ButtonVariant, ButtonSize, ScrollArea, ScrollOrientation, FadeMode, ToggleIcon, ToggleIconSize,
    UniversalSearchQuery
};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchDropdownProps {
    pub context: SearchContext,
    pub current_query: UniversalSearchQuery,
    pub on_query_change: EventHandler<UniversalSearchQuery>,
    pub children: Element,
}

#[component]
pub fn SearchDropdown(props: SearchDropdownProps) -> Element {
    // Clone props to avoid borrow issues in closures
    let current_query = props.current_query.clone();
    let on_query_change = props.on_query_change.clone();
    
    // Clone for each closure to avoid move issues
    let query_input = current_query.clone();
    let on_change_input = on_query_change.clone();
    
    let query_select = current_query.clone();
    let on_change_select = on_query_change.clone();
    
    let query_button = current_query.clone();
    let on_change_button = on_query_change.clone();
    let query_button_icon = current_query.clone();
    
    let query_checkbox = current_query.clone();
    let on_change_checkbox = on_query_change.clone();
    
    let query_previously_used = current_query.clone();
    let on_change_previously_used = on_query_change.clone();
    
    let query_multiple_characters = current_query.clone();
    let on_change_multiple_characters = on_query_change.clone();
    
    let query_recently_added = current_query.clone();
    let on_change_recently_added = on_query_change.clone();
    
    let query_has_images = current_query.clone();
    let on_change_has_images = on_query_change.clone();
    
    let query_untagged = current_query.clone();
    let on_change_untagged = on_query_change.clone();
    
    let context_label = match props.context {
        SearchContext::Stories => "stories",
        SearchContext::Characters => "characters",
        SearchContext::Scenarios => "scenarios",
    };

    let available_sort_options = vec!["Recent", "Created", "A-Z", "Rating", "Usage"];

    let sort_options: Vec<SelectOption> = available_sort_options
        .into_iter()
        .map(|option| SelectOption::new(option.to_string(), option.to_string()))
        .collect();

    rsx! {
        Card {
            class: "mb-4 p-4 flex flex-col",
            ScrollArea {
                orientation: ScrollOrientation::Vertical,
                height: "h-[65vh] sm:h-[50vh]".to_string(),
                fade_mode: FadeMode::Both,
                fade_color: Some("from-card".to_string()),
                div { class: "space-y-4 p-1",
                    // Text search
                    div {
                        Label { "Search text" }
                        Input {
                            variant: InputVariant::Default,
                            placeholder: format!("Search {}...", context_label),
                            value: query_input.search_text.clone(),
                            oninput: move |value: String| {
                                let mut query = query_input.clone();
                                query.search_text = value;
                                on_change_input.call(query);
                            },
                        }
                    }

                    // Quick filters and sorting with responsive layout
                    div {
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            // Sort options section
                            div {
                                Label { "Sort by" }
                                div { class: "flex items-center space-x-2 mt-1",
                                    div { class: "flex-1",
                                        Select {
                                            variant: SelectVariant::Default,
                                            value: query_select.sort_option.clone(),
                                            options: sort_options,
                                            placeholder: "Select sort order".to_string(),
                                            onchange: move |value: String| {
                                                let mut query = query_select.clone();
                                                query.sort_option = value;
                                                on_change_select.call(query);
                                            },
                                        }
                                    }
                                    Button {
                                        variant: ButtonVariant::Outline,
                                        size: ButtonSize::Medium,
                                        class: "w-8 px-0".to_string(),
                                        onclick: move |_| {
                                            let mut query = query_button.clone();
                                            query.sort_ascending = !query.sort_ascending;
                                            on_change_button.call(query);
                                        },
                                        // tooltip: if query_button.sort_ascending { "Ascending" } else { "Descending" },
                                        i {
                                            class: format!(
                                                "fa-solid transition-transform {}",
                                                if query_button_icon.sort_ascending { "fa-arrow-up" } else { "fa-arrow-down" }
                                            ),
                                        }
                                    }
                                }
                            }

                            // Quick filters section
                            div {
                                Label { "Quick filters" }
                                div { class: "flex flex-wrap gap-2 mt-1",
                                    // Favorites icon toggle button - matches showcase favorite theme
                                    {
                                        let query_for_button = query_checkbox.clone();
                                        rsx! {
                                            ToggleIcon {
                                                icon: "fa-star",
                                                is_active: query_for_button.favorites_only,
                                                active_color: "text-yellow-400",
                                                inactive_color: "text-gray-500",
                                                size: ToggleIconSize::Medium,
                                                aria_label: "Toggle favorites filter",
                                                tooltip: "Show only favorites".to_string(),
                                                onclick: move |_| {
                                                    let mut query = query_for_button.clone();
                                                    query.favorites_only = !query.favorites_only;
                                                    on_change_checkbox.call(query);
                                                },
                                            }
                                        }
                                    }
                                    
                                    // Previously used icon toggle button - green theme
                                    {
                                        let query_for_button = query_previously_used.clone();
                                        rsx! {
                                            ToggleIcon {
                                                icon: "fa-clock",
                                                is_active: query_for_button.previously_used_only,
                                                active_color: "text-green-500",
                                                inactive_color: "text-gray-500",
                                                size: ToggleIconSize::Medium,
                                                aria_label: "Toggle previously used filter",
                                                tooltip: "Show only previously used".to_string(),
                                                onclick: move |_| {
                                                    let mut query = query_for_button.clone();
                                                    query.previously_used_only = !query.previously_used_only;
                                                    on_change_previously_used.call(query);
                                                },
                                            }
                                        }
                                    }
                                    
                                    // Multiple characters icon toggle button - blue theme
                                    {
                                        let query_for_button = query_multiple_characters.clone();
                                        rsx! {
                                            ToggleIcon {
                                                icon: "fa-users",
                                                is_active: query_for_button.multiple_characters_only,
                                                active_color: "text-blue-500",
                                                inactive_color: "text-gray-500",
                                                size: ToggleIconSize::Medium,
                                                aria_label: "Toggle multiple characters filter",
                                                tooltip: "Show only multiple characters".to_string(),
                                                onclick: move |_| {
                                                    let mut query = query_for_button.clone();
                                                    query.multiple_characters_only = !query.multiple_characters_only;
                                                    on_change_multiple_characters.call(query);
                                                },
                                            }
                                        }
                                    }
                                    
                                    // Recently added icon toggle button - purple theme
                                    {
                                        let query_for_button = query_recently_added.clone();
                                        rsx! {
                                            ToggleIcon {
                                                icon: "fa-plus",
                                                is_active: query_for_button.recently_added_only,
                                                active_color: "text-purple-500",
                                                inactive_color: "text-gray-500",
                                                size: ToggleIconSize::Medium,
                                                aria_label: "Toggle recently added filter",
                                                tooltip: "Show only recently added".to_string(),
                                                onclick: move |_| {
                                                    let mut query = query_for_button.clone();
                                                    query.recently_added_only = !query.recently_added_only;
                                                    on_change_recently_added.call(query);
                                                },
                                            }
                                        }
                                    }
                                    
                                    // Has images icon toggle button - teal theme
                                    {
                                        let query_for_button = query_has_images.clone();
                                        rsx! {
                                            ToggleIcon {
                                                icon: "fa-image",
                                                is_active: query_for_button.has_images_only,
                                                active_color: "text-teal-500",
                                                inactive_color: "text-gray-500",
                                                size: ToggleIconSize::Medium,
                                                aria_label: "Toggle has images filter",
                                                tooltip: "Show only items with images".to_string(),
                                                onclick: move |_| {
                                                    let mut query = query_for_button.clone();
                                                    query.has_images_only = !query.has_images_only;
                                                    on_change_has_images.call(query);
                                                },
                                            }
                                        }
                                    }
                                    
                                    // Untagged icon toggle button - gray theme
                                    {
                                        let query_for_button = query_untagged.clone();
                                        rsx! {
                                            ToggleIcon {
                                                icon: "fa-tag",
                                                is_active: query_for_button.untagged_only,
                                                active_color: "text-indigo-600",
                                                inactive_color: "text-gray-500",
                                                size: ToggleIconSize::Medium,
                                                aria_label: "Toggle untagged filter",
                                                tooltip: "Show only untagged items".to_string(),
                                                onclick: move |_| {
                                                    let mut query = query_for_button.clone();
                                                    query.untagged_only = !query.untagged_only;
                                                    on_change_untagged.call(query);
                                                },
                                            }
                                        }
                                    }
                                    
                                    // TODO: Add more quick filter buttons here
                                    // - Completed (for stories)
                                    // - In Progress
                                    // - Draft
                                }
                            }
                        }
                    }

                    // Tag filtering sections (passed as children)
                    {props.children}
                }
            }
        }
    }
}
