//! Components module
//!
//! This module organizes components into logical groups:
//! - ui: Generic reusable UI components with no domain knowledge
//! - features: Feature-specific page sections with domain functionality

pub mod ui;
pub use ui::*;

pub mod features;
pub use features::*;
