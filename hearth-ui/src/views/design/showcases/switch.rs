use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Label, Switch};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SwitchShowcaseProps {
    pub switch_1: Signal<bool>,
    pub switch_2: Signal<bool>,
    pub switch_3: Signal<bool>,
}

#[component]
pub fn switch_showcase(props: SwitchShowcaseProps) -> Element {
    let SwitchShowcaseProps {
        mut switch_1,
        mut switch_2,
        switch_3,
    } = props;
    rsx! {
        ComponentShowcase {
            name: "Switch".to_string(),
            description: "Toggle switch component for boolean settings.".to_string(),
            basic_usage: r#"let mut checked = use_signal(|| false);

Switch {
    checked: checked.read().clone(),
    onchange: move |value| checked.set(value),
}"#.to_string(),
            with_props_usage: r#"let mut checked = use_signal(|| false);

Switch {
    checked: checked.read().clone(),
    size: SwitchSize::Medium,
    disabled: false,
    class: "custom-class",
    onchange: move |value| checked.set(value),
}"#.to_string(),

            ShowcaseVariant {
                title: "States".to_string(),
                div { class: "space-y-4",
                    div { class: "flex items-center space-x-3",
                        Switch {
                            checked: *switch_1.read(),
                            onchange: move |checked| switch_1.set(checked),
                        }
                        Label { "Toggle me" }
                    }
                    div { class: "flex items-center space-x-3",
                        Switch {
                            checked: *switch_2.read(),
                            onchange: move |checked| switch_2.set(checked),
                        }
                        Label { "Default on" }
                    }
                    div { class: "flex items-center space-x-3",
                        Switch {
                            disabled: true,
                            checked: *switch_3.read(),
                        }
                        Label { "Disabled" }
                    }
                }
            }
        }
    }
}
