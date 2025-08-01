//! Dropdown selection component with search capabilities and accessibility
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     Select {
//!         value: selected_value(),
//!         options: options,
//!         placeholder: "Choose an option...",
//!         onchange: move |value| selected_value.set(value),
//!     }
//! }
//! ```

use dioxus::prelude::*;
use crate::Platform;

/// Size variants controlling select dimensions and typography
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectSize {
    /// Small select size for compact layouts
    Small,
    /// Medium select size (default) for standard use
    Medium,
    /// Large select size for prominent form fields
    Large,
}

impl Default for SelectSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Visual style variants for select elements
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectVariant {
    /// Standard select with background and border
    Default,
    /// Muted background select for subtle appearance
    Filled,
}

impl Default for SelectVariant {
    fn default() -> Self {
        Self::Default
    }
}

/// Configuration for individual select options
#[derive(Debug, Clone, PartialEq)]
pub struct SelectOption {
    /// The value returned when this option is selected
    pub value: String,
    /// The text displayed to users for this option
    pub label: String,
    /// Whether this option can be selected
    pub disabled: bool,
}

impl SelectOption {
    /// Creates a new selectable option with the given value and label
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    /// Marks this option as disabled, preventing user selection
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Properties for the Select component
#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    /// Currently selected option value
    #[props(default = String::new())]
    pub value: String,
    /// Callback function called when the selection changes
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
    /// List of available options for selection
    pub options: Vec<SelectOption>,
    /// Placeholder text displayed when no option is selected
    #[props(default = "Select an option...".to_string())]
    pub placeholder: String,

    /// Whether the entire select is disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Whether the select is required for form submission
    #[props(default = false)]
    pub required: bool,
    /// Whether the select is in an error state
    #[props(default = false)]
    pub error: bool,
    /// Size variant of the select element
    #[props(default = SelectSize::default())]
    pub size: SelectSize,
    /// Visual style variant of the select
    #[props(default = SelectVariant::default())]
    pub variant: SelectVariant,

    /// Additional CSS classes to apply to the select button
    #[props(default = String::new())]
    pub class: String,
    /// HTML id attribute for accessibility and form association
    #[props(default = String::new())]
    pub id: String,
    /// ARIA label for accessibility when no visible label exists
    #[props(default = String::new())]
    pub aria_label: String,
    /// HTML name attribute for form submission
    #[props(default = String::new())]
    pub name: String,
    /// Enable search functionality with fuzzy matching
    #[props(default = false)]
    pub searchable: bool,
}

/// Performs fuzzy matching between a search query and option text
fn fuzzy_match(query: &str, text: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    let query_lower = query.to_lowercase();
    let text_lower = text.to_lowercase();

    if text_lower.contains(&query_lower) {
        return true;
    }
    let mut query_chars = query_lower.chars().peekable();
    let mut text_chars = text_lower.chars();

    while let Some(&query_char) = query_chars.peek() {
        let mut found = false;
        for text_char in text_chars.by_ref() {
            if text_char == query_char {
                query_chars.next();
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }

    query_chars.peek().is_none()
}

/// Interactive dropdown selection component with search and accessibility
#[component]
pub fn Select(props: SelectProps) -> Element {
    let mut is_open = use_signal(|| false);
    let mut search_query = use_signal(String::new);

    use_effect(move || {
        if props.searchable && is_open() {
            Platform::spawn(async move {
                Platform::sleep(std::time::Duration::from_millis(10)).await;

                #[cfg(target_arch = "wasm32")]
                {
                    use web_sys::wasm_bindgen::JsCast;
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Ok(Some(input)) = document.query_selector("#search-input") {
                                if let Ok(element) = input.dyn_into::<web_sys::HtmlElement>() {
                                    let _ = element.focus();
                                }
                            }
                        }
                    }
                }
            });
        }
    });

    let (base_classes, icon_classes) = get_size_classes(props.size);
    let variant_classes = get_variant_classes(props.variant, props.error, props.disabled);

    let combined_classes = if props.class.is_empty() {
        format!("{base_classes} {variant_classes}")
    } else {
        format!("{} {} {}", base_classes, variant_classes, props.class)
    };

    let filtered_options: Vec<_> = if props.searchable && !search_query().is_empty() {
        props
            .options
            .iter()
            .filter(|opt| fuzzy_match(&search_query(), &opt.label))
            .cloned()
            .collect()
    } else {
        props.options.clone()
    };

    let selected_option = props.options.iter().find(|opt| opt.value == props.value);

    let display_text = if let Some(option) = selected_option {
        option.label.clone()
    } else {
        props.placeholder.clone()
    };

    let select_id = if props.id.is_empty() {
        "select".to_string()
    } else {
        props.id.clone()
    };

    rsx! {
        if props.searchable && is_open() {
            div {
                class: "fixed inset-0 z-40",
                onclick: move |_| {
                    is_open.set(false);
                    search_query.set(String::new());
                },
            }
        }
        div { class: "relative",
            select {
                id: select_id.as_str(),
                name: if props.name.is_empty() { None } else { Some(props.name.as_str()) },
                value: props.value.as_str(),
                required: props.required,
                disabled: props.disabled,
                class: "sr-only",
                "aria-label": if props.aria_label.is_empty() { None } else { Some(props.aria_label.as_str()) },
                onchange: move |evt| {
                    let value = evt.value();
                    if let Some(onchange) = &props.onchange {
                        onchange.call(value);
                    }
                },
                for option in props.options.iter() {
                    option {
                        value: option.value.as_str(),
                        disabled: option.disabled,
                        selected: option.value == props.value,
                        "{option.label}"
                    }
                }
            }
            button {
                r#type: "button",
                class: combined_classes,
                disabled: props.disabled,
                "aria-expanded": is_open(),
                "aria-haspopup": "listbox",
                onclick: move |_| {
                    if !props.disabled {
                        is_open.set(!is_open());
                    }
                },
                onblur: move |_| {
                    let searchable = props.searchable;
                    if !searchable {
                        Platform::spawn(async move {
                            Platform::sleep(std::time::Duration::from_millis(150)).await;
                            is_open.set(false);
                        });
                    }
                },
                span { class: if selected_option.is_some() { "text-foreground" } else { "text-muted-foreground" },
                    "{display_text}"
                }
                span { class: format!("ml-2 flex-shrink-0 {}", icon_classes),
                    svg {
                        class: format!(
                            "transition-transform duration-200 {}",
                            if is_open() { "rotate-180" } else { "rotate-0" },
                        ),
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        stroke: "currentColor",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                            d: "M19 9l-7 7-7-7",
                        }
                    }
                }
            }
            if is_open() {
                div {
                    class: "absolute z-50 w-full mt-1 bg-popover border border-border rounded-md shadow-lg max-h-60 overflow-hidden flex flex-col",
                    role: "listbox",
                    onclick: move |evt| {
                        evt.stop_propagation();
                    },
                    if props.searchable {
                        div {
                            class: "flex-shrink-0 p-3 border-b border-border",
                            onclick: move |evt| {
                                evt.stop_propagation();
                            },
                            div { class: "relative",
                                input {
                                    r#type: "text",
                                    id: "search-input",
                                    class: "w-full pl-8 pr-3 py-2 text-sm bg-transparent border-none focus:outline-none text-foreground placeholder-muted-foreground",
                                    placeholder: "Search options...",
                                    value: search_query(),
                                    oninput: move |evt| {
                                        search_query.set(evt.value());
                                    },
                                    onkeydown: move |evt| {
                                        if evt.key() == dioxus::prelude::Key::Escape {
                                            is_open.set(false);
                                            search_query.set(String::new());
                                        }
                                        evt.stop_propagation();
                                    },
                                    onclick: move |evt| {
                                        evt.stop_propagation();
                                    },
                                }
                                div { class: "absolute left-2 top-2.5 w-4 h-4 text-muted-foreground pointer-events-none",
                                    svg {
                                        fill: "none",
                                        stroke: "currentColor",
                                        "viewBox": "0 0 24 24",
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            "stroke-width": "2",
                                            d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z",
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "flex-1 overflow-auto",
                        if props.searchable && !search_query().is_empty() && filtered_options.is_empty() {
                            div { class: "px-3 py-8 text-center text-muted-foreground text-sm",
                                "No options found matching \""
                                span { class: "font-medium", "{search_query()}" }
                                "\""
                            }
                        }
                        for option in filtered_options.iter() {
                            button {
                                r#type: "button",
                                class: format!(
                                    "w-full text-left px-3 py-2 hover:bg-accent focus:bg-accent focus:outline-none {}",
                                    if option.disabled {
                                        "text-muted-foreground cursor-not-allowed"
                                    } else if option.value == props.value {
                                        "bg-primary/10 text-primary font-medium"
                                    } else {
                                        "text-foreground"
                                    },
                                ),
                                disabled: option.disabled,
                                role: "option",
                                "aria-selected": option.value == props.value,
                                onclick: {
                                    let option_value = option.value.clone();
                                    let option_disabled = option.disabled;
                                    let searchable = props.searchable;
                                    move |_| {
                                        if !option_disabled {
                                            is_open.set(false);
                                            if searchable {
                                                search_query.set(String::new());
                                            }
                                            if let Some(onchange) = &props.onchange {
                                                onchange.call(option_value.clone());
                                            }
                                        }
                                    }
                                },
                                "{option.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Returns CSS classes for select button and icon based on size variant
fn get_size_classes(size: SelectSize) -> (&'static str, &'static str) {
    match size {
        SelectSize::Small => (
            "px-3 py-1.5 text-sm rounded-md w-full min-w-32 flex items-center justify-between",
            "w-4 h-4",
        ),
        SelectSize::Medium => (
            "px-3 py-2 text-sm rounded-md w-full min-w-40 flex items-center justify-between",
            "w-5 h-5",
        ),
        SelectSize::Large => (
            "px-4 py-2.5 text-base rounded-md w-full min-w-48 flex items-center justify-between",
            "w-5 h-5",
        ),
    }
}

/// Returns CSS classes for select styling based on variant and state
fn get_variant_classes(variant: SelectVariant, error: bool, disabled: bool) -> &'static str {
    if disabled {
        return "bg-muted border border-border text-muted-foreground cursor-not-allowed opacity-60 transition-colors";
    }

    if error {
        return match variant {
            SelectVariant::Default => "bg-background border-2 border-destructive text-foreground focus:ring-2 focus:ring-destructive focus:border-destructive transition-colors cursor-pointer",
            SelectVariant::Filled => "bg-muted border-2 border-destructive text-foreground focus:ring-2 focus:ring-destructive focus:border-destructive transition-colors cursor-pointer",
        };
    }

    match variant {
        SelectVariant::Default => "bg-background border border-input text-foreground hover:border-border focus:ring-2 focus:ring-ring focus:border-primary transition-colors cursor-pointer",
        SelectVariant::Filled => "bg-muted border border-input text-foreground hover:border-border focus:ring-2 focus:ring-ring focus:border-primary transition-colors cursor-pointer",
    }
}
