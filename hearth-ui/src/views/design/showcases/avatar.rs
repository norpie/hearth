use crate::components::Avatar;
use crate::views::design::showcase::{ComponentShowcase, ShowcaseVariant};
use dioxus::prelude::*;

#[component]
pub fn AvatarShowcase() -> Element {
    rsx! {
        ComponentShowcase {
            name: "Avatar".to_string(),
            description: "Display user profile pictures with fallback to initials".to_string(),
            basic_usage: r#"rsx! {
    Avatar {
        name: "John Doe"
    }
}"#.to_string(),
            with_props_usage: r#"rsx! {
    Avatar {
        name: "Jane Smith",
        avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        size: Some("w-16 h-16".to_string()),
        onclick: |_| {
            // Handle click event
        },
        on_hover: |hovering| {
            // Handle hover state
        }
    }
}"#.to_string(),

            ShowcaseVariant {
                title: "Default (with initials)".to_string(),
                div { class: "flex items-center gap-4",
                    Avatar { name: "John Doe".to_string() }
                    Avatar { name: "Jane Smith".to_string() }
                    Avatar { name: "Bob Johnson".to_string() }
                    Avatar { name: "Alice".to_string() }
                }
            }

            ShowcaseVariant {
                title: "With Images".to_string(),
                div { class: "flex items-center gap-4",
                    Avatar {
                        name: "User One".to_string(),
                        avatar_url: Some("https://picsum.photos/200/200?random=1".to_string())
                    }
                    Avatar {
                        name: "User Two".to_string(),
                        avatar_url: Some("https://picsum.photos/200/200?random=2".to_string())
                    }
                    Avatar {
                        name: "User Three".to_string(),
                        avatar_url: Some("https://picsum.photos/200/200?random=3".to_string())
                    }
                }
            }

            ShowcaseVariant {
                title: "Different Sizes".to_string(),
                div { class: "flex items-end gap-4",
                    Avatar {
                        name: "Small".to_string(),
                        size: Some("w-6 h-6".to_string())
                    }
                    Avatar {
                        name: "Medium".to_string(),
                        size: Some("w-10 h-10".to_string())
                    }
                    Avatar {
                        name: "Default".to_string(),
                    }
                    Avatar {
                        name: "Large".to_string(),
                        size: Some("w-16 h-16".to_string())
                    }
                    Avatar {
                        name: "XL".to_string(),
                        size: Some("w-20 h-20".to_string())
                    }
                }
            }

            ShowcaseVariant {
                title: "Clickable Avatars".to_string(),
                div { class: "flex items-center gap-4",
                    Avatar {
                        name: "Click Me".to_string(),
                        onclick: move |_| {}
                    }
                    Avatar {
                        name: "Also Clickable".to_string(),
                        avatar_url: Some("https://picsum.photos/200/200?random=4".to_string()),
                        onclick: move |_| {}
                    }
                }
            }

            ShowcaseVariant {
                title: "Single Letter Names".to_string(),
                div { class: "flex items-center gap-4",
                    Avatar { name: "A".to_string() }
                    Avatar { name: "B".to_string() }
                    Avatar { name: "X".to_string() }
                    Avatar { name: "Z".to_string() }
                }
            }
        }
    }
}
