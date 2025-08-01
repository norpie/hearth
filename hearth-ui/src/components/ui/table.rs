//! Table component for tabular data display
//!
//! Basic usage:
//! ```rust
//! rsx! {
//!     Table {
//!         variant: TableVariant::Default,
//!         TableHeader {
//!             TableRow {
//!                 TableHead { "Name" }
//!                 TableHead { "Email" }
//!             }
//!         }
//!         TableBody {
//!             TableRow {
//!                 TableCell { "John" }
//!                 TableCell { "john@example.com" }
//!             }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Properties for the Table component
#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    /// Table content including headers and body
    pub children: Element,
    /// Visual style variant
    #[props(default = TableVariant::Default)]
    pub variant: TableVariant,
    /// Size variant for spacing and typography
    #[props(default = TableSize::Medium)]
    pub size: TableSize,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Visual style variants for table appearance
#[derive(Clone, Copy, PartialEq)]
pub enum TableVariant {
    /// Default clean table styling
    Default,
    /// Striped rows for improved readability
    Striped,
    /// Full borders around all table elements
    Bordered,
    /// Interactive row hover effects
    Hoverable,
    /// Minimal styling with subtle borders
    Minimal,
}

impl TableVariant {
    /// Returns CSS classes for the variant
    pub fn class(&self) -> &'static str {
        match self {
            TableVariant::Default => "",
            TableVariant::Striped => "[&_tbody_tr:nth-child(odd)]:bg-muted[&_tbody_tr:nth-child(odd)]:bg-muted/50",
            TableVariant::Bordered => "border border-border [&_th]:border-r [&_th]:border-border [&_td]:border-r [&_td]:border-border [&_tr]:border-b [&_tr]:border-border[&_tr]:border-border",
            TableVariant::Hoverable => "[&_tbody_tr]:hover:bg-muted [&_tbody_tr]:transition-colors",
            TableVariant::Minimal => "[&_th]:border-r [&_th]:border-border [&_td]:border-r [&_td]:border-border [&_tbody_tr:not(:last-child)]:border-b [&_tbody_tr:not(:last-child)]:border-border [&_th:last-child]:border-r-0 [&_td:last-child]:border-r-0",
        }
    }
}

/// Size variants for table spacing and typography
#[derive(Clone, Copy, PartialEq)]
pub enum TableSize {
    /// Small size for dense data display
    Small,
    /// Medium size for standard display
    Medium,
    /// Large size for prominent display
    Large,
}

impl TableSize {
    /// Returns CSS classes for the size
    pub fn class(&self) -> &'static str {
        match self {
            TableSize::Small => "[&_th]:px-2 [&_th]:py-1 [&_td]:px-2 [&_td]:py-1 text-sm",
            TableSize::Medium => "[&_th]:px-4 [&_th]:py-2 [&_td]:px-4 [&_td]:py-2",
            TableSize::Large => "[&_th]:px-6 [&_th]:py-3 [&_td]:px-6 [&_td]:py-3 text-lg",
        }
    }
}

/// Table component for data display with responsive behavior
#[component]
pub fn Table(props: TableProps) -> Element {
    let base_classes = "w-full border-collapse";
    let variant_classes = props.variant.class();
    let size_classes = props.size.class();
    let custom_classes = props.class.as_deref().unwrap_or("");

    let classes = format!("{base_classes} {variant_classes} {size_classes} {custom_classes}");

    rsx! {
        div { class: "overflow-x-auto",
            table { class: "{classes}", {props.children} }
        }
    }
}

/// Properties for the TableHeader component
#[derive(Props, Clone, PartialEq)]
pub struct TableHeaderProps {
    /// Header content including TableRow and TableHead components
    pub children: Element,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Table header component for column definitions
#[component]
pub fn TableHeader(props: TableHeaderProps) -> Element {
    let base_classes = "bg-muted";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let classes = format!("{base_classes} {custom_classes}");

    rsx! {
        thead { class: "{classes}", {props.children} }
    }
}

/// Properties for the TableBody component
#[derive(Props, Clone, PartialEq)]
pub struct TableBodyProps {
    /// Body content including TableRow and TableCell components
    pub children: Element,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
}

/// Table body component for data rows
#[component]
pub fn TableBody(props: TableBodyProps) -> Element {
    let base_classes = "bg-background";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let classes = format!("{base_classes} {custom_classes}");

    rsx! {
        tbody { class: "{classes}", {props.children} }
    }
}

/// Properties for the TableRow component
#[derive(Props, Clone, PartialEq)]
pub struct TableRowProps {
    /// Row content including TableHead or TableCell components
    pub children: Element,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Optional click event handler for interactive rows
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

/// Table row component for organizing cells
#[component]
pub fn TableRow(props: TableRowProps) -> Element {
    let is_clickable = props.onclick.is_some();
    let base_classes = "";
    let hover_classes = if is_clickable {
        "cursor-pointer hover:bg-muted"
    } else {
        ""
    };
    let custom_classes = props.class.as_deref().unwrap_or("");

    let classes = format!("{base_classes} {hover_classes} {custom_classes}");

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

/// Properties for the TableHead component
#[derive(Props, Clone, PartialEq)]
pub struct TableHeadProps {
    /// Header cell content
    pub children: Element,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Number of columns to span
    #[props(default)]
    pub colspan: Option<u32>,
    /// Optional click event handler
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
}

/// Table header cell component
#[component]
pub fn TableHead(props: TableHeadProps) -> Element {
    let is_clickable = props.onclick.is_some();
    let base_classes = "text-left font-semibold text-foreground";
    let hover_classes = if is_clickable {
        "cursor-pointer hover:bg-muted"
    } else {
        ""
    };
    let custom_classes = props.class.as_deref().unwrap_or("");

    let classes = format!("{base_classes} {hover_classes} {custom_classes}");

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

/// Properties for the TableCell component
#[derive(Props, Clone, PartialEq)]
pub struct TableCellProps {
    /// Cell content
    pub children: Element,
    /// Additional CSS classes
    #[props(default)]
    pub class: Option<String>,
    /// Number of columns to span
    #[props(default)]
    pub colspan: Option<u32>,
}

/// Table data cell component
#[component]
pub fn TableCell(props: TableCellProps) -> Element {
    let base_classes = "text-foreground";
    let custom_classes = props.class.as_deref().unwrap_or("");
    let classes = format!("{base_classes} {custom_classes}");

    rsx! {
        td { class: "{classes}", colspan: props.colspan.map(|c| c.to_string()), {props.children} }
    }
}
