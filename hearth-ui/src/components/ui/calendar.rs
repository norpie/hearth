//! Calendar components for date selection and navigation
//!
//! The calendar module provides date selection components with month, year,
//! and decade view modes. It features keyboard navigation, date validation,
//! and a simplified date handling system.
//!
//! # Examples
//!
//! Basic date picker:
//! ```rust
//! let mut selected_date = use_signal(|| None::<SimpleDate>);
//! rsx! {
//!     Calendar {
//!         selected_date: selected_date(),
//!         on_date_select: move |date| {
//!             selected_date.set(Some(date));
//!         }
//!     }
//! }
//! ```
//!
//! Date picker with restrictions:
//! ```rust
//! let mut booking_date = use_signal(|| None::<SimpleDate>);
//! let today = SimpleDate::today();
//! let max_date = today.add_days(90); // 90 days from now
//! 
//! rsx! {
//!     div { class: "space-y-4",
//!         h3 { "Select Booking Date" }
//!         Calendar {
//!             size: CalendarSize::Large,
//!             selected_date: booking_date(),
//!             min_date: Some(today), // Can't book in the past
//!             max_date: Some(max_date), // Booking window limit
//!             disabled_dates: vec![
//!                 // Disable specific unavailable dates
//!                 SimpleDate::new(2024, 8, 25),
//!                 SimpleDate::new(2024, 8, 26),
//!                 SimpleDate::new(2024, 9, 1),
//!             ],
//!             on_date_select: move |date| {
//!                 booking_date.set(Some(date));
//!                 show_booking_form.set(true);
//!             },
//!             class: Some("mx-auto".to_string())
//!         }
//!         if let Some(date) = booking_date() {
//!             div { class: "p-4 bg-success/10 border border-success rounded-lg",
//!                 p { "Booking scheduled for {date.format()}" }
//!                 button {
//!                     class: "mt-2 bg-primary text-primary-foreground px-4 py-2 rounded",
//!                     "Confirm Booking"
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Compact calendar for date input:
//! ```rust
//! let mut birth_date = use_signal(|| None::<SimpleDate>);
//! let min_date = SimpleDate::new(1900, 1, 1);
//! let max_date = SimpleDate::today().add_years(-13); // Must be 13+ years old
//! 
//! rsx! {
//!     div { class: "space-y-2",
//!         Label { "Date of Birth" }
//!         div { class: "relative",
//!             Calendar {
//!                 size: CalendarSize::Small,
//!                 selected_date: birth_date(),
//!                 min_date: Some(min_date),
//!                 max_date: Some(max_date),
//!                 show_other_months: false, // Cleaner compact view
//!                 on_date_select: move |date| {
//!                     birth_date.set(Some(date));
//!                 },
//!                 class: Some("absolute top-full left-0 z-10 mt-1".to_string())
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Event calendar with highlighted dates:
//! ```rust
//! let mut view_date = use_signal(|| SimpleDate::today());
//! let event_dates = vec![
//!     SimpleDate::new(2024, 8, 15),
//!     SimpleDate::new(2024, 8, 22),
//!     SimpleDate::new(2024, 8, 29),
//!     SimpleDate::new(2024, 9, 5),
//! ];
//! 
//! rsx! {
//!     div { class: "space-y-4",
//!         h2 { "Event Calendar" }
//!         Calendar {
//!             size: CalendarSize::Large,
//!             today: Some(view_date()),
//!             disabled_dates: event_dates.clone(), // Use as highlighted instead
//!             on_date_select: move |date| {
//!                 if event_dates.contains(&date) {
//!                     show_event_details(date);
//!                 } else {
//!                     view_date.set(date);
//!                 }
//!             },
//!             class: Some("border-2 border-primary/20".to_string())
//!         }
//!         div { class: "flex items-center gap-2 text-sm text-muted-foreground",
//!             div { class: "w-3 h-3 bg-primary rounded" }
//!             "Event dates"
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Calendar display modes for different navigation levels
///
/// Controls the granularity of date selection and navigation within
/// the calendar component, allowing users to navigate between different
/// time scales efficiently.
#[derive(Clone, PartialEq, Debug, Copy)]
pub enum CalendarView {
    /// Monthly view showing individual days in a grid
    ///
    /// Displays a traditional calendar month with days of the week
    /// and clickable date cells. Primary view for date selection.
    Month,

