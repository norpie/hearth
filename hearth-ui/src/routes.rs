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
    Story { story_id: String },
    Settings,
    Design,
}

impl AppRoute for Route {
    fn get_route_name(&self) -> &'static str {
        match self {
            Route::Stories => "Stories",
            Route::Characters => "Characters",
            Route::Scenarios => "Scenarios",
            Route::Story { .. } => "Story",
            Route::Settings => "Settings",
            Route::Design => "Design",
        }
    }
}

impl Route {
    /// Get the default route for all platforms
    pub fn default_route() -> Self {
        Route::Stories
    }

    /// Get navigation icon for this route
    pub fn icon(&self) -> &'static str {
        match self {
            Route::Stories => "fa-solid fa-book",
            Route::Characters => "fa-solid fa-user",
            Route::Scenarios => "fa-solid fa-map",
            Route::Story { .. } => "fa-solid fa-feather",
            Route::Settings => "fa-solid fa-gear",
            Route::Design => "fa-solid fa-palette",
        }
    }

    /// Get navigation label for this route
    pub fn label(&self) -> &'static str {
        match self {
            Route::Stories => "Stories",
            Route::Characters => "Characters",
            Route::Scenarios => "Scenarios",
            Route::Story { .. } => "Story",
            Route::Settings => "Settings",
            Route::Design => "Design",
        }
    }
}
