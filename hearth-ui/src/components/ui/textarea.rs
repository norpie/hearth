//! Textarea component for multi-line text input
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     Textarea {
//!         placeholder: "Enter your message...",
//!         value: message_value,
//!         oninput: move |val| message_value.set(val),
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Visual style variants for textarea elements
#[derive(Clone, PartialEq, Debug)]
pub enum TextareaVariant {
    Default,
    Filled,
    Outline,
    Ghost,
}

/// Size variants for textarea dimensions
#[derive(Clone, PartialEq, Debug)]
pub enum TextareaSize {
    Small,
    Medium,
    Large,
}

/// Resize behavior options for textarea elements
#[derive(Clone, PartialEq, Debug)]
pub enum TextareaResize {
    None,
    Vertical,
    Horizontal,
    Both,
}

impl TextareaVariant {
    /// Returns CSS classes for this textarea variant
    pub fn classes(&self, has_error: bool, is_disabled: bool, is_focused: bool) -> String {
        let base = "w-full transition-colors duration-200 focus:outline-none";

        let variant_classes = match self {
            TextareaVariant::Default => "border border-input bg-background text-foreground",
            TextareaVariant::Filled => "border-0 bg-muted text-foreground",
            TextareaVariant::Outline => "border-2 border-border bg-transparent text-foreground",
            TextareaVariant::Ghost => "border-0 bg-transparent text-foreground hover:bg-accent",
        };

        let state_classes = if has_error {
            "border-destructive focus:ring-2 focus:ring-destructive/20"
        } else if is_disabled {
            "opacity-50 cursor-not-allowed"
        } else if is_focused {
            match self {
                TextareaVariant::Default | TextareaVariant::Outline => {
                    "border-primary focus:ring-2 focus:ring-ring/20"
                }
                TextareaVariant::Filled => "ring-2 ring-ring/20",
                TextareaVariant::Ghost => "bg-accent",
            }
        } else {
            match self {
                TextareaVariant::Default | TextareaVariant::Outline => "hover:border-border/80",
                TextareaVariant::Filled => "hover:bg-muted/80",
                TextareaVariant::Ghost => "",
            }
        };

        format!("{base} {variant_classes} {state_classes}")
    }
}

impl TextareaSize {
    /// Returns CSS classes for this textarea size
    pub fn classes(&self) -> &'static str {
        match self {
            TextareaSize::Small => "px-2 py-1 text-xs rounded-md min-h-16",
            TextareaSize::Medium => "px-3 py-2 text-sm rounded-md min-h-20",
            TextareaSize::Large => "px-4 py-3 text-base rounded-lg min-h-24",
        }
    }
}

impl TextareaResize {
    /// Returns CSS classes for this resize behavior
    pub fn classes(&self) -> &'static str {
        match self {
            TextareaResize::None => "resize-none",
            TextareaResize::Vertical => "resize-y",
            TextareaResize::Horizontal => "resize-x",
            TextareaResize::Both => "resize",
        }
    }
}

impl Default for TextareaVariant {
    fn default() -> Self {
        Self::Default
    }
}

impl Default for TextareaSize {
    fn default() -> Self {
        Self::Medium
    }
}

impl Default for TextareaResize {
    fn default() -> Self {
        Self::Vertical
    }
}

/// Properties for the Textarea component
#[derive(Props, Clone, PartialEq)]
pub struct TextareaProps {
    /// Visual style variant
    #[props(default)]
    pub variant: TextareaVariant,

    /// Size of the textarea
    #[props(default)]
    pub size: TextareaSize,

    /// User resize behavior
    #[props(default)]
    pub resize: TextareaResize,

    /// Current value of the textarea
    #[props(default = String::new())]
    pub value: String,

    /// Placeholder text when empty
    #[props(default = String::new())]
    pub placeholder: String,

    /// Whether the textarea is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Whether the textarea is in error state
    #[props(default = false)]
    pub error: bool,

    /// Whether the textarea is required
    #[props(default = false)]
    pub required: bool,

    /// Whether the textarea is read-only
    #[props(default = false)]
    pub readonly: bool,

    /// Maximum character limit
    #[props(default)]
    pub maxlength: Option<u32>,

    /// Number of visible rows
    #[props(default)]
    pub rows: Option<u32>,

    /// Number of visible columns
    #[props(default)]
    pub cols: Option<u32>,

    /// HTML name attribute
    #[props(default = String::new())]
    pub name: String,

    /// HTML id attribute
    #[props(default = String::new())]
    pub id: String,

    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,

    /// Change event handler
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,

    /// Input event handler
    #[props(default)]
    pub oninput: Option<EventHandler<String>>,

    /// Focus event handler
    #[props(default)]
    pub onfocus: Option<EventHandler<FocusEvent>>,

    /// Blur event handler
    #[props(default)]
    pub onblur: Option<EventHandler<FocusEvent>>,

    /// Keypress event handler
    #[props(default)]
    pub onkeypress: Option<EventHandler<KeyboardEvent>>,
}

/// Multi-line text input component with variants and sizing
#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let mut is_focused = use_signal(|| false);

    let textarea_classes = props
        .variant
        .classes(props.error, props.disabled, is_focused());
    let size_classes = props.size.classes();
    let resize_classes = props.resize.classes();

    let combined_classes = if props.class.is_empty() {
        format!("{textarea_classes} {size_classes} {resize_classes}")
    } else {
        format!(
            "{} {} {} {}",
            textarea_classes, size_classes, resize_classes, props.class
        )
    };

    rsx! {
        textarea {
            class: combined_classes,
            value: props.value,
            placeholder: props.placeholder,
            disabled: props.disabled,
            required: props.required,
            readonly: props.readonly,
            maxlength: props.maxlength.map(|len| len.to_string()),
            rows: props.rows.map(|r| r.to_string()),
            cols: props.cols.map(|c| c.to_string()),
            name: if props.name.is_empty() { None } else { Some(props.name.as_str()) },
            id: if props.id.is_empty() { None } else { Some(props.id.as_str()) },
            oninput: {
                let oninput = props.oninput;
                move |evt| {
                    if let Some(handler) = &oninput {
                        handler.call(evt.value());
                    }
                }
            },
            onchange: {
                let onchange = props.onchange;
                move |evt| {
                    if let Some(handler) = &onchange {
                        handler.call(evt.value());
                    }
                }
            },
            onfocus: {
                let onfocus = props.onfocus;
                move |evt| {
                    is_focused.set(true);
                    if let Some(handler) = &onfocus {
                        handler.call(evt);
                    }
                }
            },
            onblur: {
                let onblur = props.onblur;
                move |evt| {
                    is_focused.set(false);
                    if let Some(handler) = &onblur {
                        handler.call(evt);
                    }
                }
            },
            onkeypress: {
                let onkeypress = props.onkeypress;
                move |evt| {
                    if let Some(handler) = &onkeypress {
                        handler.call(evt);
                    }
                }
            },
        }
    }
}
