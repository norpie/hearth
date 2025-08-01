use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Label, Slider};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SliderShowcaseProps {
    pub slider_volume: Signal<f64>,
    pub slider_temperature: Signal<f64>,
    pub slider_range: Signal<f64>,
    pub slider_small: Signal<f64>,
    pub slider_medium: Signal<f64>,
    pub slider_large: Signal<f64>,
    pub slider_normal: Signal<f64>,
}

#[component]
pub fn slider_showcase(props: SliderShowcaseProps) -> Element {
    let SliderShowcaseProps {
        mut slider_volume,
        mut slider_temperature,
        slider_range: _,
        slider_small: _,
        slider_medium: _,
        slider_large: _,
        slider_normal: _,
    } = props;
    rsx! {
        ComponentShowcase {
            name: "Slider".to_string(),
            description: "Range slider component for selecting numeric values.".to_string(),
            basic_usage: r#"let mut slider_value = use_signal(|| 50.0);

Slider {
    value: slider_value.read().clone(),
    onchange: move |value| slider_value.set(value),
}"#.to_string(),
            with_props_usage: r#"Slider {
    value: slider_value.read().clone(),
    min: 0.0,
    max: 100.0,
    step: 1.0,
    size: SliderSize::Medium,
    disabled: false,
    class: "custom-class",
    onchange: move |value| slider_value.set(value),
}"#.to_string(),

            ShowcaseVariant {
                title: "Basic Usage".to_string(),
                div { class: "space-y-6",
                    div {
                        Label {
                            "Volume: {(*slider_volume.read() as f64).round() as i32}%"
                        }
                        Slider {
                            value: *slider_volume.read(),
                            min: 0.0,
                            max: 100.0,
                            step: 1.0,
                            onchange: move |value| slider_volume.set(value),
                        }
                    }
                    div {
                        Label {
                            "Temperature: {slider_temperature():.1}"
                        }
                        Slider {
                            value: *slider_temperature.read(),
                            min: 0.0,
                            max: 2.0,
                            step: 0.1,
                            onchange: move |value| slider_temperature.set(value),
                        }
                    }
                }
            }
        }
    }
}
