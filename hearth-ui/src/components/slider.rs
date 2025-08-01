//! Slider component for selecting values within a range

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum SliderSize {
    Small,
    Medium,
    Large,
}

impl Default for SliderSize {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    /// Current value
    pub value: f64,
    /// Minimum value (default: 0.0)
    #[props(default = 0.0)]
    pub min: f64,
    /// Maximum value (default: 100.0)
    #[props(default = 100.0)]
    pub max: f64,
    /// Step increment (default: 1.0)
    #[props(default = 1.0)]
    pub step: f64,
    /// Size variant
    #[props(default)]
    pub size: SliderSize,
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
    /// Show value label
    #[props(default = false)]
    pub show_value: bool,
    /// Custom label text
    #[props(default = None)]
    pub label: Option<String>,
    /// Callback when value changes
    pub onchange: EventHandler<f64>,
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
    /// HTML id attribute
    #[props(default = None)]
    pub id: Option<String>,
}

fn get_size_classes(size: SliderSize) -> (&'static str, &'static str, &'static str) {
    match size {
        SliderSize::Small => (
            "h-1",           // track height
            "w-3 h-3",       // thumb size
            "text-xs",       // label text
        ),
        SliderSize::Medium => (
            "h-2",           // track height
            "w-4 h-4",       // thumb size  
            "text-sm",       // label text
        ),
        SliderSize::Large => (
            "h-3",           // track height
            "w-5 h-5",       // thumb size
            "text-base",     // label text
        ),
    }
}

/// Slider component for selecting values within a range
#[component]
pub fn Slider(props: SliderProps) -> Element {
    let (track_height, thumb_size, label_text_size) = get_size_classes(props.size);
    
    // Calculate percentage for positioning
    let percentage = if props.max > props.min {
        ((props.value - props.min) / (props.max - props.min) * 100.0).max(0.0).min(100.0)
    } else {
        0.0
    };

    let track_classes = format!(
        "relative w-full {} rounded-full bg-gray-200 dark:bg-gray-700 {}",
        track_height,
        if props.disabled {
            "opacity-50 cursor-not-allowed"
        } else {
            "cursor-pointer"
        }
    );

    let filled_track_classes = format!(
        "absolute top-0 left-0 {} rounded-full bg-blue-600 dark:bg-blue-500 transition-all duration-[30ms]",
        track_height
    );

    let thumb_classes = format!(
        "absolute top-1/2 {} -translate-y-1/2 -translate-x-1/2 rounded-full bg-white border-2 border-blue-600 dark:border-blue-500 shadow-md transition-all duration-[30ms] {}",
        thumb_size,
        if props.disabled {
            "cursor-not-allowed"
        } else {
            "cursor-grab hover:scale-110 focus:scale-110 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-900"
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
            // Label and value display
            if props.label.is_some() || props.show_value {
                div { class: "flex justify-between items-center",
                    if let Some(label) = &props.label {
                        label {
                            class: format!("font-medium text-gray-700 dark:text-gray-300 {}", label_text_size),
                            r#for: if props.id.is_some() { props.id.as_deref() } else { None },
                            "{label}"
                        }
                    }
                    if props.show_value {
                        span {
                            class: format!("font-mono {} text-gray-600 dark:text-gray-400", label_text_size),
                            "{props.value:.1}"
                        }
                    }
                }
            }
            
            // Slider container
            div { class: "relative",
                // Hidden native range input for form handling and accessibility
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
                
                // Custom slider track
                div {
                    class: track_classes,
                    
                    // Filled portion of track
                    div {
                        class: filled_track_classes,
                        style: format!("width: {}%", percentage),
                    }
                    
                    // Slider thumb
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
                                let mut new_value = props.value;
                                
                                match key {
                                    Key::ArrowLeft | Key::ArrowDown => {
                                        new_value = (props.value - props.step).max(props.min);
                                    }
                                    Key::ArrowRight | Key::ArrowUp => {
                                        new_value = (props.value + props.step).min(props.max);
                                    }
                                    Key::Home => {
                                        new_value = props.min;
                                    }
                                    Key::End => {
                                        new_value = props.max;
                                    }
                                    _ => return,
                                }
                                
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