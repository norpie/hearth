//! Avatar Group component for displaying multiple avatars in an overlapping layout

use crate::{Avatar, Platform};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AvatarGroupProps {
    /// List of avatar data (name, optional avatar_url)
    pub avatars: Vec<AvatarData>,
    /// Maximum number of avatars to show before showing overflow indicator
    #[props(default = 3)]
    pub max_visible: usize,
    /// Size of individual avatars
    #[props(default = "w-8 h-8".to_string())]
    pub size: String,
}

#[derive(Clone, PartialEq)]
pub struct AvatarData {
    pub name: String,
    pub avatar_url: Option<String>,
}

#[component]
pub fn AvatarGroup(props: AvatarGroupProps) -> Element {
    let visible_avatars = &props.avatars[..props.max_visible.min(props.avatars.len())];
    let overflow_count = if props.avatars.len() > props.max_visible {
        props.avatars.len() - props.max_visible
    } else {
        0
    };
    let overlap_class = "-ml-4";

    rsx! {
        div { class: "flex items-center",
            // Show visible avatars with overlapping effect
            for (index, avatar) in visible_avatars.iter().enumerate() {
                div { 
                    class: format!("relative {}", if index > 0 { overlap_class } else { "" }),
                    style: format!("z-index: {}", props.max_visible - index),
                    Avatar {
                        name: avatar.name.clone(),
                        avatar_url: avatar.avatar_url.clone(),
                        size: props.size.clone(),
                    }
                }
            }
            
            // Show overflow indicator if needed
            if overflow_count > 0 {
                div {
                    class: format!("relative {} {} bg-avatar-default text-avatar-default-foreground border-2 border-border rounded-full flex items-center justify-center text-xs font-medium", overlap_class, props.size),
                    style: "z-index: 0",
                    "+{overflow_count}"
                }
            }
        }
    }
}