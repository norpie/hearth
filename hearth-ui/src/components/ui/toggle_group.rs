//! Toggle group component for mutually exclusive toggles
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     ToggleGroup {
//!         exclusive: true,
//!         ToggleGroupItem { value: "left", "Left" }
//!         ToggleGroupItem { value: "center", "Center" }
//!         ToggleGroupItem { value: "right", "Right" }
//!     }
//! }
//! ```

use crate::{ToggleSize, ToggleVariant};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ToggleGroupOrientation {
    Horizontal,
    Vertical,
}

/// Properties for the ToggleGroup container
#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupProps {
    /// Whether only one toggle can be active at a time
    #[props(default = false)]
    pub exclusive: bool,

    /// List of currently selected toggle values
    #[props(default = vec![])]
    pub value: Vec<String>,

    /// Callback when selection changes
    #[props(default)]
    pub onchange: Option<EventHandler<Vec<String>>>,

    /// Whether the entire group is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Size variant for all items in the group
    #[props(default = ToggleSize::Medium)]
    pub size: ToggleSize,

    /// Visual variant for all items in the group
    #[props(default = ToggleVariant::Default)]
    pub variant: ToggleVariant,

    /// Layout orientation for the group
    #[props(default = ToggleGroupOrientation::Horizontal)]
    pub orientation: ToggleGroupOrientation,

    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,

    /// Toggle group item children
    pub children: Element,
}

/// Properties for individual ToggleGroupItem components
#[derive(Props, Clone, PartialEq)]
pub struct ToggleGroupItemProps {
    /// Unique identifier for this toggle item
    pub value: String,

    /// Whether this item is pressed/active
    #[props(default = false)]
    pub pressed: bool,

    /// Whether this item is disabled
    #[props(default = false)]
    pub disabled: bool,

    /// Size variant for this item
    #[props(default = ToggleSize::Medium)]
    pub size: ToggleSize,

    /// Visual variant for this item
    #[props(default = ToggleVariant::Default)]
    pub variant: ToggleVariant,

    /// Orientation for visual grouping
    #[props(default = ToggleGroupOrientation::Horizontal)]
    pub orientation: ToggleGroupOrientation,

    /// Click event handler
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,

    /// Content to display inside this item
    pub children: Element,
}

/// Container for organizing multiple toggle buttons into groups
#[component]
pub fn ToggleGroup(props: ToggleGroupProps) -> Element {
    let container_classes = match props.orientation {
        ToggleGroupOrientation::Horizontal => format!("inline-flex {}", props.class),
        ToggleGroupOrientation::Vertical => format!("inline-flex flex-col {}", props.class),
    };

    rsx! {
        div {
            class: "{container_classes}",
            role: if props.exclusive { "radiogroup" } else { "group" },
            "aria-disabled": props.disabled,
            {props.children}
        }
    }
}

/// Individual toggle item for use within ToggleGroup containers
#[component]
pub fn ToggleGroupItem(props: ToggleGroupItemProps) -> Element {
    let size_classes = match props.size {
        ToggleSize::Small => "px-2.5 py-1.5 text-sm",
        ToggleSize::Medium => "px-3 py-2 text-base",
        ToggleSize::Large => "px-4 py-2.5 text-lg",
    };

    let base_classes = "inline-flex items-center justify-center border font-medium transition-all duration-150 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none active:scale-95 select-none";

    // Enhanced styling for proper button group appearance based on orientation
    let group_classes = match props.orientation {
        ToggleGroupOrientation::Horizontal => "!rounded-none first:!rounded-l-md last:!rounded-r-md relative focus:z-10 border-r-0 last:border-r",
        ToggleGroupOrientation::Vertical => "!rounded-none first:!rounded-t-md last:!rounded-b-md relative focus:z-10 border-b-0 last:border-b",
    };

    let variant_classes = match (props.variant, props.pressed) {
        (ToggleVariant::Default, false) => "bg-transparent hover:bg-muted active:bg-muted text-foreground border-transparent",
        (ToggleVariant::Default, true) => "bg-muted text-foreground border-transparent",
        (ToggleVariant::Outline, false) => "bg-transparent hover:bg-muted active:bg-muted text-foreground border-border hover:border-border",
        (ToggleVariant::Outline, true) => "bg-muted text-foreground border-border",
    };

    let combined_classes = format!(
        "{} {} {} {} {}",
        base_classes, variant_classes, size_classes, group_classes, props.class
    );

    rsx! {
        button {
            r#type: "button",
            class: "{combined_classes}",
            disabled: props.disabled,
            "aria-pressed": props.pressed,
            onclick: move |evt| {
                if !props.disabled {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                }
            },
            {props.children}
        }
    }
}
