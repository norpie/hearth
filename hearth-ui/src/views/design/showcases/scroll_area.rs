use dioxus::prelude::*;
use crate::{ScrollArea, ScrollOrientation, FadeMode, ScrollbarVisibility};
use super::super::{ComponentShowcase, ShowcaseVariant};

#[component]
pub fn scroll_area_showcase() -> Element {
    rsx! {
        ComponentShowcase {
            name: "ScrollArea".to_string(),
            description: "Custom scrollable area with fade effects and configurable scrollbars.".to_string(),
            basic_usage: r#"ScrollArea {
    height: "200px".to_string(),
    // content here
}"#.to_string(),
            with_props_usage: r#"ScrollArea {
    height: "300px".to_string(),
    orientation: ScrollOrientation::Both,
    fade_mode: FadeMode::Both,
    scrollbar_visibility: ScrollbarVisibility::Always,
    class: "custom-scroll",
    viewport_class: "custom-viewport",
    // content here
}"#.to_string(),
            
            ShowcaseVariant {
                title: "Basic Vertical Scroll".to_string(),
                div { class: "space-y-4",
                    ScrollArea {
                        height: "200px".to_string(),
                        class: "border border-gray-200 dark:border-gray-700 rounded-lg",
                        div { class: "p-4 space-y-2",
                            for i in 1..20 {
                                div { class: "p-2 bg-gray-50 dark:bg-gray-800 rounded",
                                    "Item {i}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}