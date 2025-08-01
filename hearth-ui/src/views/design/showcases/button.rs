use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Button, ButtonVariant, ToastConfig, ToastManager, ToastType};
use dioxus::prelude::*;

#[derive(Props, Clone)]
pub struct ButtonShowcaseProps {
    pub toaster: ToastManager,
}

impl PartialEq for ButtonShowcaseProps {
    fn eq(&self, _other: &Self) -> bool {
        // ToastManager doesn't implement PartialEq, so we'll always return true
        // This is okay for component props since toaster manager is typically unique
        true
    }
}

#[component]
pub fn button_showcase(props: ButtonShowcaseProps) -> Element {
    let ButtonShowcaseProps { toaster } = props;
    rsx! {
        ComponentShowcase {
            name: "Button".to_string(),
            description: "Interactive button component with multiple variants and states.".to_string(),
            basic_usage: r#"Button {
    "Click me"
}"#.to_string(),
            with_props_usage: r#"Button {
    variant: ButtonVariant::Primary,
    disabled: false,
    loading: false,
    onclick: move |_| {
        // Handle click event
    },
    "Custom Button Text"
}"#.to_string(),

            ShowcaseVariant {
                title: "Variants".to_string(),
                div { class: "flex flex-wrap gap-3",
                    Button {
                        variant: ButtonVariant::Primary,
                        "Primary"
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        "Secondary"
                    }
                    Button {
                        variant: ButtonVariant::Outline,
                        "Outline"
                    }
                    Button {
                        variant: ButtonVariant::Ghost,
                        "Ghost"
                    }
                    Button {
                        variant: ButtonVariant::Destructive,
                        "Destructive"
                    }
                }
            }

            ShowcaseVariant {
                title: "States".to_string(),
                div { class: "flex flex-wrap gap-3",
                    Button { "Default" }
                    Button {
                        disabled: true,
                        "Disabled"
                    }
                    Button {
                        onclick: move |_| {
                            toaster.add_toast(ToastConfig {
                                message: "Button clicked!".to_string(),
                                toast_type: ToastType::Success,
                                duration: Some(std::time::Duration::from_secs(3)),
                                dismissible: true,
                            });
                        },
                        "With Action"
                    }
                }
            }
        }
    }
}
