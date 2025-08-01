//! AspectRatio component for maintaining consistent aspect ratios

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AspectRatioProps {
    /// The desired aspect ratio as a float (e.g., 16.0/9.0 for 16:9)
    pub ratio: f64,
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn AspectRatio(props: AspectRatioProps) -> Element {
    let padding_bottom = (100.0 / props.ratio).to_string();
    let custom_classes = props.class.as_deref().unwrap_or("");
    
    rsx! {
        div {
            class: "relative w-full {custom_classes}",
            style: "padding-bottom: {padding_bottom}%",
            
            div {
                class: "absolute inset-0",
                {props.children}
            }
        }
    }
}