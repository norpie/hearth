//! Toast notification components for user feedback
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     div {
//!         provide_context(|| ToastManager::new()),
//!         
//!         button {
//!             onclick: move |_| {
//!                 let toaster = use_toaster();
//!                 toaster.success("Operation completed!");
//!             },
//!             "Show Toast"
//!         }
//!         
//!         Toaster { position: ToastPosition::TopRight }
//!     }
//! }
//! ```

use dioxus::prelude::*;
use crate::Platform;
use std::time::Duration;

/// Unique identifier for each toast
pub type ToastId = uuid::Uuid;

/// Semantic types for toast notifications
#[derive(Clone, PartialEq, Debug)]
pub enum ToastType {
    /// Success state for positive feedback
    Success,
    /// Error state for failures and issues
    Error,
    /// Warning state for caution and notices
    Warning,
    /// Informational state for neutral messages
    Info,
}

/// Screen positioning options for toast notifications
#[derive(Clone, PartialEq, Debug)]
pub enum ToastPosition {
    /// Top-right corner positioning
    TopRight,
    /// Top-left corner positioning
    TopLeft,
    /// Bottom-right corner positioning
    BottomRight,
    /// Bottom-left corner positioning
    BottomLeft,
    /// Top-center positioning
    TopCenter,
    /// Bottom-center positioning
    BottomCenter,
}

/// Individual toast notification data
#[derive(Clone, PartialEq, Debug)]
pub struct Toast {
    /// Unique identifier for this toast
    pub id: ToastId,
    /// Text content displayed in the toast
    pub message: String,
    /// Semantic type determining visual styling
    pub toast_type: ToastType,
    /// Optional auto-dismiss duration
    pub duration: Option<Duration>,
    /// Whether the user can manually dismiss the toast
    pub dismissible: bool,
}

/// Configuration for creating new toast notifications
#[derive(Clone, PartialEq)]
pub struct ToastConfig {
    /// Text content to display in the notification
    pub message: String,
    /// Semantic type for styling and meaning
    pub toast_type: ToastType,
    /// Optional automatic dismissal duration
    pub duration: Option<Duration>,
    /// Whether users can manually close the toast
    pub dismissible: bool,
}

impl Default for ToastConfig {
    fn default() -> Self {
        Self {
            message: String::new(),
            toast_type: ToastType::Info,
            duration: Some(Duration::from_secs(5)),
            dismissible: true,
        }
    }
}

impl ToastType {
    /// Returns the Font Awesome icon class for this toast type
    pub fn icon(&self) -> &'static str {
        match self {
            ToastType::Success => "fa-solid fa-check-circle",
            ToastType::Error => "fa-solid fa-times-circle",
            ToastType::Warning => "fa-solid fa-exclamation-triangle",
            ToastType::Info => "fa-solid fa-info-circle",
        }
    }

    /// Returns the CSS classes for toast background and border styling
    pub fn classes(&self) -> &'static str {
        match self {
            ToastType::Success => "bg-success/10 border-success text-success-foreground",
            ToastType::Error => "bg-destructive/10 border-destructive text-destructive-foreground",
            ToastType::Warning => "bg-warning/10 border-warning text-warning-foreground",
            ToastType::Info => "bg-info/10 border-info text-info-foreground",
        }
    }

    /// Returns the CSS classes for icon color theming
    pub fn icon_color(&self) -> &'static str {
        match self {
            ToastType::Success => "text-success-foreground",
            ToastType::Error => "text-destructive-foreground",
            ToastType::Warning => "text-warning-foreground",
            ToastType::Info => "text-info-foreground",
        }
    }
}

