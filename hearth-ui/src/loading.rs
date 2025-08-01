//! Loading state management system for app initialization
//!
//! This module provides centralized loading state management to prevent FOUC
//! (Flash of Unstyled Content) and coordinate app initialization across platforms.
//! It supports progressive loading stages and provides hooks for components to
//! track loading progress.

use dioxus::prelude::*;
use crate::Platform;
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
use web_time::Instant;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

/// Loading stages during app initialization
///
/// Defines the progression of app startup from initial render through
/// full readiness. Each stage can have specific UI states and messaging.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadingStage {
    /// Initial app bootstrap - showing loading screen immediately
    Initializing,
    
    /// Loading critical assets (fonts, CSS, icons)
    LoadingAssets,
    
    /// Loading user settings and preferences
    LoadingSettings,
    
    /// App is fully loaded and ready to use
    Ready,
}

impl LoadingStage {
    /// Get user-friendly message for the current loading stage
    pub fn message(&self) -> &'static str {
        match self {
            LoadingStage::Initializing => "Starting Hearth...",
            LoadingStage::LoadingAssets => "Loading interface...",
            LoadingStage::LoadingSettings => "Loading your settings...",
            LoadingStage::Ready => "Ready!",
        }
    }

    /// Get progress percentage for the current stage (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        match self {
            LoadingStage::Initializing => 0.1,
            LoadingStage::LoadingAssets => 0.4,
            LoadingStage::LoadingSettings => 0.8,
            LoadingStage::Ready => 1.0,
        }
    }

    /// Check if this stage should show the main app content
    pub fn should_show_app(&self) -> bool {
        matches!(self, LoadingStage::Ready)
    }

    /// Get minimum time to spend in this stage (for UX smoothness)
    pub fn minimum_duration(&self) -> Duration {
        match self {
            LoadingStage::Initializing => Duration::from_millis(100),
            LoadingStage::LoadingAssets => Duration::from_millis(200),
            LoadingStage::LoadingSettings => Duration::from_millis(150),
            LoadingStage::Ready => Duration::ZERO,
        }
    }
}

impl Default for LoadingStage {
    fn default() -> Self {
        LoadingStage::Initializing
    }
}

/// Loading state tracking information
#[derive(Debug, Clone)]
pub struct LoadingState {
    /// Current loading stage
    pub stage: LoadingStage,
    
    /// When the current stage started
    pub stage_start_time: Instant,
    
    /// When the overall loading process started
    pub loading_start_time: Instant,
    
    /// Optional error message if loading failed
    pub error: Option<String>,
    
    /// Whether loading has completed successfully
    pub is_complete: bool,
}

impl LoadingState {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            stage: LoadingStage::Initializing,
            stage_start_time: now,
            loading_start_time: now,
            error: None,
            is_complete: false,
        }
    }

    /// Get elapsed time in current stage
    pub fn stage_elapsed(&self) -> Duration {
        self.stage_start_time.elapsed()
    }

    /// Get total loading time
    pub fn total_elapsed(&self) -> Duration {
        self.loading_start_time.elapsed()
    }

    /// Check if minimum time for current stage has elapsed
    pub fn stage_minimum_elapsed(&self) -> bool {
        self.stage_elapsed() >= self.stage.minimum_duration()
    }
}

impl Default for LoadingState {
    fn default() -> Self {
        Self::new()
    }
}

/// Loading state context - just the signal, no wrapper struct
pub type LoadingStateSignal = Signal<LoadingState>;

/// Helper functions for loading state management
pub struct LoadingHelpers;

impl LoadingHelpers {
    /// Advance to the next loading stage
    pub fn advance_stage(mut state: LoadingStateSignal, next_stage: LoadingStage) {
        let mut current = (state)();
        current.stage = next_stage;
        current.stage_start_time = Instant::now();
        
        if next_stage == LoadingStage::Ready {
            current.is_complete = true;
        }
        
        state.set(current);
        log::debug!("Loading stage advanced to: {:?}", next_stage);
    }

    /// Set loading error
    pub fn set_error(mut state: LoadingStateSignal, error: String) {
        let mut current = (state)();
        current.error = Some(error.clone());
        state.set(current);
        log::error!("Loading error: {}", error);
    }

    /// Check if we can advance from current stage (minimum time elapsed)
    pub fn can_advance(state: LoadingStateSignal) -> bool {
        (state)().stage_minimum_elapsed()
    }

    /// Get current loading stage
    pub fn current_stage(state: LoadingStateSignal) -> LoadingStage {
        (state)().stage
    }

    /// Check if loading is complete
    pub fn is_complete(state: LoadingStateSignal) -> bool {
        (state)().is_complete
    }

    /// Check if there's a loading error
    pub fn has_error(state: LoadingStateSignal) -> bool {
        (state)().error.is_some()
    }

    /// Get current error message if any
    pub fn error_message(state: LoadingStateSignal) -> Option<String> {
        (state)().error.clone()
    }
}

/// Hook to access loading state signal directly
pub fn use_loading_state() -> LoadingStateSignal {
    use_context::<LoadingStateSignal>()
}

/// Hook to get current loading stage
pub fn use_loading_stage() -> LoadingStage {
    let loading_state = use_loading_state();
    (loading_state)().stage
}

/// Hook to check if app is still loading
pub fn use_is_loading() -> bool {
    let loading_state = use_loading_state();
    !(loading_state)().is_complete
}

/// Hook to get loading progress (0.0 to 1.0)
pub fn use_loading_progress() -> f32 {
    let stage = use_loading_stage();
    stage.progress()
}

/// Hook to get current loading message
pub fn use_loading_message() -> &'static str {
    let stage = use_loading_stage();
    stage.message()
}

/// Hook to check if loading has an error
pub fn use_loading_error() -> Option<String> {
    let loading_state = use_loading_state();
    (loading_state)().error.clone()
}

/// Async loading stage controller that manages stage transitions
pub fn use_loading_controller() -> LoadingController {
    let loading_state = use_loading_state();
    LoadingController::new(loading_state)
}

/// Controller for managing loading stage transitions
#[derive(Clone)]
pub struct LoadingController {
    state: LoadingStateSignal,
}

impl LoadingController {
    pub fn new(state: LoadingStateSignal) -> Self {
        Self { state }
    }

    /// Advance to next stage if minimum time has elapsed
    pub async fn try_advance(&self, next_stage: LoadingStage) {
        // Wait for minimum stage duration if needed
        while !LoadingHelpers::can_advance(self.state) {
            Platform::sleep(Duration::from_millis(10)).await;
        }
        
        LoadingHelpers::advance_stage(self.state, next_stage);
    }

    /// Advance to next stage immediately (for testing or instant transitions)
    pub fn advance_now(&self, next_stage: LoadingStage) {
        LoadingHelpers::advance_stage(self.state, next_stage);
    }

    /// Set a loading error
    pub fn set_error(&self, error: String) {
        LoadingHelpers::set_error(self.state, error);
    }

    /// Complete loading sequence
    pub async fn complete(&self) {
        self.try_advance(LoadingStage::Ready).await;
    }
}