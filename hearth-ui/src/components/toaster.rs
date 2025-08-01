//! Toast notification system with queue management and cross-platform support

use dioxus::prelude::*;
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;



/// Unique identifier for each toast
pub type ToastId = uuid::Uuid;

/// Types of toast notifications
#[derive(Clone, PartialEq, Debug)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

/// Position where toasts appear on screen
#[derive(Clone, PartialEq, Debug)]
pub enum ToastPosition {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    TopCenter,
    BottomCenter,
}

/// Individual toast configuration
#[derive(Clone, PartialEq, Debug)]
pub struct Toast {
    pub id: ToastId,
    pub message: String,
    pub toast_type: ToastType,
    pub duration: Option<Duration>,
    pub dismissible: bool,
}

/// Configuration for creating a new toast
#[derive(Clone, PartialEq)]
pub struct ToastConfig {
    pub message: String,
    pub toast_type: ToastType,
    pub duration: Option<Duration>,
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
    pub fn icon(&self) -> &'static str {
        match self {
            ToastType::Success => "fa-solid fa-check-circle",
            ToastType::Error => "fa-solid fa-times-circle",
            ToastType::Warning => "fa-solid fa-exclamation-triangle",
            ToastType::Info => "fa-solid fa-info-circle",
        }
    }

    pub fn classes(&self) -> &'static str {
        match self {
            ToastType::Success => "bg-green-50 border-green-200 text-green-800 dark:bg-green-900/20 dark:border-green-800 dark:text-green-400",
            ToastType::Error => "bg-red-50 border-red-200 text-red-800 dark:bg-red-900/20 dark:border-red-800 dark:text-red-400",
            ToastType::Warning => "bg-yellow-50 border-yellow-200 text-yellow-800 dark:bg-yellow-900/20 dark:border-yellow-800 dark:text-yellow-400",
            ToastType::Info => "bg-blue-50 border-blue-200 text-blue-800 dark:bg-blue-900/20 dark:border-blue-800 dark:text-blue-400",
        }
    }

    pub fn icon_color(&self) -> &'static str {
        match self {
            ToastType::Success => "text-green-500",
            ToastType::Error => "text-red-500",
            ToastType::Warning => "text-yellow-500",
            ToastType::Info => "text-blue-500",
        }
    }
}

impl ToastPosition {
    pub fn container_classes(&self) -> &'static str {
        match self {
            ToastPosition::TopRight => "fixed top-4 sm:top-4 right-2 sm:right-4 z-50 flex flex-col space-y-2 w-full max-w-sm sm:max-w-md px-2 sm:px-0",
            ToastPosition::TopLeft => "fixed top-4 sm:top-4 left-2 sm:left-4 z-50 flex flex-col space-y-2 w-full max-w-sm sm:max-w-md px-2 sm:px-0",
            ToastPosition::BottomRight => "fixed bottom-4 sm:bottom-4 right-2 sm:right-4 z-50 flex flex-col-reverse space-y-reverse space-y-2 w-full max-w-sm sm:max-w-md px-2 sm:px-0",
            ToastPosition::BottomLeft => "fixed bottom-4 sm:bottom-4 left-2 sm:left-4 z-50 flex flex-col-reverse space-y-reverse space-y-2 w-full max-w-sm sm:max-w-md px-2 sm:px-0",
            ToastPosition::TopCenter => "fixed top-4 sm:top-4 left-1/2 transform -translate-x-1/2 z-50 flex flex-col space-y-2 w-full max-w-sm sm:max-w-md px-2 sm:px-0",
            ToastPosition::BottomCenter => "fixed bottom-4 sm:bottom-4 left-1/2 transform -translate-x-1/2 z-50 flex flex-col-reverse space-y-reverse space-y-2 w-full max-w-sm sm:max-w-md px-2 sm:px-0",
        }
    }
}

/// Global toast manager for managing the toast queue
#[derive(Clone, Copy)]
pub struct ToastManager {
    toasts: Signal<Vec<Toast>>,
    exiting_toasts: Signal<Vec<ToastId>>,
}

impl ToastManager {
    pub fn new() -> Self {
        Self {
            toasts: Signal::new(Vec::new()),
            exiting_toasts: Signal::new(Vec::new()),
        }
    }

    /// Add a new toast to the queue
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

        // Auto-dismiss if duration is set
        if let Some(duration) = config.duration {
            let mut toasts = self.toasts;
            let mut exiting_toasts = self.exiting_toasts;
            
            #[cfg(target_arch = "wasm32")]
            {
                spawn_local(async move {
                    gloo_timers::future::TimeoutFuture::new(duration.as_millis() as u32).await;
                    
                    // Check if toast still exists before starting exit animation
                    if !toasts.read().iter().any(|t| t.id == toast_id) {
                        return; // Toast was already removed
                    }
                    
                    // Start exit animation
                    exiting_toasts.write().push(toast_id);
                    
                    // Remove toast after exit animation completes
                    gloo_timers::future::TimeoutFuture::new(250).await; // Match exit animation duration
                    
                    // Double-check before removing
                    if toasts.read().iter().any(|t| t.id == toast_id) {
                        toasts.write().retain(|t| t.id != toast_id);
                    }
                    exiting_toasts.write().retain(|id| *id != toast_id);
                });
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            {
                spawn(async move {
                    tokio::time::sleep(duration).await;
                    
                    // Check if toast still exists before starting exit animation
                    if !toasts.read().iter().any(|t| t.id == toast_id) {
                        return; // Toast was already removed
                    }
                    
                    // Start exit animation
                    exiting_toasts.write().push(toast_id);
                    
                    // Remove toast after exit animation completes
                    tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                    
                    // Double-check before removing
                    if toasts.read().iter().any(|t| t.id == toast_id) {
                        toasts.write().retain(|t| t.id != toast_id);
                    }
                    exiting_toasts.write().retain(|id| *id != toast_id);
                });
            }
        }

        toast_id
    }

