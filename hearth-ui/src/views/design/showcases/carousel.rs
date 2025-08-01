use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::Carousel;
use dioxus::prelude::*;

#[component]
pub fn carousel_showcase() -> Element {
    rsx! {
        ComponentShowcase {
            name: "Carousel".to_string(),
            description: "Interactive carousel component for displaying multiple items with navigation.".to_string(),
            basic_usage: r#"Carousel {
    items: vec![
        rsx! { "Item 1" },
        rsx! { "Item 2" },
        rsx! { "Item 3" }
    ]
}"#.to_string(),
            with_props_usage: r#"Carousel {
    items: carousel_items,
    aspect_ratio: 16.0 / 9.0,
    show_navigation: true,
    show_indicators: true,
    on_change: move |index| {
        // Handle carousel change
    }
}"#.to_string(),

            ShowcaseVariant {
                title: "Basic Carousel".to_string(),
                div { class: "space-y-4",
                    Carousel {
                        items: vec![
                            rsx! {
                                div { class: "h-full w-full bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white font-semibold text-xl",
                                    "Slide 1"
                                }
                            },
                            rsx! {
                                div { class: "h-full w-full bg-gradient-to-br from-pink-500 to-red-500 flex items-center justify-center text-white font-semibold text-xl",
                                    "Slide 2"
                                }
                            },
                            rsx! {
                                div { class: "h-full w-full bg-gradient-to-br from-green-500 to-emerald-600 flex items-center justify-center text-white font-semibold text-xl",
                                    "Slide 3"
                                }
                            }
                        ]
                    }
                }
            }

            ShowcaseVariant {
                title: "Image Carousel".to_string(),
                div { class: "space-y-4",
                    Carousel {
                        items: vec![
                            rsx! {
                                div { class: "h-full w-full bg-muted flex items-center justify-center relative overflow-hidden",
                                    div { class: "absolute inset-0 bg-gradient-to-r from-orange-400 to-red-500 opacity-80" }
                                    div { class: "relative z-10 text-white font-medium text-lg", "Mountain Landscape" }
                                }
                            },
                            rsx! {
                                div { class: "h-full w-full bg-muted flex items-center justify-center relative overflow-hidden",
                                    div { class: "absolute inset-0 bg-gradient-to-r from-blue-400 to-cyan-500 opacity-80" }
                                    div { class: "relative z-10 text-white font-medium text-lg", "Ocean View" }
                                }
                            },
                            rsx! {
                                div { class: "h-full w-full bg-muted flex items-center justify-center relative overflow-hidden",
                                    div { class: "absolute inset-0 bg-gradient-to-r from-purple-400 to-pink-500 opacity-80" }
                                    div { class: "relative z-10 text-white font-medium text-lg", "City Skyline" }
                                }
                            }
                        ]
                    }
                }
            }

            ShowcaseVariant {
                title: "Different Aspect Ratios".to_string(),
                div { class: "space-y-6",
                    div {
                        p { class: "text-sm font-medium mb-2", "Square (1:1)" }
                        Carousel {
                            aspect_ratio: 1.0,
                            items: vec![
                                rsx! {
                                    div { class: "h-full w-full bg-gradient-to-br from-yellow-400 to-orange-500 flex items-center justify-center text-white font-semibold",
                                        "Square 1"
                                    }
                                },
                                rsx! {
                                    div { class: "h-full w-full bg-gradient-to-br from-teal-400 to-green-500 flex items-center justify-center text-white font-semibold",
                                        "Square 2"
                                    }
                                }
                            ]
                        }
                    }

                    div {
                        p { class: "text-sm font-medium mb-2", "Portrait (3:4)" }
                        Carousel {
                            aspect_ratio: 3.0 / 4.0,
                            items: vec![
                                rsx! {
                                    div { class: "h-full w-full bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center text-white font-semibold",
                                        "Portrait 1"
                                    }
                                },
                                rsx! {
                                    div { class: "h-full w-full bg-gradient-to-br from-rose-400 to-pink-600 flex items-center justify-center text-white font-semibold",
                                        "Portrait 2"
                                    }
                                }
                            ]
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Content Cards".to_string(),
                div { class: "space-y-4",
                    Carousel {
                        items: vec![
                            rsx! {
                                div { class: "h-full w-full bg-card border border-border rounded-lg p-6 flex flex-col justify-between",
                                    div {
                                        h3 { class: "text-xl font-bold text-foreground mb-2", "Feature 1" }
                                        p { class: "text-foreground", "Powerful and flexible component system with modern design patterns." }
                                    }
                                    div { class: "text-sm text-primary-foreground font-medium", "Learn more →" }
                                }
                            },
                            rsx! {
                                div { class: "h-full w-full bg-card border border-border rounded-lg p-6 flex flex-col justify-between",
                                    div {
                                        h3 { class: "text-xl font-bold text-foreground mb-2", "Feature 2" }
                                        p { class: "text-foreground", "Cross-platform compatibility with seamless user experience across devices." }
                                    }
                                    div { class: "text-sm text-primary-foreground font-medium", "Learn more →" }
                                }
                            },
                            rsx! {
                                div { class: "h-full w-full bg-card border border-border rounded-lg p-6 flex flex-col justify-between",
                                    div {
                                        h3 { class: "text-xl font-bold text-foreground mb-2", "Feature 3" }
                                        p { class: "text-foreground", "Built-in accessibility features and keyboard navigation support." }
                                    }
                                    div { class: "text-sm text-primary-foreground font-medium", "Learn more →" }
                                }
                            }
                        ]
                    }
                }
            }

            ShowcaseVariant {
                title: "Without Navigation".to_string(),
                div { class: "space-y-4",
                    Carousel {
                        show_navigation: false,
                        show_indicators: true,
                        items: vec![
                            rsx! {
                                div { class: "h-full w-full bg-gradient-to-br from-emerald-500 to-teal-600 flex items-center justify-center text-white font-semibold text-xl",
                                    "Only indicators"
                                }
                            },
                            rsx! {
                                div { class: "h-full w-full bg-gradient-to-br from-sky-500 to-blue-600 flex items-center justify-center text-white font-semibold text-xl",
                                    "No navigation buttons"
                                }
                            }
                        ]
                    }
                }
            }

            ShowcaseVariant {
                title: "Single Item".to_string(),
                div { class: "space-y-4",
                    Carousel {
                        items: vec![
                            rsx! {
                                div { class: "h-full w-full bg-gradient-to-br from-slate-500 to-gray-600 flex items-center justify-center text-white font-semibold text-xl",
                                    "Single item (no navigation)"
                                }
                            }
                        ]
                    }
                }
            }

            ShowcaseVariant {
                title: "Empty Carousel".to_string(),
                div { class: "space-y-4",
                    Carousel {
                        items: vec![]
                    }
                }
            }
        }
    }
}
