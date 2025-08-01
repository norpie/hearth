//! Tag component for filtering with 3-state selection

use super::TagState;
use crate::{Badge, BadgeVariant, BadgeSize};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TagProps {
    pub text: String,
    pub state: TagState,
    pub onclick: EventHandler<()>,
}

#[component]
pub fn Tag(props: TagProps) -> Element {
    let (variant, class) = match props.state {
        TagState::None => (
            BadgeVariant::Outline, 
            "cursor-pointer hover:opacity-80 transition-opacity"
        ),
        TagState::Positive => (
            BadgeVariant::Success, 
            "cursor-pointer hover:opacity-80 transition-opacity border-green-500 bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-100"
        ),
        TagState::Negative => (
            BadgeVariant::Error, 
            "cursor-pointer hover:opacity-80 transition-opacity border-red-500 bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-100"
        ),
    };

    rsx! {
        Badge {
            variant,
            size: BadgeSize::Medium,
            class,
            onclick: move |_| props.onclick.call(()),
            "{props.text}"
        }
    }
}
