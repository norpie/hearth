use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::AspectRatio;
use dioxus::prelude::*;

#[component]
pub fn aspect_ratio_showcase() -> Element {
    rsx! {
        ComponentShowcase {
            name: "AspectRatio".to_string(),
            description: "Container that maintains a specific aspect ratio for its content.".to_string(),
            basic_usage: r#"AspectRatio {
    ratio: 16.0 / 9.0,
    "Content"
}"#.to_string(),
            with_props_usage: r#"AspectRatio {
    ratio: 16.0 / 9.0,
    class: "bg-muted",
    img {
        src: "https://example.com/image.jpg",
        alt: "Example image",
        class: "h-full w-full rounded-md object-cover"
    }
}"#.to_string(),

            ShowcaseVariant {
                title: "Common Ratios".to_string(),
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    div {
                        p { class: "text-sm font-medium mb-2", "16:9 (Widescreen)" }
                        AspectRatio {
                            ratio: 16.0 / 9.0,
                            class: "bg-muted rounded-md",
                            div { class: "flex items-center justify-center h-full text-foreground text-sm",
                                "16:9"
                            }
                        }
                    }

                    div {
                        p { class: "text-sm font-medium mb-2", "4:3 (Standard)" }
                        AspectRatio {
                            ratio: 4.0 / 3.0,
                            class: "bg-muted rounded-md",
                            div { class: "flex items-center justify-center h-full text-foreground text-sm",
                                "4:3"
                            }
                        }
                    }

                    div {
                        p { class: "text-sm font-medium mb-2", "1:1 (Square)" }
                        AspectRatio {
                            ratio: 1.0,
                            class: "bg-muted rounded-md",
                            div { class: "flex items-center justify-center h-full text-foreground text-sm",
                                "1:1"
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "With Images".to_string(),
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    div {
                        p { class: "text-sm font-medium mb-2", "Landscape (16:9)" }
                        AspectRatio {
                            ratio: 16.0 / 9.0,
                            class: "bg-muted rounded-md overflow-hidden",
                            div { class: "h-full w-full bg-gradient-to-br from-primary/60 to-primary flex items-center justify-center text-white font-medium",
                                "Sample Image"
                            }
                        }
                    }

                    div {
                        p { class: "text-sm font-medium mb-2", "Portrait (3:4)" }
                        AspectRatio {
                            ratio: 3.0 / 4.0,
                            class: "bg-muted rounded-md overflow-hidden",
                            div { class: "h-full w-full bg-gradient-to-br from-accent/60 to-accent flex items-center justify-center text-white font-medium",
                                "Sample Image"
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Custom Content".to_string(),
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    div {
                        p { class: "text-sm font-medium mb-2", "Video Player (16:9)" }
                        AspectRatio {
                            ratio: 16.0 / 9.0,
                            class: "bg-black rounded-md overflow-hidden",
                            div { class: "h-full w-full flex items-center justify-center",
                                div { class: "w-16 h-16 rounded-full bg-white bg-opacity-20 flex items-center justify-center",
                                    div { class: "w-0 h-0 border-l-8 border-l-white border-y-6 border-y-transparent ml-1" }
                                }
                            }
                        }
                    }

                    div {
                        p { class: "text-sm font-medium mb-2", "Card Layout (2:3)" }
                        AspectRatio {
                            ratio: 2.0 / 3.0,
                            class: "bg-card border border-border rounded-md shadow-sm",
                            div { class: "h-full w-full p-4 flex flex-col justify-between",
                                div {
                                    h3 { class: "font-semibold text-foreground", "Card Title" }
                                    p { class: "text-sm text-foreground mt-1", "Card description content." }
                                }
                                div { class: "text-xs text-foreground", "Footer content" }
                            }
                        }
                    }
                }
            }
        }
    }
}
