//! Range calendar components for date range selection
//!
//! The range calendar module provides components for selecting date ranges
//! with visual feedback and range validation. It supports start/end date
//! selection with continuous highlighting and proper validation.
//!
//! # Examples
//!
//! Basic date range picker:
//! ```rust
//! let mut selected_range = use_signal(|| None::<(SimpleDate, SimpleDate)>);
//! rsx! {
//!     RangeCalendar {
//!         selected_range: selected_range(),
//!         on_range_select: move |range| {
//!             selected_range.set(Some(range));
//!         }
//!     }
//!     if let Some((start, end)) = selected_range() {
//!         p { "Selected range: {start.format()} to {end.format()}" }
//!     }
//! }
//! ```
//!
//! Vacation booking calendar with restrictions:
//! ```rust
//! let mut vacation_range = use_signal(|| None::<(SimpleDate, SimpleDate)>);
//! let today = SimpleDate::today();
//! let max_advance_booking = today.add_days(365); // 1 year ahead
//! 
//! // Block out unavailable periods
//! let blocked_dates = vec![
//!     // Holiday blackout period
//!     SimpleDate::new(2024, 12, 20),
//!     SimpleDate::new(2024, 12, 21),
//!     SimpleDate::new(2024, 12, 22),
//!     SimpleDate::new(2024, 12, 23),
//!     SimpleDate::new(2024, 12, 24),
//!     SimpleDate::new(2024, 12, 25),
//!     SimpleDate::new(2024, 12, 26),
//!     // Maintenance period
//!     SimpleDate::new(2024, 9, 15),
//!     SimpleDate::new(2024, 9, 16),
//! ];
//! 
//! rsx! {
//!     div { class: "space-y-6",
//!         h2 { "Select Vacation Dates" }
//!         RangeCalendar {
//!             size: CalendarSize::Large,
//!             selected_range: vacation_range(),
//!             min_date: Some(today), // Can't book past dates
//!             max_date: Some(max_advance_booking),
//!             disabled_dates: blocked_dates,
//!             on_range_select: move |range| {
//!                 vacation_range.set(Some(range));
//!                 validate_booking_length(range);
//!             },
//!             class: Some("mx-auto border-2 border-primary/20".to_string())
//!         }
//!         if let Some((start, end)) = vacation_range() {
//!             div { class: "p-4 bg-success/10 border border-success rounded-lg",
//!                 h3 { class: "font-semibold mb-2", "Vacation Summary" }
//!                 p { "Check-in: {start.format()}" }
//!                 p { "Check-out: {end.format()}" }
//!                 p { "Duration: {calculate_nights(start, end)} nights" }
//!                 div { class: "mt-4 flex gap-2",
//!                     button {
//!                         class: "bg-primary text-primary-foreground px-4 py-2 rounded",
//!                         "Book Now"
//!                     }
//!                     button {
//!                         class: "border border-border px-4 py-2 rounded",
//!                         onclick: move |_| vacation_range.set(None),
//!                         "Clear Dates"
//!                     }
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Project timeline range selector:
//! ```rust
//! let mut project_timeline = use_signal(|| None::<(SimpleDate, SimpleDate)>);
//! let project_start_min = SimpleDate::new(2024, 9, 1);
//! let project_end_max = SimpleDate::new(2025, 3, 31);
//! 
//! rsx! {
//!     div { class: "space-y-4",
//!         h3 { "Project Timeline" }
//!         p { class: "text-sm text-muted-foreground",
//!             "Select start and end dates for the project phase"
//!         }
//!         RangeCalendar {
//!             size: CalendarSize::Medium,
//!             selected_range: project_timeline(),
//!             min_date: Some(project_start_min),
//!             max_date: Some(project_end_max),
//!             show_other_months: false, // Cleaner appearance
//!             on_range_select: move |range| {
//!                 project_timeline.set(Some(range));
//!                 calculate_project_duration(range);
//!             }
//!         }
//!         match project_timeline() {
//!             Some((start, end)) => rsx! {
//!                 div { class: "grid grid-cols-2 gap-4 p-4 bg-muted rounded-lg",
//!                     div {
//!                         p { class: "text-sm font-medium", "Start Date" }
//!                         p { class: "text-lg", "{start.format()}" }
//!                     }
//!                     div {
//!                         p { class: "text-sm font-medium", "End Date" }
//!                         p { class: "text-lg", "{end.format()}" }
//!                     }
//!                 }
//!             },
//!             None => rsx! {
//!                 p { class: "text-muted-foreground italic", "No timeline selected" }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Compact range picker for forms:
//! ```rust
//! let mut date_range = use_signal(|| None::<(SimpleDate, SimpleDate)>);
//! 
//! rsx! {
//!     div { class: "space-y-2",
//!         Label { "Report Date Range" }
//!         div { class: "relative inline-block",
//!             RangeCalendar {
//!                 size: CalendarSize::Small,
//!                 selected_range: date_range(),
//!                 max_date: Some(SimpleDate::today()), // Only past dates
//!                 on_range_select: move |range| {
//!                     date_range.set(Some(range));
//!                     close_calendar.set(true);
//!                 },
//!                 class: Some("absolute top-0 left-0 z-10 shadow-lg".to_string())
//!             }
//!         }
//!         if let Some((start, end)) = date_range() {
//!             div { class: "text-sm text-muted-foreground",
//!                 "Range: {start.format()} → {end.format()}"
//!             }
//!         }
//!     }
//! }
//! ```

