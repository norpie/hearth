//! Sheet component that slides in from any side with arbitrary content

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum SheetSide {
    Top,
    Right, 
    Bottom,
    Left,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SheetSize {
    Small,
    Medium,
    Large,
    Full,
}

impl Default for SheetSide {
    fn default() -> Self {
        SheetSide::Right
    }
}

impl Default for SheetSize {
    fn default() -> Self {
        SheetSize::Medium
    }
}

impl SheetSide {
    pub fn container_classes(&self) -> &'static str {
        match self {
            SheetSide::Top => "justify-center items-start",
            SheetSide::Right => "justify-end items-center", 
            SheetSide::Bottom => "justify-center items-end",
            SheetSide::Left => "justify-start items-center",
        }
    }

    pub fn sheet_classes(&self) -> &'static str {
        match self {
            SheetSide::Top => "w-full max-h-[90vh] rounded-b-xl",
            SheetSide::Right => "h-full max-w-[90vw] rounded-l-xl",
            SheetSide::Bottom => "w-full max-h-[90vh] rounded-t-xl", 
            SheetSide::Left => "h-full max-w-[90vw] rounded-r-xl",
        }
    }

    pub fn animation_classes(&self, is_open: bool) -> String {
        let base = "transform transition-transform duration-500 ease-out will-change-transform";
        let translate = if is_open {
            "translate-x-0 translate-y-0"
        } else {
            match self {
                SheetSide::Top => "-translate-y-full",
                SheetSide::Right => "translate-x-full", 
                SheetSide::Bottom => "translate-y-full",
                SheetSide::Left => "-translate-x-full",
            }
        };
        format!("{} {}", base, translate)
    }
}

impl SheetSize {
    pub fn size_classes(&self, side: SheetSide) -> &'static str {
        match (self, side) {
            // Horizontal sheets (Top/Bottom)
            (SheetSize::Small, SheetSide::Top | SheetSide::Bottom) => "h-[30vh]",
            (SheetSize::Medium, SheetSide::Top | SheetSide::Bottom) => "h-[50vh]", 
            (SheetSize::Large, SheetSide::Top | SheetSide::Bottom) => "h-[65vh]",
            (SheetSize::Full, SheetSide::Top | SheetSide::Bottom) => "h-[90vh]",
            
            // Vertical sheets (Left/Right)
            (SheetSize::Small, SheetSide::Left | SheetSide::Right) => "w-[30vw]",
            (SheetSize::Medium, SheetSide::Left | SheetSide::Right) => "w-[50vw]",
            (SheetSize::Large, SheetSide::Left | SheetSide::Right) => "w-[65vw]", 
            (SheetSize::Full, SheetSide::Left | SheetSide::Right) => "w-[90vw]",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SheetProps {
    /// Controls sheet visibility
    pub is_open: Signal<bool>,
    /// Side to slide in from
    #[props(default)]
    pub side: SheetSide,
    /// Size of the sheet
    #[props(default)] 
    pub size: SheetSize,
    /// Allow closing by clicking backdrop (default: true)
    #[props(default = true)]
    pub close_on_backdrop_click: bool,
    /// Show close button (default: true)
    #[props(default = true)] 
    pub show_close_button: bool,
    /// Sheet title displayed in header
    #[props(optional)]
    pub title: Option<String>,
    /// Hide header completely (default: false)
    #[props(default = false)]
    pub hide_header: bool,
    /// Sheet content
    pub children: Element,
}

#[component]
pub fn Sheet(props: SheetProps) -> Element {
    let mut animation_state = use_signal(|| false);
    
    // Handle animation state transitions
    use_effect(move || {
        let is_open = (props.is_open)();
        if is_open {
            // Start closed, then animate to open
            animation_state.set(false);
            spawn(async move {
                // Longer delay for mobile to ensure proper rendering
                #[cfg(target_arch = "wasm32")]
                {
                    gloo_timers::future::sleep(std::time::Duration::from_millis(50)).await;
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                }
                animation_state.set(true);
            });
        } else {
            animation_state.set(false);
        }
    });
    
    if !(props.is_open)() {
        return rsx! { div {} };
    }

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

    // Auto-focus the sheet when it opens
    use_effect(move || {
        if (props.is_open)() {
            spawn(async move {
                // Small delay to ensure the sheet is rendered
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
                            if let Ok(Some(sheet)) = document.query_selector("[data-sheet-backdrop]") {
                                if let Ok(element) = sheet.dyn_into::<web_sys::HtmlElement>() {
                                    let _ = element.focus();
                                }
                            }
                        }
                    }
                }
            });
        }
    });

    let container_classes = props.side.container_classes();
    let sheet_base_classes = props.side.sheet_classes();
    let size_classes = props.size.size_classes(props.side);
    let animation_classes = props.side.animation_classes(animation_state());

    rsx! {
        // Backdrop overlay
        div {
            class: format!("fixed inset-0 z-50 flex {}", container_classes),
            style: "background: rgba(0, 0, 0, 0.6); backdrop-filter: blur(4px);",
            onclick: backdrop_click,
            onkeydown: key_down,
            tabindex: 0,
            "data-sheet-backdrop": "",
            
            // Sheet container  
            div {
                class: format!("bg-white dark:bg-gray-900 shadow-2xl flex flex-col overflow-hidden border border-gray-200 dark:border-gray-700 {} {} {}", 
                    sheet_base_classes, size_classes, animation_classes),
                onclick: move |e| e.stop_propagation(),
                
                // Sheet header (conditional)
                if !props.hide_header {
                    div {
                        class: "flex-shrink-0 flex items-center justify-between px-6 py-4 bg-gray-50 dark:bg-gray-800/50 border-b border-gray-200 dark:border-gray-700",
                        
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
                                "aria-label": "Close sheet",
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
                
                // Sheet content
                div {
                    class: "flex-1 overflow-hidden flex flex-col",
                    {props.children}
                }
            }
        }
    }
}