//! Input component with variants, states, and accessibility support

use dioxus::prelude::*;

/// Input variants for different use cases
#[derive(Clone, PartialEq, Debug)]
pub enum InputVariant {
    Default,
    Filled,
    Outline,
    Ghost,
}

/// Input sizes
#[derive(Clone, PartialEq, Debug)]
pub enum InputSize {
    Small,
    Medium,
    Large,
}

/// Input types
#[derive(Clone, PartialEq, Debug)]
pub enum InputType {
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
    Search,
}

impl InputVariant {
    pub fn classes(&self, has_error: bool, is_disabled: bool, is_focused: bool) -> String {
        let base = "w-full transition-colors duration-200 focus:outline-none";
        
        let variant_classes = match self {
            InputVariant::Default => "border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100",
            InputVariant::Filled => "border-0 bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100",
            InputVariant::Outline => "border-2 border-gray-300 dark:border-gray-600 bg-transparent text-gray-900 dark:text-gray-100",
            InputVariant::Ghost => "border-0 bg-transparent text-gray-900 dark:text-gray-100 hover:bg-gray-50 dark:hover:bg-gray-800",
        };

        let size_classes = "px-3 py-2 rounded-md text-sm";

        let state_classes = if has_error {
            "border-red-500 dark:border-red-400 focus:ring-2 focus:ring-red-500/20 dark:focus:ring-red-400/20"
        } else if is_disabled {
            "opacity-50 cursor-not-allowed"
        } else if is_focused {
            match self {
                InputVariant::Default | InputVariant::Outline => "border-blue-500 dark:border-blue-400 focus:ring-2 focus:ring-blue-500/20 dark:focus:ring-blue-400/20",
                InputVariant::Filled => "ring-2 ring-blue-500/20 dark:ring-blue-400/20",
                InputVariant::Ghost => "bg-gray-50 dark:bg-gray-800",
            }
        } else {
            match self {
                InputVariant::Default | InputVariant::Outline => "hover:border-gray-400 dark:hover:border-gray-500",
                InputVariant::Filled => "hover:bg-gray-200 dark:hover:bg-gray-700",
                InputVariant::Ghost => "",
            }
        };

        format!("{} {} {} {}", base, variant_classes, size_classes, state_classes)
    }
}

impl InputSize {
    pub fn classes(&self) -> &'static str {
        match self {
            InputSize::Small => "px-2 py-1 text-xs",
            InputSize::Medium => "px-3 py-2 text-sm",
            InputSize::Large => "px-4 py-3 text-base",
        }
    }
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email", 
            InputType::Password => "password",
            InputType::Number => "number",
            InputType::Tel => "tel",
            InputType::Url => "url",
            InputType::Search => "search",
        }
    }
}

impl Default for InputVariant {
    fn default() -> Self {
        Self::Default
    }
}

impl Default for InputSize {
    fn default() -> Self {
        Self::Medium
    }
}

impl Default for InputType {
    fn default() -> Self {
        Self::Text
    }
}

/// Props for the Input component
#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// Input variant
    #[props(default)]
    pub variant: InputVariant,
    
    /// Input size
    #[props(default)]
    pub size: InputSize,
    
    /// Input type
    #[props(default)]
    pub input_type: InputType,
    
    /// Current value
    #[props(default = String::new())]
    pub value: String,
    
    /// Placeholder text
    #[props(default = String::new())]
    pub placeholder: String,
    
    /// Whether the input is disabled
    #[props(default = false)]
    pub disabled: bool,
    
    /// Whether the input has an error state
    #[props(default = false)]
    pub error: bool,
    
    /// Whether the input is required
    #[props(default = false)]
    pub required: bool,
    
    /// Input name attribute
    #[props(default = String::new())]
    pub name: String,
    
    /// Input id attribute
    #[props(default = String::new())]
    pub id: String,
    
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
    
    /// Input change handler
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
    
    /// Input input handler (fires on every keystroke)
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

/// Input component with variants, states, and accessibility
#[component]
pub fn Input(props: InputProps) -> Element {
    let mut is_focused = use_signal(|| false);
    
    let input_classes = props.variant.classes(props.error, props.disabled, is_focused());
    let size_classes = props.size.classes();
    let combined_classes = if props.class.is_empty() {
        format!("{} {}", input_classes, size_classes)
    } else {
        format!("{} {} {}", input_classes, size_classes, props.class)
    };

    rsx! {
        input {
            r#type: props.input_type.as_str(),
            class: combined_classes,
            value: props.value,
            placeholder: props.placeholder,
            disabled: props.disabled,
            required: props.required,
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