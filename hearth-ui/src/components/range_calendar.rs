//! Range Calendar component for date range selection

use dioxus::prelude::*;
use crate::{SimpleDate, CalendarView, CalendarSize, Weekday};

/// Range selection state
#[derive(Clone, PartialEq, Debug)]
pub enum RangeState {
    /// No date selected yet
    None,
    /// Start date selected, waiting for end date
    StartSelected(SimpleDate),
    /// Complete range selected
    Complete(SimpleDate, SimpleDate),
}

impl RangeState {
    /// Get the current range if complete
    pub fn range(&self) -> Option<(SimpleDate, SimpleDate)> {
        match self {
            RangeState::Complete(start, end) => Some((start.clone(), end.clone())),
            _ => None,
        }
    }
    
    /// Check if a date is in the selected range
    pub fn contains_date(&self, date: &SimpleDate) -> bool {
        match self {
            RangeState::Complete(start, end) => {
                let (range_start, range_end) = if start <= end { (start, end) } else { (end, start) };
                date >= range_start && date <= range_end
            }
            _ => false,
        }
    }
    
    /// Check if a date is the start of the range
    pub fn is_range_start(&self, date: &SimpleDate) -> bool {
        match self {
            RangeState::StartSelected(start) | RangeState::Complete(start, _) => start == date,
            _ => false,
        }
    }
    
