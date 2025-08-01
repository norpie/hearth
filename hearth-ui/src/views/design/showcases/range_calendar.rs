use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{CalendarSize, RangeCalendar, SimpleDate, ToastConfig, ToastManager, ToastType};
use dioxus::prelude::*;
use std::time::Duration;

#[derive(Props, Clone)]
pub struct RangeCalendarShowcaseProps {
    pub toaster: ToastManager,
}

impl PartialEq for RangeCalendarShowcaseProps {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[component]
pub fn range_calendar_showcase(props: RangeCalendarShowcaseProps) -> Element {
    let RangeCalendarShowcaseProps { toaster } = props;

    let mut selected_range = use_signal(|| None::<(SimpleDate, SimpleDate)>);
    let mut vacation_range = use_signal(|| None::<(SimpleDate, SimpleDate)>);
    let mut constrained_range = use_signal(|| None::<(SimpleDate, SimpleDate)>);

    rsx! {
        ComponentShowcase {
            name: "Range Calendar".to_string(),
            description: "Date range picker component for selecting start and end dates with visual range highlighting.".to_string(),
            basic_usage: r#"RangeCalendar {
    selected_range: Some((
        SimpleDate::new(2024, 8, 15),
        SimpleDate::new(2024, 8, 20)
    )),
    on_range_select: move |range| {
        // Handle range selection: (start_date, end_date)
    }
}
// Click to select start date, click again to select end date"#.to_string(),
            with_props_usage: r#"RangeCalendar {
    size: CalendarSize::Medium,
    selected_range: Some((start_date, end_date)),
    today: Some(SimpleDate::today()),
    disabled_dates: vec![SimpleDate::new(2024, 8, 10)],
    min_date: Some(SimpleDate::new(2024, 1, 1)),
    max_date: Some(SimpleDate::new(2024, 12, 31)),
    show_other_months: true,
    show_week_numbers: true,
    on_range_select: move |range| {
        // Handle range selection
    }
}"#.to_string(),

            ShowcaseVariant {
                title: "Basic Range Selection".to_string(),
                div { class: "flex justify-center",
                    RangeCalendar {
                        selected_range: selected_range.read().clone(),
                        on_range_select: move |range: (SimpleDate, SimpleDate)| {
                            selected_range.set(Some(range.clone()));
                            toaster.add_toast(ToastConfig {
                                message: format!("Selected range: {} to {}", range.0.format(), range.1.format()),
                                toast_type: ToastType::Success,
                                duration: Some(Duration::from_millis(3000)),
                                dismissible: true,
                            });
                        }
                    }
                }
                if let Some((start, end)) = selected_range.read().as_ref() {
                    div { class: "mt-4 text-center space-y-1",
                        p { class: "text-sm font-medium text-foreground",
                            "Selected Range"
                        }
                        p { class: "text-sm text-foreground",
                            "Start: {start.format()}"
                        }
                        p { class: "text-sm text-foreground",
                            "End: {end.format()}"
                        }
                        p { class: "text-xs text-primary-foreground mt-2",
                            {
                                let days = if end.year == start.year && end.month == start.month {
                                    (end.day as i32 - start.day as i32 + 1).abs()
                                } else {
                                    // Simplified calculation for demo
                                    let start_days = start.day as i32;
                                    let end_days = end.day as i32;
                                    let month_diff = (end.month as i32 - start.month as i32).abs();
                                    if month_diff == 1 {
                                        (30 - start_days) + end_days + 1
                                    } else {
                                        (end_days - start_days + 1).abs()
                                    }
                                };
                                format!("{} day{}", days, if days == 1 { "" } else { "s" })
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Sizes".to_string(),
                div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                    div { class: "flex flex-col items-center space-y-2",
                        h4 { class: "text-sm font-medium text-foreground", "Small" }
                        RangeCalendar {
                            size: CalendarSize::Small,
                            selected_range: Some((
                                SimpleDate::new(2024, 8, 10),
                                SimpleDate::new(2024, 8, 15)
                            )),
                        }
                    }

                    div { class: "flex flex-col items-center space-y-2",
                        h4 { class: "text-sm font-medium text-foreground", "Medium" }
                        RangeCalendar {
                            size: CalendarSize::Medium,
                            selected_range: Some((
                                SimpleDate::new(2024, 8, 5),
                                SimpleDate::new(2024, 8, 12)
                            )),
                        }
                    }

                    div { class: "flex flex-col items-center space-y-2",
                        h4 { class: "text-sm font-medium text-foreground", "Large" }
                        RangeCalendar {
                            size: CalendarSize::Large,
                            selected_range: Some((
                                SimpleDate::new(2024, 8, 20),
                                SimpleDate::new(2024, 8, 25)
                            )),
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Vacation Planner".to_string(),
                div { class: "flex justify-center",
                    RangeCalendar {
                        today: Some(SimpleDate::new(2024, 8, 15)),
                        selected_range: vacation_range.read().clone(),
                        min_date: Some(SimpleDate::new(2024, 8, 15)), // Can't book past dates
                        disabled_dates: vec![
                            // Weekend blackouts for this example
                            SimpleDate::new(2024, 8, 17),
                            SimpleDate::new(2024, 8, 18),
                            SimpleDate::new(2024, 8, 24),
                            SimpleDate::new(2024, 8, 25),
                        ],
                        on_range_select: move |range: (SimpleDate, SimpleDate)| {
                            vacation_range.set(Some(range.clone()));
                            toaster.add_toast(ToastConfig {
                                message: format!("Vacation booked: {} to {}", range.0.format(), range.1.format()),
                                toast_type: ToastType::Success,
                                duration: Some(Duration::from_millis(4000)),
                                dismissible: true,
                            });
                        }
                    }
                }
                p { class: "mt-4 text-sm text-foreground text-center",
                    "📅 Plan your vacation! Some weekends are blocked (dark dates)."
                }
                if let Some((start, end)) = vacation_range.read().as_ref() {
                    div { class: "mt-4 p-4 bg-success/20 border border-success/20 rounded-lg",
                        div { class: "text-center space-y-2",
                            p { class: "text-sm font-medium text-success-foreground",
                                "🏖️ Vacation Scheduled!"
                            }
                            p { class: "text-sm text-success-foreground",
                                "{start.format()} to {end.format()}"
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Event Duration Selector".to_string(),
                div { class: "flex justify-center",
                    RangeCalendar {
                        size: CalendarSize::Medium,
                        selected_range: constrained_range.read().clone(),
                        min_date: Some(SimpleDate::new(2024, 8, 1)),
                        max_date: Some(SimpleDate::new(2024, 8, 31)),
                        today: Some(SimpleDate::new(2024, 8, 15)),
                        on_range_select: move |range: (SimpleDate, SimpleDate)| {
                            constrained_range.set(Some(range.clone()));
                            toaster.add_toast(ToastConfig {
                                message: format!("Event duration: {} to {}", range.0.format(), range.1.format()),
                                toast_type: ToastType::Info,
                                duration: Some(Duration::from_millis(3000)),
                                dismissible: true,
                            });
                        }
                    }
                }
                p { class: "mt-4 text-sm text-foreground text-center",
                    "Select event start and end dates (limited to August 2024)"
                }
                if let Some((start, end)) = constrained_range.read().as_ref() {
                    div { class: "mt-4 p-3 bg-primary/20 border border-primary rounded-lg text-center",
                        p { class: "text-sm font-medium text-primary-foreground",
                            "📅 Event: {start.format()} → {end.format()}"
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Quick Range Examples".to_string(),
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    div { class: "flex flex-col items-center space-y-2",
                        h4 { class: "text-sm font-medium text-foreground", "Weekend Range" }
                        RangeCalendar {
                            size: CalendarSize::Small,
                            selected_range: Some((
                                SimpleDate::new(2024, 8, 17), // Saturday
                                SimpleDate::new(2024, 8, 18)  // Sunday
                            )),
                            class: Some("border-accent/20".to_string()),
                        }
                        p { class: "text-xs text-accent-foreground text-center",
                            "Perfect for weekend planning"
                        }
                    }

                    div { class: "flex flex-col items-center space-y-2",
                        h4 { class: "text-sm font-medium text-foreground", "Work Week" }
                        RangeCalendar {
                            size: CalendarSize::Small,
                            selected_range: Some((
                                SimpleDate::new(2024, 8, 19), // Monday
                                SimpleDate::new(2024, 8, 23)  // Friday
                            )),
                            class: Some("border-warning/20".to_string()),
                        }
                        p { class: "text-xs text-warning-foreground text-center",
                            "Monday to Friday selection"
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Year & Decade Navigation".to_string(),
                div { class: "flex justify-center",
                    RangeCalendar {
                        today: Some(SimpleDate::new(2020, 1, 1)),
                        size: CalendarSize::Medium,
                        on_range_select: move |range: (SimpleDate, SimpleDate)| {
                            toaster.add_toast(ToastConfig {
                                message: format!("Historical range: {} to {}", range.0.format(), range.1.format()),
                                toast_type: ToastType::Success,
                                duration: Some(Duration::from_millis(3000)),
                                dismissible: true,
                            });
                        }
                    }
                }
                div { class: "mt-4 space-y-2 text-sm text-foreground",
                    p { class: "text-center font-medium", "🗓️ Perfect for historical date ranges!" }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 text-xs",
                        div { class: "text-center p-2 bg-muted rounded",
                            div { class: "font-medium text-primary-foreground", "Month View" }
                            div { "Select start & end dates" }
                        }
                        div { class: "text-center p-2 bg-muted rounded",
                            div { class: "font-medium text-primary-foreground", "Year View" }
                            div { "Click header → jump months" }
                        }
                        div { class: "text-center p-2 bg-muted rounded",
                            div { class: "font-medium text-primary-foreground", "Decade View" }
                            div { "Click again → jump years" }
                        }
                    }
                }
            }
        }
    }
}
