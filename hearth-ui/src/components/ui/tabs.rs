//! Tab components for organizing content in panels
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     Tabs {
//!         default_value: 0,
//!         TabsList {
//!             TabsTrigger { value: 0, "Overview" }
//!             TabsTrigger { value: 1, "Settings" }
//!         }
//!         TabsContent {
//!             value: 0,
//!             div { class: "p-4", "Overview content" }
//!         }
//!         TabsContent {
//!             value: 1,
//!             div { class: "p-4", "Settings content" }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Visual style variants for tab interfaces
#[derive(Clone, PartialEq)]
pub enum TabsVariant {
    Default,
    Pills,
    Underlined,
    Contained,
}

/// Size variants for tab triggers
#[derive(Clone, PartialEq)]
pub enum TabsSize {
    Small,
    Medium,
    Large,
}

/// Layout orientation for tabs
#[derive(Clone, PartialEq)]
pub enum TabsOrientation {
    Horizontal,
    Vertical,
}

/// Properties for the main Tabs container
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// Visual style variant
    #[props(default = TabsVariant::Default)]
    pub variant: TabsVariant,

    /// Size variant for tab triggers
    #[props(default = TabsSize::Medium)]
    pub size: TabsSize,

    /// Layout orientation
    #[props(default = TabsOrientation::Horizontal)]
    pub orientation: TabsOrientation,

    /// Initially active tab index
    #[props(default = 0)]
    pub default_value: usize,

    /// Callback when active tab changes
    #[props(default)]
    pub onchange: Option<EventHandler<usize>>,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Child components
    pub children: Element,
}

impl TabsVariant {
    /// Returns CSS classes for tab list container
    pub fn list_classes(&self) -> &'static str {
        match self {
            TabsVariant::Default => "border-b border-border",
            TabsVariant::Pills => "p-1 bg-muted rounded-lg",
            TabsVariant::Underlined => "",
            TabsVariant::Contained => "border border-border rounded-lg overflow-hidden",
        }
    }

    /// Returns CSS classes for tab triggers
    pub fn trigger_classes(&self, is_active: bool) -> String {
        let base = "inline-flex items-center justify-center whitespace-nowrap transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:ring-offset-background disabled:pointer-events-none disabled:opacity-50";

        match self {
            TabsVariant::Default => {
                if is_active {
                    format!("{base} border-b-2 border-primary text-primary ")
                } else {
                    format!("{base} border-b-2 border-transparent text-foreground hover:text-muted-foreground  hover:border-border ")
                }
            }
            TabsVariant::Pills => {
                if is_active {
                    format!("{base} bg-card text-foreground shadow-sm rounded-md")
                } else {
                    format!("{base} text-foreground hover:text-muted-foreground  hover:bg-muted  rounded-md")
                }
            }
            TabsVariant::Underlined => {
                if is_active {
                    format!("{base} border-b-2 border-primary text-primary ")
                } else {
                    format!("{base} text-foreground hover:text-muted-foreground ")
                }
            }
            TabsVariant::Contained => {
                if is_active {
                    format!("{base} bg-primary  text-primary  border-r border-border")
                } else {
                    format!("{base} text-foreground hover:text-muted-foreground  hover:bg-muted  border-r border-border")
                }
            }
        }
    }
}

impl TabsSize {
    /// Returns CSS classes for tab trigger sizing
    pub fn trigger_classes(&self) -> &'static str {
        match self {
            TabsSize::Small => "px-3 py-1.5 text-sm",
            TabsSize::Medium => "px-4 py-2 text-base",
            TabsSize::Large => "px-6 py-3 text-lg",
        }
    }
}

/// Main tabs container component
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let active_tab = use_signal(|| props.default_value);

    use_effect(move || {
        let current_tab = active_tab();
        if let Some(handler) = &props.onchange {
            handler.call(current_tab);
        }
    });

    let container_classes = match props.orientation {
        TabsOrientation::Horizontal => "w-full",
        TabsOrientation::Vertical => "flex gap-4",
    };

    let custom_classes = props.class.as_deref().unwrap_or("");
    let combined_classes = format!("{container_classes} {custom_classes}");

    rsx! {
        div { class: "{combined_classes}",
            {
                use_context_provider(|| TabsContext {
                    variant: props.variant.clone(),
                    size: props.size.clone(),
                    orientation: props.orientation.clone(),
                    active_tab_signal: active_tab,
                });
            }
            {props.children}
        }
    }
}

