use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{ScrollArea, ScrollOrientation, FadeMode, ScrollbarVisibility, ScrollControl, ScrollAction, Button, ButtonVariant, ButtonSize};
use dioxus::prelude::*;

#[component]
pub fn scroll_area_showcase() -> Element {
    rsx! {
        ComponentShowcase {
            name: "ScrollArea".to_string(),
            description: "Custom scrollable area with fade effects, multiple orientations, and configurable scrollbars. Perfect for content lists, chat messages, and constrained layouts.".to_string(),
            basic_usage: r#"ScrollArea {
    height: "h-48".to_string(),
    div { class: "p-4 space-y-2",
        for item in items {
            div { class: "p-2 bg-muted rounded", "{item}" }
        }
    }
}"#.to_string(),
            with_props_usage: r#"ScrollArea {
    height: "h-72".to_string(),
    orientation: ScrollOrientation::Both,
    fade_mode: FadeMode::Both,
    scrollbar_visibility: ScrollbarVisibility::Always,
    class: "border rounded-lg",
    viewport_class: "bg-card",
    // content here
}"#.to_string(),

            ShowcaseVariant {
                title: "Fade Effects with Different Backgrounds".to_string(),
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                    div { class: "space-y-2",
                        h4 { class: "text-sm font-medium text-muted-foreground", "Default (Theme Background)" }
                        ScrollArea {
                            height: "h-48".to_string(),
                            fade_mode: FadeMode::Both,
                            fade_color: Some("from-card".to_string()),
                            class: "border border-border rounded-lg bg-card",
                            div { class: "p-4 space-y-2",
                                for i in 1..25 {
                                    div { class: "p-3 bg-muted rounded-md flex items-center justify-between",
                                        span { "List Item {i}" }
                                        span { class: "text-xs text-muted-foreground", "Value {i * 10}" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "space-y-2",
                        h4 { class: "text-sm font-medium text-muted-foreground", "Card Background" }
                        ScrollArea {
                            height: "h-48".to_string(),
                            fade_mode: FadeMode::Both,
                            fade_color: Some("from-card".to_string()),
                            class: "border border-border rounded-lg bg-card",
                            div { class: "p-4 space-y-2",
                                for i in 1..25 {
                                    div { class: "p-3 bg-muted rounded-md flex items-center justify-between",
                                        span { "List Item {i}" }
                                        span { class: "text-xs text-muted-foreground", "Value {i * 10}" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "space-y-2",
                        h4 { class: "text-sm font-medium text-muted-foreground", "Custom Color" }
                        ScrollArea {
                            height: "h-48".to_string(),
                            fade_mode: FadeMode::Both,
                            fade_color: Some("from-blue-50".to_string()),
                            class: "border border-border rounded-lg bg-blue-50",
                            div { class: "p-4 space-y-2",
                                for i in 1..25 {
                                    div { class: "p-3 bg-blue-100 rounded-md flex items-center justify-between",
                                        span { "List Item {i}" }
                                        span { class: "text-xs text-blue-600", "Value {i * 10}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Horizontal Scrolling".to_string(),
                div { class: "space-y-4",
                    ScrollArea {
                        orientation: ScrollOrientation::Horizontal,
                        height: "h-28".to_string(),
                        width: "100%".to_string(),
                        fade_mode: FadeMode::Both,
                        fade_color: Some("from-card".to_string()),
                        class: "border border-border rounded-lg bg-card",
                        div { class: "flex space-x-4 p-4 w-max",
                            for i in 1..15 {
                                div { class: "flex-shrink-0 w-32 h-20 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center text-white font-medium",
                                    "Card {i}"
                                }
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Chat-like Messages".to_string(),
                div { class: "space-y-4",
                    ScrollArea {
                        height: "h-60".to_string(),
                        fade_mode: FadeMode::Both,
                        fade_color: Some("from-card".to_string()),
                        class: "border border-border rounded-lg bg-card",
                        div { class: "p-4 space-y-3",
                            // Simulated chat messages
                            div { class: "flex justify-start",
                                div { class: "max-w-xs bg-muted rounded-lg p-3",
                                    div { class: "text-xs text-muted-foreground mb-1", "Alice • 2:30 PM" }
                                    div { "Hey everyone! How's the new design system coming along?" }
                                }
                            }
                            div { class: "flex justify-end",
                                div { class: "max-w-xs bg-primary text-primary-foreground rounded-lg p-3",
                                    div { class: "text-xs opacity-80 mb-1", "You • 2:32 PM" }
                                    div { "It's looking great! The ScrollArea component is particularly useful for chat interfaces like this." }
                                }
                            }
                            div { class: "flex justify-start",
                                div { class: "max-w-xs bg-muted rounded-lg p-3",
                                    div { class: "text-xs text-muted-foreground mb-1", "Bob • 2:33 PM" }
                                    div { "Agreed! The fade effects make it feel really polished." }
                                }
                            }
                            div { class: "flex justify-start",
                                div { class: "max-w-xs bg-muted rounded-lg p-3",
                                    div { class: "text-xs text-muted-foreground mb-1", "Charlie • 2:35 PM" }
                                    div { "Can we also use it for the sidebar navigation? It would help with longer menu lists." }
                                }
                            }
                            div { class: "flex justify-end",
                                div { class: "max-w-xs bg-primary text-primary-foreground rounded-lg p-3",
                                    div { class: "text-xs opacity-80 mb-1", "You • 2:36 PM" }
                                    div { "Absolutely! That's exactly the kind of use case it was designed for." }
                                }
                            }
                            div { class: "flex justify-start",
                                div { class: "max-w-xs bg-muted rounded-lg p-3",
                                    div { class: "text-xs text-muted-foreground mb-1", "Alice • 2:37 PM" }
                                    div { "Perfect! Let's implement it in the next sprint." }
                                }
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Both Direction Scrolling".to_string(),
                div { class: "space-y-4",
                    ScrollArea {
                        height: "h-48".to_string(),
                        orientation: ScrollOrientation::Both,
                        fade_mode: FadeMode::Both,
                        fade_color: Some("from-card".to_string()),
                        class: "border border-border rounded-lg bg-card",
                        div { class: "p-4",
                            div { class: "w-[600px] h-[300px] bg-gradient-to-br from-purple-500 via-blue-500 to-green-500 rounded-lg flex items-center justify-center text-white font-bold text-lg",
                                "Scroll both horizontally and vertically to see all content!"
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Scrollbar Visibility Options".to_string(),
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                    div { class: "space-y-2",
                        h4 { class: "text-sm font-medium text-muted-foreground", "Auto (Default)" }
                        ScrollArea {
                            height: "h-36".to_string(),
                            scrollbar_visibility: ScrollbarVisibility::Auto,
                            fade_color: Some("from-card".to_string()),
                            class: "border border-border rounded-lg bg-card",
                            div { class: "p-4 space-y-2",
                                for i in 1..15 {
                                    div { class: "p-2 bg-muted rounded text-sm", "Auto scrollbar {i}" }
                                }
                            }
                        }
                    }
                    div { class: "space-y-2",
                        h4 { class: "text-sm font-medium text-muted-foreground", "Always Visible" }
                        ScrollArea {
                            height: "h-36".to_string(),
                            scrollbar_visibility: ScrollbarVisibility::Always,
                            fade_color: Some("from-card".to_string()),
                            class: "border border-border rounded-lg bg-card",
                            div { class: "p-4 space-y-2",
                                for i in 1..15 {
                                    div { class: "p-2 bg-muted rounded text-sm", "Always visible {i}" }
                                }
                            }
                        }
                    }
                    div { class: "space-y-2",
                        h4 { class: "text-sm font-medium text-muted-foreground", "Hidden" }
                        ScrollArea {
                            height: "h-36".to_string(),
                            scrollbar_visibility: ScrollbarVisibility::Never,
                            fade_color: Some("from-card".to_string()),
                            class: "border border-border rounded-lg bg-card",
                            div { class: "p-4 space-y-2",
                                for i in 1..15 {
                                    div { class: "p-2 bg-muted rounded text-sm", "Hidden scrollbar {i}" }
                                }
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Real-world Use Cases".to_string(),
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    div { class: "space-y-2",
                        h4 { class: "text-sm font-medium text-muted-foreground", "File Explorer" }
                        ScrollArea {
                            height: "h-44".to_string(),
                            fade_mode: FadeMode::Both,
                            fade_color: Some("from-card".to_string()),
                            class: "border border-border rounded-lg bg-card",
                            div { class: "p-2",
                                for (i, name) in ["📁 Documents", "📁 Images", "📁 Videos", "📄 README.md", "📄 package.json", "📄 tailwind.config.js", "📄 tsconfig.json", "📁 src", "📁 public", "📁 assets", "📄 .gitignore", "📄 .env.local", "📁 components", "📁 utils", "📄 main.rs"].iter().enumerate() {
                                    div { class: "flex items-center space-x-2 p-2 hover:bg-muted rounded text-sm cursor-pointer",
                                        span { "{name}" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "space-y-2",
                        h4 { class: "text-sm font-medium text-muted-foreground", "Notification Feed" }
                        ScrollArea {
                            height: "h-44".to_string(),
                            fade_mode: FadeMode::Both,
                            fade_color: Some("from-card".to_string()),
                            class: "border border-border rounded-lg bg-card",
                            div { class: "divide-y divide-border",
                                for (i, (title, time, type_)) in [
                                    ("New message from Alice", "2 min ago", "💬"),
                                    ("Build completed successfully", "5 min ago", "✅"),
                                    ("Pull request merged", "12 min ago", "🔀"),
                                    ("Security update available", "1 hr ago", "🔒"),
                                    ("Weekly report generated", "2 hrs ago", "📊"),
                                    ("Backup completed", "3 hrs ago", "💾"),
                                    ("New team member joined", "1 day ago", "👋"),
                                    ("Server maintenance scheduled", "2 days ago", "🔧")
                                ].iter().enumerate() {
                                    div { class: "p-3 hover:bg-muted/50 cursor-pointer",
                                        div { class: "flex items-start space-x-3",
                                            span { class: "text-lg", "{type_}" }
                                            div { class: "flex-1 min-w-0",
                                                div { class: "text-sm font-medium text-foreground", "{title}" }
                                                div { class: "text-xs text-muted-foreground mt-1", "{time}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Programmatic Scroll Control".to_string(),
                div { class: "space-y-4",
                    div { class: "text-sm text-muted-foreground",
                        "Use the new scrolling API to control scroll position programmatically. Perfect for auto-scrolling chat messages, jumping to sections, or implementing scroll-to-top functionality."
                    }
                    
                    // Create scroll controller signal
                    {
                        let scroll_controller = use_signal(|| None::<ScrollAction>);
                        
                        rsx! {
                            div { class: "flex flex-wrap gap-2 mb-4",
                                Button {
                                    variant: ButtonVariant::Primary,
                                    size: ButtonSize::Small,
                                    onclick: move |_| ScrollControl::scroll_to_top_smooth(scroll_controller),
                                    "⬆️ Scroll to Top"
                                }
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    size: ButtonSize::Small,
                                    onclick: move |_| ScrollControl::scroll_to_bottom_smooth(scroll_controller),
                                    "⬇️ Scroll to Bottom"
                                }
                                Button {
                                    variant: ButtonVariant::Outline,
                                    size: ButtonSize::Small,
                                    onclick: {
                                        let scroll_controller = scroll_controller;
                                        move |_| {
                                            use crate::{ScrollDelta};
                                            ScrollControl::scroll_by(scroll_controller, ScrollDelta::smooth(0, -100));
                                        }
                                    },
                                    "📜 Scroll Up 100px"
                                }
                                Button {
                                    variant: ButtonVariant::Outline,
                                    size: ButtonSize::Small,
                                    onclick: {
                                        let scroll_controller = scroll_controller;
                                        move |_| {
                                            use crate::{ScrollDelta};
                                            ScrollControl::scroll_by(scroll_controller, ScrollDelta::smooth(0, 100));
                                        }
                                    },
                                    "📜 Scroll Down 100px"
                                }
                            }
                            
                            ScrollArea {
                                height: "h-60".to_string(),
                                fade_mode: FadeMode::Both,
                                fade_color: Some("from-card".to_string()),
                                class: "border border-border rounded-lg bg-card",
                                scroll_controller_signal: Some(scroll_controller),
                                div { class: "p-4 space-y-3",
                                    div { class: "text-lg font-semibold text-center p-4 bg-primary text-primary-foreground rounded-lg",
                                        "🎯 Start of Content"
                                    }
                                    
                                    for i in 1..=30 {
                                        div { class: "p-3 bg-muted rounded-md flex items-center justify-between",
                                            span { "Message {i}" }
                                            span { class: "text-xs text-muted-foreground", 
                                                if i % 5 == 0 { "⭐ Important" } else { "📝 Regular" }
                                            }
                                        }
                                    }
                                    
                                    div { class: "text-lg font-semibold text-center p-4 bg-secondary text-secondary-foreground rounded-lg",
                                        "🏁 End of Content"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