    /// Check if a date is the end of the range
    pub fn is_range_end(&self, date: &SimpleDate) -> bool {
        match self {
            RangeState::Complete(_, end) => end == date,
            _ => false,
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct RangeCalendarProps {
    #[props(default = CalendarSize::Medium)]
    pub size: CalendarSize,
    #[props(default)]
    pub selected_range: Option<(SimpleDate, SimpleDate)>,
    #[props(default)]
    pub today: Option<SimpleDate>,
    #[props(default)]
    pub disabled_dates: Vec<SimpleDate>,
    #[props(default)]
    pub min_date: Option<SimpleDate>,
    #[props(default)]
    pub max_date: Option<SimpleDate>,
    #[props(default)]
    pub on_range_select: Option<EventHandler<(SimpleDate, SimpleDate)>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default = true)]
    pub show_other_months: bool,
    #[props(default = true)]
    pub show_week_numbers: bool,
}

#[component]
pub fn RangeCalendar(props: RangeCalendarProps) -> Element {
    let RangeCalendarProps {
        size,
        selected_range,
        today,
        disabled_dates,
        min_date,
        max_date,
        on_range_select,
        class,
        show_other_months,
        show_week_numbers,
    } = props;
    
    let current_date = today.unwrap_or_else(SimpleDate::today);
    let mut current_view = use_signal(|| current_date.clone());
    let mut view_mode = use_signal(|| CalendarView::Month);
    
    // Range selection state
    let mut range_state = use_signal(|| {
        if let Some((start, end)) = selected_range {
            RangeState::Complete(start, end)
        } else {
            RangeState::None
        }
    });
    
    let container_class = format!(
        "bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg shadow-sm {}",
        size.container_classes()
    );
    
    let final_class = if let Some(custom_class) = class {
        format!("{} {}", container_class, custom_class)
    } else {
        container_class
    };
    
    // Generate calendar days for current month view
    let calendar_days = use_memo(move || {
        let view_date = current_view.read();
        let first_day = view_date.first_of_month();
        let first_weekday = first_day.weekday() as i32;
        let days_in_month = view_date.days_in_month();
        
        // Previous month days
        let prev_month = view_date.add_months(-1);
        let prev_month_days = prev_month.days_in_month();
        
        let mut days = Vec::new();
        
        // Add previous month's trailing days
        for day in (prev_month_days - first_weekday as u32 + 1)..=prev_month_days {
            if first_weekday > 0 {
                days.push((SimpleDate::new(prev_month.year, prev_month.month, day), false));
            }
        }
        
        // Add current month days
        for day in 1..=days_in_month {
            days.push((SimpleDate::new(view_date.year, view_date.month, day), true));
        }
        
        // Add next month's leading days to fill the grid
        let next_month = view_date.add_months(1);
        let remaining_cells = 42 - days.len(); // 6 rows × 7 days
        for day in 1..=remaining_cells.min(14) as u32 {
            days.push((SimpleDate::new(next_month.year, next_month.month, day), false));
        }
        
        days
    });
    
    rsx! {
        div { class: "{final_class} p-4",
            // Header with navigation
            div { class: "flex items-center justify-between mb-4",
                button {
                    class: "p-1 rounded hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-600 dark:text-gray-300",
                    onclick: move |_| {
                        current_view.with_mut(|date| {
                            *date = match *view_mode.read() {
                                CalendarView::Month => date.add_months(-1),
                                CalendarView::Year => date.add_years(-1),
                                CalendarView::Decade => date.add_years(-10),
                            };
                        });
                    },
                    "‹"
                }
                
                button {
                    class: "text-sm font-medium text-gray-900 dark:text-gray-100 hover:bg-gray-100 dark:hover:bg-gray-800 px-3 py-1 rounded",
                    onclick: move |_| {
                        let current_mode = *view_mode.read();
                        let new_mode = match current_mode {
                            CalendarView::Month => CalendarView::Year,
                            CalendarView::Year => CalendarView::Decade,
                            CalendarView::Decade => CalendarView::Month,
                        };
                        view_mode.set(new_mode);
                    },
                    {
                        let current_date = current_view.read();
                        match *view_mode.read() {
                            CalendarView::Month => format!("{} {}", current_date.month_name(), current_date.year),
                            CalendarView::Year => format!("{}", current_date.year),
                            CalendarView::Decade => {
                                let decade_start = (current_date.year / 10) * 10;
                                format!("{}-{}", decade_start, decade_start + 9)
                            }
                        }
                    }
                }
                
                button {
                    class: "p-1 rounded hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-600 dark:text-gray-300",
                    onclick: move |_| {
                        current_view.with_mut(|date| {
                            *date = match *view_mode.read() {
                                CalendarView::Month => date.add_months(1),
                                CalendarView::Year => date.add_years(1),
                                CalendarView::Decade => date.add_years(10),
                            };
                        });
                    },
                    "›"
                }
            }
            
            // Calendar grid - different views based on mode
            match *view_mode.read() {
                CalendarView::Month => rsx! {
                    div { class: "space-y-0",
                        // Weekday headers
                        div { class: "grid grid-cols-7 mb-1",
                            for weekday in [
                                Weekday::Sunday,
                                Weekday::Monday,
                                Weekday::Tuesday,
                                Weekday::Wednesday,
                                Weekday::Thursday,
                                Weekday::Friday,
                                Weekday::Saturday,
                            ] {
                                div {
                                    class: "text-center text-xs font-medium text-gray-500 dark:text-gray-400 py-2 px-1",
                                    "{weekday.short_name()}"
                                }
                            }
                        }
                        
                        // Calendar days grid
                        div { class: "grid grid-cols-7 gap-y-1",
                            for (i, (date, is_current_month)) in calendar_days.read().clone().iter().enumerate() {
                                {
                                    let date = date.clone();
                                    let current_range_state = range_state.read().clone();
                                    let is_today = date == current_date;
                                    let is_disabled = disabled_dates.iter().any(|d| d == &date) ||
                                                    min_date.as_ref().map_or(false, |min| &date < min) ||
                                                    max_date.as_ref().map_or(false, |max| &date > max);
                                    
                                    let is_in_range = current_range_state.contains_date(&date);
                                    let is_range_start = current_range_state.is_range_start(&date);
                                    let is_range_end = current_range_state.is_range_end(&date);
                                    
                                    // Determine position in week for range styling  
                                    let col_index = i % 7;
                                    let is_week_start = col_index == 0;
                                    let is_week_end = col_index == 6;
                                    
                                    // Check adjacent dates for continuous styling
                                    let next_date_in_range = if i + 1 < calendar_days.read().len() {
                                        let next_date = calendar_days.read()[i + 1].0.clone();
                                        current_range_state.contains_date(&next_date) || 
                                        current_range_state.is_range_start(&next_date) || 
                                        current_range_state.is_range_end(&next_date)
                                    } else {
                                        false
                                    };
                                    let prev_date_in_range = if i > 0 {
                                        let prev_date = calendar_days.read()[i - 1].0.clone();
                                        current_range_state.contains_date(&prev_date) ||
                                        current_range_state.is_range_start(&prev_date) ||
                                        current_range_state.is_range_end(&prev_date)
                                    } else {
                                        false
                                    };
                                    
                                    // Button styling based on state - use custom sizing without width to prevent gaps
                                    let height_class = match size {
                                        CalendarSize::Small => "h-8 text-xs",
                                        CalendarSize::Medium => "h-10 text-sm", 
                                        CalendarSize::Large => "h-12 text-base",
                                    };
                                    let mut button_classes = format!(
                                        "{} w-full flex items-center justify-center cursor-pointer transition-all duration-200", 
                                        height_class
                                    );
                                    
                                    // Range styling with proper continuous background and rounded corners
                                    if is_in_range || is_range_start || is_range_end {
                                        // Apply different backgrounds based on whether it's start/end or middle
                                        if is_range_start || is_range_end {
                                            // Brighter background for start/end dates
                                            button_classes.push_str(" bg-blue-500 text-white hover:bg-blue-600 font-medium");
                                        } else {
                                            // Softer background for middle dates
                                            button_classes.push_str(" bg-blue-100 dark:bg-blue-900/50 text-blue-900 dark:text-blue-100 hover:bg-blue-200 dark:hover:bg-blue-800");
                                        }
                                        
                                        // Determine rounded corners - start dates get left rounding, end dates get right rounding
                                        if is_range_start && is_range_end {
                                            // Single day selection (start == end) - should get full rounding
                                            button_classes.push_str(" !rounded-md");
                                        } else if is_range_start {
                                            // Check if this is just a start selection (not part of a completed range)
                                            if matches!(current_range_state, RangeState::StartSelected(_)) {
                                                // Single start selection - should get full rounding
                                                button_classes.push_str(" !rounded-md");
                                            } else {
                                                // Range start - always round left corners (top-left and bottom-left)
                                                button_classes.push_str(" rounded-tl-md rounded-bl-md");
                                            }
                                        } else if is_range_end {
                                            // Range end - always round right corners (top-right and bottom-right)
                                            button_classes.push_str(" rounded-tr-md rounded-br-md");
                                        } else if is_in_range {
                                            // Middle of range - determine rounded corners based on week boundaries for continuous flow
                                            if is_week_start && !prev_date_in_range {
                                                // Start of week, no previous range - round left corners
                                                button_classes.push_str(" rounded-tl-md rounded-bl-md");
                                            } else if is_week_end && !next_date_in_range {
                                                // End of week, no next range - round right corners
                                                button_classes.push_str(" rounded-tr-md rounded-br-md");
                                            } else if is_week_start {
                                                // Start of week, continuing from previous week - round left corners
                                                button_classes.push_str(" rounded-tl-md rounded-bl-md");
                                            } else if is_week_end {
                                                // End of week, continuing to next week - round right corners
                                                button_classes.push_str(" rounded-tr-md rounded-br-md");
                                            }
                                            // Middle of week stays completely flat (no rounding)
                                        }
                                    } else if is_today {
                                        button_classes.push_str(" bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-md");
                                    } else if is_disabled {
                                        button_classes.push_str(" text-gray-300 dark:text-gray-600 cursor-not-allowed");
                                    } else if *is_current_month || show_other_months {
                                        if *is_current_month {
                                            button_classes.push_str(" text-gray-900 dark:text-gray-100 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md");
                                        } else {
                                            button_classes.push_str(" text-gray-400 dark:text-gray-500 hover:bg-gray-50 dark:hover:bg-gray-800 rounded-md");
                                        }
                                    } else {
                                        button_classes.push_str(" invisible");
                                    }
                                    
                                    rsx! {
                                        button {
                                            key: "{date.format()}",
                                            class: "{button_classes}",
                                            disabled: is_disabled,
                                            onclick: move |_| {
                                                if !is_disabled {
                                                    let new_state = match &current_range_state {
                                                        RangeState::None => RangeState::StartSelected(date.clone()),
                                                        RangeState::StartSelected(start) => {
                                                            let range = if start <= &date {
                                                                (start.clone(), date.clone())
                                                            } else {
                                                                (date.clone(), start.clone())
                                                            };
                                                            if let Some(handler) = &on_range_select {
                                                                handler.call(range.clone());
                                                            }
                                                            RangeState::Complete(range.0, range.1)
                                                        },
                                                        RangeState::Complete(_, _) => RangeState::StartSelected(date.clone()),
                                                    };
                                                    range_state.set(new_state);
                                                }
                                            },
                                            "{date.day}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                
                CalendarView::Year => rsx! {
                    div { class: "grid grid-cols-3 gap-2",
                        for month in 1..=12 {
                            {
                                let current_year = current_view.read().year;
                                let month_date = SimpleDate::new(current_year, month, 1);
                                let is_current_month = current_view.read().month == month;
                                let is_today_month = current_date.year == current_year && current_date.month == month;
                                
                                let mut cell_classes = format!(
                                    "{} flex items-center justify-center rounded cursor-pointer transition-colors",
                                    match size {
                                        CalendarSize::Small => "h-12 text-sm",
                                        CalendarSize::Medium => "h-16 text-base",
                                        CalendarSize::Large => "h-20 text-lg",
                                    }
                                );
                                
                                if is_current_month {
                                    cell_classes.push_str(" bg-blue-600 text-white hover:bg-blue-700");
                                } else if is_today_month {
                                    cell_classes.push_str(" bg-blue-100 dark:bg-blue-900 text-blue-600 dark:text-blue-300 hover:bg-blue-200 dark:hover:bg-blue-800");
                                } else {
                                    cell_classes.push_str(" text-gray-900 dark:text-gray-100 hover:bg-gray-100 dark:hover:bg-gray-800");
                                }
                                
                                rsx! {
                                    button {
                                        key: "{month}",
                                        class: "{cell_classes}",
                                        onclick: move |_| {
                                            current_view.set(SimpleDate::new(current_year, month, 1));
                                            view_mode.set(CalendarView::Month);
                                        },
                                        "{month_date.short_month_name()}"
                                    }
                                }
                            }
                        }
                    }
                },
                
                CalendarView::Decade => {
                    let current_year = current_view.read().year;
                    let decade_start = (current_year / 10) * 10;
                    
                    rsx! {
                        div { class: "grid grid-cols-3 gap-2",
                            for year_offset in 0..12 {
                                {
                                    let year = decade_start + year_offset - 1;
                                    let is_current_year = current_year == year;
                                    let is_today_year = current_date.year == year;
                                    let is_in_decade = year_offset > 0 && year_offset < 11;
                                    
                                    let mut cell_classes = format!(
                                        "{} flex items-center justify-center rounded cursor-pointer transition-colors",
                                        match size {
                                            CalendarSize::Small => "h-12 text-sm",
                                            CalendarSize::Medium => "h-16 text-base",
                                            CalendarSize::Large => "h-20 text-lg",
                                        }
                                    );
                                    
                                    if is_current_year {
                                        cell_classes.push_str(" bg-blue-600 text-white hover:bg-blue-700");
                                    } else if is_today_year {
                                        cell_classes.push_str(" bg-blue-100 dark:bg-blue-900 text-blue-600 dark:text-blue-300 hover:bg-blue-200 dark:hover:bg-blue-800");
                                    } else if is_in_decade {
                                        cell_classes.push_str(" text-gray-900 dark:text-gray-100 hover:bg-gray-100 dark:hover:bg-gray-800");
                                    } else {
                                        cell_classes.push_str(" text-gray-400 dark:text-gray-500 hover:bg-gray-50 dark:hover:bg-gray-800");
                                    }
                                    
                                    rsx! {
                                        button {
                                            key: "{year}",
                                            class: "{cell_classes}",
                                            onclick: move |_| {
                                                let current_month = current_view.read().month;
                                                current_view.set(SimpleDate::new(year, current_month, 1));
                                                view_mode.set(CalendarView::Year);
                                            },
                                            "{year}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Selection status and controls
            div { class: "flex flex-col items-center mt-4 pt-3 border-t border-gray-200 dark:border-gray-700 space-y-2",
                // Range status
                match range_state.read().clone() {
                    RangeState::None => rsx! {
                        p { class: "text-sm text-gray-500 dark:text-gray-400",
                            "Select start date"
                        }
                    },
                    RangeState::StartSelected(start) => rsx! {
                        p { class: "text-sm text-blue-600 dark:text-blue-400",
                            "Start: {start.format()} → Select end date"
                        }
                    },
                    RangeState::Complete(start, end) => rsx! {
                        p { class: "text-sm text-green-600 dark:text-green-400 font-medium",
                            "Range: {start.format()} to {end.format()}"
                        }
                    },
                }
                
                // Control buttons
                div { class: "flex space-x-2",
                    button {
                        class: "text-sm text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 font-medium",
                        onclick: move |_| {
                            current_view.set(current_date.clone());
                            view_mode.set(CalendarView::Month);
                        },
                        "Today"
                    }
                    
                    if !matches!(range_state.read().clone(), RangeState::None) {
                        button {
                            class: "text-sm text-gray-600 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 font-medium",
                            onclick: move |_| {
                                range_state.set(RangeState::None);
                            },
                            "Clear"
                        }
                    }
                }
            }
        }
    }
}