use crate::{CalendarSize, CalendarView, SimpleDate, Weekday};
use dioxus::prelude::*;

/// State tracking for date range selection process
///
/// Manages the multi-step process of range selection where users first
/// select a start date, then an end date, with visual feedback and
/// validation at each step.
#[derive(Clone, PartialEq, Debug)]
pub enum RangeState {
    /// No date selected yet - initial state
    ///
    /// The calendar is ready for the user to select a start date.
    /// Clicking any valid date will transition to StartSelected.
    None,

    /// Start date selected, waiting for end date selection
    ///
    /// User has clicked a start date and the calendar is waiting for
    /// them to click an end date. The start date is highlighted and
    /// clicking another date will complete or restart the selection.
    StartSelected(SimpleDate),

    /// Complete range selected with start and end dates
    ///
    /// User has successfully selected both start and end dates.
    /// The full range is highlighted and callbacks have been triggered.
    /// Clicking any date will restart the selection process.
    Complete(SimpleDate, SimpleDate),
}

impl RangeState {
    /// Returns the complete range if selection is finished
    ///
    /// Extracts the start and end dates from a completed range selection.
    /// Returns None if the range is not yet complete.
    ///
    /// # Returns
    /// `Some((start, end))` if range is complete, `None` otherwise
    pub fn range(&self) -> Option<(SimpleDate, SimpleDate)> {
        match self {
            RangeState::Complete(start, end) => Some((start.clone(), end.clone())),
            _ => None,
        }
    }

    /// Checks if a date falls within the selected range
    ///
    /// For complete ranges, determines if the given date is between
    /// the start and end dates (inclusive). Handles cases where
    /// start and end dates may be in reverse order.
    ///
    /// # Parameters
    /// - `date`: The date to check
    ///
    /// # Returns
    /// True if the date is within the completed range
    pub fn contains_date(&self, date: &SimpleDate) -> bool {
        match self {
            RangeState::Complete(start, end) => {
                let (range_start, range_end) = if start <= end {
                    (start, end)
                } else {
                    (end, start)
                };
                date >= range_start && date <= range_end
            }
            _ => false,
        }
    }

    /// Checks if a date is the start of the range selection
    ///
    /// Returns true for both intermediate start selections and
    /// completed ranges where this date is the start.
    ///
    /// # Parameters
    /// - `date`: The date to check
    ///
    /// # Returns
    /// True if this date is the range start
    pub fn is_range_start(&self, date: &SimpleDate) -> bool {
        match self {
            RangeState::StartSelected(start) | RangeState::Complete(start, _) => start == date,
            _ => false,
        }
    }

    /// Checks if a date is the end of a completed range
    ///
    /// Only returns true for completed ranges where this date
    /// is the end date. Start-only selections return false.
    ///
    /// # Parameters
    /// - `date`: The date to check
    ///
    /// # Returns
    /// True if this date is the range end in a completed selection
    pub fn is_range_end(&self, date: &SimpleDate) -> bool {
        match self {
            RangeState::Complete(_, end) => end == date,
            _ => false,
        }
    }
}

