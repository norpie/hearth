//! Reusable tag button component

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TagProps {
    pub text: String,
    #[props(default)]
    pub onclick: Option<EventHandler<()>>,
}

#[component]
pub fn Tag(props: TagProps) -> Element {
    rsx! {
        button {
            class: "px-3 py-1 text-sm rounded-full transition-colors bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700",
            onclick: move |_| {
                if let Some(handler) = &props.onclick {
                    handler.call(());
                }
            },
            "{props.text}"
        }
    }
}