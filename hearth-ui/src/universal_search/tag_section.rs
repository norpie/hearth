//! Tag filtering section component

use dioxus::prelude::*;
use super::Tag;

#[derive(Props, Clone, PartialEq)]
pub struct TagSectionProps {
    pub label: String,
    pub placeholder: String,
    pub tags: Vec<String>,
    pub on_show_all: EventHandler<()>,
}

#[component]
pub fn TagSection(props: TagSectionProps) -> Element {
    rsx! {
        div {
            label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                "{props.label}"
            }
            // TAG SEARCH INPUT
            input {
                class: "w-full px-3 py-2 mb-3 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 placeholder-gray-500 dark:placeholder-gray-400 focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm",
                placeholder: "{props.placeholder}",
                r#type: "text",
            }
            // TAG CONTAINER
            div { class: "flex flex-wrap justify-between gap-2",
                for tag in &props.tags {
                    Tag { text: tag.clone() }
                }
            }
            // SHOW MORE BUTTON
            button {
                class: "mt-2 text-sm text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300",
                onclick: move |_| props.on_show_all.call(()),
                "Show all tags..."
            }
        }
    }
}