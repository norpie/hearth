//! Textarea component with variants, states, and accessibility support

use dioxus::prelude::*;

/// Textarea variants for different use cases
#[derive(Clone, PartialEq, Debug)]
pub enum TextareaVariant {
    Default,
    Filled,
    Outline,
    Ghost,
}

/// Textarea sizes
#[derive(Clone, PartialEq, Debug)]
pub enum TextareaSize {
    Small,
    Medium,
    Large,
}

/// Textarea resize options
#[derive(Clone, PartialEq, Debug)]
pub enum TextareaResize {
    None,
    Vertical,
    Horizontal,
    Both,
}

impl TextareaVariant {
    pub fn classes(&self, has_error: bool, is_disabled: bool, is_focused: bool) -> String {
        let base = "w-full transition-colors duration-200 focus:outline-none";
        
        let variant_classes = match self {
            TextareaVariant::Default => "border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100",
            TextareaVariant::Filled => "border-0 bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100",
            TextareaVariant::Outline => "border-2 border-gray-300 dark:border-gray-600 bg-transparent text-gray-900 dark:text-gray-100",
            TextareaVariant::Ghost => "border-0 bg-transparent text-gray-900 dark:text-gray-100 hover:bg-gray-50 dark:hover:bg-gray-800",
        };

        let state_classes = if has_error {
            "border-red-500 dark:border-red-400 focus:ring-2 focus:ring-red-500/20 dark:focus:ring-red-400/20"
        } else if is_disabled {
            "opacity-50 cursor-not-allowed"
        } else if is_focused {
            match self {
                TextareaVariant::Default | TextareaVariant::Outline => "border-blue-500 dark:border-blue-400 focus:ring-2 focus:ring-blue-500/20 dark:focus:ring-blue-400/20",
                TextareaVariant::Filled => "ring-2 ring-blue-500/20 dark:ring-blue-400/20",
                TextareaVariant::Ghost => "bg-gray-50 dark:bg-gray-800",
            }
        } else {
            match self {
                TextareaVariant::Default | TextareaVariant::Outline => "hover:border-gray-400 dark:hover:border-gray-500",
                TextareaVariant::Filled => "hover:bg-gray-200 dark:hover:bg-gray-700",
                TextareaVariant::Ghost => "",
            }
        };

        format!("{} {} {}", base, variant_classes, state_classes)
    }
}

impl TextareaSize {
    pub fn classes(&self) -> &'static str {
        match self {
            TextareaSize::Small => "px-2 py-1 text-xs rounded-md min-h-16",
            TextareaSize::Medium => "px-3 py-2 text-sm rounded-md min-h-20",
            TextareaSize::Large => "px-4 py-3 text-base rounded-lg min-h-24",
        }
    }
}

impl TextareaResize {
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

/// Props for the Textarea component
#[derive(Props, Clone, PartialEq)]
pub struct TextareaProps {
    /// Textarea variant
    #[props(default)]
    pub variant: TextareaVariant,
    
    /// Textarea size
    #[props(default)]
    pub size: TextareaSize,
    
    /// Resize behavior
    #[props(default)]
    pub resize: TextareaResize,
    
    /// Current value
    #[props(default = String::new())]
    pub value: String,
    
    /// Placeholder text
    #[props(default = String::new())]
    pub placeholder: String,
    
    /// Whether the textarea is disabled
    #[props(default = false)]
    pub disabled: bool,
    
    /// Whether the textarea has an error state
    #[props(default = false)]
    pub error: bool,
    
    /// Whether the textarea is required
    #[props(default = false)]
    pub required: bool,
    
    /// Whether the textarea is read-only
    #[props(default = false)]
    pub readonly: bool,
    
    /// Maximum length of input
    #[props(default)]
    pub maxlength: Option<u32>,
    
    /// Number of visible text lines
    #[props(default)]
    pub rows: Option<u32>,
    
    /// Number of visible character columns
    #[props(default)]
    pub cols: Option<u32>,
    
    /// Textarea name attribute
    #[props(default = String::new())]
    pub name: String,
    
    /// Textarea id attribute
    #[props(default = String::new())]
    pub id: String,
    
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
    
    /// Textarea change handler
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
    
    /// Textarea input handler (fires on every keystroke)
    #[props(default)]
    pub oninput: Option<EventHandler<String>>,
    
    /// Focus handler
    #[props(default)]
    pub onfocus: Option<EventHandler<FocusEvent>>,
    
    /// Blur handler
    #[props(default)]
    pub onblur: Option<EventHandler<FocusEvent>>,
    
    /// Keypress handler
    #[props(default)]
    pub onkeypress: Option<EventHandler<KeyboardEvent>>,
}

/// Textarea component with variants, states, and accessibility
#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let mut is_focused = use_signal(|| false);
    
    let textarea_classes = props.variant.classes(props.error, props.disabled, is_focused());
    let size_classes = props.size.classes();
    let resize_classes = props.resize.classes();
    
    let combined_classes = if props.class.is_empty() {
        format!("{} {} {}", textarea_classes, size_classes, resize_classes)
    } else {
        format!("{} {} {} {}", textarea_classes, size_classes, resize_classes, props.class)
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
                let oninput = props.oninput.clone();
                move |evt| {
                    if let Some(handler) = &oninput {
                        handler.call(evt.value());
                    }
                }
            },
            
            onchange: {
                let onchange = props.onchange.clone();
                move |evt| {
                    if let Some(handler) = &onchange {
                        handler.call(evt.value());
                    }
                }
            },
            
            onfocus: {
                let onfocus = props.onfocus.clone();
                move |evt| {
                    is_focused.set(true);
                    if let Some(handler) = &onfocus {
                        handler.call(evt);
                    }
                }
            },
            
            onblur: {
                let onblur = props.onblur.clone();
                move |evt| {
                    is_focused.set(false);
                    if let Some(handler) = &onblur {
                        handler.call(evt);
                    }
                }
            },
            
            onkeypress: {
                let onkeypress = props.onkeypress.clone();
                move |evt| {
                    if let Some(handler) = &onkeypress {
                        handler.call(evt);
                    }
                }
            },
        }
    }
}