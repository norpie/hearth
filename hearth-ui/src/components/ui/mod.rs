//! Generic UI components module
//!
//! This module contains reusable UI components that have no domain knowledge
//! and can be used anywhere in the application.

pub mod accordion;
pub use accordion::*;

pub mod aspect_ratio;
pub use aspect_ratio::*;

pub mod avatar;
pub use avatar::*;

pub mod avatar_group;
pub use avatar_group::*;

pub mod badge;
pub use badge::*;

pub mod button;
pub use button::*;

pub mod calendar;
pub use calendar::*;

pub mod card;
pub use card::*;

pub mod carousel;
pub use carousel::*;

pub mod checkbox;
pub use checkbox::*;

pub mod collapsible;
pub use collapsible::*;

pub mod toggle_icon;
pub use toggle_icon::*;

pub mod input;
pub use input::*;

pub mod input_otp;
pub use input_otp::*;

pub mod label;
pub use label::*;

pub mod layout;
pub use layout::*;

pub mod markdown_content;
pub use markdown_content::*;

pub mod modal;
pub use modal::*;

pub mod notice;
pub use notice::*;

pub mod popover;
pub use popover::*;

pub mod progress;
pub use progress::*;

pub mod radio;
pub use radio::*;

pub mod range_calendar;
pub use range_calendar::*;

pub mod scroll_area;
pub use scroll_area::*;

pub mod select;
pub use select::*;

pub mod separator;
pub use separator::*;

pub mod sheet;
pub use sheet::*;

pub mod skeleton;
pub use skeleton::*;

pub mod slider;
pub use slider::*;

pub mod switch;
pub use switch::*;

pub mod table;
pub use table::*;

pub mod tabs;
pub use tabs::*;

pub mod textarea;
pub use textarea::*;

pub mod toaster;
pub use toaster::*;

pub mod toggle;
pub use toggle::*;

pub mod toggle_group;
pub use toggle_group::*;

pub mod gesture_detector;
pub use gesture_detector::*;

pub mod app_loading;
pub use app_loading::*;