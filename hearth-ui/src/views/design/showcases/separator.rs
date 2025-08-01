use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Label, Separator, SeparatorOrientation, SeparatorSize, SeparatorVariant};
use dioxus::prelude::*;

#[component]
pub fn separator_showcase() -> Element {
    rsx! {
        ComponentShowcase {
            name: "Separator".to_string(),
            description: "Visual divider component for separating content with multiple styles and orientations.".to_string(),
            basic_usage: r#"Separator {}"#.to_string(),
            with_props_usage: r#"Separator {
    orientation: SeparatorOrientation::Horizontal,
    size: SeparatorSize::Medium,
    variant: SeparatorVariant::Default,
    decorative: true,
    class: "custom-class",
}"#.to_string(),

            ShowcaseVariant {
                title: "Variants".to_string(),
                div { class: "space-y-6",
                    div {
                        Label { "Default" }
                        Separator {
                            variant: SeparatorVariant::Default,
                        }
                    }
                    div {
                        Label { "Subtle" }
                        Separator {
                            variant: SeparatorVariant::Subtle,
                        }
                    }
                    div {
                        Label { "Bold" }
                        Separator {
                            variant: SeparatorVariant::Bold,
                        }
                    }
                    div {
                        Label { "Dashed" }
                        Separator {
                            variant: SeparatorVariant::Dashed,
                        }
                    }
                    div {
                        Label { "Dotted" }
                        Separator {
                            variant: SeparatorVariant::Dotted,
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Sizes".to_string(),
                div { class: "space-y-6",
                    div {
                        Label { "Small" }
                        Separator {
                            size: SeparatorSize::Small,
                        }
                    }
                    div {
                        Label { "Medium (default)" }
                        Separator {
                            size: SeparatorSize::Medium,
                        }
                    }
                    div {
                        Label { "Large" }
                        Separator {
                            size: SeparatorSize::Large,
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Orientations".to_string(),
                div { class: "space-y-6",
                    div {
                        Label { "Horizontal (default)" }
                        Separator {
                            orientation: SeparatorOrientation::Horizontal,
                        }
                    }
                    div {
                        Label { "Vertical" }
                        div { class: "flex items-center h-20 space-x-4",
                            span { "Content" }
                            Separator {
                                orientation: SeparatorOrientation::Vertical,
                            }
                            span { "More Content" }
                        }
                    }
                }
            }
        }
    }
}
