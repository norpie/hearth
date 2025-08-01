//! Main search dropdown component

use dioxus::prelude::*;
use crate::SearchContext;

#[derive(Props, Clone, PartialEq)]
pub struct SearchDropdownProps {
    pub context: SearchContext,
    pub children: Element,
}

#[component]
pub fn SearchDropdown(props: SearchDropdownProps) -> Element {
    let context_label = match props.context {
        SearchContext::Stories => "stories",
        SearchContext::Characters => "characters", 
        SearchContext::Scenarios => "scenarios",
    };

    let available_sort_options = match props.context {
        SearchContext::Stories => vec!["Recent", "A-Z"],
        SearchContext::Characters => vec!["Recent", "Created", "A-Z", "Usage"],
        SearchContext::Scenarios => vec!["Recent", "Created", "A-Z", "Rating", "Usage"],
    };

    rsx! {
        div { class: "mb-4 p-4 bg-white dark:bg-gray-950 border border-gray-200 dark:border-gray-700 rounded-lg max-h-96 overflow-y-auto",
            div { class: "space-y-4",
                // Text search
                div {
                    label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                        "Search text"
                    }
                    input {
                        class: "w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 placeholder-gray-500 dark:placeholder-gray-400 focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                        placeholder: "Search {context_label}...",
                        r#type: "text",
                    }
                }

                // Tag filtering sections (passed as children)
                {props.children}

                // Sort and filter options
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    // Sort options
                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                            "Sort by"
                        }
                        div { class: "flex space-x-2",
                            select { class: "flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100",
                                for sort_option in &available_sort_options {
                                    option { "{sort_option}" }
                                }
                            }
                            button { class: "px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors",
                                "↓"
                            }
                        }
                    }

                    // Quick filters
                    div {
                        label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2",
                            "Quick filters"
                        }
                        div { class: "space-y-2",
                            // Favorites filter
                            label { class: "flex items-center space-x-2 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    class: "rounded border-gray-300 dark:border-gray-600 text-blue-600 focus:ring-blue-500",
                                }
                                span { class: "text-sm text-gray-700 dark:text-gray-300",
                                    "Favorites only"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}