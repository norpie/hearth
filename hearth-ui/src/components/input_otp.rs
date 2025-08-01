//! Input OTP component for one-time password entry

use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use {
    gloo_timers,
    web_sys::wasm_bindgen::JsCast,
};

/// Props for the InputOTP component
#[derive(Props, Clone, PartialEq)]
pub struct InputOTPProps {
    /// Number of OTP digits
    #[props(default = 6)]
    pub length: usize,
    
    /// Initial OTP value
    #[props(default = String::new())]
    pub value: String,
    
    /// Whether to show a separator dot in the middle for even-numbered OTPs
    #[props(default = true)]
    pub show_middle: bool,
    
    /// Whether the input is disabled
    #[props(default = false)]
    pub disabled: bool,
    
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
    
    /// Change handler when OTP value changes
    #[props(default)]
    pub onchange: Option<EventHandler<String>>,
    
    /// Complete handler when all digits are filled
    #[props(default)]
    pub oncomplete: Option<EventHandler<String>>,
}

/// Input OTP component for one-time password entry
#[component]
pub fn InputOTP(props: InputOTPProps) -> Element {
    let mut focused_index = use_signal(|| Option::<usize>::None);
    let mut internal_value = use_signal(|| String::new()); // Start with empty value, ignore props.value completely
    
    // Generate unique component ID to prevent conflicts with multiple OTP components on same page
    let component_id = use_signal(|| uuid::Uuid::new_v4().to_string());
    
    // Auto-focus effect when focused_index changes (using modal's proven technique)
    use_effect(move || {
        if let Some(index) = focused_index() {
            spawn(async move {
                // Small delay to ensure the input is rendered (same as modal)
                #[cfg(target_arch = "wasm32")]
                {
                    gloo_timers::future::sleep(std::time::Duration::from_millis(10)).await;
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                }
                
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
                    // For desktop/mobile, use eval to focus the element
                    use dioxus_document::eval;
                    let id = format!("otp-input-{}-{}", component_id(), index);
                    let script = format!("document.getElementById('{}')?.focus()", id);
                    eval(&script);
                }
            });
        }
    });
    
    // Pad or truncate internal value to match length for display
    let padded_value = {
        let current_value = internal_value.read();
        let mut chars: Vec<char> = current_value.chars().take(props.length).collect();
        chars.resize(props.length, ' ');
        chars
    };
    
    let mut handle_input = {
        let oncomplete = props.oncomplete.clone();
        let length = props.length;
        
        move |index: usize, new_char: String| {
            // Get current value from internal state
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
            
            let result: String = chars.iter()
                .map(|&c| if c == ' ' { String::new() } else { c.to_string() })
                .collect::<Vec<_>>()
                .join("");
            
            // Update internal state only - no parent notification during input
            internal_value.set(result.clone());
            
            // Check if complete (all digits filled) - only notify parent when complete
            if result.len() == length && result.chars().all(|c| c.is_ascii_digit()) {
                if let Some(handler) = &oncomplete {
                    handler.call(result);
                }
            }
        }
    };
    
    let mut handle_keydown = {
        let mut handle_input = handle_input.clone();
        let length = props.length;
        move |index: usize, evt: KeyboardEvent| {
            let key = evt.key();
            match key {
                dioxus::prelude::Key::Backspace => {
                    // Get current value from internal state to check if digit exists
                    let current_value = {
                        let val = internal_value.read();
                        val.clone()
                    };
                    let mut chars: Vec<char> = current_value.chars().take(length).collect();
                    chars.resize(length, ' ');
                    
                    if chars[index] != ' ' {
                        // Clear current digit
                        handle_input(index, String::new());
                    } else if index > 0 {
                        // Move to previous digit and clear it
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
                dioxus::prelude::Key::Character(ch) if ch.len() == 1 && ch.chars().next().unwrap().is_ascii_digit() => {
                    handle_input(index, ch.to_string());
                    // Move to next digit
                    if index < length - 1 {
                        focused_index.set(Some(index + 1));
                    }
                }
                _ => {}
            }
        }
    };
    
    // Base classes for the container
    let container_classes = if props.class.is_empty() {
        "flex items-center gap-2".to_string()
    } else {
        format!("flex items-center gap-2 {}", props.class)
    };
    
    // Check if we should show middle separator
    let show_separator = props.show_middle && props.length % 2 == 0;
    let middle_index = props.length / 2;
    
    rsx! {
        div {
            class: container_classes,
            
            for (index, &char) in padded_value.iter().enumerate() {
                // Show separator dot before middle digit on even-numbered OTPs
                if show_separator && index == middle_index {
                    div {
                        class: "w-2 h-2 bg-gray-400 dark:bg-gray-500 rounded-full",
                        "aria-hidden": "true"
                    }
                }
                
                input {
                    r#type: "text",
                    id: format!("otp-input-{}-{}", component_id(), index),
                    class: "w-12 h-12 text-center text-lg font-mono border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 rounded-md transition-colors duration-200 focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 focus:ring-2 focus:ring-blue-500/20 dark:focus:ring-blue-400/20 disabled:opacity-50 disabled:cursor-not-allowed",
                    value: if char == ' ' { "" } else { char.to_string() },
                    maxlength: "1",
                    disabled: props.disabled,
                    
                    onfocus: {
                        let index = index;
                        move |_| {
                            focused_index.set(Some(index));
                        }
                    },
                    
                    onblur: {
                        move |_| {
                            focused_index.set(None);
                        }
                    },
                    
                    oninput: {
                        let mut handle_input = handle_input.clone();
                        let length = props.length;
                        move |evt| {
                            let value = evt.value();
                            handle_input(index, value.clone());
                            
                            // Auto-focus next digit when current digit is filled
                            if !value.is_empty() && value.chars().next().unwrap().is_ascii_digit() && index < length - 1 {
                                focused_index.set(Some(index + 1));
                            }
                        }
                    },
                    
                    onkeydown: {
                        let mut handle_keydown = handle_keydown.clone();
                        move |evt| {
                            handle_keydown(index, evt);
                        }
                    },
                }
            }
        }
    }
}
