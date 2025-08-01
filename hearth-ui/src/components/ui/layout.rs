//! Generic layout helper components for organizing content and UI elements
//!
//! The layout module provides reusable components for structuring content and
//! creating consistent layouts across the application. These components are
//! domain-agnostic and focus on visual organization, spacing, and interaction
//! patterns without containing business logic.
//!
//! # Features
//!
//! - **Settings organization**: Section and item components for settings pages
//! - **Filter interfaces**: Tab components for filtering and categorization
//! - **Consistent spacing**: Standardized margins and padding patterns
//! - **Interactive elements**: Hover effects and click handling
//! - **Flexible content**: Support for icons, labels, descriptions, and custom trailing elements
//! - **Responsive design**: Mobile-friendly layouts with touch-optimized interactions
//! - **Accessibility**: Proper semantic structure and keyboard navigation
//! - **Themeable**: Uses design system colors and spacing tokens
//!
//! # Examples
//!
//! Settings page layout with sections:
//! ```rust
//! rsx! {
//!     div { class: "max-w-2xl mx-auto p-6",
//!         SettingsSection {
//!             title: "Account Settings",
//!             SettingsItem {
//!                 icon: "fas fa-user",
//!                 label: "Profile",
//!                 description: Some("Manage your profile information"),
//!                 on_click: move |_| navigate_to_profile(),
//!                 i { class: "fas fa-chevron-right text-muted-foreground" }
//!             }
//!             SettingsItem {
//!                 icon: "fas fa-bell",
//!                 label: "Notifications",
//!                 description: Some("Configure notification preferences"),
//!                 on_click: move |_| navigate_to_notifications(),
//!                 Switch {
//!                     checked: notifications_enabled(),
//!                     onchange: move |enabled| set_notifications(enabled)
//!                 }
//!             }
//!         }
//!         SettingsSection {
//!             title: "Privacy & Security",
//!             SettingsItem {
//!                 icon: "fas fa-lock",
//!                 label: "Two-Factor Authentication",
//!                 description: Some("Add an extra layer of security"),
//!                 on_click: move |_| navigate_to_2fa(),
//!                 span { class: "text-sm text-muted-foreground", "Not enabled" }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Filter tabs for content categorization:
//! ```rust
//! let mut current_filter = use_signal(|| ContentFilter::All);
//! rsx! {
//!     div { class: "flex space-x-2 mb-6",
//!         FilterTab {
//!             filter: ContentFilter::All,
//!             current_filter: current_filter(),
//!             label: "All",
//!             count: Some(total_items),
//!             on_select: move |filter| current_filter.set(filter)
//!         }
//!         FilterTab {
//!             filter: ContentFilter::Recent,
//!             current_filter: current_filter(),
//!             label: "Recent",
//!             count: Some(recent_count),
//!             on_select: move |filter| current_filter.set(filter)
//!         }
//!         FilterTab {
//!             filter: ContentFilter::Favorites,
//!             current_filter: current_filter(),
//!             label: "Favorites",
//!             count: Some(favorites_count),
//!             on_select: move |filter| current_filter.set(filter)
//!         }
//!     }
//! }
//! ```
//!
//! Simple settings item without description:
//! ```rust
//! rsx! {
//!     SettingsItem {
//!         icon: "fas fa-palette",
//!         label: "Theme",
//!         description: None,
//!         on_click: move |_| toggle_theme_picker(),
//!         span { class: "text-sm font-medium", "{current_theme()}" }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Container component for organizing settings into logical sections
///
/// The SettingsSection component provides a semantic container for grouping
/// related settings items with a descriptive title. It applies consistent
/// spacing and typography for a cohesive settings page layout.
///
/// # Features
///
/// - **Section grouping**: Clear visual separation between setting categories
/// - **Consistent spacing**: Standardized margins and padding
/// - **Typography hierarchy**: Proper title styling with muted foreground
/// - **Flexible content**: Accepts any child components within the section
///
/// # Implementation Details
///
/// The component renders as a div container with bottom margin for section
/// separation. The title uses small, medium-weight typography with muted
/// foreground color. Child content is wrapped in a container with vertical
/// spacing between items.
///
/// # Accessibility
///
/// - Uses semantic heading structure for proper document outline
/// - Provides clear section boundaries for screen readers
/// - Maintains proper heading hierarchy with h2 elements
///
/// # Parameters
///
/// - `title`: Section heading text displayed above the content
/// - `children`: Settings items and other content to display in this section
#[component]
pub fn SettingsSection(title: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "mb-8",
            h2 { class: "text-sm font-medium text-muted-foreground mb-6",
                "{title}"
            }
            div { class: "space-y-1", {children} }
        }
    }
}