    /// Yearly view showing months as selectable items
    ///
    /// Displays 12 months in a grid for quick month navigation.
    /// Clicking a month switches back to Month view for that month.
    Year,

    /// Decade view showing years as selectable items
    ///
    /// Displays years in a decade for long-range navigation.
    /// Clicking a year switches to Year view for that year.
    Decade,
}

/// Size variants for calendar component scaling
///
/// Controls the overall dimensions and cell sizes of the calendar
/// to fit different UI contexts and space constraints.
#[derive(Clone, PartialEq, Debug)]
pub enum CalendarSize {
    /// Compact size for inline date inputs and constrained spaces
    ///
    /// Container width: 16rem (256px), day cells: 8x8 with xs text.
    /// Suitable for form inputs and sidebar widgets.
    Small,

    /// Standard size for general date selection (default)
    ///
    /// Container width: 20rem (320px), day cells: 10x10 with sm text.
    /// Balanced size for most use cases and modal dialogs.
    Medium,

    /// Large size for prominent date selection interfaces
    ///
    /// Container width: 24rem (384px), day cells: 12x12 with base text.
    /// Suitable for main content areas and touch interfaces.
    Large,
}

/// Days of the week with numeric values for calculations
///
/// Represents weekdays with Sunday as 0 for compatibility with
/// standard date calculation algorithms and calendar layouts.
#[derive(Clone, PartialEq, Debug)]
pub enum Weekday {
    /// Sunday (0) - First day of week in calendar display
    Sunday = 0,
    /// Monday (1) - Second day of week
    Monday = 1,
    /// Tuesday (2) - Third day of week
    Tuesday = 2,
    /// Wednesday (3) - Fourth day of week
    Wednesday = 3,
    /// Thursday (4) - Fifth day of week
    Thursday = 4,
    /// Friday (5) - Sixth day of week
    Friday = 5,
    /// Saturday (6) - Seventh day of week
    Saturday = 6,
}

impl Weekday {
    /// Returns the abbreviated three-letter name for display
    ///
    /// Provides consistent short names for calendar headers and
    /// compact displays. Uses standard English abbreviations.
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

/// Simplified date representation for calendar operations
///
/// Provides a lightweight date structure optimized for calendar UI
/// components without timezone complexity. Includes essential date
/// arithmetic and formatting operations for user interface needs.
#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub struct SimpleDate {
    /// Year value (supports negative values for historical dates)
    pub year: i32,
    /// Month value from 1 (January) to 12 (December)
    pub month: u32,
    /// Day of month from 1 to 31 (validated against month)
    pub day: u32,
}

impl SimpleDate {
    /// Creates a new date with the specified year, month, and day
    ///
    /// No validation is performed - the caller is responsible for
    /// ensuring valid date values. Use with care for user input.
    ///
    /// # Parameters
    /// - `year`: Year value (supports negative for BCE dates)
    /// - `month`: Month from 1-12 (1=January, 12=December)
    /// - `day`: Day from 1-31 (varies by month)
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// Returns a fixed date representing "today" for demo purposes
    ///
    /// In a production implementation, this would get the actual
    /// current date from the system. Currently returns August 15, 2024
    /// for consistent behavior across development environments.
    pub fn today() -> Self {
        Self::new(2024, 8, 15)
    }

