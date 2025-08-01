//! Modal component showcase

use dioxus::prelude::*;
use crate::{Modal, ModalSize, Button, ButtonVariant};
use super::super::{ComponentShowcase, ShowcaseVariant};

#[component]
pub fn modal_showcase() -> Element {
    let mut is_basic_modal_open = use_signal(|| false);
    let mut is_info_modal_open = use_signal(|| false);
    let mut is_form_modal_open = use_signal(|| false);
    let mut is_small_modal_open = use_signal(|| false);
    let mut is_large_modal_open = use_signal(|| false);
    let mut is_headerless_modal_open = use_signal(|| false);
    let mut is_no_backdrop_modal_open = use_signal(|| false);

    rsx! {
        ComponentShowcase {
            name: "Modal".to_string(),
            description: "Flexible modal dialogs with configurable headers, sizes, and behavior options.".to_string(),
            basic_usage: r#"Modal {
    title: Some("Modal Title".to_string()),
    is_open: modal_state,
    div { class: "p-6", "Modal content goes here" }
}"#.to_string(),
            with_props_usage: r#"Modal {
    title: Some("Confirmation".to_string()),
    is_open: confirmation_open,
    size: ModalSize::Small,
    show_close_button: false,
    close_on_backdrop_click: false,
    div { class: "p-6",
        p { "Are you sure you want to continue?" }
        div { class: "flex justify-end gap-2 mt-4",
            Button { onclick: move |_| close_modal(), "Cancel" }
            Button { variant: ButtonVariant::Primary, "Confirm" }
        }
    }
}"#.to_string(),
            
            ShowcaseVariant {
                title: "Basic Modal".to_string(),
                
                div { class: "space-y-4",
                    Button {
                        onclick: move |_| is_basic_modal_open.set(true),
                        "Open Basic Modal"
                    }
                    
                    p { class: "text-sm text-gray-600 dark:text-gray-400",
                        "Standard modal with title, close button, and backdrop click to close."
                    }
                }
                
                Modal {
                    title: Some("Basic Modal".to_string()),
                    is_open: is_basic_modal_open,
                    div { class: "p-6 space-y-4",
                        p { class: "text-gray-700 dark:text-gray-300",
                            "This is a basic modal with improved styling and better accessibility. The header includes a proper close button with an SVG icon."
                        }
                        p { class: "text-gray-600 dark:text-gray-400",
                            "You can close this modal by clicking the X button, clicking outside the modal, or pressing the Escape key."
                        }
                        div { class: "flex justify-end pt-4 border-t border-gray-200 dark:border-gray-700",
                            Button {
                                onclick: move |_| is_basic_modal_open.set(false),
                                "Done"
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
                            variant: ButtonVariant::Outline,
                            onclick: move |_| is_small_modal_open.set(true),
                            "Small Modal"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| is_info_modal_open.set(true),
                            "Medium Modal"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| is_large_modal_open.set(true),
                            "Large Modal"
                        }
                    }
                    
                    p { class: "text-sm text-gray-600 dark:text-gray-400",
                        "Modals come in different sizes: Small (28rem), Medium (48rem), Large (72rem), and Full (95vw)."
                    }
                }
                
                // Small modal
                Modal {
                    title: Some("Small Modal".to_string()),
                    is_open: is_small_modal_open,
                    size: ModalSize::Small,
                    div { class: "p-6",
                        p { class: "text-gray-700 dark:text-gray-300 mb-4",
                            "This is a small modal, perfect for confirmations or simple messages."
                        }
                        div { class: "flex justify-end",
                            Button {
                                onclick: move |_| is_small_modal_open.set(false),
                                "Close"
                            }
                        }
                    }
                }
                
                // Medium modal (info)
                Modal {
                    title: Some("About Hearth".to_string()),
                    is_open: is_info_modal_open,
                    size: ModalSize::Medium,
                    div { class: "p-6 space-y-6",
                        div { class: "space-y-3",
                            h4 { class: "font-semibold text-gray-900 dark:text-gray-100", "Features" }
                            ul { class: "list-disc list-inside space-y-1 text-gray-700 dark:text-gray-300",
                                li { "Cross-platform LLM roleplay application" }
                                li { "Flexible deployment options" }
                                li { "Bring your own API approach" }
                                li { "Multi-user, multi-device syncing" }
                            }
                        }
                        
                        div { class: "space-y-3",
                            h4 { class: "font-semibold text-gray-900 dark:text-gray-100", "Platforms" }
                            div { class: "flex gap-2",
                                span { class: "px-2 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 rounded text-sm", "Web" }
                                span { class: "px-2 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded text-sm", "Desktop" }
                                span { class: "px-2 py-1 bg-purple-100 dark:bg-purple-900 text-purple-800 dark:text-purple-200 rounded text-sm", "Mobile" }
                            }
                        }
                        
                        div { class: "flex justify-end pt-4 border-t border-gray-200 dark:border-gray-700",
                            Button {
                                onclick: move |_| is_info_modal_open.set(false),
                                "Got it"
                            }
                        }
                    }
                }
                
                // Large modal
                Modal {
                    title: Some("Large Modal".to_string()),
                    is_open: is_large_modal_open,
                    size: ModalSize::Large,
                    div { class: "p-6",
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                            div {
                                h4 { class: "font-semibold text-gray-900 dark:text-gray-100 mb-3", "Left Column" }
                                p { class: "text-gray-700 dark:text-gray-300 mb-3",
                                    "This is a large modal that can accommodate more content and complex layouts."
                                }
                                ul { class: "list-disc list-inside space-y-1 text-gray-700 dark:text-gray-300",
                                    li { "Wide layout for complex forms" }
                                    li { "Multiple columns of content" }
                                    li { "Rich media displays" }
                                    li { "Data tables and charts" }
                                }
                            }
                            div {
                                h4 { class: "font-semibold text-gray-900 dark:text-gray-100 mb-3", "Right Column" }
                                div { class: "space-y-3",
                                    div { class: "p-4 bg-gray-100 dark:bg-gray-800 rounded-lg",
                                        p { class: "text-sm text-gray-600 dark:text-gray-400", "Sample content area" }
                                    }
                                    div { class: "p-4 bg-gray-100 dark:bg-gray-800 rounded-lg",
                                        p { class: "text-sm text-gray-600 dark:text-gray-400", "Another content section" }
                                    }
                                }
                            }
                        }
                        div { class: "flex justify-end pt-6 mt-6 border-t border-gray-200 dark:border-gray-700",
                            Button {
                                onclick: move |_| is_large_modal_open.set(false),
                                "Close Large Modal"
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
                            onclick: move |_| is_form_modal_open.set(true),
                            "Form Modal"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| is_headerless_modal_open.set(true),
                            "No Header"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: move |_| is_no_backdrop_modal_open.set(true),
                            "No Backdrop Close"
                        }
                    }
                    
                    p { class: "text-sm text-gray-600 dark:text-gray-400",
                        "Modals can be configured with different behaviors: hide headers, disable backdrop clicking, or remove close buttons."
                    }
                }
                
                // Form modal
                Modal {
                    title: Some("Create New Character".to_string()),
                    is_open: is_form_modal_open,
                    div { class: "flex flex-col h-full",
                        div { class: "flex-1 overflow-y-auto p-6 space-y-4",
                            div { class: "space-y-2",
                                label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300", "Character Name" }
                                input { 
                                    class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100",
                                    placeholder: "Enter character name..."
                                }
                            }
                            
                            div { class: "space-y-2",
                                label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300", "Description" }
                                textarea { 
                                    class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 h-24 resize-none",
                                    placeholder: "Describe your character..."
                                }
                            }
                            
                            div { class: "space-y-2",
                                label { class: "block text-sm font-medium text-gray-700 dark:text-gray-300", "Category" }
                                select { 
                                    class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100",
                                    option { "Fantasy" }
                                    option { "Sci-Fi" }
                                    option { "Modern" }
                                    option { "Historical" }
                                }
                            }
                        }
                        
                        div { class: "flex-shrink-0 p-6 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800/50",
                            div { class: "flex justify-end gap-3",
                                Button {
                                    variant: ButtonVariant::Outline,
                                    onclick: move |_| is_form_modal_open.set(false),
                                    "Cancel"
                                }
                                Button {
                                    variant: ButtonVariant::Primary,
                                    onclick: move |_| is_form_modal_open.set(false),
                                    "Create Character"
                                }
                            }
                        }
                    }
                }
                
                // Headerless modal
                Modal {
                    title: None,
                    is_open: is_headerless_modal_open,
                    hide_header: true,
                    size: ModalSize::Small,
                    div { class: "p-8 text-center",
                        div { class: "mx-auto w-16 h-16 bg-green-100 dark:bg-green-900 rounded-full flex items-center justify-center mb-4",
                            svg {
                                class: "w-8 h-8 text-green-600 dark:text-green-400",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round", 
                                    stroke_width: "2",
                                    d: "M5 13l4 4L19 7"
                                }
                            }
                        }
                        h3 { class: "text-lg font-semibold text-gray-900 dark:text-gray-100 mb-2", "Success!" }
                        p { class: "text-gray-600 dark:text-gray-400 mb-6",
                            "This modal has no header, giving you full control over the content layout."
                        }
                        Button {
                            onclick: move |_| is_headerless_modal_open.set(false),
                            "Continue"
                        }
                    }
                }
                
                // No backdrop close modal
                Modal {
                    title: Some("Confirmation Required".to_string()),
                    is_open: is_no_backdrop_modal_open,
                    close_on_backdrop_click: false,
                    show_close_button: false,
                    size: ModalSize::Small,
                    div { class: "p-6",
                        p { class: "text-gray-700 dark:text-gray-300 mb-6",
                            "This modal cannot be closed by clicking outside or using the close button. You must use one of the action buttons below."
                        }
                        div { class: "flex justify-end gap-3",
                            Button {
                                variant: ButtonVariant::Outline,
                                onclick: move |_| is_no_backdrop_modal_open.set(false),
                                "Cancel"
                            }
                            Button {
                                variant: ButtonVariant::Primary,
                                onclick: move |_| is_no_backdrop_modal_open.set(false),
                                "Confirm"
                            }
                        }
                    }
                }
            }
        }
    }
}