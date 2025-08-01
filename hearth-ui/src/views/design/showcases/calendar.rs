use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Calendar, CalendarSize, SimpleDate, ToastConfig, ToastManager, ToastType};
use dioxus::prelude::*;
use std::time::Duration;

#[derive(Props, Clone)]
pub struct CalendarShowcaseProps {
    pub toaster: ToastManager,
}

impl PartialEq for CalendarShowcaseProps {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[component]
pub fn calendar_showcase(props: CalendarShowcaseProps) -> Element {
    let CalendarShowcaseProps { toaster } = props;

    let mut selected_date = use_signal(|| None::<SimpleDate>);
    let mut selected_date_with_constraints = use_signal(|| None::<SimpleDate>);
    let mut selected_date_small = use_signal(|| None::<SimpleDate>);
    let mut selected_date_large = use_signal(|| None::<SimpleDate>);

    rsx! {
        ComponentShowcase {
            name: "Calendar".to_string(),
            description: "Date picker component with month, year, and decade navigation for easy date selection.".to_string(),
            basic_usage: r#"Calendar {
    selected_date: Some(SimpleDate::new(2024, 8, 15)),
    on_date_select: move |date| {
        // Handle date selection
    }
}
// Click header to switch between Month → Year → Decade views"#.to_string(),
            with_props_usage: r#"Calendar {
    size: CalendarSize::Medium,
    selected_date: Some(SimpleDate::new(2024, 8, 15)),
    today: Some(SimpleDate::today()),
    disabled_dates: vec![SimpleDate::new(2024, 8, 10)],
    min_date: Some(SimpleDate::new(2024, 1, 1)),
    max_date: Some(SimpleDate::new(2024, 12, 31)),
    show_other_months: true,
    show_week_numbers: true,
    on_date_select: move |date| {
        // Handle date selection
    }
}"#.to_string(),

            ShowcaseVariant {
                title: "Basic Calendar".to_string(),
                div { class: "flex justify-center",
                    Calendar {
                        selected_date: selected_date.read().clone(),
                        on_date_select: move |date: SimpleDate| {
                            selected_date.set(Some(date.clone()));
                            toaster.add_toast(ToastConfig {
                                message: format!("Selected date: {}", date.format()),
                                toast_type: ToastType::Success,
                                duration: Some(Duration::from_millis(2000)),
                                dismissible: true,
                            });
                        }
                    }
                }
                if let Some(date) = selected_date.read().as_ref() {
                    p { class: "mt-4 text-sm text-foreground text-center",
                        "Selected: {date.format()}"
                    }
                }
            }

            ShowcaseVariant {
                title: "Sizes".to_string(),
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                    div { class: "flex flex-col items-center space-y-2",
                        h4 { class: "text-sm font-medium text-foreground", "Small" }
                        Calendar {
                            size: CalendarSize::Small,
                            selected_date: selected_date_small.read().clone(),
                            on_date_select: move |date: SimpleDate| {
                                selected_date_small.set(Some(date.clone()));
                                toaster.add_toast(ToastConfig {
                                    message: format!("Small calendar: {}", date.format()),
                                    toast_type: ToastType::Info,
                                    duration: Some(Duration::from_millis(2000)),
                                    dismissible: true,
                                });
                            }
                        }
                    }

                    div { class: "flex flex-col items-center space-y-2",
                        h4 { class: "text-sm font-medium text-foreground", "Medium" }
                        Calendar {
                            size: CalendarSize::Medium,
                        }
                    }

                    div { class: "flex flex-col items-center space-y-2",
                        h4 { class: "text-sm font-medium text-foreground", "Large" }
                        Calendar {
                            size: CalendarSize::Large,
                            selected_date: selected_date_large.read().clone(),
                            on_date_select: move |date: SimpleDate| {
                                selected_date_large.set(Some(date.clone()));
                                toaster.add_toast(ToastConfig {
                                    message: format!("Large calendar: {}", date.format()),
                                    toast_type: ToastType::Info,
                                    duration: Some(Duration::from_millis(2000)),
                                    dismissible: true,
                                });
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "With Constraints".to_string(),
                div { class: "flex justify-center",
                    Calendar {
                        selected_date: selected_date_with_constraints.read().clone(),
                        min_date: Some(SimpleDate::new(2024, 8, 1)),
                        max_date: Some(SimpleDate::new(2024, 8, 25)),
                        disabled_dates: vec![
                            SimpleDate::new(2024, 8, 5),
                            SimpleDate::new(2024, 8, 10),
                            SimpleDate::new(2024, 8, 15),
                        ],
                        on_date_select: move |date: SimpleDate| {
                            selected_date_with_constraints.set(Some(date.clone()));
                            toaster.add_toast(ToastConfig {
                                message: format!("Selected constrained date: {}", date.format()),
                                toast_type: ToastType::Success,
                                duration: Some(Duration::from_millis(2000)),
                                dismissible: true,
                            });
                        }
                    }
                }
                p { class: "mt-4 text-sm text-foreground text-center",
                    "Limited to August 1-25, 2024. Days 5, 10, and 15 are disabled."
                }
                if let Some(date) = selected_date_with_constraints.read().as_ref() {
                    p { class: "mt-2 text-sm text-foreground text-center",
                        "Selected: {date.format()}"
                    }
                }
            }

            ShowcaseVariant {
                title: "Custom Styling".to_string(),
                div { class: "flex justify-center",
                    Calendar {
                        class: Some("border-2 border-primary shadow-lg".to_string()),
                        show_other_months: false,
                        today: Some(SimpleDate::new(2024, 8, 20)),
                        selected_date: Some(SimpleDate::new(2024, 8, 15)),
                    }
                }
                p { class: "mt-4 text-sm text-foreground text-center",
                    "Custom border styling with other months hidden"
                }
            }

            ShowcaseVariant {
                title: "Year & Decade Navigation".to_string(),
                div { class: "flex justify-center",
                    Calendar {
                        today: Some(SimpleDate::new(1995, 6, 15)), // Birthday example
                        size: CalendarSize::Medium,
                        on_date_select: move |date: SimpleDate| {
                            toaster.add_toast(ToastConfig {
                                message: format!("Selected birthday: {}", date.format()),
                                toast_type: ToastType::Success,
                                duration: Some(Duration::from_millis(3000)),
                                dismissible: true,
                            });
                        }
                    }
                }
                div { class: "mt-4 space-y-2 text-sm text-foreground",
                    p { class: "text-center font-medium", "📅 Perfect for birthdays & historical dates!" }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 text-xs",
                        div { class: "text-center p-2 bg-muted rounded",
                            div { class: "font-medium text-primary-foreground", "Month View" }
                            div { "Navigate days within a month" }
                        }
                        div { class: "text-center p-2 bg-muted rounded",
                            div { class: "font-medium text-primary-foreground", "Year View" }
                            div { "Click header → select months quickly" }
                        }
                        div { class: "text-center p-2 bg-muted rounded",
                            div { class: "font-medium text-primary-foreground", "Decade View" }
                            div { "Click again → jump years rapidly" }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Multiple Calendars".to_string(),
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    div { class: "flex flex-col items-center space-y-2",
                        h4 { class: "text-sm font-medium text-foreground", "August 2024" }
                        Calendar {
                            today: Some(SimpleDate::new(2024, 8, 15)),
                        }
                    }

                    div { class: "flex flex-col items-center space-y-2",
                        h4 { class: "text-sm font-medium text-foreground", "September 2024" }
                        Calendar {
                            today: Some(SimpleDate::new(2024, 9, 1)),
                        }
                    }
                }
                p { class: "mt-4 text-sm text-foreground text-center",
                    "Two calendars showing different months"
                }
            }
        }
    }
}