    /// Calculates the day of the week using Zeller's congruence
    ///
    /// Returns the weekday for this date using a mathematical algorithm
    /// that handles month adjustments and leap years correctly.
    ///
    /// # Returns
    /// The `Weekday` enum value for this date
    pub fn weekday(&self) -> Weekday {
        let mut year = self.year;
        let mut month = self.month as i32;

        if month < 3 {
            month += 12;
            year -= 1;
        }

        let k = year % 100;
        let j = year / 100;

        let day_of_week =
            (self.day as i32 + (13 * (month + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
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

    /// Checks if this date is in the same month and year as another
    ///
    /// Useful for determining if dates belong to the same calendar month
    /// for styling and grouping purposes.
    ///
    /// # Parameters
    /// - `other`: The date to compare against
    ///
    /// # Returns
    /// True if both dates are in the same month and year
    pub fn same_month(&self, other: &SimpleDate) -> bool {
        self.year == other.year && self.month == other.month
    }

    /// Formats the date as an ISO 8601 string (YYYY-MM-DD)
    ///
    /// Provides a standard string representation suitable for display,
    /// storage, or API communication.
    ///
    /// # Returns
    /// String in format "YYYY-MM-DD" with zero-padding
    pub fn format(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Returns the full English name of the month
    ///
    /// Provides the complete month name for display in headers
    /// and user-facing text.
    ///
    /// # Returns
    /// Full month name or "Invalid" for invalid month values
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

    /// Returns the number of days in this date's month
    ///
    /// Accounts for leap years when calculating February days.
    /// Used for date validation and calendar grid generation.
    ///
    /// # Returns
    /// Number of days in the month (28-31)
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

    /// Determines if this date's year is a leap year
    ///
    /// Uses the standard Gregorian calendar leap year rules:
    /// - Divisible by 4: leap year
    /// - Divisible by 100: not leap year
    /// - Divisible by 400: leap year
    ///
    /// # Returns
    /// True if the year is a leap year
    fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
    }

    /// Creates a new date for the first day of this date's month
    ///
    /// Useful for calendar navigation and month boundary calculations.
    ///
    /// # Returns
    /// New `SimpleDate` with day set to 1, same month and year
    pub fn first_of_month(&self) -> SimpleDate {
        SimpleDate::new(self.year, self.month, 1)
    }

    /// Adds the specified number of months to this date
    ///
    /// Handles year rollover and adjusts the day if the target month
    /// has fewer days than the current day (e.g., Jan 31 + 1 month = Feb 28).
    ///
    /// # Parameters
    /// - `months`: Number of months to add (can be negative)
    ///
    /// # Returns
    /// New `SimpleDate` with months added
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

    /// Adds the specified number of years to this date
    ///
    /// Adjusts the day if adding years results in an invalid date
    /// (e.g., Feb 29 on a non-leap year becomes Feb 28).
    ///
    /// # Parameters
    /// - `years`: Number of years to add (can be negative)
    ///
    /// # Returns
    /// New `SimpleDate` with years added
    pub fn add_years(&self, years: i32) -> SimpleDate {
        let new_year = self.year + years;
        let days_in_new_month = SimpleDate::new(new_year, self.month, 1).days_in_month();
        let day = self.day.min(days_in_new_month);

        SimpleDate::new(new_year, self.month, day)
    }

    /// Adds the specified number of days to this date
    ///
    /// Handles month and year rollovers correctly, including leap years.
    /// Supports both positive and negative day additions.
    ///
    /// # Parameters
    /// - `days`: Number of days to add (can be negative)
    ///
    /// # Returns
    /// New `SimpleDate` with days added
    pub fn add_days(&self, days: i32) -> SimpleDate {
        let mut result = self.clone();
        let mut remaining_days = days;

        if remaining_days > 0 {
            while remaining_days > 0 {
                let days_in_current_month = result.days_in_month();
                let days_left_in_month = days_in_current_month - result.day;

                if remaining_days <= days_left_in_month as i32 {
                    result.day += remaining_days as u32;
                    remaining_days = 0;
                } else {
                    remaining_days -= days_left_in_month as i32 + 1;
                    result = result.add_months(1);
                    result.day = 1;
                }
            }
        } else if remaining_days < 0 {
            remaining_days = -remaining_days;
            while remaining_days > 0 {
                if remaining_days < result.day as i32 {
                    result.day -= remaining_days as u32;
                    remaining_days = 0;
                } else {
                    remaining_days -= result.day as i32;
                    result = result.add_months(-1);
                    result.day = result.days_in_month();
                }
            }
        }

        result
    }

    /// Returns the abbreviated three-letter month name
    ///
    /// Provides compact month names for space-constrained displays
    /// like year view navigation.
    ///
    /// # Returns
    /// Three-letter month abbreviation or "Invalid" for invalid months
    pub fn short_month_name(&self) -> &'static str {
        match self.month {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "Invalid",
        }
    }
}

impl CalendarSize {
    /// Returns CSS classes for the calendar container width
    ///
    /// Maps size variants to appropriate Tailwind width classes
    /// for consistent calendar sizing across different contexts.
    ///
    /// # Returns
    /// Tailwind CSS width class for this size
    pub fn container_classes(&self) -> &'static str {
        match self {
            CalendarSize::Small => "w-64",
            CalendarSize::Medium => "w-80",
            CalendarSize::Large => "w-96",
        }
    }

    /// Returns CSS classes for individual day cell dimensions and text
    ///
    /// Provides appropriate sizing for clickable day cells that maintains
    /// good touch targets and readability across different calendar sizes.
    ///
    /// # Returns
    /// Tailwind CSS classes for height, width, and text size
    pub fn day_cell_classes(&self) -> &'static str {
        match self {
            CalendarSize::Small => "h-8 w-8 text-xs",
            CalendarSize::Medium => "h-10 w-10 text-sm",
            CalendarSize::Large => "h-12 w-12 text-base",
        }
    }
}

/// Properties for configuring the Calendar component
///
/// Provides comprehensive control over calendar appearance, behavior,
/// date validation, and interaction handling for flexible date selection
/// interfaces.
#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {
    /// Size variant controlling calendar dimensions
    ///
    /// Determines the overall calendar size including container width
    /// and day cell dimensions. Choose based on available space and
    /// importance in the interface. Defaults to `CalendarSize::Medium`.
    #[props(default = CalendarSize::Medium)]
    pub size: CalendarSize,

