use dioxus::prelude::*;
use crate::AspectRatio;
use super::super::{ComponentShowcase, ShowcaseVariant};

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
                            class: "bg-gray-100 dark:bg-gray-800 rounded-md",
                            div { class: "flex items-center justify-center h-full text-gray-600 dark:text-gray-400 text-sm",
                                "16:9"
                            }
                        }
                    }
                    
                    div {
                        p { class: "text-sm font-medium mb-2", "4:3 (Standard)" }
                        AspectRatio {
                            ratio: 4.0 / 3.0,
                            class: "bg-gray-100 dark:bg-gray-800 rounded-md",
                            div { class: "flex items-center justify-center h-full text-gray-600 dark:text-gray-400 text-sm",
                                "4:3"
                            }
                        }
                    }
                    
                    div {
                        p { class: "text-sm font-medium mb-2", "1:1 (Square)" }
                        AspectRatio {
                            ratio: 1.0,
                            class: "bg-gray-100 dark:bg-gray-800 rounded-md",
                            div { class: "flex items-center justify-center h-full text-gray-600 dark:text-gray-400 text-sm",
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
                            class: "bg-gray-200 dark:bg-gray-700 rounded-md overflow-hidden",
                            div { class: "h-full w-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center text-white font-medium",
                                "Sample Image"
                            }
                        }
                    }
                    
                    div {
                        p { class: "text-sm font-medium mb-2", "Portrait (3:4)" }
                        AspectRatio {
                            ratio: 3.0 / 4.0,
                            class: "bg-gray-200 dark:bg-gray-700 rounded-md overflow-hidden",
                            div { class: "h-full w-full bg-gradient-to-br from-purple-400 to-purple-600 flex items-center justify-center text-white font-medium",
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
                            class: "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-md shadow-sm",
                            div { class: "h-full w-full p-4 flex flex-col justify-between",
                                div {
                                    h3 { class: "font-semibold text-gray-900 dark:text-gray-100", "Card Title" }
                                    p { class: "text-sm text-gray-600 dark:text-gray-400 mt-1", "Card description content." }
                                }
                                div { class: "text-xs text-gray-500 dark:text-gray-500", "Footer content" }
                            }
                        }
                    }
                }
            }
        }
    }
}