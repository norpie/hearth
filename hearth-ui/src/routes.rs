//! Unified application routes for all platforms

use crate::models::AppRoute;

#[derive(Clone, PartialEq)]
pub enum SearchContext {
    Stories,
    Characters,
    Scenarios,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Route {
    Stories,
    Characters,
    Scenarios,
    Settings,
    Design,
    Chat { story_id: String },
}

impl AppRoute for Route {
    fn is_chat_route(&self) -> bool {
        matches!(self, Route::Chat { .. })
    }

    fn get_route_name(&self) -> &'static str {
        match self {
            Route::Stories => "Stories",
            Route::Characters => "Characters",
            Route::Scenarios => "Scenarios",
            Route::Settings => "Settings",
            Route::Design => "Design",
            Route::Chat { .. } => "Chat",
        }
    }
}

impl Route {
    /// Get the default route for all platforms
    pub fn default_route() -> Self {
        Route::Design
    }

    /// Get navigation icon for this route
    pub fn icon(&self) -> &'static str {
        match self {
            Route::Stories => "fa-solid fa-book",
            Route::Characters => "fa-solid fa-user",
            Route::Scenarios => "fa-solid fa-map",
            Route::Settings => "fa-solid fa-gear",
            Route::Design => "fa-solid fa-palette",
            Route::Chat { .. } => "fa-solid fa-message",
        }
    }

    /// Get navigation label for this route
    pub fn label(&self) -> &'static str {
        match self {
            Route::Stories => "Stories",
            Route::Characters => "Characters",
            Route::Scenarios => "Scenarios",
            Route::Settings => "Settings",
            Route::Design => "Design",
            Route::Chat { .. } => "Chat",
        }
    }
}