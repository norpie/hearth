//! Tag modal component for showing all available tags

use dioxus::prelude::*;
use crate::Modal;
use super::Tag;

#[derive(Props, Clone, PartialEq)]
pub struct TagModalProps {
    pub title: String,
    pub tags: Vec<String>,
    pub is_open: Signal<bool>,
    pub search_query: Signal<String>,
}

#[component]
pub fn TagModal(mut props: TagModalProps) -> Element {
    if !(props.is_open)() {
        return rsx! {
            div {}
        };
    }

    let filtered_tags: Vec<&String> = if (props.search_query)().is_empty() {
        props.tags.iter().collect()
    } else {
        props.tags.iter().filter(|tag| 
            tag.to_lowercase().contains(&(props.search_query)().to_lowercase())
        ).collect()
    };

    rsx! {
        Modal { title: props.title.clone(), is_open: props.is_open,
            // Search input
            div { class: "flex-shrink-0 p-4 border-b border-gray-200 dark:border-gray-700",
                input {
                    class: "w-full px-4 py-3 text-base border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 placeholder-gray-500 dark:placeholder-gray-400 focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                    placeholder: "Search tags...",
                    r#type: "text",
                    value: (props.search_query)(),
                    oninput: move |e| (props.search_query).set(e.value().clone()),
                }
            }
            // Tags list
            div { class: "flex-1 overflow-y-auto p-4 min-h-0",
                if filtered_tags.is_empty() {
                    div { class: "flex items-center justify-center h-32 text-center text-gray-500 dark:text-gray-400",
                        "No tags found matching your search."
                    }
                } else {
                    div { class: "flex flex-wrap justify-between gap-2",
                        for tag in filtered_tags {
                            Tag {
                                text: tag.clone(),
                                onclick: move |_| {
                                    (props.is_open).set(false);
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}