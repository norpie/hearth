//! Unified main app with adaptive routing

use crate::{
    provide_settings_context, AdaptiveLayout, CharactersView, ChatView, DarkModeContext, Design,
    Route, ScenariosView, SettingsView, StoriesView, ToastManager, Toaster, ViewportProvider,
};
use dioxus::prelude::*;
use dioxus_document::{Link, Stylesheet};
use hearth_core::{init_logging, SettingsManager, Theme};
use manganis::AssetOptions;

const _: Asset = asset!("/assets/tailwind.css");
const _: Asset = asset!("/assets/fontawesome.css");

const FAVICON: Asset = asset!(
    "/assets/favicon.ico",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);

#[component]
pub fn App() -> Element {
    let _: Asset = asset!(
        "/assets/fonts/InterVariable.woff2",
        AssetOptions::builder()
            .with_hash_suffix(false)
            .into_asset_options()
    );
    let _: Asset = asset!(
        "/assets/fonts/InterVariable-Italic.woff2",
        AssetOptions::builder()
            .with_hash_suffix(false)
            .into_asset_options()
    );
    let _: Asset = asset!(
        "/assets/fonts/fa-solid-900.woff2",
        AssetOptions::builder()
            .with_hash_suffix(false)
            .into_asset_options()
    );
    let _: Asset = asset!(
        "/assets/fonts/fa-regular-400.woff2",
        AssetOptions::builder()
            .with_hash_suffix(false)
            .into_asset_options()
    );
    let _: Asset = asset!(
        "/assets/fonts/fa-brands-400.woff2",
        AssetOptions::builder()
            .with_hash_suffix(false)
            .into_asset_options()
    );

    // Initialize logging system early
    use_hook(|| {
        if let Err(e) = init_logging() {
            // Fall back to console logging if available
            #[cfg(target_arch = "wasm32")]
            web_sys::console::error_1(&format!("Failed to initialize logging: {}", e).into());
        } else {
            log::info!("Hearth application starting up");
        }
    });

    let mut current_route = use_signal(Route::default_route);

    // Load and provide settings context
    let settings_manager = SettingsManager::new();
    log::debug!("Settings manager initialized");
    provide_settings_context(settings_manager);

    // Get theme from settings
    let settings = crate::use_settings();
    let is_dark = use_signal(|| matches!(settings.read().get().theme, Theme::Dark));
    use_context_provider(|| DarkModeContext { is_dark });

    // Provide toast manager context
    let toast_manager = use_context_provider(|| ToastManager::new());

    // On Android, clear toasts when route changes to prevent stale toasts
    #[cfg(not(target_arch = "wasm32"))]
    use_effect(move || {
        // This effect runs whenever current_route changes
        let _ = current_route();
        toast_manager.clear_all();
    });

    let navigate_to = move |route: Route| {
        log::debug!("Navigating to: {:?}", route);
        current_route.set(route);
    };

    rsx! {
        // Global assets
        Stylesheet { href: asset!("/assets/tailwind.css") },
        // Web-only favicon
        if cfg!(target_arch = "wasm32") {
            Link { rel: "icon", href: FAVICON }
        }
        ViewportProvider {
            div {
                class: format!(
                    "{} h-screen bg-gray-50 dark:bg-gray-900",
                    if is_dark() { "dark" } else { "" },
                ),
                AdaptiveLayout {
                    current_route: current_route(),
                    navigate_to,
                    match current_route() {
                        Route::Stories => rsx! {
                            StoriesView { navigate_to }
                        },
                        Route::Characters => rsx! {
                            CharactersView { navigate_to }
                        },
                        Route::Scenarios => rsx! {
                            ScenariosView { navigate_to }
                        },
                        Route::Settings => rsx! {
                            SettingsView { navigate_to }
                        },
                        Route::Design => rsx! {
                            Design {}
                        },
                        Route::Chat { story_id: _ } => rsx! {
                            ChatView { navigate_to }
                        },
                    }
                }

                // Global toast container
                Toaster {}
            }
        }
    }
}
