//! Generic item card component for characters, scenarios, and other card items

use dioxus::prelude::*;
use crate::{models::{CardItem, CardMetadata}, Avatar};

#[derive(Props, Clone, PartialEq)]
pub struct ItemCardProps<T: CardItem + Clone + PartialEq + 'static> {
    pub item: T,
    pub metadata: Vec<CardMetadata>,
    pub on_select: EventHandler<String>,
    pub on_favorite: EventHandler<String>,
}

#[component]
pub fn ItemCard<T: CardItem + Clone + PartialEq + 'static>(props: ItemCardProps<T>) -> Element {
    let item_id = props.item.id().to_string();
    let item_id_2 = props.item.id().to_string();

    rsx! {
        button {
            class: "w-full p-5 bg-gray-100 dark:bg-gray-900 hover:bg-gray-200 dark:hover:bg-gray-800 transition-colors text-left rounded-xl",
            onclick: move |_| props.on_select.call(item_id.clone()),
            
            div { class: "flex items-start space-x-3 mb-3",
                // Avatar
                Avatar { 
                    name: props.item.name().to_string(), 
                    avatar_url: props.item.avatar_url().clone() 
                }
                
                // Content
                div { class: "flex-1 min-w-0",
                    h3 { class: "font-medium text-gray-900 dark:text-gray-100 mb-1",
                        "{props.item.name()}"
                    }
                    div { class: "text-sm text-gray-600 dark:text-gray-400 overflow-x-auto whitespace-nowrap pb-1",
                        "{props.item.description()}"
                    }
                }
            }
            
            // Tags
            div { class: "flex gap-1 mb-3 overflow-x-auto pb-1",
                for tag in props.item.tags() {
                    span { class: "px-2 py-1 bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 text-xs rounded whitespace-nowrap flex-shrink-0",
                        "{tag}"
                    }
                }
            }
            
            // Bottom section with scrollable metadata and favorite button
            div { class: "flex items-center justify-between",
                // Scrollable metadata (like tags)
                div { class: "flex gap-1 overflow-x-auto pb-1",
                    for meta in &props.metadata {
                        if meta.count > 0 {
                            span { class: "px-2 py-1 bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 text-xs rounded whitespace-nowrap flex-shrink-0 flex items-center space-x-1",
                                i { class: "{meta.icon}" }
                                span { "{meta.count} {meta.label}" }
                            }
                        }
                    }
                }
                
                // Favorite button
                button {
                    class: if props.item.is_favorite() {
                        "p-1 transition-colors text-yellow-500"
                    } else {
                        "p-1 transition-colors text-gray-400 hover:text-yellow-500"
                    },
                    onclick: move |evt| {
                        evt.stop_propagation();
                        props.on_favorite.call(item_id_2.clone());
                    },
                    if props.item.is_favorite() { 
                        i { class: "fa-solid fa-star" }
                    } else { 
                        i { class: "fa-solid fa-star opacity-30" }
                    }
                }
            }
        }
    }
}