impl ToastPosition {
    /// Returns CSS classes for positioning the toast container
    pub fn container_classes(&self) -> &'static str {
        match self {
            ToastPosition::TopRight => "fixed top-4 sm:top-4 right-2 sm:right-4 z-50 flex flex-col space-y-2 w-full max-w-sm sm:max-w-lg px-2 sm:px-0",
            ToastPosition::TopLeft => "fixed top-4 sm:top-4 left-2 sm:left-4 z-50 flex flex-col space-y-2 w-full max-w-sm sm:max-w-lg px-2 sm:px-0",
            ToastPosition::BottomRight => "fixed bottom-4 sm:bottom-4 right-2 sm:right-4 z-50 flex flex-col-reverse space-y-reverse space-y-2 w-full max-w-sm sm:max-w-lg px-2 sm:px-0",
            ToastPosition::BottomLeft => "fixed bottom-4 sm:bottom-4 left-2 sm:left-4 z-50 flex flex-col-reverse space-y-reverse space-y-2 w-full max-w-sm sm:max-w-lg px-2 sm:px-0",
            ToastPosition::TopCenter => "fixed top-4 sm:top-4 left-1/2 transform -translate-x-1/2 z-50 flex flex-col space-y-2 w-full max-w-sm sm:max-w-lg px-2 sm:px-0",
            ToastPosition::BottomCenter => "fixed bottom-4 sm:bottom-4 left-1/2 transform -translate-x-1/2 z-50 flex flex-col-reverse space-y-reverse space-y-2 w-full max-w-sm sm:max-w-lg px-2 sm:px-0",
        }
    }
}

/// Global toast notification manager
#[derive(Clone, Copy)]
pub struct ToastManager {
    /// Active toast notifications queue
    toasts: Signal<Vec<Toast>>,
    /// Toasts currently in exit animation state
    exiting_toasts: Signal<Vec<ToastId>>,
}

impl Default for ToastManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ToastManager {
    /// Creates a new toast manager instance
    pub fn new() -> Self {
        Self {
            toasts: Signal::new(Vec::new()),
            exiting_toasts: Signal::new(Vec::new()),
        }
    }

    /// Adds a new toast notification with automatic lifecycle management
    pub fn add_toast(mut self, config: ToastConfig) -> ToastId {
        let toast = Toast {
            id: uuid::Uuid::new_v4(),
            message: config.message,
            toast_type: config.toast_type,
            duration: config.duration,
            dismissible: config.dismissible,
        };

        let toast_id = toast.id;
        self.toasts.write().push(toast);

        if let Some(duration) = config.duration {
            let mut toasts = self.toasts;
            let mut exiting_toasts = self.exiting_toasts;

            Platform::spawn(async move {
                Platform::sleep(duration).await;

                if !toasts.read().iter().any(|t| t.id == toast_id) {
                    return; // Toast was already removed
                }

                exiting_toasts.write().push(toast_id);

                Platform::sleep(std::time::Duration::from_millis(250)).await; // Match exit animation duration

                if toasts.read().iter().any(|t| t.id == toast_id) {
                    toasts.write().retain(|t| t.id != toast_id);
                }
                exiting_toasts.write().retain(|id| *id != toast_id);
            });
        }

        toast_id
    }

    /// Removes a toast notification immediately by ID
    pub fn remove_toast(mut self, id: ToastId) {
        self.toasts.write().retain(|toast| toast.id != id);
        self.exiting_toasts.write().retain(|exit_id| *exit_id != id);
    }

    /// Returns the signal containing the current toast queue
    pub fn toasts(self) -> Signal<Vec<Toast>> {
        self.toasts
    }

    /// Checks if a toast is currently in exit animation state
    pub fn is_toast_exiting(self, id: ToastId) -> bool {
        self.exiting_toasts.read().contains(&id)
    }

    /// Removes all toast notifications immediately
    pub fn clear_all(mut self) {
        self.toasts.write().clear();
        self.exiting_toasts.write().clear();
    }

    /// Creates a success toast with default settings
    pub fn success(self, message: impl Into<String>) -> ToastId {
        self.add_toast(ToastConfig {
            message: message.into(),
            toast_type: ToastType::Success,
            duration: Some(Duration::from_secs(5)),
            dismissible: true,
        })
    }

    /// Creates an error toast with default settings
    pub fn error(self, message: impl Into<String>) -> ToastId {
        self.add_toast(ToastConfig {
            message: message.into(),
            toast_type: ToastType::Error,
            duration: Some(Duration::from_secs(5)),
            dismissible: true,
        })
    }

