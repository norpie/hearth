use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Label, Progress};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProgressShowcaseProps {
    pub animated_progress: Signal<f64>,
    pub staggered_progress: Signal<f64>,
}

#[component]
pub fn progress_showcase(props: ProgressShowcaseProps) -> Element {
    let ProgressShowcaseProps {
        animated_progress,
        staggered_progress,
    } = props;
    rsx! {
        ComponentShowcase {
            name: "Progress".to_string(),
            description: "Progress indicator component for showing completion status.".to_string(),
            basic_usage: r#"Progress {
    value: 50.0,
}"#.to_string(),
            with_props_usage: r#"Progress {
    value: 75.0,
    max: 100.0,
    size: ProgressSize::Medium,
    show_percentage: true,
    label: Some("Loading...".to_string()),
    class: "custom-class",
}"#.to_string(),

            ShowcaseVariant {
                title: "Basic Usage".to_string(),
                div { class: "space-y-6",
                    Progress {
                        value: 25.0,
                        label: Some("Loading...".to_string()),
                        show_percentage: true,
                    }
                    Progress {
                        value: 65.0,
                        show_percentage: true,
                    }
                    Progress {
                        value: 100.0,
                        label: Some("Complete".to_string()),
                    }
                }
            }

            ShowcaseVariant {
                title: "Animated Examples".to_string(),
                div { class: "space-y-6",
                    div {
                        Label { "Smooth Animation" }
                        Progress {
                            value: animated_progress(),
                            label: Some("Processing...".to_string()),
                            show_percentage: true,
                        }
                    }
                    div {
                        Label { "Realistic Loading Pattern" }
                        Progress {
                            value: staggered_progress(),
                            label: Some("Loading assets...".to_string()),
                            show_percentage: true,
                        }
                    }
                }
            }
        }
    }
}
