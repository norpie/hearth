//! One-time password (OTP) input component for secure authentication
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     InputOTP {
//!         length: 6,
//!         oncomplete: move |otp| verify_otp(otp),
//!     }
//! }
//! ```

use dioxus::prelude::*;
use crate::Platform;

#[cfg(target_arch = "wasm32")]
use web_sys::wasm_bindgen::JsCast;

/// Properties for the InputOTP component
#[derive(Props, Clone, PartialEq)]
pub struct InputOTPProps {
    /// Number of digits in the OTP sequence
    #[props(default = 6)]
    pub length: usize,
    /// Initial value for the OTP input
    #[props(default = String::new())]
    pub value: String,
    /// Whether to show a visual separator in the middle
    #[props(default = true)]
    pub show_middle: bool,
    /// Whether all input fields are disabled
    #[props(default = false)]
    pub disabled: bool,
    /// Additional CSS classes to apply to the container
    #[props(default = String::new())]
    pub class: String,
    /// Handler for OTP value changes
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
    /// Handler called when the OTP is completely filled
    #[props(default)]
    pub oncomplete: Option<EventHandler<String>>,
}

/// One-time password input component with auto-focus and keyboard navigation
#[component]
pub fn InputOTP(props: InputOTPProps) -> Element {
    let mut focused_index = use_signal(|| Option::<usize>::None);
    let mut internal_value = use_signal(String::new);

    let component_id = use_signal(|| uuid::Uuid::new_v4().to_string());

    use_effect(move || {
        if let Some(index) = focused_index() {
            Platform::spawn(async move {
                Platform::sleep(std::time::Duration::from_millis(10)).await;

                #[cfg(target_arch = "wasm32")]
                {
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            let id = format!("otp-input-{}-{}", component_id(), index);
                            if let Some(element) = document.get_element_by_id(&id) {
                                if let Ok(input) = element.dyn_into::<web_sys::HtmlInputElement>() {
                                    let _ = input.focus();
                                }
                            }
                        }
                    }
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    use dioxus_document::eval;
                    let id = format!("otp-input-{}-{}", component_id(), index);
                    let script = format!("document.getElementById('{id}')?.focus()");
                    eval(&script);
                }
            });
        }
    });
    let padded_value = {
        let current_value = internal_value.read();
        let mut chars: Vec<char> = current_value.chars().take(props.length).collect();
        chars.resize(props.length, ' ');
        chars
    };

    let handle_input = {
        let oncomplete = props.oncomplete;
        let length = props.length;

        move |index: usize, new_char: String| {
            let current_value = {
                let val = internal_value.read();
                val.clone()
            };
            let mut chars: Vec<char> = current_value.chars().take(length).collect();
            chars.resize(length, ' ');

            if let Some(ch) = new_char.chars().next() {
                if ch.is_ascii_digit() {
                    chars[index] = ch;
                }
            } else if new_char.is_empty() {
                chars[index] = ' ';
            }

            let result: String = chars
                .iter()
                .map(|&c| {
                    if c == ' ' {
                        String::new()
                    } else {
                        c.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("");

            internal_value.set(result.clone());

            if result.len() == length && result.chars().all(|c| c.is_ascii_digit()) {
                if let Some(handler) = &oncomplete {
                    handler.call(result);
                }
            }
        }
    };

    let handle_keydown = {
        let mut handle_input = handle_input;
        let length = props.length;
        move |index: usize, evt: KeyboardEvent| {
            let key = evt.key();
            match key {
                dioxus::prelude::Key::Backspace => {
                    let current_value = {
                        let val = internal_value.read();
                        val.clone()
                    };
                    let mut chars: Vec<char> = current_value.chars().take(length).collect();
                    chars.resize(length, ' ');

                    if chars[index] != ' ' {
                        handle_input(index, String::new());
                    } else if index > 0 {
                        focused_index.set(Some(index - 1));
                        handle_input(index - 1, String::new());
                    }
                }
                dioxus::prelude::Key::ArrowLeft => {
                    if index > 0 {
                        focused_index.set(Some(index - 1));
                    }
                }
                dioxus::prelude::Key::ArrowRight => {
                    if index < length - 1 {
                        focused_index.set(Some(index + 1));
                    }
                }
                dioxus::prelude::Key::Character(ch)
                    if ch.len() == 1 && ch.chars().next().unwrap().is_ascii_digit() =>
                {
                    handle_input(index, ch.to_string());
                    if index < length - 1 {
                        focused_index.set(Some(index + 1));
                    }
                }
                _ => {}
            }
        }
    };

    let container_classes = if props.class.is_empty() {
        "flex items-center gap-2".to_string()
    } else {
        format!("flex items-center gap-2 {}", props.class)
    };

    let show_separator = props.show_middle && props.length % 2 == 0;
    let middle_index = props.length / 2;

    rsx! {
        div { class: container_classes,
            for (index , & char) in padded_value.iter().enumerate() {
                if show_separator && index == middle_index {
                    div {
                        class: "w-2 h-2 bg-muted rounded-full",
                        "aria-hidden": "true",
                    }
                }
                input {
                    r#type: "text",
                    id: format!("otp-input-{}-{}", component_id(), index),
                    class: "w-12 h-12 text-center text-lg font-mono border border-border bg-card text-foreground rounded-md transition-colors duration-200 focus:outline-none focus:border-primary focus:ring-2 focus:ring-ring/20 disabled:opacity-50 disabled:cursor-not-allowed",
                    value: if char == ' ' { "" } else { char.to_string() },
                    maxlength: "1",
                    disabled: props.disabled,
                    onfocus: move |_| {
                        focused_index.set(Some(index));
                    },
                    onblur: {
                        move |_| {
                            focused_index.set(None);
                        }
                    },
                    oninput: {
                        let mut handle_input = handle_input;
                        let length = props.length;
                        move |evt| {
                            let value = evt.value();
                            handle_input(index, value.clone());

                            if !value.is_empty() && value.chars().next().unwrap().is_ascii_digit()
                                && index < length - 1
                            {
                                focused_index.set(Some(index + 1));
                            }
                        }
                    },
                    onkeydown: {
                        let mut handle_keydown = handle_keydown;
                        move |evt| {
                            handle_keydown(index, evt);
                        }
                    },
                }
            }
        }
    }
}
