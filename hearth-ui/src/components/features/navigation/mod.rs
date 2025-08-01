//! Navigation components for cross-platform application navigation
//!
//! This module provides navigation components that handle routing and UI state
//! across different platforms. It includes sidebar navigation for desktop/web
//! and bottom navigation for mobile platforms.

pub mod sidebar;
pub mod bottom_nav;

pub use sidebar::*;
pub use bottom_nav::*;