/// Interactive settings item with icon, label, description, and trailing content
///
/// The SettingsItem component provides a standardized layout for individual
/// settings with consistent styling, hover effects, and flexible content areas.
/// It supports icons, labels, optional descriptions, and custom trailing elements
/// like switches, buttons, or status indicators.
///
/// # Features
///
/// - **Consistent layout**: Standardized spacing and alignment for all settings
/// - **Interactive feedback**: Hover effects and click handling
/// - **Flexible content**: Support for various trailing elements (switches, chevrons, text)
/// - **Icon integration**: FontAwesome or custom icon support
/// - **Optional descriptions**: Secondary text for additional context
/// - **Accessibility**: Proper button semantics and keyboard navigation
///
/// # Implementation Details
///
/// The component renders as a full-width button with flexbox layout for proper
/// alignment. The icon, label/description, and trailing content are arranged
/// horizontally with appropriate spacing. Hover effects use theme colors for
/// consistent interaction feedback.
///
/// # Accessibility
///
/// - Uses semantic button element for proper keyboard navigation
/// - Click events are properly handled for both mouse and keyboard interaction
/// - Text content is structured for screen reader comprehension
/// - Focus states are visible and well-defined
///
/// # Parameters
///
/// - `icon`: CSS class string for the icon (e.g., "fas fa-user")
/// - `label`: Primary text label for the setting
/// - `description`: Optional secondary text for additional context
/// - `on_click`: Event handler called when the item is clicked
/// - `trailing`: Custom element displayed on the right side (switches, arrows, etc.)
#[component]
pub fn SettingsItem(
    icon: &'static str,
    label: &'static str,
    description: Option<&'static str>,
    on_click: EventHandler<()>,
    trailing: Element,
) -> Element {
    rsx! {
        button {
            class: "w-full py-4 px-4 flex items-center space-x-4 hover:bg-muted transition-colors text-left rounded-lg",
            onclick: move |_| on_click.call(()),

            i { class: "{icon} text-xl text-foreground" }
            div { class: "flex-1 min-w-0",
                div { class: "font-medium text-muted-foreground", "{label}" }
                if let Some(desc) = description {
                    div { class: "text-sm text-foreground mt-0.5", "{desc}" }
                }
            }
            div { class: "flex items-center", {trailing} }
        }
    }
}

/// Generic filter tab component for content categorization and filtering
///
/// The FilterTab component provides an interactive tab interface for filtering
/// content by different categories. It supports generic filter types, optional
/// item counts, and visual state indication for the currently active filter.
///
/// # Features
///
/// - **Generic filter types**: Works with any type implementing Clone + PartialEq
/// - **Visual state indication**: Active/inactive styling based on current selection
/// - **Item counts**: Optional count badges showing items in each category
/// - **Hover effects**: Interactive feedback for better user experience
/// - **Responsive design**: Touch-friendly sizing for mobile interfaces
/// - **Accessibility**: Proper button semantics and keyboard navigation
///
/// # Implementation Details
///
/// The component renders as a button with conditional styling based on whether
/// the filter matches the current selection. Count badges are conditionally
/// rendered when provided and styled to match the active state.
///
/// The generic type parameter allows the component to work with enums, strings,
/// or any other filterable data type that can be compared for equality.
///
/// # Accessibility
///
/// - Uses semantic button elements for proper keyboard navigation
/// - Visual state changes are communicated through styling
/// - Click events properly handle filter selection
/// - Focus states are clearly visible
///
/// # Type Parameters
///
/// - `T`: Filter type that must implement Clone + PartialEq + 'static
///
/// # Parameters
///
/// - `filter`: The filter value this tab represents
/// - `current_filter`: The currently active filter for comparison
/// - `label`: Display text for the tab
/// - `count`: Optional number of items in this filter category
/// - `on_select`: Event handler called when this filter is selected
#[component]
pub fn FilterTab<T: Clone + PartialEq + 'static>(
    filter: T,
    current_filter: T,
    label: &'static str,
    count: Option<usize>,
    on_select: EventHandler<T>,
) -> Element {
    let is_active = filter == current_filter;

    rsx! {
        button {
            class: format!(
                "px-4 py-2 rounded-lg text-sm font-medium transition-colors {}",
                if is_active {
                    "bg-accent/10 text-accent-foreground"
                } else {
                    "text-foreground hover:text-muted-foreground hover:bg-muted"
                },
            ),
            onclick: {
                let filter = filter.clone();
                move |_| on_select.call(filter.clone())
            },
            "{label}"
            if let Some(count) = count {
                span {
                    class: format!(
                        "ml-2 px-2 py-0.5 text-xs rounded-full {}",
                        if is_active {
                            "bg-accent/20 text-accent-foreground"
                        } else {
                            "bg-muted text-foreground"
                        },
                    ),
                    "{count}"
                }
            }
        }
    }
}