    /// Creates a warning toast with default settings
    pub fn warning(self, message: impl Into<String>) -> ToastId {
        self.add_toast(ToastConfig {
            message: message.into(),
            toast_type: ToastType::Warning,
            duration: Some(Duration::from_secs(5)),
            dismissible: true,
        })
    }

    /// Creates an info toast with default settings
    pub fn info(self, message: impl Into<String>) -> ToastId {
        self.add_toast(ToastConfig {
            message: message.into(),
            toast_type: ToastType::Info,
            duration: Some(Duration::from_secs(5)),
            dismissible: true,
        })
    }
}

/// Properties for configuring individual ToastItem components
#[derive(Props, Clone, PartialEq)]
pub struct ToastItemProps {
    /// The toast data to render
    pub toast: Toast,
    /// Event handler called when the toast should be dismissed
    pub on_dismiss: EventHandler<ToastId>,
}

/// Individual toast notification component with animations and interactions
#[component]
pub fn ToastItem(props: ToastItemProps) -> Element {
    let toast_manager = use_context::<ToastManager>();
    let is_exiting = toast_manager.is_toast_exiting(props.toast.id);

    let base_classes = "flex items-start gap-3 p-3 sm:p-4 rounded-lg border shadow-lg backdrop-blur-sm transition-all duration-200 ease-in-out min-w-0";
    let type_classes = props.toast.toast_type.classes();
    let animation_class = if is_exiting {
        "toast-exit"
    } else {
        "toast-enter"
    };
    let combined_classes = format!("{base_classes} {type_classes} {animation_class}");

    rsx! {
        div { role: "alert", aria_live: "polite", class: "{combined_classes}",
            div { class: "flex-shrink-0 pt-0.5",
                i { class: "w-5 h-5 {props.toast.toast_type.icon()} {props.toast.toast_type.icon_color()}" }
            }
            div { 
                class: "flex-1 text-sm sm:text-sm font-medium break-words whitespace-pre-wrap leading-relaxed", 
                "{props.toast.message}" 
            }
            if props.toast.dismissible {
                button {
                    class: "flex-shrink-0 ml-2 text-muted-foreground hover:text-foreground transition-colors duration-200 mt-0.5",
                    onclick: {
                        let on_dismiss = props.on_dismiss;
                        let toast_id = props.toast.id;
                        let mut exiting_toasts = toast_manager.exiting_toasts;
                        move |_| {
                                    exiting_toasts.write().push(toast_id);

                            on_dismiss.call(toast_id);
                        }
                    },
                    aria_label: "Dismiss notification",
                    i { class: "w-4 h-4 fa-solid fa-times" }
                }
            }
        }
    }
}

/// Properties for configuring the main Toaster container component
#[derive(Props, Clone, PartialEq)]
pub struct ToasterProps {
    /// Screen position for toast notifications
    #[props(default = default_position())]
    pub position: ToastPosition,
    /// Maximum number of toasts to display simultaneously
    #[props(default = 5)]
    pub max_toasts: usize,
}

/// Returns the default toast position based on platform features
fn default_position() -> ToastPosition {
    #[cfg(feature = "mobile")]
    return ToastPosition::TopCenter;

    #[cfg(not(feature = "mobile"))]
    ToastPosition::TopRight
}

/// Main toast notification container component with queue management
#[component]
pub fn Toaster(props: ToasterProps) -> Element {
    let toast_manager = use_context::<ToastManager>();
    let toasts = toast_manager.toasts();

    let visible_toasts: Vec<Toast> = toasts
        .read()
        .iter()
        .rev() // Show newest first
        .take(props.max_toasts)
        .cloned()
        .collect();

    rsx! {
        div { class: "{props.position.container_classes()}",
            for toast in visible_toasts {
                ToastItem {
                    key: "{toast.id}",
                    toast: toast.clone(),
                    on_dismiss: move |id| {
                        toast_manager.remove_toast(id);
                    },
                }
            }
        }
    }
}

/// Hook for accessing the global toast manager from components
pub fn use_toaster() -> ToastManager {
    use_context::<ToastManager>()
}
