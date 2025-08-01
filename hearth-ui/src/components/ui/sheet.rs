//! Sheet components for sliding overlay panels and drawers
//!
//! The sheet module provides slide-in overlay components that can appear from any
//! side of the screen. These are commonly used for navigation drawers, settings panels,
//! and modal content that slides in from screen edges.
//!
//! # Examples
//!
//! Basic navigation sheet:
//! ```rust
//! let mut show_menu = use_signal(|| false);
//! rsx! {
//!     Sheet {
//!         is_open: show_menu,
//!         side: SheetSide::Right,
//!         size: SheetSize::Medium,
//!         title: Some("Navigation".to_string()),
//!         // Content here
//!     }
//! }
//! ```
//!
//! Bottom sheet for mobile:
//! ```rust
//! rsx! {
//!     Sheet {
//!         is_open: show_actions,
//!         side: SheetSide::Bottom,
//!         size: SheetSize::Small,
//!         hide_header: true,
//!         // Action buttons here
//!     }
//! }
//! ```

use dioxus::prelude::*;
use crate::Platform;

/// Side options controlling sheet slide direction and positioning
///
/// Determines which edge of the screen the sheet slides in from:
/// - `Top`: Slides down from the top edge
/// - `Right`: Slides left from the right edge (default)
/// - `Bottom`: Slides up from the bottom edge
/// - `Left`: Slides right from the left edge
#[derive(Clone, Copy, PartialEq, Default)]
pub enum SheetSide {
    /// Sheet slides down from the top edge
    Top,
    /// Sheet slides left from the right edge (default)
    #[default]
    Right,
    /// Sheet slides up from the bottom edge
    Bottom,
    /// Sheet slides right from the left edge
    Left,
}

/// Size variants controlling sheet dimensions
///
/// Provides preset size options for different use cases:
/// - `Small`: 30% of viewport (30vh/30vw) for quick actions
/// - `Medium`: 50% of viewport (50vh/50vw) for standard content (default)
/// - `Large`: 65% of viewport (65vh/65vw) for detailed content
/// - `Full`: 90% of viewport (90vh/90vw) for maximum content area
#[derive(Clone, Copy, PartialEq, Default)]
pub enum SheetSize {
    /// Small sheet size (30% of viewport) for quick actions
    Small,
    /// Medium sheet size (50% of viewport) for standard content (default)
    #[default]
    Medium,
    /// Large sheet size (65% of viewport) for detailed content
    Large,
    /// Full sheet size (90% of viewport) for maximum content area
    Full,
}

impl SheetSide {
    /// Returns CSS flexbox alignment classes for sheet container positioning
    ///
    /// Maps each side to appropriate justify and align classes for positioning
    /// the sheet at the correct edge of the viewport.
    pub fn container_classes(&self) -> &'static str {
        match self {
            SheetSide::Top => "justify-center items-start",
            SheetSide::Right => "justify-end items-center",
            SheetSide::Bottom => "justify-center items-end",
            SheetSide::Left => "justify-start items-center",
        }
    }

    /// Returns base CSS classes for sheet dimensions and border radius
    ///
    /// Provides appropriate width/height constraints and rounded corners
    /// based on the sheet's slide direction and positioning.
    pub fn sheet_classes(&self) -> &'static str {
        match self {
            SheetSide::Top => "w-full max-h-[90vh] rounded-b-xl",
            SheetSide::Right => "h-full max-w-[90vw] rounded-l-xl",
            SheetSide::Bottom => "w-full max-h-[90vh] rounded-t-xl",
            SheetSide::Left => "h-full max-w-[90vw] rounded-r-xl",
        }
    }

    /// Returns CSS animation classes for sheet slide transitions
    ///
    /// Generates transform and transition classes for smooth slide animations
    /// based on the sheet side and current open/closed state.
    ///
    /// # Parameters
    ///
    /// - `is_open`: Whether the sheet is currently in the open state
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
        format!("{base} {translate}")
    }
}

impl SheetSize {
    /// Returns CSS size classes based on sheet size and slide direction
    ///
    /// Maps size variants to appropriate viewport-relative dimensions.
    /// Horizontal sheets (top/bottom) use height constraints, while
    /// vertical sheets (left/right) use width constraints.
    ///
    /// # Parameters
    ///
    /// - `side`: The sheet side to determine which dimension to constrain
    pub fn size_classes(&self, side: SheetSide) -> &'static str {
        match (self, side) {
            (SheetSize::Small, SheetSide::Top | SheetSide::Bottom) => "h-[30vh]",
            (SheetSize::Medium, SheetSide::Top | SheetSide::Bottom) => "h-[50vh]",
            (SheetSize::Large, SheetSide::Top | SheetSide::Bottom) => "h-[65vh]",
            (SheetSize::Full, SheetSide::Top | SheetSide::Bottom) => "h-[90vh]",

            (SheetSize::Small, SheetSide::Left | SheetSide::Right) => "w-[30vw]",
            (SheetSize::Medium, SheetSide::Left | SheetSide::Right) => "w-[50vw]",
            (SheetSize::Large, SheetSide::Left | SheetSide::Right) => "w-[65vw]",
            (SheetSize::Full, SheetSide::Left | SheetSide::Right) => "w-[90vw]",
        }
    }
}