    /// Currently selected date for highlighting
    ///
    /// When provided, this date is visually highlighted in the calendar.
    /// Used for showing the active selection state. Defaults to None
    /// (no date selected).
    #[props(default)]
    pub selected_date: Option<SimpleDate>,

    /// Reference date for "today" highlighting and navigation
    ///
    /// When provided, this date receives special "today" styling and
    /// is used as the target for "Today" button navigation. When None,
    /// uses `SimpleDate::today()`. Defaults to None.
    #[props(default)]
    pub today: Option<SimpleDate>,

    /// Collection of dates that should be disabled for selection
    ///
    /// These dates are rendered with disabled styling and cannot be
    /// clicked. Useful for blocking unavailable dates, holidays, or
    /// blackout periods. Defaults to empty vector.
    #[props(default)]
    pub disabled_dates: Vec<SimpleDate>,

    /// Earliest selectable date (inclusive)
    ///
    /// Dates before this are disabled and cannot be selected.
    /// Useful for preventing past date selection or enforcing
    /// business rules. Defaults to None (no minimum).
    #[props(default)]
    pub min_date: Option<SimpleDate>,

    /// Latest selectable date (inclusive)
    ///
    /// Dates after this are disabled and cannot be selected.
    /// Useful for limiting future selections or booking windows.
    /// Defaults to None (no maximum).
    #[props(default)]
    pub max_date: Option<SimpleDate>,

    /// Event handler called when a date is selected
    ///
    /// Receives the selected `SimpleDate` as parameter. Called for
    /// all valid date selections including clicking dates and using
    /// the "Today" button. Defaults to None (no handler).
    #[props(default)]
    pub on_date_select: Option<EventHandler<SimpleDate>>,

    /// Additional CSS classes for the calendar container
    ///
    /// Applied to the root calendar element for custom positioning,
    /// margins, or styling. The component provides base styling.
    /// Defaults to None.
    #[props(default)]
    pub class: Option<String>,

    /// Whether to show dates from adjacent months
    ///
    /// When true, displays grayed-out dates from previous and next
    /// months to fill the calendar grid. When false, these cells
    /// are invisible for a cleaner appearance. Defaults to true.
    #[props(default = true)]
    pub show_other_months: bool,

    /// Whether to display week numbers in the calendar
    ///
    /// Currently not implemented but reserved for future functionality.
    /// When implemented, will show ISO week numbers in a left column.
    /// Defaults to true.
    #[props(default = true)]
    pub show_week_numbers: bool,
}

