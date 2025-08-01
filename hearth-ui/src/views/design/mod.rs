use crate::{
    use_toaster, Button, ButtonSize, ButtonVariant, FadeMode, Label, ScrollArea, Select,
    SelectOption, PageHeader, Platform,
};
use dioxus::prelude::*;

pub mod showcase;
pub use showcase::*;

pub mod showcases;
use showcases::*;

#[component]
pub fn Design() -> Element {
    let toaster = use_toaster();

    // Component selection state
    let mut selected_component = use_signal(|| "GestureDetector".to_string());

    // Available components
    let components = vec![
        SelectOption::new("Accordion", "Accordion"),
        SelectOption::new("AspectRatio", "AspectRatio"),
        SelectOption::new("Avatar", "Avatar"),
        SelectOption::new("Badge", "Badge"),
        SelectOption::new("Button", "Button"),
        SelectOption::new("Calendar", "Calendar"),
        SelectOption::new("Card", "Card"),
        SelectOption::new("Carousel", "Carousel"),
        SelectOption::new("Checkbox", "Checkbox"),
        SelectOption::new("Collapsible", "Collapsible"),
        SelectOption::new("GestureDetector", "GestureDetector"),
        SelectOption::new("ToggleIcon", "ToggleIcon"),
        SelectOption::new("Input", "Input"),
        SelectOption::new("InputOTP", "InputOTP"),
        SelectOption::new("Label", "Label"),
        SelectOption::new("Modal", "Modal"),
        SelectOption::new("Notice", "Notice"),
        SelectOption::new("Popover", "Popover"),
        SelectOption::new("Progress", "Progress"),
        SelectOption::new("Radio", "Radio"),
        SelectOption::new("RangeCalendar", "RangeCalendar"),
        SelectOption::new("ScrollArea", "ScrollArea"),
        SelectOption::new("Select", "Select"),
        SelectOption::new("Separator", "Separator"),
        SelectOption::new("Sheet", "Sheet"),
        SelectOption::new("Skeleton", "Skeleton"),
        SelectOption::new("Slider", "Slider"),
        SelectOption::new("Switch", "Switch"),
        SelectOption::new("Table", "Table"),
        SelectOption::new("Tabs", "Tabs"),
        SelectOption::new("Textarea", "Textarea"),
        SelectOption::new("Toggle", "Toggle"),
        SelectOption::new("ToggleGroup", "ToggleGroup"),
        SelectOption::new("Typography", "Typography"),
    ];

    // Navigation functions
    let navigate_previous = {
        let components = components.clone();
        move |_| {
            let current = selected_component.read().clone();
            if let Some(current_index) = components.iter().position(|c| c.value == current) {
                let prev_index = if current_index == 0 {
                    components.len() - 1
                } else {
                    current_index - 1
                };
                selected_component.set(components[prev_index].value.clone());
            }
        }
    };

    let navigate_next = {
        let components = components.clone();
        move |_| {
            let current = selected_component.read().clone();
            if let Some(current_index) = components.iter().position(|c| c.value == current) {
                let next_index = (current_index + 1) % components.len();
                selected_component.set(components[next_index].value.clone());
            }
        }
    };

    // Component state signals
    let default_input = use_signal(String::new);
    let filled_input = use_signal(String::new);
    let password_input = use_signal(String::new);
    let error_input = use_signal(String::new);
    let basic_otp = use_signal(String::new);
    let custom_length_otp = use_signal(String::new);
    let no_separator_otp = use_signal(String::new);
    let default_textarea = use_signal(String::new);
    let filled_textarea = use_signal(String::new);
    let outline_textarea = use_signal(String::new);
    let ghost_textarea = use_signal(String::new);
    let switch_1 = use_signal(|| false);
    let switch_2 = use_signal(|| true);
    let switch_3 = use_signal(|| false);
    let checkbox_1 = use_signal(|| false);
    let checkbox_2 = use_signal(|| true);
    let checkbox_3 = use_signal(|| false);
    let radio_theme = use_signal(|| "light".to_string());
    let radio_size = use_signal(|| "medium".to_string());
    let select_theme = use_signal(String::new);
    let select_size = use_signal(String::new);
    let select_framework = use_signal(String::new);
    let slider_volume = use_signal(|| 50.0);
    let slider_temperature = use_signal(|| 0.7);
    let slider_range = use_signal(|| 25.0);
    let slider_small = use_signal(|| 25.0);
    let slider_medium = use_signal(|| 50.0);
    let slider_large = use_signal(|| 75.0);
    let slider_normal = use_signal(|| 60.0);
    let animated_progress = use_signal(|| 0.0);
    let staggered_progress = use_signal(|| 0.0);

    // Animation for progress demo - cross-platform approach
    {
        let mut animated_progress = animated_progress;
        use_effect(move || {
            Platform::spawn(async move {
                loop {
                    // Animate from 0 to 100 over 3 seconds
                    for i in 0..=100 {
                        animated_progress.set(i as f64);
                        Platform::sleep(std::time::Duration::from_millis(30)).await;
                    }
                    // Reset to 0
                    animated_progress.set(0.0);
                    // Hold at 0 for 1 second before restarting
                    Platform::sleep(std::time::Duration::from_secs(1)).await;
                }
            });
        });
    }

    // Staggered progress animation - mimics real loading scenarios
    {
        let mut staggered_progress = staggered_progress;
        use_effect(move || {
            Platform::spawn(async move {
                let stages = [0.0, 20.0, 23.0, 64.0, 100.0];
                let delays = [1000, 800, 2000, 1500, 2000]; // Different delays for realistic feel

                loop {
                    for (i, &stage) in stages.iter().enumerate() {
                        staggered_progress.set(stage);

                        let delay_ms = delays[i];
                        Platform::sleep(std::time::Duration::from_millis(delay_ms)).await;
                    }
                }
            });
        });
    }

    rsx! {
        div { class: "flex-1 flex flex-col min-h-0",
            // PageHeader inside the flex container
            PageHeader { title: "Design System".to_string(), back_button: None }
            
            // Scrollable Content Area
            div { class: "flex-1 min-h-0",
                ScrollArea { height: "h-full".to_string(), fade_mode: FadeMode::Both,
                    div { class: "p-6 space-y-6",
                        // Component selector with navigation
                        div { class: "flex items-center space-x-4 pb-6 border-b border-border",
                            Label { r#for: "component-selector", "Component:" }
                            div { class: "flex items-center space-x-2",
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    size: ButtonSize::Small,
                                    onclick: navigate_previous,
                                    class: Some("px-2".to_string()),
                                    i { class: "fa-solid fa-chevron-left w-3 h-3" }
                                }
                                Select {
                                    id: "component-selector",
                                    options: components,
                                    placeholder: "Select a component",
                                    searchable: true,
                                    value: selected_component.read().clone(),
                                    onchange: move |value| {
                                        selected_component.set(value);
                                    },
                                }
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    size: ButtonSize::Small,
                                    onclick: navigate_next,
                                    class: Some("px-2".to_string()),
                                    i { class: "fa-solid fa-chevron-right w-3 h-3" }
                                }
                            }
                        }
                        // Component showcase content
                        match selected_component.read().as_str() {
                            "AspectRatio" => rsx! {
                                aspect_ratio_showcase {}
                            },

                            "Carousel" => rsx! {
                                carousel_showcase {}
                            },

                            "Button" => rsx! {


                                button_showcase { toaster }
                            },
                            "Calendar" => rsx! {
                                calendar_showcase { toaster }
                            },
                            "RangeCalendar" => rsx! {
                                range_calendar_showcase { toaster }
                            },
                            "Input" => rsx! {
                                input_showcase {
                                    default_input,
                                    filled_input,
                                    password_input,
                                    error_input,
                                }
                            },
                            "InputOTP" => rsx! {
                                input_otp_showcase { basic_otp, custom_length_otp, no_separator_otp }
                            },
                            "Textarea" => rsx! {
                                textarea_showcase {
                                    default_textarea,
                                    filled_textarea,
                                    outline_textarea,
                                    ghost_textarea,
                                }
                            },
                            "Separator" => rsx! {
                                separator_showcase {}
                            },
                            "ScrollArea" => rsx! {
                                scroll_area_showcase {}
                            },
                            "Label" => rsx! {
                                label_showcase {}
                            },
                            "Modal" => rsx! {
                                modal_showcase {}
                            },
                            "Notice" => rsx! {
                                notice_showcase {}
                            },
                            "Sheet" => rsx! {
                                sheet_showcase {}
                            },
                            "Switch" => rsx! {
                                switch_showcase { switch_1, switch_2, switch_3 }
                            },
                            "Checkbox" => rsx! {
                                checkbox_showcase { checkbox_1, checkbox_2, checkbox_3 }
                            },
                            "Collapsible" => rsx! {
                                collapsible_showcase {}
                            },
                            "GestureDetector" => rsx! {
                                GestureDetectorShowcase {}
                            },
                            "ToggleIcon" => rsx! {
                                toggle_icon_showcase { toaster }
                            },
                            "Radio" => rsx! {
                                radio_showcase { radio_theme, radio_size }
                            },
                            "Select" => rsx! {
                                select_showcase { select_theme, select_size, select_framework }
                            },
                            "Slider" => rsx! {
                                slider_showcase {
                                    slider_volume,
                                    slider_temperature,
                                    slider_range,
                                    slider_small,
                                    slider_medium,
                                    slider_large,
                                    slider_normal,
                                }
                            },
                            "Progress" => rsx! {
                                progress_showcase { animated_progress, staggered_progress }
                            },
                            "Card" => rsx! {
                                card_showcase {}
                            },
                            "Tabs" => rsx! {
                                tabs_showcase {}
                            },
                            "ToggleGroup" => rsx! {
                                toggle_group_showcase {}
                            },
                            "Toggle" => rsx! {
                                toggle_showcase {}
                            },
                            "Accordion" => rsx! {
                                accordion_showcase {}
                            },
                            "Avatar" => rsx! {
                                AvatarShowcase {}
                            },
                            "Badge" => rsx! {
                                BadgeShowcase {}
                            },
                            "Table" => rsx! {
                                TableShowcase {}
                            },
                            "Typography" => rsx! {
                                TypographyShowcase {}
                            },
                            "Skeleton" => rsx! {
                                skeleton_showcase {}
                            },
                            "Popover" => rsx! {
                                popover_showcase {}
                            },
                            _ => rsx! {
                                div { class: "text-center py-8 text-muted-foreground", "Select a component to view its showcase" }
                            },
                        }
                    }
                }
            }
        }
    }
}