    /// Remove a toast by ID
    pub fn remove_toast(mut self, id: ToastId) {
        self.toasts.write().retain(|toast| toast.id != id);
        self.exiting_toasts.write().retain(|exit_id| *exit_id != id);
    }

    /// Get current toasts
    pub fn toasts(self) -> Signal<Vec<Toast>> {
        self.toasts
    }

    /// Check if a toast is currently exiting
    pub fn is_toast_exiting(self, id: ToastId) -> bool {
        self.exiting_toasts.read().contains(&id)
    }

    /// Clear all toasts
    pub fn clear_all(mut self) {
        self.toasts.write().clear();
        self.exiting_toasts.write().clear();
    }

    /// Convenience methods for different toast types
    pub fn success(self, message: impl Into<String>) -> ToastId {
        self.add_toast(ToastConfig {
            message: message.into(),
            toast_type: ToastType::Success,
            duration: Some(Duration::from_secs(5)),
            dismissible: true,
        })
    }

    pub fn error(self, message: impl Into<String>) -> ToastId {
        self.add_toast(ToastConfig {
            message: message.into(),
            toast_type: ToastType::Error,
            duration: Some(Duration::from_secs(5)),
            dismissible: true,
        })
    }

    pub fn warning(self, message: impl Into<String>) -> ToastId {
        self.add_toast(ToastConfig {
            message: message.into(),
            toast_type: ToastType::Warning,
            duration: Some(Duration::from_secs(5)),
            dismissible: true,
        })
    }

    pub fn info(self, message: impl Into<String>) -> ToastId {
        self.add_toast(ToastConfig {
            message: message.into(),
            toast_type: ToastType::Info,
            duration: Some(Duration::from_secs(5)),
            dismissible: true,
        })
    }
}

/// Props for the ToastItem component
#[derive(Props, Clone, PartialEq)]
pub struct ToastItemProps {
    pub toast: Toast,
    pub on_dismiss: EventHandler<ToastId>,
}

/// Individual toast item component
#[component]
pub fn ToastItem(props: ToastItemProps) -> Element {
    let toast_manager = use_context::<ToastManager>();
    let is_exiting = toast_manager.is_toast_exiting(props.toast.id);
    
    let base_classes = "flex items-center gap-3 p-3 sm:p-4 rounded-lg border shadow-lg backdrop-blur-sm transition-all duration-200 ease-in-out min-w-0";
    let type_classes = props.toast.toast_type.classes();
    let animation_class = if is_exiting { "toast-exit" } else { "toast-enter" };
    let combined_classes = format!("{} {} {}", base_classes, type_classes, animation_class);

    rsx! {
        div {
            role: "alert",
            aria_live: "polite",
            class: "{combined_classes}",
            
            // Icon
            div {
                class: "flex-shrink-0",
                i {
                    class: "w-5 h-5 {props.toast.toast_type.icon()} {props.toast.toast_type.icon_color()}",
                }
            }
            
            // Message
            div {
                class: "flex-1 text-sm sm:text-sm font-medium break-words",
                "{props.toast.message}"
            }
            
            // Dismiss button
            if props.toast.dismissible {
                button {
                    class: "flex-shrink-0 ml-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors duration-200",
                    onclick: {
                        let on_dismiss = props.on_dismiss.clone();
                        let toast_id = props.toast.id;
                        let mut exiting_toasts = toast_manager.exiting_toasts;
                        move |_| {
                            // Start exit animation
                            exiting_toasts.write().push(toast_id);
                            
                            // Immediately dismiss - CSS animation will handle visual transition
                            on_dismiss.call(toast_id);
                        }
                    },
                    aria_label: "Dismiss notification",
                    i {
                        class: "w-4 h-4 fa-solid fa-times",
                    }
                }
            }
        }
    }
}

/// Props for the Toaster component
#[derive(Props, Clone, PartialEq)]
pub struct ToasterProps {
    #[props(default = default_position())]
    pub position: ToastPosition,
    #[props(default = 5)]
    pub max_toasts: usize,
}

fn default_position() -> ToastPosition {
    #[cfg(feature = "mobile")]
    return ToastPosition::TopCenter;
    
    #[cfg(not(feature = "mobile"))]
    ToastPosition::TopRight
}

/// Main toaster component that renders all toasts
#[component]
pub fn Toaster(props: ToasterProps) -> Element {
    let toast_manager = use_context::<ToastManager>();
    let toasts = toast_manager.toasts();

    // Limit the number of visible toasts
    let visible_toasts: Vec<Toast> = toasts
        .read()
        .iter()
        .rev() // Show newest first
        .take(props.max_toasts)
        .cloned()
        .collect();

    rsx! {
        div {
            class: "{props.position.container_classes()}",
            for toast in visible_toasts {
                ToastItem {
                    key: "{toast.id}",
                    toast: toast.clone(),
                    on_dismiss: {
                        let toast_manager = toast_manager.clone();
                        move |id| {
                            toast_manager.remove_toast(id);
                        }
                    },
                }
            }
        }
    }
}

/// Hook to get access to the toast manager
pub fn use_toaster() -> ToastManager {
    use_context::<ToastManager>()
}