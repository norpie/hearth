//! App loading screen component
//!
//! Provides a full-screen loading overlay that prevents FOUC and shows
//! loading progress during app initialization. Includes branded loading
//! experience with skeleton previews of the main app interface.

use crate::{
    Logo, Progress,
    use_loading_message, use_loading_progress, use_loading_stage, LoadingStage,
};
use dioxus::prelude::*;

/// App loading screen properties
#[derive(Props, Clone, PartialEq)]
pub struct AppLoadingProps {
    /// Additional CSS classes for the loading container
    #[props(default)]
    pub class: Option<String>,
    
    /// Whether to show loading progress bar
    #[props(default = true)]
    pub show_progress: bool,
    
    /// Whether to show loading messages
    #[props(default = true)]
    pub show_messages: bool,
}

/// Full-screen app loading component
///
/// Shows a branded loading screen during app initialization to prevent FOUC.
/// Includes progress indication, loading messages, and optional skeleton preview
/// of the main app interface to give users context of what's loading.
///
/// # Features
///
/// - **Immediate display**: Shows instantly to prevent flash of unstyled content
/// - **Platform awareness**: Adapts layout for mobile vs desktop
/// - **Progress indication**: Visual progress bar and stage-specific messages
/// - **Smooth transitions**: Coordinated with main app reveal
/// - **Branded experience**: Uses app logo and design system colors
/// - **Responsive design**: Works across all screen sizes
///
/// # Loading Stages
///
/// The component responds to different loading stages:
/// - **Initializing**: Shows logo and "Starting Hearth..." message
/// - **LoadingAssets**: Shows "Loading interface..." with progress
/// - **LoadingSettings**: Shows "Loading your settings..." 
/// - **Ready**: Transitions to main app (component hidden)
///
/// # Usage
///
/// ```rust
/// rsx! {
///     if use_is_loading() {
///         AppLoading {
///             show_progress: true,
///             show_messages: true,
///         }
///     } else {
///         MainAppContent {}
///     }
/// }
/// ```
///
/// For minimal loading (testing or fast connections):
/// ```rust
/// rsx! {
///     AppLoading {
///         show_progress: false,
///         show_messages: false,
///     }
/// }
/// ```
#[component]
pub fn AppLoading(props: AppLoadingProps) -> Element {
    let stage = use_loading_stage();
    let progress = use_loading_progress();
    let message = use_loading_message();
    
    // Get dark mode context for conditional styling
    let dark_mode = use_context::<crate::DarkModeContext>();
    let is_dark = (dark_mode.is_dark)();
    
    let container_classes = format!(
        "fixed inset-0 z-50 bg-background flex flex-col items-center justify-center {}",
        props.class.as_deref().unwrap_or("")
    );

    // Define colors based on theme
    let (bg_color, text_color, muted_color) = if is_dark {
        ("#141414", "#ffffff", "#a1a1aa") // Dark mode colors
    } else {
        ("#ffffff", "#0a0a0a", "#6b7280") // Light mode colors  
    };

    rsx! {
        div { 
            class: "{container_classes}",
            style: "
                position: fixed; 
                top: 0; 
                left: 0; 
                right: 0; 
                bottom: 0; 
                z-index: 50; 
                background-color: {bg_color}; 
                display: flex; 
                flex-direction: column; 
                align-items: center; 
                justify-content: center;
                font-family: 'Inter Variable', 'Inter', ui-sans-serif, system-ui, sans-serif;
                color: {text_color};
            ",
            
            // Main loading content
            div { 
                class: "flex flex-col items-center justify-center space-y-8 max-w-md w-full px-6",
                style: "
                    display: flex; 
                    flex-direction: column; 
                    align-items: center; 
                    justify-content: center; 
                    gap: 2rem; 
                    max-width: 28rem; 
                    width: 100%; 
                    padding: 0 1.5rem;
                ",
                
                // Logo section
                div { 
                    class: "flex flex-col items-center space-y-4",
                    style: "
                        display: flex; 
                        flex-direction: column; 
                        align-items: center; 
                        gap: 1rem;
                    ",
                    
                    div { 
                        class: "w-20 h-20 md:w-24 md:h-24",
                        style: "width: 5rem; height: 5rem;",
                        Logo { class: "w-full h-full".to_string() }
                    }
                }

                // Progress section
                if props.show_progress {
                    div { 
                        class: "w-full space-y-3",
                        style: "width: 100%; display: flex; flex-direction: column; gap: 0.75rem;",
                        
                        Progress {
                            value: (progress * 100.0).into(),
                            class: "h-2".to_string(),
                        }
                        
                        if props.show_messages {
                            p { 
                                class: "text-center text-sm text-muted-foreground animate-pulse",
                                style: "
                                    text-align: center; 
                                    font-size: 0.875rem; 
                                    color: {muted_color}; 
                                    margin: 0;
                                    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
                                ",
                                "{message}"
                            }
                        }
                    }
                }
            }

            // Footer with version or loading info
            div { 
                class: "absolute bottom-8 left-1/2 transform -translate-x-1/2",
                style: "
                    position: absolute; 
                    bottom: 2rem; 
                    left: 50%; 
                    transform: translateX(-50%);
                ",
                p { 
                    class: "text-xs text-muted-foreground/60",
                    style: "
                        font-size: 0.75rem; 
                        color: {muted_color}; 
                        margin: 0; 
                        opacity: 0.6;
                    ",
                    "Hearth v0.1.0"
                }
            }
        }
    }
}