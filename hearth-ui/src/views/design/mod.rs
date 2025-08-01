use dioxus::prelude::*;
use crate::{Button, ButtonVariant, ButtonSize, Input, InputVariant, InputSize, InputType, InputOTP, Label, Switch, SwitchSize, Checkbox, CheckboxSize, Radio, RadioGroup, RadioSize, RadioDirection, Select, SelectOption, SelectSize, SelectVariant, Slider, SliderSize, Progress, ProgressSize, Textarea, TextareaVariant, TextareaSize, TextareaResize, Separator, SeparatorOrientation, SeparatorSize, SeparatorVariant, ScrollArea, ScrollOrientation, FadeMode, ScrollbarVisibility, Card, CardVariant, CardSize, CardHeader, CardTitle, CardDescription, CardContent, CardFooter, Tabs, TabsVariant, TabsSize, TabsList, TabsTrigger, TabsContent, Avatar, Badge, BadgeVariant, BadgeSize, Table, TableHeader, TableBody, TableRow, TableHead, TableCell, TableVariant, TableSize, Collapsible, Accordion, Toggle, ToggleGroup, Modal, ModalSize, Sheet, SheetSide, SheetSize, AspectRatio, Carousel, Popover, Tooltip, Calendar, CalendarSize, RangeCalendar, SimpleDate, use_toaster, Platform};

pub mod showcase;
pub use showcase::*;

pub mod showcases;
use showcases::*;

