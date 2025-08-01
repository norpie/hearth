//! Tabs component for organizing content into switchable panels

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum TabsVariant {
    Default,
    Pills,
    Underlined,
    Contained,
}

#[derive(Clone, PartialEq)]
pub enum TabsSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, PartialEq)]
pub enum TabsOrientation {
    Horizontal,
    Vertical,
}

#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    #[props(default = TabsVariant::Default)]
    pub variant: TabsVariant,
    #[props(default = TabsSize::Medium)]
    pub size: TabsSize,
    #[props(default = TabsOrientation::Horizontal)]
    pub orientation: TabsOrientation,
    #[props(default = 0)]
    pub default_value: usize,
    #[props(default)]
    pub onchange: Option<EventHandler<usize>>,
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

impl TabsVariant {
    pub fn list_classes(&self) -> &'static str {
        match self {
            TabsVariant::Default => "border-b border-gray-200 dark:border-gray-700",
            TabsVariant::Pills => "p-1 bg-gray-100 dark:bg-gray-800 rounded-lg",
            TabsVariant::Underlined => "",
            TabsVariant::Contained => "border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden",
        }
    }

    pub fn trigger_classes(&self, is_active: bool) -> String {
        let base = "inline-flex items-center justify-center whitespace-nowrap transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-900 disabled:pointer-events-none disabled:opacity-50";
        
        match self {
            TabsVariant::Default => {
                if is_active {
                    format!("{} border-b-2 border-blue-600 text-blue-600 dark:text-blue-400 dark:border-blue-400", base)
                } else {
                    format!("{} border-b-2 border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300 dark:hover:border-gray-600", base)
                }
            },
            TabsVariant::Pills => {
                if is_active {
                    format!("{} bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 shadow-sm rounded-md", base)
                } else {
                    format!("{} text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-750 rounded-md", base)
                }
            },
            TabsVariant::Underlined => {
                if is_active {
                    format!("{} border-b-2 border-blue-600 text-blue-600 dark:text-blue-400 dark:border-blue-400", base)
                } else {
                    format!("{} text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300", base)
                }
            },
            TabsVariant::Contained => {
                if is_active {
                    format!("{} bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400 border-r border-gray-200 dark:border-gray-700", base)
                } else {
                    format!("{} text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800 border-r border-gray-200 dark:border-gray-700", base)
                }
            },
        }
    }
}

impl TabsSize {
    pub fn trigger_classes(&self) -> &'static str {
        match self {
            TabsSize::Small => "px-3 py-1.5 text-sm",
            TabsSize::Medium => "px-4 py-2 text-base",
            TabsSize::Large => "px-6 py-3 text-lg",
        }
    }
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let active_tab = use_signal(|| props.default_value);
    
    // Watch for changes to active tab and call onchange handler
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
    let combined_classes = format!("{} {}", container_classes, custom_classes);

    rsx! {
        div {
            class: "{combined_classes}",
            
            // Provide context for child components
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

#[derive(Clone)]
pub struct TabsContext {
    pub variant: TabsVariant,
    pub size: TabsSize,
    pub orientation: TabsOrientation,
    pub active_tab_signal: Signal<usize>,
}

#[derive(Props, Clone, PartialEq)]
pub struct TabsListProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn TabsList(props: TabsListProps) -> Element {
    let ctx = use_context::<TabsContext>();
    
    let list_classes = ctx.variant.list_classes();
    let orientation_classes = match ctx.orientation {
        TabsOrientation::Horizontal => "flex",
        TabsOrientation::Vertical => "flex flex-col min-w-0",
    };
    
    let custom_classes = props.class.as_deref().unwrap_or("");
    let combined_classes = format!("{} {} {}", list_classes, orientation_classes, custom_classes);

    rsx! {
        div {
            class: "{combined_classes}",
            role: "tablist",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TabsTriggerProps {
    pub value: usize,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn TabsTrigger(props: TabsTriggerProps) -> Element {
    let mut ctx = use_context::<TabsContext>();
    let is_active = (ctx.active_tab_signal)() == props.value;
    
    let trigger_classes = ctx.variant.trigger_classes(is_active);
    let size_classes = ctx.size.trigger_classes();
    let custom_classes = props.class.as_deref().unwrap_or("");
    
    let combined_classes = format!("{} {} {}", trigger_classes, size_classes, custom_classes);

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
                        },
                        dioxus::prelude::Key::Character(ch) if ch == " " => {
                            evt.prevent_default();
                            ctx.active_tab_signal.set(props.value);
                        },
                        _ => {}
                    }
                }
            },
            
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TabsContentProps {
    pub value: usize,
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn TabsContent(props: TabsContentProps) -> Element {
    let ctx = use_context::<TabsContext>();
    let is_active = (ctx.active_tab_signal)() == props.value;
    
    if !is_active {
        return rsx! { div { hidden: true } };
    }
    
    let base_classes = "mt-4 focus:outline-none";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let combined_classes = format!("{} {}", base_classes, custom_classes);

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