//! Progress components for displaying completion status and loading states
//!
//! The progress module provides visual indicators for showing completion percentages,
//! loading states, and progress through multi-step processes. It includes customizable
//! sizing, labeling, and percentage display options.
//!
//! # Examples
//!
//! Basic progress bar:
//! ```rust
//! rsx! {
//!     Progress {
//!         value: 65.0,
//!         max: 100.0,
//!         size: ProgressSize::Medium,
//!     }
//! }
//! ```
//!
//! Progress with label and percentage:
//! ```rust
//! rsx! {
//!     Progress {
//!         value: 450.0,
//!         max: 500.0,
//!         label: Some("Download Progress".to_string()),
//!         show_percentage: true,
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Size variants for progress bar dimensions and typography
///
/// Controls the visual prominence and spacing of progress indicators
/// for different contexts and layout requirements:
#[derive(Clone, Copy, PartialEq)]
pub enum ProgressSize {
    /// Small progress bar for compact layouts
    ///
    /// Uses minimal height (h-1) with extra-small text (text-xs) for labels.
    /// Ideal for inline progress indicators, list items, or dense layouts.
    Small,

    /// Medium progress bar for standard use
    ///
    /// Uses standard height (h-2) with small text (text-sm) for labels.
    /// The default size for most progress bar applications.
    Medium,

    /// Large progress bar for prominent display
    ///
    /// Uses generous height (h-3) with base text size (text-base) for labels.
    /// Best for important progress indicators that need visual emphasis.
    Large,
}

impl Default for ProgressSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Properties for configuring the Progress component
#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// Current progress value within the specified range
    ///
    /// Can be any numeric value from 0.0 to the max value. Values outside
    /// the range are automatically clamped. Required parameter.
    pub value: f64,

    /// Maximum value representing 100% completion
    ///
    /// Defines the upper bound of the progress range. The progress percentage
    /// is calculated as (value / max) * 100. Must be greater than 0.
    /// Defaults to 100.0.
    #[props(default = 100.0)]
    pub max: f64,

    /// Size variant controlling bar height and label text size
    ///
    /// Affects the visual prominence and space requirements:
    /// - `Small`: Compact height (h-1) with small text (text-xs)
    /// - `Medium`: Standard height (h-2) with small text (text-sm)
    /// - `Large`: Prominent height (h-3) with base text (text-base)
    /// Defaults to `ProgressSize::Medium`.
    #[props(default)]
    pub size: ProgressSize,

    /// Whether to display the completion percentage
    ///
    /// When true, shows the calculated percentage with monospace formatting
    /// next to the label or on its own. Useful for providing precise
    /// completion information. Defaults to false.
    #[props(default = false)]
    pub show_percentage: bool,

    /// Optional label text for the progress bar
    ///
    /// Provides descriptive text that appears above the progress bar.
    /// Also used for accessibility as the ARIA label. When provided,
    /// creates a proper label association for screen readers.
    /// Defaults to None (no label).
    #[props(default = None)]
    pub label: Option<String>,

    /// Additional CSS classes to apply to the container
    ///
    /// Custom classes are applied to the root container element.
    /// Use for margins, width constraints, or other layout styling.
    /// Defaults to empty string.
    #[props(default = String::new())]
    pub class: String,

    /// HTML id attribute for the progress element
    ///
    /// Used for accessibility associations and DOM references.
    /// When provided with a label, creates proper label-progressbar
    /// associations for screen readers. Defaults to None.
    #[props(default = None)]
    pub id: Option<String>,
}

/// Helper function to get CSS classes for different progress sizes
///
/// Maps each size variant to appropriate track height and label text size
/// classes for consistent visual hierarchy and proportions.
///
/// # Parameters
///
/// - `size`: The ProgressSize variant to get classes for
///
/// # Returns
///
/// A tuple containing (track_height_class, label_text_size_class)
fn get_size_classes(size: ProgressSize) -> (&'static str, &'static str) {
    match size {
        ProgressSize::Small => (
            "h-1",     // track height
            "text-xs", // label text
        ),
        ProgressSize::Medium => (
            "h-2",     // track height
            "text-sm", // label text
        ),
        ProgressSize::Large => (
            "h-3",       // track height
            "text-base", // label text
        ),
    }
}

/// Progress component for displaying completion status and loading states
///
/// The Progress component provides a visual indicator for showing completion
/// percentages, loading progress, or advancement through multi-step processes.
/// It supports different sizes, optional labels, and percentage display.
#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let (track_height, label_text_size) = get_size_classes(props.size);

    let percentage = if props.max > 0.0 {
        ((props.value / props.max) * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };

    let track_classes =
        format!("relative w-full {track_height} rounded-full bg-secondary overflow-hidden");

    let filled_track_classes =
        "h-full rounded-full bg-primary transition-all duration-300 ease-out".to_string();

    let combined_classes = if props.class.is_empty() {
        "space-y-2".to_string()
    } else {
        format!("space-y-2 {}", props.class)
    };

    rsx! {
        div { class: combined_classes,
            if props.label.is_some() || props.show_percentage {
                div { class: "flex justify-between items-center",
                    if let Some(label) = &props.label {
                        label {
                            class: format!("font-medium text-foreground {}", label_text_size),
                            r#for: if props.id.is_some() { props.id.as_deref() } else { None },
                            "{label}"
                        }
                    }
                    if props.show_percentage {
                        span { class: format!("font-mono {} text-muted-foreground", label_text_size),
                            "{percentage:.0}%"
                        }
                    }
                }
            }
            div {
                class: track_classes,
                role: "progressbar",
                "aria-valuenow": "{props.value}",
                "aria-valuemax": "{props.max}",
                "aria-valuemin": "0",
                "aria-label": if props.label.is_some() { props.label.as_deref() } else { None },
                id: if props.id.is_some() { props.id.as_deref() } else { None },
                div {
                    class: filled_track_classes,
                    style: format!("width: {}%", percentage),
                }
            }
        }
    }
}
