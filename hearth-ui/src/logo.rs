//! Hearth logo component

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LogoProps {
    /// Optional custom width (defaults to 48px)
    #[props(default = 48)]
    pub width: u32,
    /// Optional custom height (defaults to 48px)
    #[props(default = 48)]
    pub height: u32,
    /// Optional CSS classes
    #[props(default = String::new())]
    pub class: String,
}

#[component]
pub fn Logo(props: LogoProps) -> Element {
    let logo_svg = asset!("/assets/icons/logo.svg");

    rsx! {
        img {
            src: "{logo_svg}",
            alt: "Hearth Logo",
            width: "{props.width}",
            height: "{props.height}",
            class: "{props.class}",
        }
    }
}

#[component]
pub fn LogoWithText(props: LogoProps) -> Element {
    rsx! {
        div { class: format!("flex items-center space-x-3 {}", props.class),
            Logo { width: props.width, height: props.height }
            span { class: "text-xl font-bold text-foreground", "Hearth" }
        }
    }
}
