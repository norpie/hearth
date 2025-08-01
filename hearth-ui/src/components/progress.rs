//! Progress component for displaying completion status

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ProgressSize {
    Small,
    Medium,
    Large,
}

impl Default for ProgressSize {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// Current progress value (0.0 to max)
    pub value: f64,
    /// Maximum value (default: 100.0)
    #[props(default = 100.0)]
    pub max: f64,
    /// Size variant
    #[props(default)]
    pub size: ProgressSize,
    /// Show percentage indicator
    #[props(default = false)]
    pub show_percentage: bool,
    /// Custom label text
    #[props(default = None)]
    pub label: Option<String>,
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
    /// HTML id attribute
    #[props(default = None)]
    pub id: Option<String>,
}

fn get_size_classes(size: ProgressSize) -> (&'static str, &'static str) {
    match size {
        ProgressSize::Small => (
            "h-1",        // track height
            "text-xs",    // label text
        ),
        ProgressSize::Medium => (
            "h-2",        // track height
            "text-sm",    // label text
        ),
        ProgressSize::Large => (
            "h-3",        // track height
            "text-base",  // label text
        ),
    }
}

/// Progress component for displaying completion status
#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let (track_height, label_text_size) = get_size_classes(props.size);
    
    // Calculate percentage for positioning
    let percentage = if props.max > 0.0 {
        ((props.value / props.max) * 100.0).max(0.0).min(100.0)
    } else {
        0.0
    };

    let track_classes = format!(
        "relative w-full {} rounded-full bg-gray-200 dark:bg-gray-700 overflow-hidden",
        track_height
    );

    let filled_track_classes = format!(
        "h-full rounded-full bg-blue-600 dark:bg-blue-500 transition-all duration-300 ease-out"
    );

    let combined_classes = if props.class.is_empty() {
        "space-y-2".to_string()
    } else {
        format!("space-y-2 {}", props.class)
    };

    rsx! {
        div { class: combined_classes,
            // Label and percentage display
            if props.label.is_some() || props.show_percentage {
                div { class: "flex justify-between items-center",
                    if let Some(label) = &props.label {
                        label {
                            class: format!("font-medium text-gray-700 dark:text-gray-300 {}", label_text_size),
                            r#for: if props.id.is_some() { props.id.as_deref() } else { None },
                            "{label}"
                        }
                    }
                    if props.show_percentage {
                        span {
                            class: format!("font-mono {} text-gray-600 dark:text-gray-400", label_text_size),
                            "{percentage:.0}%"
                        }
                    }
                }
            }
            
            // Progress track
            div {
                class: track_classes,
                role: "progressbar",
                "aria-valuenow": "{props.value}",
                "aria-valuemax": "{props.max}",
                "aria-valuemin": "0",
                "aria-label": if props.label.is_some() { props.label.as_deref() } else { None },
                id: if props.id.is_some() { props.id.as_deref() } else { None },
                
                // Filled portion
                div {
                    class: filled_track_classes,
                    style: format!("width: {}%", percentage),
                }
            }
        }
    }
}