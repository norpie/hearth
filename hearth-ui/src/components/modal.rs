//! Reusable modal component

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ModalSize {
    Small,
    Medium, 
    Large,
    Full,
}

impl ModalSize {
    pub fn max_width(&self) -> &'static str {
        match self {
            ModalSize::Small => "28rem",
            ModalSize::Medium => "48rem", 
            ModalSize::Large => "72rem",
            ModalSize::Full => "95vw",
        }
    }
}

impl Default for ModalSize {
    fn default() -> Self {
        ModalSize::Medium
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    /// Modal title displayed in header
    #[props(optional)]
    pub title: Option<String>,
    /// Controls modal visibility
    pub is_open: Signal<bool>,
    /// Modal content
    pub children: Element,
    /// Show/hide close button in header (default: true)
    #[props(default = true)]
    pub show_close_button: bool,
    /// Allow closing by clicking backdrop (default: true) 
    #[props(default = true)]
    pub close_on_backdrop_click: bool,
    /// Modal size (default: Medium)
    #[props(default = ModalSize::Medium)]
    pub size: ModalSize,
    /// Hide header completely (default: false)
    #[props(default = false)]
    pub hide_header: bool,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    if !(props.is_open)() {
        return rsx! {
            div {}
        };
    }

    // Create closures outside the rsx! macro
    let backdrop_click = {
        let mut is_open = props.is_open;
        let close_on_backdrop_click = props.close_on_backdrop_click;
        move |_| {
            if close_on_backdrop_click {
                is_open.set(false);
            }
        }
    };

    let close_button_click = {
        let mut is_open = props.is_open;
        move |_| is_open.set(false)
    };

    let key_down = {
        let mut is_open = props.is_open;
        move |evt: KeyboardEvent| {
            if evt.key() == dioxus::prelude::Key::Escape {
                is_open.set(false);
            }
        }
    };

    // Auto-focus the modal when it opens
    use_effect(move || {
        if (props.is_open)() {
            spawn(async move {
                // Small delay to ensure the modal is rendered
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
                    use web_sys::wasm_bindgen::JsCast;
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Ok(Some(modal)) = document.query_selector("[data-modal-backdrop]") {
                                if let Ok(element) = modal.dyn_into::<web_sys::HtmlElement>() {
                                    let _ = element.focus();
                                }
                            }
                        }
                    }
                }
            });
        }
    });

    rsx! {
        // Backdrop overlay
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center",
            style: "background: rgba(0, 0, 0, 0.6); backdrop-filter: blur(4px); padding: 1rem;",
            onclick: backdrop_click,
            onkeydown: key_down,
            tabindex: 0,
            "data-modal-backdrop": "",
            
            // Modal container
            div {
                class: "bg-white dark:bg-gray-900 rounded-xl shadow-2xl w-full flex flex-col overflow-hidden border border-gray-200 dark:border-gray-700",
                style: format!("max-width: {}; max-height: calc(100vh - 2rem);", props.size.max_width()),
                onclick: move |e| e.stop_propagation(),
                
                // Modal header (conditional)
                if !props.hide_header {
                    div { 
                        class: "flex-shrink-0 flex items-center justify-between px-6 py-4 bg-gray-50 dark:bg-gray-800/50",
                        
                        // Title (if provided)
                        if let Some(title) = &props.title {
                            h2 { 
                                class: "text-lg font-semibold text-gray-900 dark:text-gray-100 border-b-0",
                                "{title}"
                            }
                        } else {
                            div {}
                        }
                        
                        // Close button (conditional)
                        if props.show_close_button {
                            button {
                                class: "p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-lg transition-all duration-150 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800",
                                onclick: close_button_click,
                                "aria-label": "Close modal",
                                svg {
                                    class: "w-5 h-5",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round", 
                                        stroke_width: "2",
                                        d: "M6 18L18 6M6 6l12 12"
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Modal content
                div { 
                    class: "flex-1 overflow-hidden flex flex-col",
                    {props.children} 
                }
            }
        }
    }
}