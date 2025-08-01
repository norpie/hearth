//! Card component for displaying content in a structured container

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum CardVariant {
    Default,
    Outline,
    Elevated,
    Flat,
}

#[derive(Clone, PartialEq)]
pub enum CardSize {
    Small,
    Medium,
    Large,
}

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    #[props(default = CardVariant::Default)]
    pub variant: CardVariant,
    #[props(default = CardSize::Medium)]
    pub size: CardSize,
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

impl CardVariant {
    pub fn classes(&self) -> &'static str {
        match self {
            CardVariant::Default => "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 shadow-sm",
            CardVariant::Outline => "bg-white dark:bg-gray-800 border-2 border-gray-300 dark:border-gray-600",
            CardVariant::Elevated => "bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 shadow-lg",
            CardVariant::Flat => "bg-gray-50 dark:bg-gray-900 border-0",
        }
    }
}

impl CardSize {
    pub fn classes(&self) -> &'static str {
        match self {
            CardSize::Small => "p-4",
            CardSize::Medium => "p-6",
            CardSize::Large => "p-8",
        }
    }
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let base_classes = "rounded-lg transition-colors";
    let variant_classes = props.variant.classes();
    let size_classes = props.size.classes();
    let custom_classes = props.class.as_deref().unwrap_or("");
    
    let combined_classes = format!("{} {} {} {}", base_classes, variant_classes, size_classes, custom_classes);

    rsx! {
        div {
            class: "{combined_classes}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    let base_classes = "flex flex-col space-y-1.5 pb-6";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let combined_classes = format!("{} {}", base_classes, custom_classes);

    rsx! {
        div {
            class: "{combined_classes}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardTitleProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn CardTitle(props: CardTitleProps) -> Element {
    let base_classes = "text-2xl font-semibold leading-none tracking-tight text-gray-900 dark:text-gray-100";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let combined_classes = format!("{} {}", base_classes, custom_classes);

    rsx! {
        h3 {
            class: "{combined_classes}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardDescriptionProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn CardDescription(props: CardDescriptionProps) -> Element {
    let base_classes = "text-sm text-gray-500 dark:text-gray-400";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let combined_classes = format!("{} {}", base_classes, custom_classes);

    rsx! {
        p {
            class: "{combined_classes}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardContentProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn CardContent(props: CardContentProps) -> Element {
    let base_classes = "pt-0";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let combined_classes = format!("{} {}", base_classes, custom_classes);

    rsx! {
        div {
            class: "{combined_classes}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardFooterProps {
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    let base_classes = "flex items-center pt-6";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let combined_classes = format!("{} {}", base_classes, custom_classes);

    rsx! {
        div {
            class: "{combined_classes}",
            {props.children}
        }
    }
}