use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Input, Label};
use dioxus::prelude::*;

#[component]
pub fn label_showcase() -> Element {
    rsx! {
        ComponentShowcase {
            name: "Label".to_string(),
            description: "Form label component for accessibility and styling.".to_string(),
            basic_usage: r#"Label {
    "Email Address"
}"#.to_string(),
            with_props_usage: r#"Label {
    r#for: "email-input",
    required: true,
    class: "custom-class",
    "Email Address"
}"#.to_string(),

            ShowcaseVariant {
                title: "Basic Usage".to_string(),
                div { class: "space-y-4",
                    div {
                        Label {
                            r#for: "email-input",
                            "Email Address"
                        }
                        Input {
                            id: "email-input",
                            placeholder: "Enter your email",
                        }
                    }
                    div {
                        Label {
                            required: true,
                            "Required Field"
                        }
                        Input {
                            placeholder: "This field is required",
                        }
                    }
                }
            }
        }
    }
}