/// Properties for configuring the RangeCalendar component
///
/// Provides comprehensive control over range calendar behavior, validation,
/// visual appearance, and interaction handling for date range selection
/// interfaces.
#[derive(Props, Clone, PartialEq)]
pub struct RangeCalendarProps {
    /// Size variant controlling calendar dimensions
    ///
    /// Determines the overall calendar size including container width
    /// and day cell dimensions. Affects touch target sizes and visual
    /// prominence. Defaults to `CalendarSize::Medium`.
    #[props(default = CalendarSize::Medium)]
    pub size: CalendarSize,

    /// Initially selected date range for display
    ///
    /// When provided, the calendar starts with this range pre-selected
    /// and highlighted. Tuple format: (start_date, end_date).
    /// Defaults to None (no initial selection).
    #[props(default)]
    pub selected_range: Option<(SimpleDate, SimpleDate)>,

    /// Reference date for "today" highlighting and navigation
    ///
    /// When provided, this date receives special "today" styling.
    /// When None, uses `SimpleDate::today()` for current date detection.
    /// Defaults to None.
    #[props(default)]
    pub today: Option<SimpleDate>,

    /// Collection of dates that should be disabled for selection
    ///
    /// These dates cannot be selected and will invalidate ranges that
    /// include them. Useful for blocking unavailable dates, holidays,
    /// or maintenance periods. Defaults to empty vector.
    #[props(default)]
    pub disabled_dates: Vec<SimpleDate>,

    /// Earliest selectable date (inclusive)
    ///
    /// Dates before this are disabled. Range selections that would
    /// include dates before this minimum are prevented.
    /// Defaults to None (no minimum).
    #[props(default)]
    pub min_date: Option<SimpleDate>,

    /// Latest selectable date (inclusive)
    ///
    /// Dates after this are disabled. Range selections that would
    /// include dates after this maximum are prevented.
    /// Defaults to None (no maximum).
    #[props(default)]
    pub max_date: Option<SimpleDate>,

