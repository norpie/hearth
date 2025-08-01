//! Shared data models and types

use dioxus::prelude::*;
pub use hearth_core::{models::*, sample::*, settings::*, storage::*};

// Platform-specific async utilities
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures;
#[cfg(target_arch = "wasm32")]
use gloo_timers;

use std::time::Duration;

// Route trait for generic navigation
pub trait AppRoute: Clone + PartialEq {
    fn get_route_name(&self) -> &'static str;
}

// Dark mode context for platform-agnostic theming
#[derive(Clone, Copy)]
pub struct DarkModeContext {
    pub is_dark: Signal<bool>,
}

// Platform detection utilities
#[derive(Clone, Copy, PartialEq)]
pub enum Platform {
    Mobile,
    Desktop,
    Web,
}

impl Platform {
    pub fn current() -> Self {
        if cfg!(feature = "mobile") {
            Platform::Mobile
        } else if cfg!(target_arch = "wasm32") {
            Platform::Web
        } else {
            Platform::Desktop
        }
    }

    pub fn is_mobile(&self) -> bool {
        matches!(self, Platform::Mobile)
    }

    pub fn uses_bottom_navigation(&self) -> bool {
        matches!(self, Platform::Mobile)
    }

    pub fn uses_sidebar(&self) -> bool {
        !self.uses_bottom_navigation()
    }

    pub fn can_edit_backend_settings(&self) -> bool {
        matches!(self, Platform::Desktop | Platform::Mobile)
    }

    /// Spawn an async task using the appropriate runtime for the current platform
    /// 
    /// On WASM, uses wasm_bindgen_futures::spawn_local
    /// On native platforms, uses dioxus::prelude::spawn
    pub fn spawn<F>(future: F)
    where
        F: std::future::Future<Output = ()> + 'static,
    {
        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(future);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            spawn(future);
        }
    }

    /// Cross-platform async delay function
    /// 
    /// On WASM, uses gloo_timers::future::sleep
    /// On native platforms, uses tokio::time::sleep
    pub async fn sleep(duration: Duration) {
        #[cfg(target_arch = "wasm32")]
        {
            gloo_timers::future::sleep(duration).await;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            tokio::time::sleep(duration).await;
        }
    }
}

// Sidebar visibility context for web/desktop platforms
#[derive(Clone, Copy, PartialEq)]
pub struct SidebarContext {
    pub is_visible: Signal<bool>,
}

// Mobile navbar visibility context for mobile platforms
#[derive(Clone, Copy)]
pub struct MobileNavbarContext {
    pub is_visible: Signal<bool>,
}

// Sidebar route configuration
#[derive(Clone, PartialEq)]
pub struct SidebarRoute<T> {
    pub route: Option<T>,
    pub icon: &'static str,
    pub label: &'static str,
    pub is_separator: bool,
}

impl<T> SidebarRoute<T> {
    pub fn new(route: T, icon: &'static str, label: &'static str) -> Self {
        Self {
            route: Some(route),
            icon,
            label,
            is_separator: false,
        }
    }

    pub fn separator() -> Self {
        Self {
            route: None,
            icon: "",
            label: "",
            is_separator: true,
        }
    }
}

// Story message models for interactive storytelling interface
#[derive(Clone, PartialEq)]
pub struct StoryMessage {
    pub id: String,
    pub role: StoryRole,
    pub content: String,
}

#[derive(Clone, PartialEq)]
pub enum StoryRole {
    User { name: String },
    Narrator,
    Character { name: String },
}

// Character selection menu models
#[derive(Clone, PartialEq)]
pub struct CharacterOption {
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub is_narrator: bool,
}

// Universal search models
use std::collections::{HashMap, HashSet};

// Tag selection state for 3-state selection (none, positive, negative)
#[derive(Debug, Clone, PartialEq)]
pub enum TagState {
    None,      // Not selected
    Positive,  // Want this tag (green)
    Negative,  // Don't want this tag (red)
}

impl Default for TagState {
    fn default() -> Self {
        TagState::None
    }
}

// Search state management for internal use
#[derive(Clone, Default, PartialEq)]
pub struct SearchState {
    pub search_text: String,
    pub sort_option: String,
    pub sort_ascending: bool,
    pub favorites_only: bool,
    pub previously_used_only: bool,
    pub multiple_characters_only: bool,
    pub recently_added_only: bool,
    pub has_images_only: bool,
    pub untagged_only: bool,
}

// Universal search query that gets emitted on any change
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UniversalSearchQuery {
    pub search_text: String,
    pub sort_option: String,
    pub sort_ascending: bool,
    pub favorites_only: bool,
    pub previously_used_only: bool,
    pub multiple_characters_only: bool,
    pub recently_added_only: bool,
    pub has_images_only: bool,
    pub untagged_only: bool,
    pub wanted_character_tags: HashSet<String>,
    pub unwanted_character_tags: HashSet<String>,
    pub wanted_scenario_tags: HashSet<String>,
    pub unwanted_scenario_tags: HashSet<String>,
}

// Internal state management (kept for backwards compatibility)
#[derive(Clone, Default, PartialEq)]
pub struct UniversalSearchState {
    pub search_state: SearchState,
    pub character_tag_states: HashMap<String, TagState>,
    pub scenario_tag_states: HashMap<String, TagState>,
    pub expanded: bool,
}

impl UniversalSearchState {
    pub fn reset(&mut self) {
        self.search_state = SearchState::default();
        self.character_tag_states.clear();
        self.scenario_tag_states.clear();
    }

    pub fn to_query(&self) -> UniversalSearchQuery {
        let wanted_character_tags = self.character_tag_states
            .iter()
            .filter_map(|(tag, state)| {
                if *state == TagState::Positive {
                    Some(tag.clone())
                } else {
                    None
                }
            })
            .collect();

        let unwanted_character_tags = self.character_tag_states
            .iter()
            .filter_map(|(tag, state)| {
                if *state == TagState::Negative {
                    Some(tag.clone())
                } else {
                    None
                }
            })
            .collect();

        let wanted_scenario_tags = self.scenario_tag_states
            .iter()
            .filter_map(|(tag, state)| {
                if *state == TagState::Positive {
                    Some(tag.clone())
                } else {
                    None
                }
            })
            .collect();

        let unwanted_scenario_tags = self.scenario_tag_states
            .iter()
            .filter_map(|(tag, state)| {
                if *state == TagState::Negative {
                    Some(tag.clone())
                } else {
                    None
                }
            })
            .collect();

        UniversalSearchQuery {
            search_text: self.search_state.search_text.clone(),
            sort_option: self.search_state.sort_option.clone(),
            sort_ascending: self.search_state.sort_ascending,
            favorites_only: self.search_state.favorites_only,
            previously_used_only: self.search_state.previously_used_only,
            multiple_characters_only: self.search_state.multiple_characters_only,
            recently_added_only: self.search_state.recently_added_only,
            has_images_only: self.search_state.has_images_only,
            untagged_only: self.search_state.untagged_only,
            wanted_character_tags,
            unwanted_character_tags,
            wanted_scenario_tags,
            unwanted_scenario_tags,
        }
    }
}
