use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    /// Table content
    pub children: Element,
    /// Table variant
    #[props(default = TableVariant::Default)]
    pub variant: TableVariant,
    /// Table size
    #[props(default = TableSize::Medium)]
    pub size: TableSize,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TableVariant {
    Default,
    Striped,
    Bordered,
    Hoverable,
    Minimal,
}

impl TableVariant {
    pub fn class(&self) -> &'static str {
        match self {
            TableVariant::Default => "",
            TableVariant::Striped => "[&_tbody_tr:nth-child(odd)]:bg-gray-50 dark:[&_tbody_tr:nth-child(odd)]:bg-gray-800/50",
            TableVariant::Bordered => "border border-gray-200 dark:border-gray-700 [&_th]:border-r [&_th]:border-gray-200 dark:[&_th]:border-gray-700 [&_td]:border-r [&_td]:border-gray-200 dark:[&_td]:border-gray-700 [&_tr]:border-b [&_tr]:border-gray-200 dark:[&_tr]:border-gray-700",
            TableVariant::Hoverable => "[&_tbody_tr]:hover:bg-gray-50 dark:[&_tbody_tr]:hover:bg-gray-800/50 [&_tbody_tr]:transition-colors",
            TableVariant::Minimal => "[&_th]:border-r [&_th]:border-gray-200 dark:[&_th]:border-gray-700 [&_td]:border-r [&_td]:border-gray-200 dark:[&_td]:border-gray-700 [&_tbody_tr:not(:last-child)]:border-b [&_tbody_tr:not(:last-child)]:border-gray-200 dark:[&_tbody_tr:not(:last-child)]:border-gray-700 [&_th:last-child]:border-r-0 [&_td:last-child]:border-r-0",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum TableSize {
    Small,
    Medium,
    Large,
}

impl TableSize {
    pub fn class(&self) -> &'static str {
        match self {
            TableSize::Small => "[&_th]:px-2 [&_th]:py-1 [&_td]:px-2 [&_td]:py-1 text-sm",
            TableSize::Medium => "[&_th]:px-4 [&_th]:py-2 [&_td]:px-4 [&_td]:py-2",
            TableSize::Large => "[&_th]:px-6 [&_th]:py-3 [&_td]:px-6 [&_td]:py-3 text-lg",
        }
    }
}

#[component]
pub fn Table(props: TableProps) -> Element {
    let base_classes = "w-full border-collapse";
    let variant_classes = props.variant.class();
    let size_classes = props.size.class();
    let custom_classes = props.class.as_deref().unwrap_or("");
    
    let classes = format!("{} {} {} {}", 
        base_classes, variant_classes, size_classes, custom_classes);

    rsx! {
        div { class: "overflow-x-auto",
            table {
                class: "{classes}",
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableHeaderProps {
    /// Header content
    pub children: Element,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

#[component]
pub fn TableHeader(props: TableHeaderProps) -> Element {
    let base_classes = "bg-gray-50 dark:bg-gray-800";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let classes = format!("{} {}", base_classes, custom_classes);

    rsx! {
        thead {
            class: "{classes}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableBodyProps {
    /// Body content
    pub children: Element,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

#[component]
pub fn TableBody(props: TableBodyProps) -> Element {
    let base_classes = "bg-white dark:bg-gray-900";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let classes = format!("{} {}", base_classes, custom_classes);

    rsx! {
        tbody {
            class: "{classes}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableRowProps {
    /// Row content
    pub children: Element,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Click handler
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn TableRow(props: TableRowProps) -> Element {
    let is_clickable = props.onclick.is_some();
    let base_classes = "";
    let hover_classes = if is_clickable { "cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-700" } else { "" };
    let custom_classes = props.class.as_deref().unwrap_or("");
    
    let classes = format!("{} {} {}", base_classes, hover_classes, custom_classes);

    rsx! {
        tr {
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

#[derive(Props, Clone, PartialEq)]
pub struct TableHeadProps {
    /// Header cell content
    pub children: Element,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Column span
    #[props(default)]
    pub colspan: Option<u32>,
    /// Click handler for sortable headers
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn TableHead(props: TableHeadProps) -> Element {
    let is_clickable = props.onclick.is_some();
    let base_classes = "text-left font-semibold text-gray-900 dark:text-gray-100";
    let hover_classes = if is_clickable { "cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-600" } else { "" };
    let custom_classes = props.class.as_deref().unwrap_or("");
    
    let classes = format!("{} {} {}", base_classes, hover_classes, custom_classes);

    rsx! {
        th {
            class: "{classes}",
            colspan: props.colspan.map(|c| c.to_string()),
            onclick: move |event| {
                if let Some(handler) = &props.onclick {
                    handler.call(event);
                }
            },
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableCellProps {
    /// Cell content
    pub children: Element,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Column span
    #[props(default)]
    pub colspan: Option<u32>,
}

#[component]
pub fn TableCell(props: TableCellProps) -> Element {
    let base_classes = "text-gray-900 dark:text-gray-100";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let classes = format!("{} {}", base_classes, custom_classes);

    rsx! {
        td {
            class: "{classes}",
            colspan: props.colspan.map(|c| c.to_string()),
            {props.children}
        }
    }
}