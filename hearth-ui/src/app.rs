//! Unified main app with adaptive routing

use crate::{
    provide_settings_context, AdaptiveLayout, AppLoading, CharactersView, DarkModeContext, Design,
    LoadingState, LoadingStage, Route, ScenariosView, SettingsView, 
    StoriesView, StoryView, ToastManager, Toaster, ViewportProvider, use_is_loading, use_loading_controller, Platform,
};
use dioxus::prelude::*;
use dioxus_document::{Link, Stylesheet};
use hearth_core::{init_logging, SettingsManager, Theme};
use manganis::AssetOptions;
use std::time::Duration;

const _: Asset = asset!("/dist/tailwind.css");
const _: Asset = asset!("/assets/fontawesome.css");

const FAVICON: Asset = asset!(
    "/assets/icons/favicon.ico",
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

    // Initialize loading state signal first (before anything else)
    let _loading_state = use_context_provider(|| Signal::new(LoadingState::new()));
    let loading_controller = use_loading_controller();
    
    // Initialize logging system early
    use_hook(|| {
        if let Err(e) = init_logging() {
            log::error!("Failed to initialize logging: {e}");
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
    let toast_manager = use_context_provider(ToastManager::new);
    
    // Async loading sequence - run only once on mount
    use_effect(use_reactive((), move |_| {
        let loading_controller = loading_controller.clone();
        Platform::spawn(async move {
            // Stage 1: Loading assets (immediate for web)
            #[cfg(target_arch = "wasm32")]
            {
                // On web, advance through stages more quickly
                loading_controller.advance_now(LoadingStage::LoadingAssets);
                Platform::sleep(Duration::from_millis(100)).await;
                
                loading_controller.advance_now(LoadingStage::LoadingSettings);
                Platform::sleep(Duration::from_millis(100)).await;
                
                loading_controller.advance_now(LoadingStage::Ready);
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                // On native platforms, use the proper timing
                loading_controller.try_advance(LoadingStage::LoadingAssets).await;
                loading_controller.try_advance(LoadingStage::LoadingSettings).await;
                loading_controller.complete().await;
            }
        });
    }));

    // On Android, clear toasts when route changes to prevent stale toasts
    #[cfg(not(target_arch = "wasm32"))]
    use_effect(move || {
        // This effect runs whenever current_route changes
        let _ = current_route();
        toast_manager.clear_all();
    });

    let navigate_to = move |route: Route| {
        log::debug!("Navigating to: {route:?}");
        current_route.set(route);
    };

    rsx! {
        // Global assets
        Stylesheet { href: asset!("/dist/tailwind.css") }
        // Web-only favicon
        if cfg!(target_arch = "wasm32") {
            Link { rel: "icon", href: FAVICON }
        }
        ViewportProvider {
            div { class: format!("{} h-screen bg-background", if is_dark() { "dark" } else { "" }),
                
                // Show loading screen while app is loading
                if use_is_loading() {
                    AppLoading {
                        show_progress: true,
                        show_messages: true,
                    }
                } else {
                    // Main app content
                    AdaptiveLayout { current_route: current_route(), navigate_to,
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
                            Route::Story { story_id } => rsx! {
                                StoryView { story_id: story_id.clone(), navigate_to }
                            },
                            Route::Settings => rsx! {
                                SettingsView { navigate_to }
                            },
                            Route::Design => rsx! {
                                Design {}
                            },
                        }
                    }

                    // Global toast container
                    Toaster {}
                }
            }
        }
    }
}
