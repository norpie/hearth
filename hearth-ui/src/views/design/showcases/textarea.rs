use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Label, Textarea, TextareaResize, TextareaSize, TextareaVariant};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TextareaShowcaseProps {
    pub default_textarea: Signal<String>,
    pub filled_textarea: Signal<String>,
    pub outline_textarea: Signal<String>,
    pub ghost_textarea: Signal<String>,
}

#[component]
pub fn textarea_showcase(props: TextareaShowcaseProps) -> Element {
    let TextareaShowcaseProps {
        mut default_textarea,
        mut filled_textarea,
        mut outline_textarea,
        mut ghost_textarea,
    } = props;
    rsx! {
        ComponentShowcase {
            name: "Textarea".to_string(),
            description: "Multi-line text input component with resizing and validation states.".to_string(),
            basic_usage: r#"let mut textarea_value = use_signal(|| String::new());

Textarea {
    value: textarea_value.read().clone(),
    oninput: move |value: String| textarea_value.set(value),
}"#.to_string(),
            with_props_usage: r#"let mut textarea_value = use_signal(|| String::new());

Textarea {
    variant: TextareaVariant::Default,
    size: TextareaSize::Medium,
    resize: TextareaResize::Vertical,
    value: textarea_value.read().clone(),
    placeholder: "Enter your message here...",
    disabled: false,
    error: false,
    required: true,
    readonly: false,
    rows: Some(4),
    maxlength: Some(500),
    name: "message",
    id: "message-textarea",
    class: "custom-class",
    oninput: move |value: String| textarea_value.set(value),
}"#.to_string(),

            ShowcaseVariant {
                title: "Variants".to_string(),
                div { class: "space-y-4",
                    Textarea {
                        variant: TextareaVariant::Default,
                        placeholder: "Default textarea",
                        value: default_textarea.read().clone(),
                        oninput: move |value: String| default_textarea.set(value),
                    }
                    Textarea {
                        variant: TextareaVariant::Filled,
                        placeholder: "Filled textarea",
                        value: filled_textarea.read().clone(),
                        oninput: move |value: String| filled_textarea.set(value),
                    }
                    Textarea {
                        variant: TextareaVariant::Outline,
                        placeholder: "Outline textarea",
                        value: outline_textarea.read().clone(),
                        oninput: move |value: String| outline_textarea.set(value),
                    }
                    Textarea {
                        variant: TextareaVariant::Ghost,
                        placeholder: "Ghost textarea",
                        value: ghost_textarea.read().clone(),
                        oninput: move |value: String| ghost_textarea.set(value),
                    }
                }
            }

            ShowcaseVariant {
                title: "States".to_string(),
                div { class: "space-y-4",
                    Textarea {
                        placeholder: "Normal textarea",
                    }
                    Textarea {
                        disabled: true,
                        placeholder: "Disabled textarea",
                        value: "Cannot edit this text".to_string(),
                    }
                    Textarea {
                        readonly: true,
                        placeholder: "Read-only textarea",
                        value: "This text is read-only".to_string(),
                    }
                    Textarea {
                        error: true,
                        placeholder: "Error state",
                        value: "This textarea has an error".to_string(),
                    }
                }
            }

            ShowcaseVariant {
                title: "Sizes".to_string(),
                div { class: "space-y-4",
                    div {
                        Label { "Small" }
                        Textarea {
                            size: TextareaSize::Small,
                            placeholder: "Small textarea",
                        }
                    }
                    div {
                        Label { "Medium (default)" }
                        Textarea {
                            size: TextareaSize::Medium,
                            placeholder: "Medium textarea",
                        }
                    }
                    div {
                        Label { "Large" }
                        Textarea {
                            size: TextareaSize::Large,
                            placeholder: "Large textarea",
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Resize Options".to_string(),
                div { class: "space-y-4",
                    div {
                        Label { "No Resize" }
                        Textarea {
                            resize: TextareaResize::None,
                            placeholder: "Cannot be resized",
                        }
                    }
                    div {
                        Label { "Vertical Resize (default)" }
                        Textarea {
                            resize: TextareaResize::Vertical,
                            placeholder: "Can be resized vertically",
                        }
                    }
                    div {
                        Label { "Horizontal Resize" }
                        Textarea {
                            resize: TextareaResize::Horizontal,
                            placeholder: "Can be resized horizontally",
                        }
                    }
                    div {
                        Label { "Both Resize" }
                        Textarea {
                            resize: TextareaResize::Both,
                            placeholder: "Can be resized in both directions",
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "With Constraints".to_string(),
                div { class: "space-y-4",
                    div {
                        Label { "With Row Count (4 rows)" }
                        Textarea {
                            rows: Some(4),
                            placeholder: "Exactly 4 rows high",
                        }
                    }
                    div {
                        Label { "With Max Length (100 chars)" }
                        Textarea {
                            maxlength: Some(100),
                            placeholder: "Maximum 100 characters allowed",
                        }
                    }
                }
            }
        }
    }
}