#[component]
pub fn Design() -> Element {
    let toaster = use_toaster();
    let platform = Platform::current();
    
    // Component selection state
    let mut selected_component = use_signal(|| "Tooltip".to_string());
    
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
        SelectOption::new("Input", "Input"),
        SelectOption::new("InputOTP", "InputOTP"),
        SelectOption::new("Label", "Label"),
        SelectOption::new("Modal", "Modal"),
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
        SelectOption::new("Tooltip", "Tooltip"),
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
    let mut default_input = use_signal(|| String::new());
    let mut filled_input = use_signal(|| String::new());
    let mut password_input = use_signal(|| String::new());
    let mut error_input = use_signal(|| String::new());
    let mut basic_otp = use_signal(|| String::new());
    let mut custom_length_otp = use_signal(|| String::new());
    let mut no_separator_otp = use_signal(|| String::new());
    let mut default_textarea = use_signal(|| String::new());
    let mut filled_textarea = use_signal(|| String::new());
    let mut outline_textarea = use_signal(|| String::new());
    let mut ghost_textarea = use_signal(|| String::new());
    let mut switch_1 = use_signal(|| false);
    let mut switch_2 = use_signal(|| true);
    let mut switch_3 = use_signal(|| false);
    let mut checkbox_1 = use_signal(|| false);
    let mut checkbox_2 = use_signal(|| true);
    let mut checkbox_3 = use_signal(|| false);
    let mut radio_theme = use_signal(|| "light".to_string());
    let mut radio_size = use_signal(|| "medium".to_string());
    let mut select_theme = use_signal(|| String::new());
    let mut select_size = use_signal(|| String::new());
    let mut select_framework = use_signal(|| String::new());
    let mut slider_volume = use_signal(|| 50.0);
    let mut slider_temperature = use_signal(|| 0.7);
    let mut slider_range = use_signal(|| 25.0);
    let mut slider_small = use_signal(|| 25.0);
    let mut slider_medium = use_signal(|| 50.0);
    let mut slider_large = use_signal(|| 75.0);
    let mut slider_normal = use_signal(|| 60.0);
    let animated_progress = use_signal(|| 0.0);
    let staggered_progress = use_signal(|| 0.0);

    // Animation for progress demo - cross-platform approach
    {
        let mut animated_progress = animated_progress.clone();
        use_effect(move || {
            spawn(async move {
                loop {
                    // Animate from 0 to 100 over 3 seconds
                    for i in 0..=100 {
                        animated_progress.set(i as f64);
                        #[cfg(target_arch = "wasm32")]
                        {
                            gloo_timers::future::sleep(std::time::Duration::from_millis(30)).await;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            tokio::time::sleep(std::time::Duration::from_millis(30)).await;
                        }
                    }
                    // Reset to 0
                    animated_progress.set(0.0);
                    // Hold at 0 for 1 second before restarting
                    #[cfg(target_arch = "wasm32")]
                    {
                        gloo_timers::future::sleep(std::time::Duration::from_secs(1)).await;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    }
                }
            });
        });
    }

    // Staggered progress animation - mimics real loading scenarios
    {
        let mut staggered_progress = staggered_progress.clone();
        use_effect(move || {
            spawn(async move {
                let stages = vec![0.0, 20.0, 23.0, 64.0, 100.0];
                let delays = vec![1000, 800, 2000, 1500, 2000]; // Different delays for realistic feel
                
                loop {
                    for (i, &stage) in stages.iter().enumerate() {
                        staggered_progress.set(stage);
                        
                        let delay_ms = delays[i];
                        #[cfg(target_arch = "wasm32")]
                        {
                            gloo_timers::future::sleep(std::time::Duration::from_millis(delay_ms)).await;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                        }
                    }
                }
            });
        });
    }

    rsx! {
        div {
            class: "flex-1 flex flex-col min-h-0",
            
            // Fixed Header
            div {
                class: "p-6 pb-6",
                div {
                    class: "border-b border-gray-200 dark:border-gray-700 pb-6 space-y-4",
                    h1 {
                        class: "text-2xl font-bold text-gray-900 dark:text-gray-100",
                        "Design System"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-400",
                        "Component library and design system showcase"
                    }
                    
                    // Component selector with navigation
                    div { class: "flex items-center space-x-4",
                        Label {
                            r#for: "component-selector",
                            "Component:"
                        }
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
                                }
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
                }
            }
            
            // Scrollable Content Area
            div {
                class: "flex-1 min-h-0",
                ScrollArea {
                    height: "100%".to_string(),
                    fade_mode: FadeMode::Both,
                div {
                    class: "p-6 pt-0",
                    
                    // Component showcase content
                match selected_component.read().as_str() {
                    "AspectRatio" => rsx! {
                        aspect_ratio_showcase {}
                    },
                    
                    "Carousel" => rsx! {
                        carousel_showcase {}
                    },
                    
                    "Button" => rsx! {
                        button_showcase { toaster: toaster }
                    },
                    
                    "Calendar" => rsx! {
                        calendar_showcase { toaster: toaster }
                    },
                    
                    "RangeCalendar" => rsx! {
                        range_calendar_showcase { toaster: toaster }
                    },
                    
                    "Input" => rsx! {
                        input_showcase { 
                            default_input: default_input,
                            filled_input: filled_input,
                            password_input: password_input,
                            error_input: error_input
                        }
                    },
                    
                    "InputOTP" => rsx! {
                        input_otp_showcase { 
                            basic_otp: basic_otp,
                            custom_length_otp: custom_length_otp,
                            no_separator_otp: no_separator_otp
                        }
                    },
                    
                    "Textarea" => rsx! {
                        textarea_showcase { 
                            default_textarea: default_textarea,
                            filled_textarea: filled_textarea,
                            outline_textarea: outline_textarea,
                            ghost_textarea: ghost_textarea
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
                    
                    "Sheet" => rsx! {
                        sheet_showcase {}
                    },
                    
                    "Switch" => rsx! {
                        switch_showcase { 
                            switch_1: switch_1,
                            switch_2: switch_2,
                            switch_3: switch_3
                        }
                    },
                    
                    "Checkbox" => rsx! {
                        checkbox_showcase { 
                            checkbox_1: checkbox_1,
                            checkbox_2: checkbox_2,
                            checkbox_3: checkbox_3
                        }
                    },
                    
                    "Collapsible" => rsx! {
                        collapsible_showcase {}
                    },
                    
                    "Radio" => rsx! {
                        radio_showcase { 
                            radio_theme: radio_theme,
                            radio_size: radio_size
                        }
                    },
                    
                    "Select" => rsx! {
                        select_showcase { 
                            select_theme: select_theme,
                            select_size: select_size,
                            select_framework: select_framework
                        }
                    },
                    
                    "Slider" => rsx! {
                        slider_showcase { 
                            slider_volume: slider_volume,
                            slider_temperature: slider_temperature,
                            slider_range: slider_range,
                            slider_small: slider_small,
                            slider_medium: slider_medium,
                            slider_large: slider_large,
                            slider_normal: slider_normal
                        }
                    },
                    
                    "Progress" => rsx! {
                        progress_showcase { 
                            animated_progress: animated_progress,
                            staggered_progress: staggered_progress
                        }
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
                    
                    "Tooltip" => rsx! {
                        tooltip_showcase {}
                    },
                    
                    _ => rsx! {
                        div { class: "text-center py-8 text-gray-500",
                            "Select a component to view its showcase"
                        }
                    }
                }
            }
            }
            }
        }
    }
}
