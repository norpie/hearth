//! Skeleton loading component for placeholder content

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum SkeletonVariant {
    Text,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Avatar,
    Custom,
}

impl Default for SkeletonVariant {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SkeletonProps {
    #[props(default)]
    pub variant: SkeletonVariant,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub width: Option<String>,
    #[props(default)]
    pub height: Option<String>,
    #[props(default)]
    pub size: Option<String>, // For avatar variant
}

#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    let base_classes = "animate-pulse bg-gray-200 dark:bg-gray-700 rounded-lg";
    
    let variant_classes = match props.variant {
        SkeletonVariant::Text => "h-4 w-3/4",
        SkeletonVariant::H1 => "h-10 w-full",
        SkeletonVariant::H2 => "h-8 w-5/6",
        SkeletonVariant::H3 => "h-7 w-4/5",
        SkeletonVariant::H4 => "h-6 w-3/4",
        SkeletonVariant::H5 => "h-5 w-2/3",
        SkeletonVariant::H6 => "h-4 w-1/2",
        SkeletonVariant::Avatar => {
            let size = props.size.as_deref().unwrap_or("w-12 h-12");
            return rsx! {
                div {
                    class: "{base_classes} {size} rounded-full {props.class.as_deref().unwrap_or(\"\")}"
                }
            };
        }
        SkeletonVariant::Custom => "",
    };
    
    let width = props.width.as_deref().unwrap_or("");
    let height = props.height.as_deref().unwrap_or("");
    let custom_class = props.class.as_deref().unwrap_or("");
    
    let final_classes = if props.variant == SkeletonVariant::Custom {
        format!("{} {} {} {}", base_classes, width, height, custom_class)
    } else {
        format!("{} {} {}", base_classes, variant_classes, custom_class)
    };
    
    rsx! {
        div {
            class: "{final_classes}"
        }
    }
}