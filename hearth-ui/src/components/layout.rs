//! Generic layout components with no domain knowledge

use dioxus::prelude::*;

// Generic settings section component
#[component]
pub fn SettingsSection(title: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "mb-8",
            h2 { class: "text-sm font-medium text-gray-900 dark:text-white mb-6", "{title}" }
            div { class: "space-y-1", {children} }
        }
    }
}

// Generic settings item component
#[component]
pub fn SettingsItem(
    icon: &'static str,
    label: &'static str,
    description: Option<&'static str>,
    on_click: EventHandler<()>,
    trailing: Element,
) -> Element {
    rsx! {
        button {
            class: "w-full py-4 px-4 flex items-center space-x-4 hover:bg-gray-200 dark:hover:bg-gray-800 transition-colors text-left rounded-lg",
            onclick: move |_| on_click.call(()),

            i { class: "{icon} text-xl text-gray-600 dark:text-gray-400" }
            div { class: "flex-1 min-w-0",
                div { class: "font-medium text-gray-900 dark:text-white", "{label}" }
                if let Some(desc) = description {
                    div { class: "text-sm text-gray-500 dark:text-gray-400 mt-0.5",
                        "{desc}"
                    }
                }
            }
            div { class: "flex items-center", {trailing} }
        }
    }
}

// Generic filter tab
#[component]
pub fn FilterTab<T: Clone + PartialEq + 'static>(
    filter: T,
    current_filter: T,
    label: &'static str,
    count: Option<usize>,
    on_select: EventHandler<T>,
) -> Element {
    let is_active = filter == current_filter;
    
    rsx! {
        button {
            class: format!("px-4 py-2 rounded-lg text-sm font-medium transition-colors {}",
                if is_active {
                    "bg-purple-100 dark:bg-purple-900/50 text-purple-900 dark:text-purple-300"
                } else {
                    "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800"
                }),
            onclick: {
                let filter = filter.clone();
                move |_| on_select.call(filter.clone())
            },
            "{label}"
            if let Some(count) = count {
                span {
                    class: format!("ml-2 px-2 py-0.5 text-xs rounded-full {}",
                        if is_active {
                            "bg-purple-200 dark:bg-purple-800 text-purple-800 dark:text-purple-200"
                        } else {
                            "bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-400"
                        }),
                    "{count}"
                }
            }
        }
    }
}