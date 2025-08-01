//! Calendar component for date selection

use dioxus::prelude::*;

/// Calendar view modes
#[derive(Clone, PartialEq, Debug, Copy)]
pub enum CalendarView {
    Month,
    Year,
    Decade,
}

/// Calendar sizes
#[derive(Clone, PartialEq, Debug)]
pub enum CalendarSize {
    Small,
    Medium,
    Large,
}

/// Day of the week
#[derive(Clone, PartialEq, Debug)]
pub enum Weekday {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

impl Weekday {
    pub fn short_name(&self) -> &'static str {
        match self {
            Weekday::Sunday => "Sun",
            Weekday::Monday => "Mon",
            Weekday::Tuesday => "Tue",
            Weekday::Wednesday => "Wed",
            Weekday::Thursday => "Thu",
            Weekday::Friday => "Fri",
            Weekday::Saturday => "Sat",
        }
    }
}

/// Represents a simple date without time zone
#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub struct SimpleDate {
    pub year: i32,
    pub month: u32,  // 1-12
    pub day: u32,    // 1-31
}

impl SimpleDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }
    
    pub fn today() -> Self {
        // For demo purposes, return a fixed date
        // In a real implementation, this would get the actual current date
        Self::new(2024, 8, 15)
    }
    
    /// Get the weekday for this date (simplified calculation)
    pub fn weekday(&self) -> Weekday {
        // Simplified day of week calculation using Zeller's congruence
        let mut year = self.year;
        let mut month = self.month as i32;
        
        if month < 3 {
            month += 12;
            year -= 1;
        }
        
        let k = year % 100;
        let j = year / 100;
        
        let day_of_week = (self.day as i32 + (13 * (month + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
        let day_of_week = (day_of_week + 7) % 7; // Ensure positive result
        
        match day_of_week {
            0 => Weekday::Saturday,
            1 => Weekday::Sunday,
            2 => Weekday::Monday,
            3 => Weekday::Tuesday,
            4 => Weekday::Wednesday,
            5 => Weekday::Thursday,
            6 => Weekday::Friday,
            _ => Weekday::Sunday,
        }
    }
    
    /// Check if this date is in the same month as another
    pub fn same_month(&self, other: &SimpleDate) -> bool {
        self.year == other.year && self.month == other.month
    }
    
    /// Format date as string
    pub fn format(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
    
    /// Get month name
    pub fn month_name(&self) -> &'static str {
        match self.month {
            1 => "January",
            2 => "February", 
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Invalid",
        }
    }
    
    /// Get days in this month
    pub fn days_in_month(&self) -> u32 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }
    
    /// Check if year is leap year
    fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
    }
    
    /// Get first day of the month
    pub fn first_of_month(&self) -> SimpleDate {
        SimpleDate::new(self.year, self.month, 1)
    }
    
    /// Add months
    pub fn add_months(&self, months: i32) -> SimpleDate {
        let mut year = self.year;
        let mut month = self.month as i32 + months;
        
        while month > 12 {
            month -= 12;
            year += 1;
        }
        while month < 1 {
            month += 12;
            year -= 1;
        }
        
        let days_in_new_month = SimpleDate::new(year, month as u32, 1).days_in_month();
        let day = self.day.min(days_in_new_month);
        
        SimpleDate::new(year, month as u32, day)
    }
    
    /// Add years
    pub fn add_years(&self, years: i32) -> SimpleDate {
        let new_year = self.year + years;
        let days_in_new_month = SimpleDate::new(new_year, self.month, 1).days_in_month();
        let day = self.day.min(days_in_new_month);
        
        SimpleDate::new(new_year, self.month, day)
    }
    
    /// Get short month name
    pub fn short_month_name(&self) -> &'static str {
        match self.month {
            1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr",
            5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Aug",
            9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
            _ => "Invalid",
        }
    }
}

