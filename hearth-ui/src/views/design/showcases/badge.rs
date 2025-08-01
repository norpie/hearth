use crate::components::{Badge, BadgeSize, BadgeVariant};
use crate::views::design::showcase::{ComponentShowcase, ShowcaseVariant};
use dioxus::prelude::*;

#[component]
pub fn BadgeShowcase() -> Element {
    rsx! {
        ComponentShowcase {
            name: "Badge".to_string(),
            description: "Display status, labels, and other metadata".to_string(),
            basic_usage: r#"rsx! {
    Badge {
        "New"
    }
}"#.to_string(),
            with_props_usage: r#"rsx! {
    Badge {
        variant: BadgeVariant::Success,
        size: BadgeSize::Large,
        onclick: |_| {
            // Handle click event
        },
        "Completed"
    }
}"#.to_string(),

            ShowcaseVariant {
                title: "Variants".to_string(),
                div { class: "flex flex-wrap items-center gap-2",
                    Badge { variant: BadgeVariant::Default, "Default" }
                    Badge { variant: BadgeVariant::Secondary, "Secondary" }
                    Badge { variant: BadgeVariant::Success, "Success" }
                    Badge { variant: BadgeVariant::Warning, "Warning" }
                    Badge { variant: BadgeVariant::Error, "Error" }
                    Badge { variant: BadgeVariant::Info, "Info" }
                    Badge { variant: BadgeVariant::Outline, "Outline" }
                }
            }

            ShowcaseVariant {
                title: "Sizes".to_string(),
                div { class: "flex items-center gap-3",
                    Badge { size: BadgeSize::Small, "Small" }
                    Badge { size: BadgeSize::Medium, "Medium" }
                    Badge { size: BadgeSize::Large, "Large" }
                }
            }

            ShowcaseVariant {
                title: "Interactive Badges".to_string(),
                div { class: "flex flex-wrap items-center gap-2",
                    Badge {
                        variant: BadgeVariant::Info,
                        onclick: move |_| {},
                        "Clickable"
                    }
                    Badge {
                        variant: BadgeVariant::Error,
                        onclick: move |_| {},
                        "Remove ×"
                    }
                    Badge {
                        variant: BadgeVariant::Success,
                        size: BadgeSize::Large,
                        onclick: move |_| {},
                        "Action"
                    }
                }
            }

            ShowcaseVariant {
                title: "With Icons/Symbols".to_string(),
                div { class: "flex flex-wrap items-center gap-2",
                    Badge { variant: BadgeVariant::Success, "✓ Verified" }
                    Badge { variant: BadgeVariant::Warning, "⚠ Warning" }
                    Badge { variant: BadgeVariant::Error, "✕ Failed" }
                    Badge { variant: BadgeVariant::Info, "★ Premium" }
                    Badge { variant: BadgeVariant::Secondary, "• Draft" }
                }
            }

            ShowcaseVariant {
                title: "Status Examples".to_string(),
                div { class: "flex flex-wrap items-center gap-2",
                    Badge { variant: BadgeVariant::Success, "Online" }
                    Badge { variant: BadgeVariant::Warning, "Pending" }
                    Badge { variant: BadgeVariant::Error, "Offline" }
                    Badge { variant: BadgeVariant::Info, "In Progress" }
                    Badge { variant: BadgeVariant::Default, "Inactive" }
                    Badge { variant: BadgeVariant::Secondary, size: BadgeSize::Small, "v1.2.0" }
                }
            }

            ShowcaseVariant {
                title: "Numbers & Counts".to_string(),
                div { class: "flex items-center gap-3",
                    Badge { variant: BadgeVariant::Error, size: BadgeSize::Small, "99+" }
                    Badge { variant: BadgeVariant::Info, "42" }
                    Badge { variant: BadgeVariant::Success, "1,234" }
                    Badge { variant: BadgeVariant::Warning, size: BadgeSize::Large, "New: 5" }
                }
            }
        }
    }
}
