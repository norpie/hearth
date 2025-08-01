use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Input, InputSize, InputType, InputVariant};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InputShowcaseProps {
    pub default_input: Signal<String>,
    pub filled_input: Signal<String>,
    pub password_input: Signal<String>,
    pub error_input: Signal<String>,
}

#[component]
pub fn input_showcase(props: InputShowcaseProps) -> Element {
    let InputShowcaseProps {
        mut default_input,
        mut filled_input,
        mut password_input,
        mut error_input,
    } = props;
    rsx! {
        ComponentShowcase {
            name: "Input".to_string(),
            description: "Text input component with validation states and multiple variants.".to_string(),
            basic_usage: r#"let mut input_value = use_signal(|| String::new());

Input {
    value: input_value.read().clone(),
    oninput: move |value: String| input_value.set(value),
}"#.to_string(),
            with_props_usage: r#"let mut input_value = use_signal(|| String::new());

Input {
    variant: InputVariant::Default,
    size: InputSize::Medium,
    input_type: InputType::Text,
    value: input_value.read().clone(),
    placeholder: "Enter text here",
    disabled: false,
    error: false,
    required: true,
    name: "username",
    id: "username-input",
    class: "custom-class",
    oninput: move |value: String| input_value.set(value),
}"#.to_string(),

            ShowcaseVariant {
                title: "Variants".to_string(),
                div { class: "space-y-4",
                    Input {
                        variant: InputVariant::Default,
                        placeholder: "Default input",
                        value: default_input.read().clone(),
                        oninput: move |value: String| default_input.set(value),
                    }
                    Input {
                        variant: InputVariant::Filled,
                        placeholder: "Filled input",
                        value: filled_input.read().clone(),
                        oninput: move |value: String| filled_input.set(value),
                    }
                }
            }

            ShowcaseVariant {
                title: "Types".to_string(),
                div { class: "space-y-4",
                    Input {
                        input_type: InputType::Password,
                        placeholder: "Password",
                        value: password_input.read().clone(),
                        oninput: move |value: String| password_input.set(value),
                    }
                    Input {
                        input_type: InputType::Email,
                        placeholder: "email@example.com",
                    }
                    Input {
                        input_type: InputType::Number,
                        placeholder: "42",
                    }
                }
            }

            ShowcaseVariant {
                title: "States".to_string(),
                div { class: "space-y-4",
                    Input {
                        placeholder: "Normal input",
                    }
                    Input {
                        disabled: true,
                        placeholder: "Disabled input",
                    }
                    Input {
                        error: true,
                        placeholder: "Error state",
                        value: error_input.read().clone(),
                        oninput: move |value: String| error_input.set(value),
                    }
                }
            }

            ShowcaseVariant {
                title: "Sizes".to_string(),
                div { class: "space-y-4",
                    Input {
                        size: InputSize::Small,
                        placeholder: "Small",
                    }
                    Input {
                        size: InputSize::Medium,
                        placeholder: "Medium (default)",
                    }
                    Input {
                        size: InputSize::Large,
                        placeholder: "Large",
                    }
                }
            }
        }
    }
}