impl CalendarSize {
    pub fn container_classes(&self) -> &'static str {
        match self {
            CalendarSize::Small => "w-64",
            CalendarSize::Medium => "w-80",
            CalendarSize::Large => "w-96",
        }
    }
    
    pub fn day_cell_classes(&self) -> &'static str {
        match self {
            CalendarSize::Small => "h-8 w-8 text-xs",
            CalendarSize::Medium => "h-10 w-10 text-sm",
            CalendarSize::Large => "h-12 w-12 text-base",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    #[props(default = CalendarSize::Medium)]
    pub size: CalendarSize,
    #[props(default)]
    pub selected_date: Option<SimpleDate>,
    #[props(default)]
    pub today: Option<SimpleDate>,
    #[props(default)]
    pub disabled_dates: Vec<SimpleDate>,
    #[props(default)]
    pub min_date: Option<SimpleDate>,
    #[props(default)]
    pub max_date: Option<SimpleDate>,
    #[props(default)]
    pub on_date_select: Option<EventHandler<SimpleDate>>,
    #[props(default)]
    pub class: Option<String>,
    #[props(default = true)]
    pub show_other_months: bool,
    #[props(default = true)]
    pub show_week_numbers: bool,
}

#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    let CalendarProps {
        size,
        selected_date,
        today,
        disabled_dates,
        min_date,
        max_date,
        on_date_select,
        class,
        show_other_months,
        show_week_numbers,
    } = props;
    
    let current_date = today.unwrap_or_else(SimpleDate::today);
    let mut current_view = use_signal(|| current_date.clone());
    let mut view_mode = use_signal(|| CalendarView::Month);
    
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
                    div { class: "grid grid-cols-7 gap-1",
                        // Weekday headers
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
                                class: "text-center text-xs font-medium text-gray-500 dark:text-gray-400 py-2",
                                "{weekday.short_name()}"
                            }
                        }
                        
                        // Calendar days
                        for (date, is_current_month) in calendar_days.read().clone().iter() {
                            {
                                let date = date.clone();
                                let is_selected = selected_date.as_ref().map_or(false, |d| d == &date);
                                let is_today = date == current_date;
                                let is_disabled = disabled_dates.iter().any(|d| d == &date) ||
                                                min_date.as_ref().map_or(false, |min| &date < min) ||
                                                max_date.as_ref().map_or(false, |max| &date > max);
                                
                                let mut cell_classes = format!(
                                    "{} flex items-center justify-center rounded cursor-pointer transition-colors",
                                    size.day_cell_classes()
                                );
                                
                                if is_selected {
                                    cell_classes.push_str(" bg-blue-600 text-white hover:bg-blue-700");
                                } else if is_today {
                                    cell_classes.push_str(" bg-blue-100 dark:bg-blue-900 text-blue-600 dark:text-blue-300 hover:bg-blue-200 dark:hover:bg-blue-800");
                                } else if is_disabled {
                                    cell_classes.push_str(" text-gray-300 dark:text-gray-600 cursor-not-allowed");
                                } else if *is_current_month || show_other_months {
                                    if *is_current_month {
                                        cell_classes.push_str(" text-gray-900 dark:text-gray-100 hover:bg-gray-100 dark:hover:bg-gray-800");
                                    } else {
                                        cell_classes.push_str(" text-gray-400 dark:text-gray-500 hover:bg-gray-50 dark:hover:bg-gray-800");
                                    }
                                } else {
                                    cell_classes.push_str(" invisible");
                                }
                                
                                rsx! {
                                    button {
                                        key: "{date.format()}",
                                        class: "{cell_classes}",
                                        disabled: is_disabled,
                                        onclick: move |_| {
                                            if !is_disabled {
                                                if let Some(handler) = &on_date_select {
                                                    handler.call(date.clone());
                                                }
                                            }
                                        },
                                        "{date.day}"
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
                                    let year = decade_start + year_offset - 1; // Start one year before decade
                                    let is_current_year = current_year == year;
                                    let is_today_year = current_date.year == year;
                                    let is_in_decade = year_offset > 0 && year_offset < 11; // Years 1-10 are in the current decade
                                    
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
            
            // Today button (optional)
            div { class: "flex justify-center mt-4 pt-3 border-t border-gray-200 dark:border-gray-700",
                button {
                    class: "text-sm text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 font-medium",
                    onclick: move |_| {
                        current_view.set(current_date.clone());
                        view_mode.set(CalendarView::Month);
                        if let Some(handler) = &on_date_select {
                            handler.call(current_date.clone());
                        }
                    },
                    "Today"
                }
            }
        }
    }
}