//! Modal component showcase

use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{
    Button, ButtonVariant, Input, Label, Modal, ModalSize, Select, SelectOption, Textarea,
};
use dioxus::prelude::*;

#[component]
pub fn modal_showcase() -> Element {
    let mut is_basic_modal_open = use_signal(|| false);
    let mut is_info_modal_open = use_signal(|| false);
    let mut is_form_modal_open = use_signal(|| false);
    let mut is_small_modal_open = use_signal(|| false);
    let mut is_large_modal_open = use_signal(|| false);
    let mut is_headerless_modal_open = use_signal(|| false);
    let mut is_no_backdrop_modal_open = use_signal(|| false);

    // Form state
    let mut character_name = use_signal(String::new);
    let mut character_description = use_signal(String::new);
    let mut character_category = use_signal(String::new);

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

                    p { class: "text-sm text-foreground",
                        "Standard modal with title, close button, and backdrop click to close."
                    }
                }

                Modal {
                    title: Some("Basic Modal".to_string()),
                    is_open: is_basic_modal_open,
                    div { class: "p-6 space-y-4",
                        p { class: "text-foreground",
                            "This is a basic modal with improved styling and better accessibility. The header includes a proper close button with an SVG icon."
                        }
                        p { class: "text-foreground",
                            "You can close this modal by clicking the X button, clicking outside the modal, or pressing the Escape key."
                        }
                        div { class: "flex justify-end pt-4 border-t border-border",
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

                    p { class: "text-sm text-foreground",
                        "Modals come in different sizes: Small (28rem), Medium (48rem), Large (72rem), and Full (95vw)."
                    }
                }

                // Small modal
                Modal {
                    title: Some("Small Modal".to_string()),
                    is_open: is_small_modal_open,
                    size: ModalSize::Small,
                    div { class: "p-6",
                        p { class: "text-foreground mb-4",
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
                            h4 { class: "font-semibold text-foreground", "Features" }
                            ul { class: "list-disc list-inside space-y-1 text-foreground",
                                li { "Cross-platform LLM roleplay application" }
                                li { "Flexible deployment options" }
                                li { "Bring your own API approach" }
                                li { "Multi-user, multi-device syncing" }
                            }
                        }

                        div { class: "space-y-3",
                            h4 { class: "font-semibold text-foreground", "Platforms" }
                            div { class: "flex gap-2",
                                span { class: "px-2 py-1 bg-primary text-primary-foreground rounded text-sm", "Web" }
                                span { class: "px-2 py-1 bg-success text-success-foreground rounded text-sm", "Desktop" }
                                span { class: "px-2 py-1 bg-accent/10 text-accent-foreground rounded text-sm", "Mobile" }
                            }
                        }

                        div { class: "flex justify-end pt-4 border-t border-border",
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
                                h4 { class: "font-semibold text-foreground mb-3", "Left Column" }
                                p { class: "text-foreground mb-3",
                                    "This is a large modal that can accommodate more content and complex layouts."
                                }
                                ul { class: "list-disc list-inside space-y-1 text-foreground",
                                    li { "Wide layout for complex forms" }
                                    li { "Multiple columns of content" }
                                    li { "Rich media displays" }
                                    li { "Data tables and charts" }
                                }
                            }
                            div {
                                h4 { class: "font-semibold text-foreground mb-3", "Right Column" }
                                div { class: "space-y-3",
                                    div { class: "p-4 bg-muted rounded-lg",
                                        p { class: "text-sm text-foreground", "Sample content area" }
                                    }
                                    div { class: "p-4 bg-muted rounded-lg",
                                        p { class: "text-sm text-foreground", "Another content section" }
                                    }
                                }
                            }
                        }
                        div { class: "flex justify-end pt-6 mt-6 border-t border-border",
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

                    p { class: "text-sm text-foreground",
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
                                Label { "Character Name" }
                                Input {
                                    placeholder: "Enter character name...",
                                    value: character_name.read().clone(),
                                    oninput: move |value: String| character_name.set(value),
                                }
                            }

                            div { class: "space-y-2",
                                Label { "Description" }
                                Textarea {
                                    placeholder: "Describe your character...",
                                    rows: 3,
                                    value: character_description.read().clone(),
                                    oninput: move |value: String| character_description.set(value),
                                }
                            }

                            div { class: "space-y-2",
                                Label { "Category" }
                                Select {
                                    options: vec![
                                        SelectOption::new("fantasy", "Fantasy"),
                                        SelectOption::new("scifi", "Sci-Fi"),
                                        SelectOption::new("modern", "Modern"),
                                        SelectOption::new("historical", "Historical"),
                                    ],
                                    placeholder: "Select a category",
                                    value: character_category.read().clone(),
                                    onchange: move |value| character_category.set(value),
                                }
                            }
                        }

                        div { class: "flex-shrink-0 p-6 border-t border-border bg-muted/50",
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
                        div { class: "mx-auto w-16 h-16 bg-success rounded-full flex items-center justify-center mb-4",
                            svg {
                                class: "w-8 h-8 text-success-foreground",
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
                        h3 { class: "text-lg font-semibold text-foreground mb-2", "Success!" }
                        p { class: "text-foreground mb-6",
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
                        p { class: "text-foreground mb-6",
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