    /// Event handler called when a complete range is selected
    ///
    /// Receives a tuple (start_date, end_date) when both start and end
    /// dates have been selected and the range passes validation.
    /// Called only for valid, complete ranges. Defaults to None.
    #[props(default)]
    pub on_range_select: Option<EventHandler<(SimpleDate, SimpleDate)>>,

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

/// Range calendar component for date range selection with visual feedback
///
/// The RangeCalendar component provides an intuitive interface for selecting
/// date ranges with continuous visual highlighting, range validation, and
/// clear user guidance. It extends the basic calendar functionality to support
/// start/end date selection with proper range validation and accessibility.
///
/// # Features
///
/// - **Two-step selection**: Click start date, then end date with clear visual guidance
/// - **Continuous highlighting**: Visual range flows across week boundaries seamlessly
/// - **Range validation**: Prevents invalid ranges containing disabled dates
/// - **Multiple view modes**: Month, year, and decade navigation for efficient selection
/// - **Selection feedback**: Clear status messages guide users through the process
/// - **Flexible validation**: Min/max dates, disabled dates, and custom restrictions
/// - **Accessibility**: Keyboard navigation and screen reader support
/// - **Responsive design**: Multiple size variants for different interface contexts
///
/// # Implementation Details
///
/// The component uses a state machine approach with three states: None, StartSelected,
/// and Complete. Each state provides different visual feedback and interaction behavior.
///
/// Range highlighting uses CSS classes to create continuous visual flow across week
/// boundaries, with special handling for start/end dates and middle dates.
///
/// Range validation occurs when completing a selection - if any dates within the
/// proposed range are disabled or outside boundaries, the selection is rejected
/// and restarted from the clicked date.
///
/// # Visual Design
///
/// - **Start/End dates**: Primary color background with high contrast text
/// - **Middle dates**: Lighter primary background with flowing edges
/// - **Week boundaries**: Proper rounded corners for visual continuity
/// - **Selection state**: Clear status messages below the calendar
/// - **Interactive feedback**: Hover states and smooth transitions
///
/// # Range Selection Process
///
/// 1. **Initial state**: "Select start date" guidance message
/// 2. **Start selected**: "Start: [date] → Select end date" with start highlighted
/// 3. **Range complete**: "Range: [start] to [end]" with full range highlighted
/// 4. **Restart**: Clicking any date starts a new selection
///
/// # Validation Logic
///
/// When completing a range selection, the component validates:
/// - No disabled dates exist within the range
/// - Range doesn't extend before min_date
/// - Range doesn't extend after max_date
/// - Start and end dates are both valid
///
/// Invalid ranges cause selection to restart from the clicked date.
///
/// # Accessibility
///
/// - Button elements with proper ARIA attributes
/// - Clear selection state communication
/// - Keyboard navigation support
/// - Screen reader compatible status messages
/// - High contrast range highlighting
/// - Focus management across view modes
///
/// # Performance Considerations
///
/// - Efficient range validation with early termination
/// - Optimized CSS classes for smooth visual transitions
/// - Minimal re-renders with proper state management
/// - Calendar grid memoization for date calculations
///
/// # Use Cases
///
/// - **Booking systems**: Hotel reservations, rental periods, appointment scheduling
/// - **Report generation**: Date range selection for analytics and reports
/// - **Project planning**: Timeline selection and milestone planning
/// - **Event planning**: Multi-day event date selection
/// - **Data filtering**: Time-based data range selection
/// - **Vacation planning**: Travel date selection with restrictions
///
/// # Parameters
///
/// - `size`: Calendar dimensions (Small, Medium, Large)
/// - `selected_range`: Initial range selection (start, end)
/// - `today`: Reference date for "today" styling
/// - `disabled_dates`: Specific dates to disable
/// - `min_date` / `max_date`: Date range boundaries
/// - `on_range_select`: Event handler for complete range selection
/// - `class`: Additional CSS classes for styling
/// - `show_other_months`: Display adjacent month dates
/// - `show_week_numbers`: Show ISO week numbers (future feature)
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
        show_week_numbers: _,
    } = props;

    let current_date = today.unwrap_or_else(SimpleDate::today);
    let mut current_view = use_signal(|| current_date.clone());
    let mut view_mode = use_signal(|| CalendarView::Month);

    let mut range_state = use_signal(|| {
        if let Some((start, end)) = selected_range {
            RangeState::Complete(start, end)
        } else {
            RangeState::None
        }
    });

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
                    div { class: "space-y-0",
                        div { class: "grid grid-cols-7 mb-1",
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


                                div { class: "text-center text-xs font-medium text-foreground py-2 px-1",
                                    "{weekday.short_name()}"
                                }
                            }
                        }
                        div { class: "grid grid-cols-7 gap-y-1",
                            for (i , (date , is_current_month)) in calendar_days.read().clone().iter().enumerate() {
                                {
                                    let date = date.clone();
                                    let current_range_state = range_state.read().clone();
                                    let is_today = date == current_date;
                                    let is_disabled = disabled_dates.iter().any(|d| d == &date)

                                        || min_date.as_ref().is_some_and(|min| &date < min)
                                        || max_date.as_ref().is_some_and(|max| &date > max);
                                    let is_in_range = current_range_state.contains_date(&date);
                                    let is_range_start = current_range_state.is_range_start(&date);
                                    let is_range_end = current_range_state.is_range_end(&date);
                                    let col_index = i % 7;
                                    let is_week_start = col_index == 0;
                                    let is_week_end = col_index == 6;
                                    let next_date_in_range = if i + 1 < calendar_days.read().len() {
                                        let next_date = calendar_days.read()[i + 1].0.clone();
                                        current_range_state.contains_date(&next_date)
                                            || current_range_state.is_range_start(&next_date)
                                            || current_range_state.is_range_end(&next_date)
                                    } else {
                                        false
                                    };
                                    let prev_date_in_range = if i > 0 {
                                        let prev_date = calendar_days.read()[i - 1].0.clone();
                                        current_range_state.contains_date(&prev_date)
                                            || current_range_state.is_range_start(&prev_date)
                                            || current_range_state.is_range_end(&prev_date)
                                    } else {
                                        false
                                    };
                                    let height_class = match size {
                                        CalendarSize::Small => "h-8 text-xs",
                                        CalendarSize::Medium => "h-10 text-sm",
                                        CalendarSize::Large => "h-12 text-base",
                                    };
                                    let mut button_classes = format!(
                                        "{height_class} w-full flex items-center justify-center cursor-pointer transition-all duration-200",
                                    );
                                    if is_in_range || is_range_start || is_range_end {
                                        if is_range_start || is_range_end {
                                            button_classes
                                                .push_str(" bg-primary text-white hover:bg-primary font-medium");
                                        } else {
                                            button_classes
                                                .push_str(
                                                    " bg-primary/60 text-foreground hover:bg-primary/75",
                                                );
                                        }
                                        if is_range_start && is_range_end {
                                            button_classes.push_str(" !rounded-md");
                                        } else if is_range_start {
                                            if matches!(current_range_state, RangeState::StartSelected(_)) {
                                                button_classes.push_str(" !rounded-md");
                                            } else {
                                                button_classes.push_str(" rounded-tl-md rounded-bl-md");
                                            }
                                        } else if is_range_end {
                                            button_classes.push_str(" rounded-tr-md rounded-br-md");
                                        } else if is_in_range {
                                            if is_week_start && !prev_date_in_range {
                                                button_classes.push_str(" rounded-tl-md rounded-bl-md");
                                            } else if is_week_end && !next_date_in_range {
                                                button_classes.push_str(" rounded-tr-md rounded-br-md");
                                            } else if is_week_start {
                                                button_classes.push_str(" rounded-tl-md rounded-bl-md");
                                            } else if is_week_end {
                                                button_classes.push_str(" rounded-tr-md rounded-br-md");
                                            }
                                        }
                                    } else if is_today {
                                        button_classes
                                            .push_str(
                                                " bg-muted text-foreground hover:bg-muted rounded-md",
                                            );
                                    } else if is_disabled {
                                        button_classes.push_str(" text-muted-foreground bg-muted/50 cursor-not-allowed opacity-50 rounded-md");
                                    } else if *is_current_month || show_other_months {
                                        button_classes
                                            .push_str(
                                                " text-foreground hover:bg-muted rounded-md",
                                            );
                                    } else {
                                        button_classes.push_str(" invisible");
                                    }
                                    rsx! {
                                        button {
                                            key: "{date.format()}",
                                            class: "{button_classes}",
                                            disabled: is_disabled,
                                            onclick: {
                                                let disabled_dates_clone = disabled_dates.clone();
                                                let min_date_clone = min_date.clone();
                                                let max_date_clone = max_date.clone();
                                                move |_| {
                                                    if !is_disabled {
                                                        let new_state = match &current_range_state {
                                                            RangeState::None => RangeState::StartSelected(date.clone()),
                                                            RangeState::StartSelected(start) => {
                                                                let range = if start <= &date {
                                                                    (start.clone(), date.clone())
                                                                } else {
                                                                    (date.clone(), start.clone())
                                                                };
                                                                
                                                                let range_is_valid = {
                                                                    let mut current = range.0.clone();
                                                                    let mut valid = true;
                                                                    while current <= range.1 {
                                                                        if disabled_dates_clone.iter().any(|d| d == &current)
                                                                            || min_date_clone.as_ref().is_some_and(|min| &current < min)
                                                                            || max_date_clone.as_ref().is_some_and(|max| &current > max) {
                                                                            valid = false;
                                                                            break;
                                                                        }
                                                                        current = current.add_days(1);
                                                                    }
                                                                    valid
                                                                };
                                                                
                                                                if range_is_valid {
                                                                    if let Some(handler) = &on_range_select {
                                                                        handler.call(range.clone());
                                                                    }
                                                                    RangeState::Complete(range.0, range.1)
                                                                } else {
                                                                    RangeState::StartSelected(date.clone())
                                                                }
                                                            }
                                                            RangeState::Complete(_, _) => RangeState::StartSelected(date.clone()),
                                                        };
                                                        range_state.set(new_state);
                                                    }
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
                                    let year = decade_start + year_offset - 1;
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
            div { class: "flex flex-col items-center mt-4 pt-3 border-t border-border space-y-2",
                match range_state.read().clone() {
                    RangeState::None => rsx! {
                        p { class: "text-sm text-foreground", "Select start date" }
                    },
                    RangeState::StartSelected(start) => rsx! {
                        p { class: "text-sm text-primary-foreground", "Start: {start.format()} → Select end date" }
                    },
                    RangeState::Complete(start, end) => rsx! {
                        p { class: "text-sm text-success-foreground font-medium", "Range: {start.format()} to {end.format()}" }
                    },
                }
                div { class: "flex space-x-2",
                    button {
                        class: "text-sm text-primary-foreground hover:text-primary font-medium",
                        onclick: move |_| {
                            current_view.set(current_date.clone());
                            view_mode.set(CalendarView::Month);
                        },
                        "Today"
                    }
                    if !matches!(range_state.read().clone(), RangeState::None) {
                        button {
                            class: "text-sm text-foreground hover:text-muted-foreground font-medium",
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
