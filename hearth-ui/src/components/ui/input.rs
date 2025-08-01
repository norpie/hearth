//! Input component with multiple variants and states
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     Input {
//!         placeholder: "Enter text",
//!         value: input_value(),
//!         oninput: move |val| input_value.set(val),
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Visual style variants for input fields
/// 
/// Each variant provides a different visual treatment suitable for various UI contexts:
/// - `Default`: Standard bordered input with background fill
/// - `Filled`: Background-filled input without borders, subtle appearance
/// - `Outline`: Prominent double border for high-emphasis fields
/// - `Ghost`: Minimal styling with hover effects, lowest visual impact
#[derive(Clone, PartialEq, Debug)]
pub enum InputVariant {
    /// Standard input with single border and background
    Default,
    /// Background-filled input without borders
    Filled,
    /// Input with prominent double border styling
    Outline,
    /// Minimal input with transparent background and hover effects
    Ghost,
}

/// Size variants controlling input dimensions and typography
///
/// Provides consistent sizing across the design system:
/// - `Small`: Compact inputs for dense layouts (px-2 py-1, text-xs)
/// - `Medium`: Standard size for most use cases (px-3 py-2, text-sm)
/// - `Large`: Prominent inputs for important fields (px-4 py-3, text-base)
#[derive(Clone, PartialEq, Debug)]
pub enum InputSize {
    /// Small input size for compact layouts
    Small,
    /// Medium input size (default) for standard use
    Medium,
    /// Large input size for prominent form fields
    Large,
}

/// HTML input types for semantic input behavior
///
/// Maps to standard HTML input types to provide appropriate:
/// - Browser validation and input methods
/// - Virtual keyboard layouts on mobile devices
/// - Accessibility hints for screen readers
/// - Auto-completion behavior
#[derive(Clone, PartialEq, Debug)]
pub enum InputType {
    /// General text input (default)
    Text,
    /// Email address input with validation
    Email,
    /// Password input with character masking
    Password,
    /// Numeric input with number keyboard
    Number,
    /// Telephone number input
    Tel,
    /// URL input with validation
    Url,
    /// Search input with search-specific styling
    Search,
}

impl InputVariant {
    /// Returns CSS classes for this input variant with dynamic state handling
    /// 
    /// Generates the complete CSS class string based on the variant and current state.
    /// The state parameters control error styling, disabled appearance, and focus effects.
    /// 
    /// # Parameters
    /// 
    /// - `has_error`: When true, applies error styling (red borders/focus rings)
    /// - `is_disabled`: When true, applies disabled styling (reduced opacity)
    /// - `is_focused`: When true, applies focus styling (enhanced borders/backgrounds)
    pub fn classes(&self, has_error: bool, is_disabled: bool, is_focused: bool) -> String {
        let base = "w-full transition-colors duration-200 focus:outline-none";

        let variant_classes = match self {
            InputVariant::Default => "border border-input bg-background text-foreground",
            InputVariant::Filled => "border-0 bg-muted text-foreground",
            InputVariant::Outline => "border-2 border-border bg-transparent text-foreground",
            InputVariant::Ghost => "border-0 bg-transparent text-foreground hover:bg-accent",
        };

        let size_classes = "px-3 py-2 rounded-md text-sm";

        let state_classes = if has_error {
            "border-destructive focus:ring-2 focus:ring-destructive/20"
        } else if is_disabled {
            "opacity-50 cursor-not-allowed"
        } else if is_focused {
            match self {
                InputVariant::Default | InputVariant::Outline => {
                    "border-primary focus:ring-2 focus:ring-ring/20"
                }
                InputVariant::Filled => "ring-2 ring-ring/20",
                InputVariant::Ghost => "bg-accent",
            }
        } else {
            match self {
                InputVariant::Default | InputVariant::Outline => "hover:border-border/80",
                InputVariant::Filled => "hover:bg-muted/80",
                InputVariant::Ghost => "",
            }
        };

        format!("{base} {variant_classes} {size_classes} {state_classes}")
    }
}

impl InputSize {
    /// Returns CSS classes for this input size
    /// 
    /// Provides padding and text size classes that determine the input's dimensions
    /// and typography scale. All sizes include border radius and consistent spacing.
    pub fn classes(&self) -> &'static str {
        match self {
            InputSize::Small => "px-2 py-1 text-xs",
            InputSize::Medium => "px-3 py-2 text-sm",
            InputSize::Large => "px-4 py-3 text-base",
        }
    }
}

impl InputType {
    /// Returns the HTML input type attribute value
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

/// Properties for the Input component
#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// Visual style variant of the input
    #[props(default)]
    pub variant: InputVariant,
    /// Size of the input field
    #[props(default)]
    pub size: InputSize,
    /// HTML input type for semantic behavior
    #[props(default)]
    pub input_type: InputType,
    /// Current value of the input field
    #[props(default = String::new())]
    pub value: String,
    #[props(default = String::new())]
    pub placeholder: String,
    /// Whether the input is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Whether the input is in an error state
    #[props(default = false)]
    pub error: bool,
    /// Whether the input is required for form submission
    #[props(default = false)]
    pub required: bool,
    /// HTML name attribute for form submission
    #[props(default = String::new())]
    pub name: String,
    /// HTML id attribute for accessibility and labeling
    #[props(default = String::new())]
    pub id: String,
    /// Additional CSS classes to apply
    #[props(default = String::new())]
    pub class: String,
    /// Handler for input change events
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
    /// Handler for input events (real-time)
    #[props(default)]
    pub oninput: Option<EventHandler<String>>,
    /// Handler for focus events
    #[props(default)]
    pub onfocus: Option<EventHandler<FocusEvent>>,
    /// Handler for blur events
    #[props(default)]
    pub onblur: Option<EventHandler<FocusEvent>>,
    /// Handler for keypress events
    #[props(default)]
    pub onkeypress: Option<EventHandler<KeyboardEvent>>,
}

/// Interactive text input field with multiple variants and states
#[component]
pub fn Input(props: InputProps) -> Element {
    let mut is_focused = use_signal(|| false);

    let input_classes = props
        .variant
        .classes(props.error, props.disabled, is_focused());
    let size_classes = props.size.classes();
    let combined_classes = if props.class.is_empty() {
        format!("{input_classes} {size_classes}")
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
