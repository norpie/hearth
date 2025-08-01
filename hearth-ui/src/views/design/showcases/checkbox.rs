use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Checkbox, Label};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CheckboxShowcaseProps {
    pub checkbox_1: Signal<bool>,
    pub checkbox_2: Signal<bool>,
    pub checkbox_3: Signal<bool>,
}

#[component]
pub fn checkbox_showcase(props: CheckboxShowcaseProps) -> Element {
    let CheckboxShowcaseProps {
        mut checkbox_1,
        mut checkbox_2,
        checkbox_3,
    } = props;
    rsx! {
        ComponentShowcase {
            name: "Checkbox".to_string(),
            description: "Checkbox input component for multiple selections.".to_string(),
            basic_usage: r#"let mut checked = use_signal(|| false);

Checkbox {
    checked: checked.read().clone(),
    onchange: move |value| checked.set(value),
}"#.to_string(),
            with_props_usage: r#"let mut checked = use_signal(|| false);

Checkbox {
    checked: checked.read().clone(),
    size: CheckboxSize::Medium,
    disabled: false,
    class: "custom-class",
    onchange: move |value| checked.set(value),
}"#.to_string(),

            ShowcaseVariant {
                title: "States".to_string(),
                div { class: "space-y-4",
                    div { class: "flex items-center space-x-3",
                        Checkbox {
                            checked: *checkbox_1.read(),
                            onchange: move |checked| checkbox_1.set(checked),
                        }
                        Label { "Check me" }
                    }
                    div { class: "flex items-center space-x-3",
                        Checkbox {
                            checked: *checkbox_2.read(),
                            onchange: move |checked| checkbox_2.set(checked),
                        }
                        Label { "Default checked" }
                    }
                    div { class: "flex items-center space-x-3",
                        Checkbox {
                            disabled: true,
                            checked: *checkbox_3.read(),
                        }
                        Label { "Disabled" }
                    }
                }
            }
        }
    }
}
