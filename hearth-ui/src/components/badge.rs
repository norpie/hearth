use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    /// Badge content
    pub children: Element,
    /// Badge variant
    #[props(default = BadgeVariant::Default)]
    pub variant: BadgeVariant,
    /// Badge size
    #[props(default = BadgeSize::Medium)]
    pub size: BadgeSize,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Click handler
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum BadgeVariant {
    Default,
    Secondary,
    Success,
    Warning,
    Error,
    Info,
    Outline,
}

impl BadgeVariant {
    pub fn class(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-300",
            BadgeVariant::Secondary => "bg-gray-200 text-gray-900 dark:bg-gray-700 dark:text-gray-100",
            BadgeVariant::Success => "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300",
            BadgeVariant::Warning => "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300",
            BadgeVariant::Error => "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300",
            BadgeVariant::Info => "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300",
            BadgeVariant::Outline => "bg-transparent border border-gray-300 text-gray-700 dark:border-gray-600 dark:text-gray-300",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum BadgeSize {
    Small,
    Medium,
    Large,
}

impl BadgeSize {
    pub fn class(&self) -> &'static str {
        match self {
            BadgeSize::Small => "px-2 py-0.5 text-xs",
            BadgeSize::Medium => "px-2.5 py-1 text-sm",
            BadgeSize::Large => "px-3 py-1.5 text-base",
        }
    }
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let is_clickable = props.onclick.is_some();
    let base_classes = "inline-flex items-center font-medium rounded-full transition-colors";
    let variant_classes = props.variant.class();
    let size_classes = props.size.class();
    let hover_classes = if is_clickable { "cursor-pointer hover:opacity-80" } else { "" };
    let custom_classes = props.class.as_deref().unwrap_or("");
    
    let classes = format!("{} {} {} {} {}", 
        base_classes, variant_classes, size_classes, hover_classes, custom_classes);

    rsx! {
        span {
            class: "{classes}",
            onclick: move |event| {
                if let Some(handler) = &props.onclick {
                    handler.call(event);
                }
            },
            {props.children}
        }
    }
}