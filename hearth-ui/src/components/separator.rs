//! Separator component for visual content division

use dioxus::prelude::*;

/// Orientation options for the Separator component
#[derive(Clone, PartialEq, Debug)]
pub enum SeparatorOrientation {
    Horizontal,
    Vertical,
}

/// Size variants for the Separator component
#[derive(Clone, PartialEq, Debug)]
pub enum SeparatorSize {
    Small,
    Medium,
    Large,
}

/// Visual variants for the Separator component
#[derive(Clone, PartialEq, Debug)]
pub enum SeparatorVariant {
    Default,
    Subtle,
    Bold,
    Dashed,
    Dotted,
}

impl SeparatorOrientation {
    pub fn classes(&self) -> &'static str {
        match self {
            SeparatorOrientation::Horizontal => "w-full h-px",
            SeparatorOrientation::Vertical => "h-full w-px",
        }
    }
}

impl SeparatorSize {
    pub fn classes(&self, orientation: &SeparatorOrientation) -> &'static str {
        match (self, orientation) {
            (SeparatorSize::Small, SeparatorOrientation::Horizontal) => "h-px",
            (SeparatorSize::Medium, SeparatorOrientation::Horizontal) => "h-0.5",
            (SeparatorSize::Large, SeparatorOrientation::Horizontal) => "h-1",
            (SeparatorSize::Small, SeparatorOrientation::Vertical) => "w-px",
            (SeparatorSize::Medium, SeparatorOrientation::Vertical) => "w-0.5",
            (SeparatorSize::Large, SeparatorOrientation::Vertical) => "w-1",
        }
    }
}

impl SeparatorVariant {
    pub fn classes(&self) -> &'static str {
        match self {
            SeparatorVariant::Default => "bg-gray-200 dark:bg-gray-700",
            SeparatorVariant::Subtle => "bg-gray-100 dark:bg-gray-800",
            SeparatorVariant::Bold => "bg-gray-400 dark:bg-gray-500",
            SeparatorVariant::Dashed => "border-0 border-t border-dashed border-gray-200 dark:border-gray-700 bg-transparent",
            SeparatorVariant::Dotted => "border-0 border-t border-dotted border-gray-200 dark:border-gray-700 bg-transparent",
        }
    }
}

impl Default for SeparatorOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

impl Default for SeparatorSize {
    fn default() -> Self {
        Self::Medium
    }
}

impl Default for SeparatorVariant {
    fn default() -> Self {
        Self::Default
    }
}

/// Props for the Separator component
#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {
    /// Orientation of the separator
    #[props(default)]
    pub orientation: SeparatorOrientation,
    
    /// Size of the separator
    #[props(default)]
    pub size: SeparatorSize,
    
    /// Visual variant of the separator
    #[props(default)]
    pub variant: SeparatorVariant,
    
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
    
    /// ARIA role for accessibility
    #[props(default = "separator".to_string())]
    pub role: String,
    
    /// ARIA orientation for accessibility
    #[props(default)]
    pub aria_orientation: Option<String>,
    
    /// Decorative separators don't have semantic meaning
    #[props(default = true)]
    pub decorative: bool,
}

/// Separator component for visual content division
#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let orientation_classes = props.orientation.classes();
    let size_classes = props.size.classes(&props.orientation);
    let variant_classes = props.variant.classes();
    
    // For dashed and dotted variants, we need different base classes
    let base_classes = match props.variant {
        SeparatorVariant::Dashed | SeparatorVariant::Dotted => {
            match props.orientation {
                SeparatorOrientation::Horizontal => "w-full",
                SeparatorOrientation::Vertical => "h-full border-l border-t-0",
            }
        }
        _ => orientation_classes
    };
    
    let combined_classes = if props.class.is_empty() {
        format!("{} {} {}", base_classes, size_classes, variant_classes)
    } else {
        format!("{} {} {} {}", base_classes, size_classes, variant_classes, props.class)
    };
    
    // Determine ARIA orientation
    let aria_orientation = props.aria_orientation.unwrap_or_else(|| {
        match props.orientation {
            SeparatorOrientation::Horizontal => "horizontal".to_string(),
            SeparatorOrientation::Vertical => "vertical".to_string(),
        }
    });

    rsx! {
        div {
            class: combined_classes,
            role: if props.decorative { "none" } else { props.role.as_str() },
            "aria-orientation": if !props.decorative { Some(aria_orientation.as_str()) } else { None },
            "data-orientation": match props.orientation {
                SeparatorOrientation::Horizontal => "horizontal",
                SeparatorOrientation::Vertical => "vertical",
            },
        }
    }
}