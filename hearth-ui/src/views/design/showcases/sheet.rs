//! Sheet component showcase

use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{
    Button, ButtonVariant, Input, Label, Select, SelectOption, Sheet, SheetSide, SheetSize,
    Textarea,
};
use dioxus::prelude::*;

#[component]
pub fn sheet_showcase() -> Element {
    let mut is_right_sheet_open = use_signal(|| false);
    let mut is_left_sheet_open = use_signal(|| false);
    let mut is_top_sheet_open = use_signal(|| false);
    let mut is_bottom_sheet_open = use_signal(|| false);
    let mut is_small_sheet_open = use_signal(|| false);
    let mut is_large_sheet_open = use_signal(|| false);
    let mut is_full_sheet_open = use_signal(|| false);
    let mut is_form_sheet_open = use_signal(|| false);
    let mut is_headerless_sheet_open = use_signal(|| false);
    let mut is_no_backdrop_sheet_open = use_signal(|| false);

    // Form state
    let mut story_title = use_signal(String::new);
    let mut story_genre = use_signal(String::new);
    let mut story_description = use_signal(String::new);
    let mut story_tags = use_signal(String::new);

    rsx! {
        ComponentShowcase {
            name: "Sheet".to_string(),
            description: "Sliding panels that can appear from any side of the screen with background blur and arbitrary content.".to_string(),
            basic_usage: r#"Sheet {
    is_open: sheet_state,
    side: SheetSide::Right,
    title: Some("Sheet Title".to_string()),
    div { class: "p-6", "Sheet content goes here" }
}"#.to_string(),
            with_props_usage: r#"Sheet {
    is_open: navigation_open,
    side: SheetSide::Left,
    size: SheetSize::Medium,
    show_close_button: false,
    close_on_backdrop_click: false,
    title: Some("Navigation".to_string()),
    div { class: "p-6",
        // Navigation content
    }
}"#.to_string(),

            ShowcaseVariant {
                title: "Different Sides".to_string(),

                div { class: "space-y-4",
                    div { class: "grid grid-cols-2 gap-3",
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| is_left_sheet_open.set(true),
                            "← Left Sheet"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| is_right_sheet_open.set(true),
                            "Right Sheet →"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| is_top_sheet_open.set(true),
                            "↑ Top Sheet"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| is_bottom_sheet_open.set(true),
                            "↓ Bottom Sheet"
                        }
                    }

                    p { class: "text-sm text-foreground",
                        "Sheets can slide in from any of the four sides: left, right, top, or bottom with smooth animations."
                    }
                }

                // Right sheet
                Sheet {
                    is_open: is_right_sheet_open,
                    side: SheetSide::Right,
                    title: Some("Right Sheet".to_string()),
                    div { class: "p-6 space-y-4",
                        p { class: "text-foreground",
                            "This sheet slides in from the right side of the screen. It's perfect for navigation menus, settings panels, or detailed information."
                        }
                        ul { class: "list-disc list-inside space-y-2 text-foreground",
                            li { "Smooth slide animation" }
                            li { "Background blur effect" }
                            li { "Escape key support" }
                            li { "Backdrop click to close" }
                        }
                        div { class: "flex justify-end pt-4 border-t border-border",
                            Button {
                                onclick: move |_| is_right_sheet_open.set(false),
                                "Close Sheet"
                            }
                        }
                    }
                }

                // Left sheet
                Sheet {
                    is_open: is_left_sheet_open,
                    side: SheetSide::Left,
                    title: Some("Left Sheet".to_string()),
                    div { class: "p-6 space-y-4",
                        h4 { class: "font-semibold text-foreground", "Navigation Menu" }
                        nav { class: "space-y-2",
                            a { class: "block px-3 py-2 text-foreground hover:bg-muted rounded", href: "#", "Dashboard" }
                            a { class: "block px-3 py-2 text-foreground hover:bg-muted rounded", href: "#", "Characters" }
                            a { class: "block px-3 py-2 text-foreground hover:bg-muted rounded", href: "#", "Stories" }
                            a { class: "block px-3 py-2 text-foreground hover:bg-muted rounded", href: "#", "Settings" }
                        }
                        div { class: "pt-4 border-t border-border",
                            Button {
                                onclick: move |_| is_left_sheet_open.set(false),
                                "Close"
                            }
                        }
                    }
                }

                // Top sheet
                Sheet {
                    is_open: is_top_sheet_open,
                    side: SheetSide::Top,
                    title: Some("Top Sheet".to_string()),
                    div { class: "p-6",
                        div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                            div { class: "text-center",
                                div { class: "w-12 h-12 bg-primary rounded-full flex items-center justify-center mx-auto mb-2",
                                    span { class: "text-primary-foreground font-semibold", "1" }
                                }
                                h5 { class: "font-medium text-foreground", "Step One" }
                                p { class: "text-sm text-foreground", "Configure your settings" }
                            }
                            div { class: "text-center",
                                div { class: "w-12 h-12 bg-success rounded-full flex items-center justify-center mx-auto mb-2",
                                    span { class: "text-success-foreground font-semibold", "2" }
                                }
                                h5 { class: "font-medium text-foreground", "Step Two" }
                                p { class: "text-sm text-foreground", "Import your data" }
                            }
                            div { class: "text-center",
                                div { class: "w-12 h-12 bg-accent/10 rounded-full flex items-center justify-center mx-auto mb-2",
                                    span { class: "text-accent-foreground font-semibold", "3" }
                                }
                                h5 { class: "font-medium text-foreground", "Step Three" }
                                p { class: "text-sm text-foreground", "Start creating" }
                            }
                        }
                        div { class: "flex justify-center pt-6 mt-6 border-t border-border",
                            Button {
                                onclick: move |_| is_top_sheet_open.set(false),
                                "Get Started"
                            }
                        }
                    }
                }

                // Bottom sheet
                Sheet {
                    is_open: is_bottom_sheet_open,
                    side: SheetSide::Bottom,
                    title: Some("Bottom Sheet".to_string()),
                    div { class: "p-6 space-y-4",
                        div { class: "flex items-center space-x-3",
                            div { class: "w-10 h-10 bg-warning rounded-full flex items-center justify-center",
                                svg {
                                    class: "w-5 h-5 text-warning-foreground",
                                    fill: "currentColor",
                                    view_box: "0 0 20 20",
                                    path {
                                        fill_rule: "evenodd",
                                        d: "M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z",
                                        clip_rule: "evenodd"
                                    }
                                }
                            }
                            div {
                                h4 { class: "font-semibold text-foreground", "Important Notice" }
                                p { class: "text-foreground", "This action cannot be undone. Please review before proceeding." }
                            }
                        }
                        div { class: "flex justify-end gap-3",
                            Button {
                                variant: ButtonVariant::Outline,
                                onclick: move |_| is_bottom_sheet_open.set(false),
                                "Cancel"
                            }
                            Button {
                                variant: ButtonVariant::Primary,
                                onclick: move |_| is_bottom_sheet_open.set(false),
                                "Understand"
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Different Sizes".to_string(),

                div { class: "space-y-4",
                    div { class: "flex gap-3 flex-wrap",
                        Button {
                            onclick: move |_| is_small_sheet_open.set(true),
                            "Small Sheet"
                        }
                        Button {
                            onclick: move |_| is_right_sheet_open.set(true),
                            "Medium Sheet"
                        }
                        Button {
                            onclick: move |_| is_large_sheet_open.set(true),
                            "Large Sheet"
                        }
                        Button {
                            onclick: move |_| is_full_sheet_open.set(true),
                            "Full Sheet"
                        }
                    }

                    p { class: "text-sm text-foreground",
                        "Sheets come in different sizes: Small (30vw/30vh), Medium (50vw/50vh), Large (65vw/65vh), and Full (90vw/90vh)."
                    }
                }

                // Small sheet
                Sheet {
                    is_open: is_small_sheet_open,
                    side: SheetSide::Right,
                    size: SheetSize::Small,
                    title: Some("Small Sheet".to_string()),
                    div { class: "p-6 space-y-4",
                        p { class: "text-foreground",
                            "This is a compact sheet perfect for quick actions or minimal information display."
                        }
                        ul { class: "list-disc list-inside space-y-1 text-foreground text-sm",
                            li { "Quick settings" }
                            li { "Simple forms" }
                            li { "Notifications" }
                        }
                        Button {
                            onclick: move |_| is_small_sheet_open.set(false),
                            "Close"
                        }
                    }
                }

                // Large sheet
                Sheet {
                    is_open: is_large_sheet_open,
                    side: SheetSide::Right,
                    size: SheetSize::Large,
                    title: Some("Large Sheet".to_string()),
                    div { class: "p-6 space-y-6",
                        div { class: "space-y-4",
                            h4 { class: "font-semibold text-foreground", "Detailed Information" }
                            p { class: "text-foreground",
                                "Large sheets provide ample space for complex content, detailed forms, or rich media displays."
                            }

                            div { class: "grid grid-cols-1 gap-4",
                                div { class: "p-4 bg-muted rounded-lg",
                                    h5 { class: "font-medium text-foreground mb-2", "Feature Overview" }
                                    ul { class: "list-disc list-inside space-y-1 text-foreground text-sm",
                                        li { "Responsive design" }
                                        li { "Cross-platform support" }
                                        li { "Accessibility features" }
                                        li { "Keyboard navigation" }
                                    }
                                }
                                div { class: "p-4 bg-muted rounded-lg",
                                    h5 { class: "font-medium text-foreground mb-2", "Technical Details" }
                                    p { class: "text-foreground text-sm",
                                        "Built with Dioxus RSX and Tailwind CSS for optimal performance and styling flexibility."
                                    }
                                }
                            }
                        }

                        div { class: "flex justify-end pt-4 border-t border-border",
                            Button {
                                onclick: move |_| is_large_sheet_open.set(false),
                                "Close Large Sheet"
                            }
                        }
                    }
                }

                // Full sheet
                Sheet {
                    is_open: is_full_sheet_open,
                    side: SheetSide::Right,
                    size: SheetSize::Full,
                    title: Some("Full Sheet".to_string()),
                    div { class: "p-6 space-y-6",
                        div { class: "text-center mb-8",
                            h3 { class: "text-2xl font-bold text-foreground mb-4", "Full Screen Experience" }
                            p { class: "text-lg text-foreground max-w-2xl mx-auto",
                                "This full-size sheet takes up 90% of the viewport, providing maximum space for complex interfaces, data tables, detailed forms, or immersive content experiences."
                            }
                        }

                        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                            div { class: "bg-muted rounded-lg p-6",
                                h4 { class: "font-semibold text-foreground mb-3", "Dashboard Overview" }
                                div { class: "space-y-3",
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-foreground", "Active Users" }
                                        span { class: "font-semibold text-success-foreground", "2,847" }
                                    }
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-foreground", "Revenue" }
                                        span { class: "font-semibold text-primary-foreground", "$24,391" }
                                    }
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-foreground", "Conversion" }
                                        span { class: "font-semibold text-accent-foreground", "3.2%" }
                                    }
                                }
                            }

                            div { class: "bg-muted rounded-lg p-6",
                                h4 { class: "font-semibold text-foreground mb-3", "Recent Activity" }
                                div { class: "space-y-3",
                                    div { class: "flex items-center space-x-3",
                                        div { class: "w-8 h-8 bg-primary rounded-full flex items-center justify-center",
                                            span { class: "text-primary-foreground text-sm font-semibold", "U" }
                                        }
                                        div {
                                            div { class: "font-medium text-foreground text-sm", "User signed up" }
                                            div { class: "text-foreground text-xs", "2 minutes ago" }
                                        }
                                    }
                                    div { class: "flex items-center space-x-3",
                                        div { class: "w-8 h-8 bg-success rounded-full flex items-center justify-center",
                                            span { class: "text-success-foreground text-sm font-semibold", "$" }
                                        }
                                        div {
                                            div { class: "font-medium text-foreground text-sm", "Payment received" }
                                            div { class: "text-foreground text-xs", "5 minutes ago" }
                                        }
                                    }
                                }
                            }

                            div { class: "bg-muted rounded-lg p-6",
                                h4 { class: "font-semibold text-foreground mb-3", "System Status" }
                                div { class: "space-y-3",
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-foreground", "API Status" }
                                        div { class: "flex items-center space-x-2",
                                            div { class: "w-2 h-2 bg-success rounded-full" }
                                            span { class: "text-success-foreground text-sm", "Operational" }
                                        }
                                    }
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-foreground", "Database" }
                                        div { class: "flex items-center space-x-2",
                                            div { class: "w-2 h-2 bg-success rounded-full" }
                                            span { class: "text-success-foreground text-sm", "Healthy" }
                                        }
                                    }
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-foreground", "CDN" }
                                        div { class: "flex items-center space-x-2",
                                            div { class: "w-2 h-2 bg-warning rounded-full" }
                                            span { class: "text-warning-foreground text-sm", "Degraded" }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "flex justify-end pt-6 mt-8 border-t border-border",
                            Button {
                                variant: ButtonVariant::Primary,
                                onclick: move |_| is_full_sheet_open.set(false),
                                "Close Full Sheet"
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Configuration Options".to_string(),

                div { class: "space-y-4",
                    div { class: "flex gap-3 flex-wrap",
                        Button {
                            variant: ButtonVariant::Primary,
                            onclick: move |_| is_form_sheet_open.set(true),
                            "Form Sheet"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| is_headerless_sheet_open.set(true),
                            "No Header"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| is_no_backdrop_sheet_open.set(true),
                            "No Backdrop Close"
                        }
                    }

                    p { class: "text-sm text-foreground",
                        "Sheets can be configured with different behaviors: hide headers, disable backdrop clicking, or remove close buttons."
                    }
                }

                // Form sheet
                Sheet {
                    is_open: is_form_sheet_open,
                    side: SheetSide::Right,
                    title: Some("Create New Story".to_string()),
                    div { class: "flex flex-col h-full",
                        div { class: "flex-1 overflow-y-auto p-6 space-y-4",
                            div { class: "space-y-2",
                                Label { "Story Title" }
                                Input {
                                    placeholder: "Enter story title...",
                                    value: story_title.read().clone(),
                                    oninput: move |value: String| story_title.set(value),
                                }
                            }

                            div { class: "space-y-2",
                                Label { "Genre" }
                                Select {
                                    options: vec![
                                        SelectOption::new("fantasy", "Fantasy"),
                                        SelectOption::new("scifi", "Sci-Fi"),
                                        SelectOption::new("romance", "Romance"),
                                        SelectOption::new("adventure", "Adventure"),
                                    ],
                                    placeholder: "Select a genre",
                                    value: story_genre.read().clone(),
                                    onchange: move |value| story_genre.set(value),
                                }
                            }

                            div { class: "space-y-2",
                                Label { "Description" }
                                Textarea {
                                    placeholder: "Describe your story...",
                                    rows: 4,
                                    value: story_description.read().clone(),
                                    oninput: move |value: String| story_description.set(value),
                                }
                            }

                            div { class: "space-y-2",
                                Label { "Tags" }
                                Input {
                                    placeholder: "Add tags (comma separated)...",
                                    value: story_tags.read().clone(),
                                    oninput: move |value: String| story_tags.set(value),
                                }
                            }
                        }

                        div { class: "flex-shrink-0 p-6 border-t border-border bg-muted/50",
                            div { class: "flex justify-end gap-3",
                                Button {
                                    variant: ButtonVariant::Outline,
                                    onclick: move |_| is_form_sheet_open.set(false),
                                    "Cancel"
                                }
                                Button {
                                    variant: ButtonVariant::Primary,
                                    onclick: move |_| is_form_sheet_open.set(false),
                                    "Create Story"
                                }
                            }
                        }
                    }
                }

                // Headerless sheet
                Sheet {
                    is_open: is_headerless_sheet_open,
                    side: SheetSide::Bottom,
                    hide_header: true,
                    size: SheetSize::Medium,
                    div { class: "p-8 text-center",
                        div { class: "mx-auto w-16 h-16 bg-primary rounded-full flex items-center justify-center mb-4",
                            svg {
                                class: "w-8 h-8 text-primary-foreground",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                }
                            }
                        }
                        h3 { class: "text-lg font-semibold text-foreground mb-2", "Information" }
                        p { class: "text-foreground mb-6",
                            "This sheet has no header, giving you complete control over the content layout and presentation."
                        }
                        Button {
                            onclick: move |_| is_headerless_sheet_open.set(false),
                            "Got it"
                        }
                    }
                }

                // No backdrop close sheet
                Sheet {
                    is_open: is_no_backdrop_sheet_open,
                    side: SheetSide::Right,
                    close_on_backdrop_click: false,
                    show_close_button: false,
                    title: Some("Action Required".to_string()),
                    div { class: "p-6 space-y-4",
                        div { class: "flex items-center space-x-3",
                            div { class: "w-10 h-10 bg-destructive rounded-full flex items-center justify-center",
                                svg {
                                    class: "w-5 h-5 text-destructive-foreground",
                                    fill: "currentColor",
                                    view_box: "0 0 20 20",
                                    path {
                                        fill_rule: "evenodd",
                                        d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
                                        clip_rule: "evenodd"
                                    }
                                }
                            }
                            div {
                                h4 { class: "font-semibold text-foreground", "Unsaved Changes" }
                                p { class: "text-foreground", "You have unsaved changes that will be lost if you continue." }
                            }
                        }

                        p { class: "text-foreground",
                            "This sheet cannot be closed by clicking outside or using the Escape key. You must choose one of the actions below."
                        }

                        div { class: "flex justify-end gap-3",
                            Button {
                                variant: ButtonVariant::Outline,
                                onclick: move |_| is_no_backdrop_sheet_open.set(false),
                                "Discard Changes"
                            }
                            Button {
                                variant: ButtonVariant::Primary,
                                onclick: move |_| is_no_backdrop_sheet_open.set(false),
                                "Save & Continue"
                            }
                        }
                    }
                }
            }
        }
    }
}