/// Properties for configuring the Sheet component
#[derive(Props, Clone, PartialEq)]
pub struct SheetProps {
    /// Signal controlling sheet visibility and open/closed state
    ///
    /// A reactive signal that determines whether the sheet is currently visible.
    /// Setting this to true opens the sheet with slide-in animation, while
    /// false closes it with slide-out animation. Required parameter.
    pub is_open: Signal<bool>,

    /// Side of the screen the sheet slides in from
    ///
    /// Determines the slide direction and sheet positioning. Affects animation
    /// direction, corner rounding, and container alignment.
    /// Defaults to `SheetSide::Right`.
    #[props(default)]
    pub side: SheetSide,

    /// Size variant controlling sheet dimensions
    ///
    /// Determines the viewport percentage the sheet occupies. Size is applied
    /// to the appropriate dimension based on the slide direction (width for
    /// left/right, height for top/bottom). Defaults to `SheetSize::Medium`.
    #[props(default)]
    pub size: SheetSize,

    /// Whether clicking the backdrop closes the sheet
    ///
    /// When true, clicking the dark overlay behind the sheet will close it.
    /// When false, the sheet can only be closed via the close button, escape
    /// key, or programmatic control. Defaults to true.
    #[props(default = true)]
    pub close_on_backdrop_click: bool,

    /// Whether to display the close button in the header
    ///
    /// When true, shows an "X" close button in the sheet header. When false,
    /// the sheet must be closed via backdrop click, escape key, or programmatic
    /// control. Defaults to true.
    #[props(default = true)]
    pub show_close_button: bool,

    /// Optional title text displayed in the sheet header
    ///
    /// When provided, displays a title in the sheet header area. The header
    /// will be shown even if hide_header is false when a title is present.
    /// Defaults to None (no title).
    #[props(optional)]
    pub title: Option<String>,

    /// Whether to completely hide the header section
    ///
    /// When true, removes the entire header area including title and close
    /// button. Content starts at the top of the sheet. When false, shows
    /// the header with title (if provided) and close button (if enabled).
    /// Defaults to false.
    #[props(default = false)]
    pub hide_header: bool,

    /// Content to display within the sheet body
    ///
    /// The main content area of the sheet. Content is rendered in a flex
    /// container that takes up the remaining space after the header.
    pub children: Element,
}

/// Sliding overlay sheet component for side panels and drawers
///
/// The Sheet component provides slide-in overlays that can appear from any edge
/// of the screen. It features smooth animations, flexible sizing, and backdrop
/// interaction for creating navigation drawers and modal-like experiences.
#[component]
pub fn Sheet(props: SheetProps) -> Element {
    let mut animation_state = use_signal(|| false);

    use_effect(move || {
        let is_open = (props.is_open)();
        if is_open {
            animation_state.set(false);
            Platform::spawn(async move {
                Platform::sleep(std::time::Duration::from_millis(50)).await;
                animation_state.set(true);
            });
        } else {
            animation_state.set(false);
        }
    });

    if !(props.is_open)() {
        return rsx! {
            div {}
        };
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

    use_effect(move || {
        if (props.is_open)() {
            Platform::spawn(async move {
                Platform::sleep(std::time::Duration::from_millis(10)).await;

                #[cfg(target_arch = "wasm32")]
                {
                    use web_sys::wasm_bindgen::JsCast;
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Ok(Some(sheet)) =
                                document.query_selector("[data-sheet-backdrop]")
                            {
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
        div {
            class: format!(
                "fixed inset-0 z-50 flex bg-black/60 backdrop-blur-sm {}",
                container_classes,
            ),
            onclick: backdrop_click,
            onkeydown: key_down,
            tabindex: 0,
            "data-sheet-backdrop": "",
            div {
                class: format!(
                    "bg-popover shadow-2xl flex flex-col overflow-hidden border border-border {} {} {}",
                    sheet_base_classes,
                    size_classes,
                    animation_classes,
                ),
                onclick: move |e| e.stop_propagation(),
                if !props.hide_header {
                    div { class: "flex-shrink-0 flex items-center justify-between px-6 py-4 bg-muted/50 border-b border-border",
                        if let Some(title) = &props.title {
                            h2 { class: "text-lg font-semibold text-popover-foreground border-b-0",
                                "{title}"
                            }
                        } else {
                            div {}
                        }
                        if props.show_close_button {
                            button {
                                class: "p-2 text-muted-foreground hover:text-foreground hover:bg-accent rounded-lg transition-all duration-150 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-popover",
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
                                        d: "M6 18L18 6M6 6l12 12",
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "flex-1 overflow-hidden flex flex-col", {props.children} }
            }
        }
    }
}
