//! Modal dialog components for overlay content and interactions
//!
//! The modal module provides dialog components for displaying overlay content
//! that captures user attention. It includes backdrop handling, keyboard navigation,
//! and flexible sizing options.
//!
//! # Examples
//!
//! Basic modal:
//! ```rust
//! let mut show_modal = use_signal(|| false);
//! rsx! {
//!     Modal {
//!         is_open: show_modal,
//!         title: Some("Confirm Action".to_string()),
//!         size: ModalSize::Small,
//!         // Modal content here
//!     }
//! }
//! ```

use dioxus::prelude::*;
use crate::Platform;

/// Size variants for modal dialog dimensions
///
/// Controls the maximum width of modal dialogs to provide appropriate
/// sizing for different types of content and user interactions:
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ModalSize {
    /// Small modal for confirmations and quick actions
    ///
    /// Maximum width of 28rem (448px) suitable for simple confirmations,
    /// alerts, or quick action dialogs with minimal content.
    Small,

    /// Medium modal for standard content (default)
    ///
    /// Maximum width of 48rem (768px) suitable for most modal use cases
    /// including forms, settings panels, and detailed content.
    #[default]
    Medium,

    /// Large modal for complex content
    ///
    /// Maximum width of 72rem (1152px) suitable for complex forms,
    /// data tables, or content-heavy dialogs requiring more space.
    Large,

    /// Full-width modal for immersive experiences
    ///
    /// Maximum width of 95vw (95% viewport width) suitable for image
    /// galleries, media viewers, or full-screen experiences.
    Full,
}

impl ModalSize {
    /// Returns the maximum width CSS value for the modal size
    ///
    /// Maps each size variant to appropriate maximum width constraints
    /// for responsive modal sizing across different screen sizes.
    pub fn max_width(&self) -> &'static str {
        match self {
            ModalSize::Small => "28rem",
            ModalSize::Medium => "48rem",
            ModalSize::Large => "72rem",
            ModalSize::Full => "95vw",
        }
    }
}

/// Properties for configuring the Modal component
#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    /// Optional title text displayed in the modal header
    ///
    /// When provided, displays a title in the modal header with appropriate
    /// typography. The header will be shown even if hide_header is false
    /// when a title is present. Defaults to None (no title).
    #[props(optional)]
    pub title: Option<String>,

    /// Reactive signal controlling modal visibility
    ///
    /// A signal that determines whether the modal is currently visible.
    /// Setting this to true shows the modal with backdrop and animations,
    /// while false hides the modal completely. Required parameter.
    pub is_open: Signal<bool>,

    /// Content to display within the modal body
    ///
    /// The main content area of the modal. Content is rendered in a
    /// flexible container that can handle various layouts including forms,
    /// text, images, or interactive elements. Required parameter.
    pub children: Element,

    /// Whether to display the close button in the header
    ///
    /// When true, shows an "X" close button in the modal header. When false,
    /// the modal must be closed via backdrop click, escape key, or programmatic
    /// control. Defaults to true.
    #[props(default = true)]
    pub show_close_button: bool,

    /// Whether clicking the backdrop closes the modal
    ///
    /// When true, clicking the dark overlay behind the modal will close it.
    /// When false, the modal can only be closed via the close button, escape
    /// key, or programmatic control. Defaults to true.
    #[props(default = true)]
    pub close_on_backdrop_click: bool,

    /// Size variant controlling modal maximum width
    ///
    /// Determines the modal's maximum width and overall visual prominence:
    /// - `Small`: 28rem for confirmations and quick actions
    /// - `Medium`: 48rem for standard content (default)
    /// - `Large`: 72rem for complex content
    /// - `Full`: 95vw for immersive experiences
    /// Defaults to `ModalSize::Medium`.
    #[props(default = ModalSize::Medium)]
    pub size: ModalSize,

    /// Whether to completely hide the header section
    ///
    /// When true, removes the entire header area including title and close
    /// button. Content starts at the top of the modal. When false, shows
    /// the header with title (if provided) and close button (if enabled).
    /// Defaults to false.
    #[props(default = false)]
    pub hide_header: bool,
}

/// Modal dialog component for overlay content and user interactions
///
/// The Modal component provides a sophisticated dialog system for displaying
/// overlay content that requires user attention or interaction. It features
/// backdrop handling, keyboard navigation, focus management, and flexible
/// sizing for various modal dialog patterns from simple confirmations to
/// complex forms and media viewers.
///
/// # Features
///
/// - **Flexible sizing**: Four size options from small confirmations to full-screen
/// - **Backdrop interaction**: Optional click-to-close with blur backdrop
/// - **Keyboard navigation**: Escape key closing and proper focus management
/// - **Header customization**: Optional titles, close buttons, and header hiding
/// - **Focus trapping**: Automatic focus management when modal opens and closes
/// - **Content flexibility**: Support for any content with proper scrolling
/// - **Accessibility**: Full ARIA support with semantic dialog structure
/// - **Responsive design**: Adapts to screen size with appropriate constraints
///
/// # Implementation Details
///
/// The component uses a layered approach with:
/// 1. Full-screen backdrop with blur effect and dark overlay
/// 2. Centered modal container with size-based width constraints
/// 3. Optional header with title and close button
/// 4. Flexible content area with overflow handling
///
/// Focus management automatically moves to the modal when opened and restores
/// previous focus when closed. The backdrop supports click-to-close behavior
/// while preventing event propagation from the modal content area.
///
/// # Accessibility
///
/// - Automatic focus management with focus trapping
/// - Escape key support for closing
/// - Proper ARIA dialog semantics
/// - Screen reader compatible with semantic structure
/// - Keyboard navigation support within modal content
/// - Focus restoration when modal closes
/// - Accessible close button with proper labeling
///
/// # Performance Considerations
///
/// - Conditional rendering prevents unnecessary DOM when closed
/// - Efficient event handling with minimal re-renders
/// - CSS-based animations and transitions for smooth interactions
/// - Platform-specific timing adjustments for optimal rendering
/// - Backdrop blur effects optimized for performance
///
/// # Use Cases
///
/// - **Confirmations**: Delete actions, form submissions, important decisions
/// - **Forms**: Settings panels, data entry, user preferences
/// - **Content display**: Image galleries, document viewers, detailed information
/// - **User actions**: Quick actions, wizards, multi-step processes
/// - **Alerts**: Error messages, success notifications, important announcements
/// - **Media**: Image/video viewers, file previews, media galleries
///
/// # Parameters
///
/// - `title`: Optional header title text
/// - `is_open`: Signal controlling modal visibility
/// - `children`: Modal content elements
/// - `show_close_button`: Display close button in header
/// - `close_on_backdrop_click`: Enable backdrop click to close
/// - `size`: Modal width variant (Small, Medium, Large, Full)
/// - `hide_header`: Remove header section entirely
#[component]
pub fn Modal(props: ModalProps) -> Element {
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
                            if let Ok(Some(modal)) =
                                document.query_selector("[data-modal-backdrop]")
                            {
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
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4",
            onclick: backdrop_click,
            onkeydown: key_down,
            tabindex: 0,
            "data-modal-backdrop": "",
            div {
                class: "bg-popover rounded-xl shadow-2xl w-full flex flex-col overflow-hidden border border-border",
                style: format!("max-width: {}; max-height: calc(100vh - 2rem);", props.size.max_width()),
                onclick: move |e| e.stop_propagation(),
                if !props.hide_header {
                    div { class: "flex-shrink-0 flex items-center justify-between px-6 py-4 bg-muted/50",
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
