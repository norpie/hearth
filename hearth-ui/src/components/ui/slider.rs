//! Range slider component for numeric value selection with accessibility
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     Slider {
//!         value: volume(),
//!         min: 0.0,
//!         max: 100.0,
//!         onchange: move |val| volume.set(val),
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Size variants controlling slider dimensions and visual scale
#[derive(Clone, Copy, PartialEq)]
pub enum SliderSize {
    /// Small slider size for compact layouts
    Small,
    /// Medium slider size (default) for standard use
    Medium,
    /// Large slider size for prominent controls
    Large,
}

impl Default for SliderSize {
    fn default() -> Self {
        Self::Medium
    }
}

/// Properties for the Slider component
#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    /// Current value of the slider
    pub value: f64,
    /// Minimum selectable value
    #[props(default = 0.0)]
    pub min: f64,
    /// Maximum selectable value
    #[props(default = 100.0)]
    pub max: f64,
    /// Step increment for value changes
    #[props(default = 1.0)]
    pub step: f64,
    /// Size variant of the slider
    #[props(default)]
    pub size: SliderSize,
    #[props(default = false)]
    pub disabled: bool,
    /// Whether to display the current value
    #[props(default = false)]
    pub show_value: bool,
    /// Optional label text for the slider
    #[props(default = None)]
    pub label: Option<String>,
    /// Callback function called when the value changes
    pub onchange: EventHandler<f64>,
    /// Additional CSS classes to apply to the container
    #[props(default = String::new())]
    pub class: String,
    /// HTML id attribute for accessibility and form association
    #[props(default = None)]
    pub id: Option<String>,
}

/// Returns CSS classes for slider components based on size variant
///
/// Maps each size variant to appropriate styling classes for the track height,
/// thumb dimensions, and label text size. Ensures consistent proportions
/// across all slider size variants.
///
/// # Returns
///
/// A tuple of (track_height, thumb_size, label_text_size) as static string references.
fn get_size_classes(size: SliderSize) -> (&'static str, &'static str, &'static str) {
    match size {
        SliderSize::Small => (
            "h-1",
            "w-3 h-3",
            "text-xs",
        ),
        SliderSize::Medium => (
            "h-2",
            "w-4 h-4",
            "text-sm",
        ),
        SliderSize::Large => (
            "h-3",
            "w-5 h-5",
            "text-base",
        ),
    }
}

/// Interactive range slider for numeric value selection with accessibility
#[component]
pub fn Slider(props: SliderProps) -> Element {
    let (track_height, thumb_size, label_text_size) = get_size_classes(props.size);

    let percentage = if props.max > props.min {
        ((props.value - props.min) / (props.max - props.min) * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };

    let track_classes = format!(
        "relative w-full {} rounded-full bg-secondary {}",
        track_height,
        if props.disabled {
            "opacity-50 cursor-not-allowed"
        } else {
            "cursor-pointer"
        }
    );

    let filled_track_classes = format!(
        "absolute top-0 left-0 {track_height} rounded-full bg-primary transition-all duration-[30ms]"
    );

    let thumb_classes = format!(
        "absolute top-1/2 {} -translate-y-1/2 -translate-x-1/2 rounded-full bg-background border-2 border-primary shadow-md transition-all duration-[30ms] {}",
        thumb_size,
        if props.disabled {
            "cursor-not-allowed"
        } else {
            "cursor-grab hover:scale-110 focus:scale-110 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background"
        }
    );

    let combined_classes = if props.class.is_empty() {
        "space-y-2".to_string()
    } else {
        format!("space-y-2 {}", props.class)
    };

    let handle_input = move |evt: Event<FormData>| {
        if let Ok(new_value) = evt.value().parse::<f64>() {
            let clamped_value = new_value.max(props.min).min(props.max);
            props.onchange.call(clamped_value);
        }
    };

    rsx! {
        div { class: combined_classes,
            if props.label.is_some() || props.show_value {
                div { class: "flex justify-between items-center",
                    if let Some(label) = &props.label {
                        label {
                            class: format!("font-medium text-foreground {}", label_text_size),
                            r#for: if props.id.is_some() { props.id.as_deref() } else { None },
                            "{label}"
                        }
                    }
                    if props.show_value {
                        span { class: format!("font-mono {} text-muted-foreground", label_text_size),
                            "{props.value:.1}"
                        }
                    }
                }
            }
            div { class: "relative",
                input {
                    r#type: "range",
                    class: "absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10",
                    min: "{props.min}",
                    max: "{props.max}",
                    step: "{props.step}",
                    value: "{props.value}",
                    disabled: props.disabled,
                    oninput: handle_input,
                    id: if props.id.is_some() { props.id.as_deref() } else { None },
                }
                div { class: track_classes,
                    div {
                        class: filled_track_classes,
                        style: format!("width: {}%", percentage),
                    }
                    div {
                        class: thumb_classes,
                        style: format!("left: {}%", percentage),
                        tabindex: if props.disabled { "-1" } else { "0" },
                        role: "slider",
                        "aria-valuemin": "{props.min}",
                        "aria-valuemax": "{props.max}",
                        "aria-valuenow": "{props.value}",
                        "aria-label": if props.label.is_some() { props.label.as_deref() } else { None },
                        onkeydown: move |evt| {
                            if !props.disabled {
                                let key = evt.data().key();
                                let new_value = match key {
                                    Key::ArrowLeft | Key::ArrowDown => {
                                        (props.value - props.step).max(props.min)
                                    }
                                    Key::ArrowRight | Key::ArrowUp => {
                                        (props.value + props.step).min(props.max)
                                    }
                                    Key::Home => {
                                        props.min
                                    }
                                    Key::End => {
                                        props.max
                                    }
                                    _ => return,
                                };

                                if new_value != props.value {
                                    props.onchange.call(new_value);
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}