/// Context for sharing tabs state across components
#[derive(Clone)]
pub struct TabsContext {
    pub variant: TabsVariant,
    pub size: TabsSize,
    pub orientation: TabsOrientation,
    pub active_tab_signal: Signal<usize>,
}

/// Properties for the TabsList container
#[derive(Props, Clone, PartialEq)]
pub struct TabsListProps {
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Tab trigger components
    pub children: Element,
}

/// Container for tab trigger buttons
#[component]
pub fn TabsList(props: TabsListProps) -> Element {
    let ctx = use_context::<TabsContext>();

    let list_classes = ctx.variant.list_classes();
    let orientation_classes = match ctx.orientation {
        TabsOrientation::Horizontal => "flex",
        TabsOrientation::Vertical => "flex flex-col min-w-0",
    };

    let custom_classes = props.class.as_deref().unwrap_or("");
    let combined_classes = format!("{list_classes} {orientation_classes} {custom_classes}");

    rsx! {
        div { class: "{combined_classes}", role: "tablist", {props.children} }
    }
}

/// Properties for individual TabsTrigger components
#[derive(Props, Clone, PartialEq)]
pub struct TabsTriggerProps {
    /// Unique index for this tab trigger
    pub value: usize,

    /// Whether this trigger is disabled
    #[props(default)]
    pub disabled: bool,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Content to display in the trigger
    pub children: Element,
}

/// Individual tab trigger button
#[component]
pub fn TabsTrigger(props: TabsTriggerProps) -> Element {
    let mut ctx = use_context::<TabsContext>();
    let is_active = (ctx.active_tab_signal)() == props.value;

    let trigger_classes = ctx.variant.trigger_classes(is_active);
    let size_classes = ctx.size.trigger_classes();
    let custom_classes = props.class.as_deref().unwrap_or("");

    let combined_classes = format!("{trigger_classes} {size_classes} {custom_classes}");

    rsx! {
        button {
            class: "{combined_classes}",
            role: "tab",
            "aria-selected": "{is_active}",
            "aria-controls": "tabpanel-{props.value}",
            id: "tab-{props.value}",
            tabindex: if is_active { "0" } else { "-1" },
            disabled: props.disabled,
            onclick: move |_| {
                if !props.disabled {
                    ctx.active_tab_signal.set(props.value);
                }
            },
            onkeydown: move |evt: KeyboardEvent| {
                if !props.disabled {
                    let key = evt.key();
                    match key {
                        dioxus::prelude::Key::Enter => {
                            evt.prevent_default();
                            ctx.active_tab_signal.set(props.value);
                        }
                        dioxus::prelude::Key::Character(ch) if ch == " " => {
                            evt.prevent_default();
                            ctx.active_tab_signal.set(props.value);
                        }
                        _ => {}
                    }
                }
            },
            {props.children}
        }
    }
}

/// Properties for TabsContent panel components
#[derive(Props, Clone, PartialEq)]
pub struct TabsContentProps {
    /// Tab index this content corresponds to
    pub value: usize,

    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,

    /// Content to display when tab is active
    pub children: Element,
}

/// Content panel component for tab content
#[component]
pub fn TabsContent(props: TabsContentProps) -> Element {
    let ctx = use_context::<TabsContext>();
    let is_active = (ctx.active_tab_signal)() == props.value;

    if !is_active {
        return rsx! {
            div { hidden: true }
        };
    }

    let base_classes = "mt-4 focus:outline-none";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let combined_classes = format!("{base_classes} {custom_classes}");

    rsx! {
        div {
            class: "{combined_classes}",
            role: "tabpanel",
            id: "tabpanel-{props.value}",
            "aria-labelledby": "tab-{props.value}",
            tabindex: "0",
            {props.children}
        }
    }
}
