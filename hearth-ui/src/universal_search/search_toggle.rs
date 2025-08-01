//! Search toggle button component

use dioxus::prelude::*;
use crate::SearchContext;

#[derive(Props, Clone, PartialEq)]
pub struct SearchToggleProps {
    pub context: SearchContext,
    pub expanded: Signal<bool>,
}

#[component]
pub fn SearchToggle(mut props: SearchToggleProps) -> Element {
    let context_label = match props.context {
        SearchContext::Stories => "stories",
        SearchContext::Characters => "characters", 
        SearchContext::Scenarios => "scenarios",
    };

    rsx! {
        button {
            class: "w-full mb-4 py-3 bg-white dark:bg-gray-950 border border-gray-200 dark:border-gray-700 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-900 transition-colors flex items-center justify-center space-x-2",
            onclick: move |_| (props.expanded).set(!(props.expanded)()),
            i { class: "fa-solid fa-magnifying-glass text-gray-600 dark:text-gray-400" }
            span { class: "text-gray-700 dark:text-gray-300 font-medium",
                "Search & Filter {context_label}"
            }
            i {
                class: "fa-solid fa-chevron-down text-gray-400 dark:text-gray-500 transition-transform",
                style: if (props.expanded)() { "transform: rotate(180deg)" } else { "" },
            }
        }
    }
}