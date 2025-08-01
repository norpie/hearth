//! Viewport abstraction for cross-platform UI
//!
//! Provides consistent viewport information across web, desktop, and mobile platforms.

use dioxus::prelude::*;
use crate::Platform;

#[cfg(target_arch = "wasm32")]
use web_sys;

#[cfg(all(not(target_arch = "wasm32"), not(feature = "mobile")))]
use dioxus_desktop::use_window;

#[derive(Clone, Copy, Debug)]
pub struct ViewportInfo {
    pub width: f64,
    pub height: f64,
}

impl ViewportInfo {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

/// Context for viewport information
#[derive(Clone, Copy)]
pub struct ViewportContext {
    pub info: Signal<ViewportInfo>,
}

/// Hook to access viewport information
pub fn use_viewport() -> Signal<ViewportInfo> {
    let ctx = use_context::<ViewportContext>();
    ctx.info
}

/// Initialize viewport tracking for the current platform
pub fn use_viewport_provider() -> ViewportContext {
    let mut info = use_signal(|| ViewportInfo::new(1280.0, 800.0)); // Initial fallback

    // Set up platform-specific viewport tracking
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            // Web platform: Use browser window APIs
            let mut info_clone = info.clone();
            Platform::spawn(async move {
                if let Some(window) = web_sys::window() {
                    // Set initial values
                    if let (Ok(width), Ok(height)) = (window.inner_width(), window.inner_height()) {
                        if let (Some(w), Some(h)) = (width.as_f64(), height.as_f64()) {
                            info_clone.set(ViewportInfo::new(w, h));
                        }
                    }
                }
            });
        }

        #[cfg(all(not(target_arch = "wasm32"), feature = "mobile"))]
        {
            // Mobile: Use common device dimensions
            info.set(ViewportInfo::new(390.0, 844.0));
        }

        #[cfg(all(not(target_arch = "wasm32"), not(feature = "mobile")))]
        {
            // Desktop/native: Use actual window dimensions
            let window = use_window();
            let size = window.inner_size();
            info.set(ViewportInfo::new(size.width as f64, size.height as f64));
        }
    });

    ViewportContext { info }
}

/// Provider component that sets up viewport context
#[component]
pub fn ViewportProvider(children: Element) -> Element {
    let viewport_ctx = use_viewport_provider();
    use_context_provider(|| viewport_ctx);

    rsx! {
        {children}
    }
}
