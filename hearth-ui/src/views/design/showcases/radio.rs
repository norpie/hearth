use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Label, Radio, RadioGroup};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RadioShowcaseProps {
    pub radio_theme: Signal<String>,
    pub radio_size: Signal<String>,
}

#[component]
pub fn radio_showcase(props: RadioShowcaseProps) -> Element {
    let RadioShowcaseProps {
        mut radio_theme,
        radio_size: _,
    } = props;
    rsx! {
        ComponentShowcase {
            name: "Radio".to_string(),
            description: "Radio button component for single selection from multiple options.".to_string(),
            basic_usage: r#"let mut selected = use_signal(|| "option1".to_string());

RadioGroup {
    name: "options",
    value: selected.read().clone(),
    onchange: move |value| selected.set(value),
    Radio { 
        value: "option1".to_string(), 
        selected: selected.read().clone(), 
        name: "options".to_string(), 
        label: "Option 1".to_string(),
        onchange: move |value| selected.set(value),
    }
    Radio { 
        value: "option2".to_string(), 
        selected: selected.read().clone(), 
        name: "options".to_string(), 
        label: "Option 2".to_string(),
        onchange: move |value| selected.set(value),
    }
}"#.to_string(),
            with_props_usage: r#"RadioGroup {
    name: "options",
    value: selected.read().clone(),
    direction: RadioDirection::Vertical,
    disabled: false,
    size: RadioSize::Medium,
    class: "custom-class",
    onchange: move |value| selected.set(value),
    Radio { 
        value: "option1".to_string(), 
        selected: selected.read().clone(), 
        name: "options".to_string(), 
        label: "Option 1".to_string(),
        onchange: move |value| selected.set(value),
        disabled: false,
        size: RadioSize::Medium,
    }
    // Add more Radio components as needed...
}"#.to_string(),

            ShowcaseVariant {
                title: "Groups".to_string(),
                div { class: "space-y-6",
                    div {
                        Label { "Theme Selection" }
                        RadioGroup {
                            name: "theme",
                            value: radio_theme.read().clone(),
                            onchange: move |value| radio_theme.set(value),
                            Radio {
                                value: "light".to_string(),
                                selected: radio_theme.read().clone(),
                                name: "theme".to_string(),
                                label: "Light".to_string(),
                                onchange: move |value| radio_theme.set(value),
                            }
                            Radio {
                                value: "dark".to_string(),
                                selected: radio_theme.read().clone(),
                                name: "theme".to_string(),
                                label: "Dark".to_string(),
                                onchange: move |value| radio_theme.set(value),
                            }
                            Radio {
                                value: "system".to_string(),
                                selected: radio_theme.read().clone(),
                                name: "theme".to_string(),
                                label: "System".to_string(),
                                onchange: move |value| radio_theme.set(value),
                            }
                        }
                    }
                }
            }
        }
    }
}
