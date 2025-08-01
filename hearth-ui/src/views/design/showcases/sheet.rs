//! Sheet component showcase

use dioxus::prelude::*;
use crate::{Sheet, SheetSide, SheetSize, Button, ButtonVariant};
use super::super::{ComponentShowcase, ShowcaseVariant};

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
                    
                    p { class: "text-sm text-gray-600 dark:text-gray-400",
                        "Sheets can slide in from any of the four sides: left, right, top, or bottom with smooth animations."
                    }
                }
                
                // Right sheet
                Sheet {
                    is_open: is_right_sheet_open,
                    side: SheetSide::Right,
                    title: Some("Right Sheet".to_string()),
                    div { class: "p-6 space-y-4",
                        p { class: "text-gray-700 dark:text-gray-300",
                            "This sheet slides in from the right side of the screen. It's perfect for navigation menus, settings panels, or detailed information."
                        }
                        ul { class: "list-disc list-inside space-y-2 text-gray-600 dark:text-gray-400",
                            li { "Smooth slide animation" }
                            li { "Background blur effect" } 
                            li { "Escape key support" }
                            li { "Backdrop click to close" }
                        }
                        div { class: "flex justify-end pt-4 border-t border-gray-200 dark:border-gray-700",
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
                        h4 { class: "font-semibold text-gray-900 dark:text-gray-100", "Navigation Menu" }
                        nav { class: "space-y-2",
                            a { class: "block px-3 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded", href: "#", "Dashboard" }
                            a { class: "block px-3 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded", href: "#", "Characters" }
                            a { class: "block px-3 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded", href: "#", "Stories" }
                            a { class: "block px-3 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded", href: "#", "Settings" }
                        }
                        div { class: "pt-4 border-t border-gray-200 dark:border-gray-700",
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
                                div { class: "w-12 h-12 bg-blue-100 dark:bg-blue-900 rounded-full flex items-center justify-center mx-auto mb-2",
                                    span { class: "text-blue-600 dark:text-blue-400 font-semibold", "1" }
                                }
                                h5 { class: "font-medium text-gray-900 dark:text-gray-100", "Step One" }
                                p { class: "text-sm text-gray-600 dark:text-gray-400", "Configure your settings" }
                            }
                            div { class: "text-center",
                                div { class: "w-12 h-12 bg-green-100 dark:bg-green-900 rounded-full flex items-center justify-center mx-auto mb-2",
                                    span { class: "text-green-600 dark:text-green-400 font-semibold", "2" }
                                }
                                h5 { class: "font-medium text-gray-900 dark:text-gray-100", "Step Two" }
                                p { class: "text-sm text-gray-600 dark:text-gray-400", "Import your data" }
                            }
                            div { class: "text-center",
                                div { class: "w-12 h-12 bg-purple-100 dark:bg-purple-900 rounded-full flex items-center justify-center mx-auto mb-2",
                                    span { class: "text-purple-600 dark:text-purple-400 font-semibold", "3" }
                                }
                                h5 { class: "font-medium text-gray-900 dark:text-gray-100", "Step Three" }
                                p { class: "text-sm text-gray-600 dark:text-gray-400", "Start creating" }
                            }
                        }
                        div { class: "flex justify-center pt-6 mt-6 border-t border-gray-200 dark:border-gray-700",
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
                            div { class: "w-10 h-10 bg-yellow-100 dark:bg-yellow-900 rounded-full flex items-center justify-center",
                                svg {
                                    class: "w-5 h-5 text-yellow-600 dark:text-yellow-400",
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
                                h4 { class: "font-semibold text-gray-900 dark:text-gray-100", "Important Notice" }
                                p { class: "text-gray-600 dark:text-gray-400", "This action cannot be undone. Please review before proceeding." }
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
                    
                    p { class: "text-sm text-gray-600 dark:text-gray-400",
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
                        p { class: "text-gray-700 dark:text-gray-300",
                            "This is a compact sheet perfect for quick actions or minimal information display."
                        }
                        ul { class: "list-disc list-inside space-y-1 text-gray-600 dark:text-gray-400 text-sm",
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
                            h4 { class: "font-semibold text-gray-900 dark:text-gray-100", "Detailed Information" }
                            p { class: "text-gray-700 dark:text-gray-300",
                                "Large sheets provide ample space for complex content, detailed forms, or rich media displays."
                            }
                            
                            div { class: "grid grid-cols-1 gap-4",
                                div { class: "p-4 bg-gray-50 dark:bg-gray-800 rounded-lg",
                                    h5 { class: "font-medium text-gray-900 dark:text-gray-100 mb-2", "Feature Overview" }
                                    ul { class: "list-disc list-inside space-y-1 text-gray-600 dark:text-gray-400 text-sm",
                                        li { "Responsive design" }
                                        li { "Cross-platform support" }
                                        li { "Accessibility features" }
                                        li { "Keyboard navigation" }
                                    }
                                }
                                div { class: "p-4 bg-gray-50 dark:bg-gray-800 rounded-lg",
                                    h5 { class: "font-medium text-gray-900 dark:text-gray-100 mb-2", "Technical Details" }
                                    p { class: "text-gray-600 dark:text-gray-400 text-sm",
                                        "Built with Dioxus RSX and Tailwind CSS for optimal performance and styling flexibility."
                                    }
                                }
                            }
                        }
                        
                        div { class: "flex justify-end pt-4 border-t border-gray-200 dark:border-gray-700",
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
                            h3 { class: "text-2xl font-bold text-gray-900 dark:text-gray-100 mb-4", "Full Screen Experience" }
                            p { class: "text-lg text-gray-700 dark:text-gray-300 max-w-2xl mx-auto",
                                "This full-size sheet takes up 90% of the viewport, providing maximum space for complex interfaces, data tables, detailed forms, or immersive content experiences."
                            }
                        }
                        
                        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                            div { class: "bg-gray-50 dark:bg-gray-800 rounded-lg p-6",
                                h4 { class: "font-semibold text-gray-900 dark:text-gray-100 mb-3", "Dashboard Overview" }
                                div { class: "space-y-3",
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-gray-600 dark:text-gray-400", "Active Users" }
                                        span { class: "font-semibold text-green-600 dark:text-green-400", "2,847" }
                                    }
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-gray-600 dark:text-gray-400", "Revenue" }
                                        span { class: "font-semibold text-blue-600 dark:text-blue-400", "$24,391" }
                                    }
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-gray-600 dark:text-gray-400", "Conversion" }
                                        span { class: "font-semibold text-purple-600 dark:text-purple-400", "3.2%" }
                                    }
                                }
                            }
                            
                            div { class: "bg-gray-50 dark:bg-gray-800 rounded-lg p-6",
                                h4 { class: "font-semibold text-gray-900 dark:text-gray-100 mb-3", "Recent Activity" }
                                div { class: "space-y-3",
                                    div { class: "flex items-center space-x-3",
                                        div { class: "w-8 h-8 bg-blue-100 dark:bg-blue-900 rounded-full flex items-center justify-center",
                                            span { class: "text-blue-600 dark:text-blue-400 text-sm font-semibold", "U" }
                                        }
                                        div {
                                            div { class: "font-medium text-gray-900 dark:text-gray-100 text-sm", "User signed up" }
                                            div { class: "text-gray-500 dark:text-gray-500 text-xs", "2 minutes ago" }
                                        }
                                    }
                                    div { class: "flex items-center space-x-3",
                                        div { class: "w-8 h-8 bg-green-100 dark:bg-green-900 rounded-full flex items-center justify-center",
                                            span { class: "text-green-600 dark:text-green-400 text-sm font-semibold", "$" }
                                        }
                                        div {
                                            div { class: "font-medium text-gray-900 dark:text-gray-100 text-sm", "Payment received" }
                                            div { class: "text-gray-500 dark:text-gray-500 text-xs", "5 minutes ago" }
                                        }
                                    }
                                }
                            }
                            
                            div { class: "bg-gray-50 dark:bg-gray-800 rounded-lg p-6",
                                h4 { class: "font-semibold text-gray-900 dark:text-gray-100 mb-3", "System Status" }
                                div { class: "space-y-3",
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-gray-600 dark:text-gray-400", "API Status" }
                                        div { class: "flex items-center space-x-2",
                                            div { class: "w-2 h-2 bg-green-500 rounded-full" }
                                            span { class: "text-green-600 dark:text-green-400 text-sm", "Operational" }
                                        }
                                    }
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-gray-600 dark:text-gray-400", "Database" }
                                        div { class: "flex items-center space-x-2",
                                            div { class: "w-2 h-2 bg-green-500 rounded-full" }
                                            span { class: "text-green-600 dark:text-green-400 text-sm", "Healthy" }
                                        }
                                    }
                                    div { class: "flex justify-between items-center",
                                        span { class: "text-gray-600 dark:text-gray-400", "CDN" }
                                        div { class: "flex items-center space-x-2",
                                            div { class: "w-2 h-2 bg-yellow-500 rounded-full" }
                                            span { class: "text-yellow-600 dark:text-yellow-400 text-sm", "Degraded" }
                                        }
                                    }
                                }
                            }
                        }
                        
                        div { class: "flex justify-end pt-6 mt-8 border-t border-gray-200 dark:border-gray-700",
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
                    
                    p { class: "text-sm text-gray-600 dark:text-gray-400",
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
                                label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300", "Story Title" }
                                input {
                                    class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100",
                                    placeholder: "Enter story title..."
                                }
                            }
                            
                            div { class: "space-y-2",
                                label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300", "Genre" }
                                select {
                                    class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100",
                                    option { "Fantasy" }
                                    option { "Sci-Fi" }
                                    option { "Romance" }
                                    option { "Adventure" }
                                }
                            }
                            
                            div { class: "space-y-2",
                                label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300", "Description" }
                                textarea {
                                    class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 h-32 resize-none",
                                    placeholder: "Describe your story..."
                                }
                            }
                            
                            div { class: "space-y-2",
                                label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300", "Tags" }
                                input {
                                    class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100",
                                    placeholder: "Add tags (comma separated)..."
                                }
                            }
                        }
                        
                        div { class: "flex-shrink-0 p-6 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50",
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
                        div { class: "mx-auto w-16 h-16 bg-blue-100 dark:bg-blue-900 rounded-full flex items-center justify-center mb-4",
                            svg {
                                class: "w-8 h-8 text-blue-600 dark:text-blue-400",
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
                        h3 { class: "text-lg font-semibold text-gray-900 dark:text-gray-100 mb-2", "Information" }
                        p { class: "text-gray-600 dark:text-gray-400 mb-6",
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
                            div { class: "w-10 h-10 bg-red-100 dark:bg-red-900 rounded-full flex items-center justify-center",
                                svg {
                                    class: "w-5 h-5 text-red-600 dark:text-red-400",
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
                                h4 { class: "font-semibold text-gray-900 dark:text-gray-100", "Unsaved Changes" }
                                p { class: "text-gray-600 dark:text-gray-400", "You have unsaved changes that will be lost if you continue." }
                            }
                        }
                        
                        p { class: "text-gray-700 dark:text-gray-300",
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