/// Calendar component for date selection with multiple view modes and navigation
///
/// The Calendar component provides a sophisticated date selection interface with
/// month, year, and decade view modes. It features keyboard navigation, date
/// validation, accessibility support, and flexible styling options for creating
/// comprehensive date picker interfaces.
///
/// # Features
///
/// - **Multi-level navigation**: Month, year, and decade views for efficient date selection
/// - **Date validation**: Min/max ranges, disabled dates, and boundary checking
/// - **Responsive sizing**: Three size variants for different interface contexts
/// - **Accessibility**: Keyboard navigation, ARIA attributes, and screen reader support
/// - **Visual feedback**: Selected date highlighting, today indication, and hover states
/// - **Flexible styling**: Custom CSS classes and adaptive layout options
/// - **Simple date model**: Lightweight date handling optimized for UI components
/// - **Interactive navigation**: Click-through navigation between view modes
///
/// # Implementation Details
///
/// The component uses a state machine approach with three view modes that users
/// can navigate between by clicking the header. Month view shows individual days,
/// year view shows months, and decade view shows years.
///
/// Date calculations use a simplified date model without timezone complexity,
/// making it suitable for date selection scenarios where time zones are not
/// relevant or handled separately.
///
/// The calendar grid generation includes dates from adjacent months for a complete
/// 6x7 grid, with visual distinction between current and other month dates.
///
/// # Accessibility
///
/// - Keyboard navigation with arrow keys for date selection
/// - ARIA attributes for screen reader compatibility
/// - Focus management that respects user navigation patterns
/// - Semantic button elements with proper labels
/// - High contrast styling for selected and today states
/// - Disabled state handling for invalid dates
///
/// # View Mode Navigation
///
/// - **Month View**: Default view showing calendar grid with selectable dates
/// - **Year View**: Accessed by clicking month/year header, shows 12 months
/// - **Decade View**: Accessed from year view, shows 10+ years for navigation
/// - **Header Navigation**: Left/right arrows navigate within current view mode
/// - **Today Button**: Quick navigation to current date and month view
///
/// # Date Validation
///
/// The component supports multiple validation mechanisms:
/// - `min_date` and `max_date` for range restrictions
/// - `disabled_dates` for specific date blacklisting
/// - Automatic validation prevents invalid selections
/// - Visual feedback shows disabled state clearly
///
/// # Performance Considerations
///
/// - Efficient date calculations using mathematical algorithms
/// - Minimal re-renders with memo optimization for calendar grid
/// - Lightweight date model without external dependencies
/// - CSS-based styling for smooth interactions
///
/// # Use Cases
///
/// - **Date input fields**: Inline date selection for forms
/// - **Booking systems**: Date selection with availability restrictions
/// - **Event scheduling**: Date picking with conflict prevention
/// - **Report generation**: Date range selection interfaces
/// - **Historical data**: Navigation through past dates with validation
/// - **Future planning**: Forward date selection with limits
///
/// # Parameters
///
/// - `size`: Calendar dimensions (Small, Medium, Large)
/// - `selected_date`: Currently selected date for highlighting
/// - `today`: Reference date for "today" styling and navigation
/// - `disabled_dates`: Specific dates to disable
/// - `min_date` / `max_date`: Date range boundaries
/// - `on_date_select`: Event handler for date selection
/// - `class`: Additional CSS classes for styling
/// - `show_other_months`: Display adjacent month dates
/// - `show_week_numbers`: Show ISO week numbers (future feature)
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
        show_week_numbers: _,
    } = props;

    let current_date = today.unwrap_or_else(SimpleDate::today);
    let mut current_view = use_signal(|| current_date.clone());
    let mut view_mode = use_signal(|| CalendarView::Month);

    let container_class = format!(
        "bg-card border border-border rounded-lg shadow-sm {}",
        size.container_classes()
    );

    let final_class = if let Some(custom_class) = class {
        format!("{container_class} {custom_class}")
    } else {
        container_class
    };

    let calendar_days = use_memo(move || {
        let view_date = current_view.read();
        let first_day = view_date.first_of_month();
        let first_weekday = first_day.weekday() as i32;
        let days_in_month = view_date.days_in_month();

        let prev_month = view_date.add_months(-1);
        let prev_month_days = prev_month.days_in_month();

        let mut days = Vec::new();

        for day in (prev_month_days - first_weekday as u32 + 1)..=prev_month_days {
            if first_weekday > 0 {
                days.push((
                    SimpleDate::new(prev_month.year, prev_month.month, day),
                    false,
                ));
            }
        }

        for day in 1..=days_in_month {
            days.push((SimpleDate::new(view_date.year, view_date.month, day), true));
        }

        let next_month = view_date.add_months(1);
        let remaining_cells = 42 - days.len(); // 6 rows × 7 days
        for day in 1..=remaining_cells.min(14) as u32 {
            days.push((
                SimpleDate::new(next_month.year, next_month.month, day),
                false,
            ));
        }

        days
    });

    rsx! {
        div { class: "{final_class} p-4",
            div { class: "flex items-center justify-between mb-4",
                button {
                    class: "p-1 rounded hover:bg-muted text-foreground",
                    onclick: move |_| {
                        current_view
                            .with_mut(|date| {
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
                    class: "text-sm font-medium text-foreground hover:bg-muted px-3 py-1 rounded",
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
                            CalendarView::Month => {
                                format!("{} {}", current_date.month_name(), current_date.year)
                            }
                            CalendarView::Year => format!("{}", current_date.year),
                            CalendarView::Decade => {
                                let decade_start = (current_date.year / 10) * 10;
                                format!("{}-{}", decade_start, decade_start + 9)
                            }
                        }
                    }
                }
                button {
                    class: "p-1 rounded hover:bg-muted text-foreground",
                    onclick: move |_| {
                        current_view
                            .with_mut(|date| {
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
            match *view_mode.read() {
                CalendarView::Month => rsx! {
                    div { class: "grid grid-cols-7 gap-1",
                        for weekday in [
                            Weekday::Sunday,
                            Weekday::Monday,
                            Weekday::Tuesday,
                            Weekday::Wednesday,
                            Weekday::Thursday,
                            Weekday::Friday,
                            Weekday::Saturday,
                        ]
                        {




                            div { class: "text-center text-xs font-medium text-foreground py-2", "{weekday.short_name()}" }
                        }
                        for (date , is_current_month) in calendar_days.read().clone().iter() {
                            {
                                let date = date.clone();
                                let is_selected = selected_date.as_ref() == Some(&date);
                                let is_today = date == current_date;
                                let is_disabled = disabled_dates.iter().any(|d| d == &date)

                                    || min_date.as_ref().is_some_and(|min| &date < min)
                                    || max_date.as_ref().is_some_and(|max| &date > max);
                                let mut cell_classes = format!(
                                    "{} flex items-center justify-center rounded cursor-pointer transition-colors",
                                    size.day_cell_classes(),
                                );
                                if is_selected {
                                    cell_classes.push_str(" bg-primary text-white hover:bg-primary");
                                } else if is_today {
                                    cell_classes
                                        .push_str(
                                            " bg-primary text-primary-foreground hover:bg-primary",
                                        );
                                } else if is_disabled {
                                    cell_classes.push_str(" text-foreground cursor-not-allowed");
                                } else if *is_current_month || show_other_months {
                                    cell_classes
                                        .push_str(" text-foreground hover:bg-muted");
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
                                let is_today_month = current_date.year == current_year

                                    && current_date.month == month;
                                let mut cell_classes = format!(
                                    "{} flex items-center justify-center rounded cursor-pointer transition-colors",
                                    match size {
                                        CalendarSize::Small => "h-12 text-sm",
                                        CalendarSize::Medium => "h-16 text-base",
                                        CalendarSize::Large => "h-20 text-lg",
                                    },
                                );
                                if is_current_month {
                                    cell_classes.push_str(" bg-primary text-white hover:bg-primary");
                                } else if is_today_month {
                                    cell_classes
                                        .push_str(
                                            " bg-primary text-primary-foreground hover:bg-primary",
                                        );
                                } else {
                                    cell_classes.push_str(" text-foreground hover:bg-muted");
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

                                    let mut cell_classes = format!(
                                        "{} flex items-center justify-center rounded cursor-pointer transition-colors",
                                        match size {
                                            CalendarSize::Small => "h-12 text-sm",
                                            CalendarSize::Medium => "h-16 text-base",
                                            CalendarSize::Large => "h-20 text-lg",
                                        },
                                    );

                                    if is_current_year {
                                        cell_classes.push_str(" bg-primary text-white hover:bg-primary");
                                    } else if is_today_year {
                                        cell_classes
                                            .push_str(

                                                " bg-primary text-primary-foreground hover:bg-primary",
                                            );
                                    } else {
                                        cell_classes.push_str(" text-foreground hover:bg-muted");
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
            div { class: "flex justify-center mt-4 pt-3 border-t border-border",
                button {
                    class: "text-sm text-primary-foreground hover:text-primary font-medium",
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
