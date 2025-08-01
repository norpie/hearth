//! Avatar components for user and entity representation
//!
//! The avatar module provides avatar components for displaying user profile pictures
//! or placeholder initials. It includes automatic fallback handling, responsive sizing,
//! and optional interactive capabilities.
//!
//! # Examples
//!
//! Basic avatar with image:
//! ```rust
//! rsx! {
//!     Avatar {
//!         name: "John Doe".to_string(),
//!         avatar_url: Some("https://example.com/avatar.jpg".to_string()),
//!         size: Some("w-12 h-12".to_string()),
//!     }
//! }
//! ```
//!
//! Avatar with fallback initials:
//! ```rust
//! rsx! {
//!     Avatar {
//!         name: "Jane Smith".to_string(),
//!         avatar_url: None,
//!         size: Some("w-10 h-10".to_string()),
//!     }
//! }
//! ```

static DEFAULT_AVATAR: Asset = asset!("./assets/avatar/default.png");

use dioxus::{html::img, prelude::*};
use crate::AspectRatio;

/// Avatar variant types
#[derive(Clone, PartialEq)]
pub enum AvatarVariant {
    /// Default circular avatar
    Circle,
    /// 9:16 portrait rectangle with rounded corners using AspectRatio
    Portrait,
}

impl Default for AvatarVariant {
    fn default() -> Self {
        Self::Circle
    }
}

/// Properties for configuring the Avatar component
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// Display name for the avatar
    ///
    /// Used to generate fallback initials when no image is available or when
    /// the image fails to load. The first character is extracted and displayed
    /// as an uppercase initial. Also used for accessibility as the alt text
    /// for the avatar image. Required parameter.
    pub name: String,

    /// Optional URL for the avatar image
    ///
    /// When provided, attempts to load and display the image. If the image
    /// fails to load or is not provided, falls back to displaying the first
    /// character of the name. Supports any valid image URL format.
    /// Defaults to None (shows initials only).
    pub avatar_url: Option<String>,

    /// Optional CSS size classes for the avatar dimensions
    ///
    /// Accepts Tailwind CSS size classes like "w-6 h-6", "w-10 h-10", "w-12 h-12",
    /// etc. Automatically adjusts text size based on avatar size for optimal
    /// proportions. When not provided, defaults to "w-12 h-12" (48px).
    /// Defaults to None (uses default size).
    #[props(default)]
    pub size: Option<String>,

    /// Optional hover event handler
    ///
    /// Called when the avatar is hovered (mouse enter/leave). Receives a boolean
    /// indicating whether the mouse is currently over the avatar (true) or has
    /// left (false). Useful for showing tooltips or hover effects.
    /// Defaults to None (no hover handling).
    #[props(default)]
    pub on_hover: Option<EventHandler<bool>>,

    /// Optional click event handler
    ///
    /// Called when the avatar is clicked. When provided, the avatar becomes
    /// interactive with hover effects and cursor pointer styling. Useful for
    /// opening user profiles, menus, or other actions.
    /// Defaults to None (non-interactive).
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// Avatar variant type
    ///
    /// Specifies the shape and aspect ratio of the avatar. Circle creates a
    /// standard circular avatar, while Portrait creates a 9:16 rectangle with
    /// rounded corners using AspectRatio component, ideal for character portraits in storytelling.
    /// Defaults to Circle.
    #[props(default)]
    pub variant: AvatarVariant,
}

/// Avatar component for user and entity representation
///
/// The Avatar component displays user profile pictures with automatic fallback
/// to initials when images are unavailable. It supports different sizes and
/// optional interactive capabilities for click and hover events.
#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    // Handle size and shape based on variant
    let (additional_class, aspect_ratio) = match props.variant {
        AvatarVariant::Circle => {
            ("rounded-full border-2 border-border bg-avatar-default", 1.0)
        }
        AvatarVariant::Portrait => {
            ("rounded-lg border-2 border-border", 10.0 / 13.0)
        }
    };
    
    // Get the size, defaulting to w-12 h-12 if not provided
    let full_size_class = props.size.clone().unwrap_or_else(|| "w-12 h-12".to_string());
    
    // For AspectRatio, we only need the width - it calculates height from ratio
    let width_class = if let Some(width) = full_size_class.split_whitespace().find(|s| s.starts_with("w-")) {
        width.to_string()
    } else {
        "w-12".to_string()
    };
    
    let container_class = match props.variant {
        AvatarVariant::Circle => full_size_class, // Circle needs both w and h
        AvatarVariant::Portrait => width_class,   // AspectRatio only needs width
    };
    
    let is_clickable = props.onclick.is_some();

    rsx! {
        div {
            class: if is_clickable { "flex-shrink-0 cursor-pointer hover:opacity-80 transition-opacity select-none touch-manipulation" } else { "flex-shrink-0 select-none" },
            style: "user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
            onmouseenter: move |_| {
                if let Some(handler) = &props.on_hover {
                    handler.call(true);
                }
            },
            onmouseleave: move |_| {
                if let Some(handler) = &props.on_hover {
                    handler.call(false);
                }
            },
            onclick: move |event| {
                if let Some(handler) = &props.onclick {
                    handler.call(event);
                }
            },
            match props.variant {
                AvatarVariant::Circle => rsx! {
                    div {
                        class: format!("{} {} overflow-hidden", container_class, additional_class),
                        AvatarImage {
                            url: props.avatar_url.clone(),
                            name: props.name.clone(),
                            class: "w-full h-full object-contain select-none".to_string(),
                        }
                    }
                },
                AvatarVariant::Portrait => rsx! {
                    div {
                        class: format!("{}", container_class),
                        AspectRatio {
                            ratio: aspect_ratio,
                            class: Some(format!("{} overflow-hidden", additional_class)),
                            AvatarImage {
                                url: props.avatar_url.clone(),
                                name: props.name.clone(),
                                class: "w-full h-full object-contain select-none".to_string(),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct AvatarImageProps {
    url: Option<String>,
    name: String,
    class: String,
}

#[component]
fn AvatarImage(props: AvatarImageProps) -> Element {
    let AvatarImageProps {
        url,
        name,
        class
    } = props;
    match url {
        Some(url) => rsx! {
            img {
                class: class,
                src: url,
                alt: name,
            },
        },
        None => rsx! { 
            img {
                class: class,
                src: DEFAULT_AVATAR,
                alt: name,
            }
        }
    }
}
