//! Search toggle button component

use crate::{SearchContext, Button, ButtonVariant, ButtonSize};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchToggleProps {
    pub context: SearchContext,
    pub expanded: bool,
    pub on_toggle: EventHandler<()>,
}

#[component]
pub fn SearchToggle(props: SearchToggleProps) -> Element {
    let context_label = match props.context {
        SearchContext::Stories => "stories",
        SearchContext::Characters => "characters",
        SearchContext::Scenarios => "scenarios",
    };

    rsx! {
        Button {
            variant: ButtonVariant::Outline,
            size: ButtonSize::Large,
            class: "w-full mb-4 justify-center space-x-2",
            onclick: move |_| props.on_toggle.call(()),
            i { class: "fa-solid fa-magnifying-glass" }
            span { "Search & Filter {context_label}" }
            i {
                class: format!(
                    "fa-solid fa-chevron-down transition-transform {}", 
                    if props.expanded { "rotate-180" } else { "" }
                ),
            }
        }
    }
}
