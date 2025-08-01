//! ToggleGroup component showcase

use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{ToggleGroup, ToggleGroupItem, ToggleGroupOrientation, ToggleSize, ToggleVariant};
use dioxus::prelude::*;

#[component]
pub fn toggle_group_showcase() -> Element {
    // State for multi-select toolbar
    let mut toolbar_selection = use_signal(|| vec!["copy".to_string()]);

    // State for exclusive view mode
    let mut view_mode = use_signal(|| vec!["list".to_string()]);

    // State for text formatting
    let mut text_format = use_signal(|| vec!["bold".to_string()]);

    // State for alignment (exclusive)
    let mut text_align = use_signal(|| vec!["center".to_string()]);

    // State for file types (multi-select)
    let mut file_types = use_signal(|| vec!["pdf".to_string(), "docx".to_string()]);

    // State for vertical orientation
    let mut vertical_selection = use_signal(|| vec!["option2".to_string()]);

    rsx! {
        ComponentShowcase {
            name: "ToggleGroup".to_string(),
            description: "A group of toggle buttons that can work in multi-select or exclusive (radio-like) mode.".to_string(),
            basic_usage: r#"ToggleGroup {
    exclusive: false,
    value: selected_items,
    onchange: move |new_value| { /* handle change */ },
    ToggleGroupItem { value: "item1", "Option 1" }
    ToggleGroupItem { value: "item2", "Option 2" }
}"#.to_string(),
            with_props_usage: r#"ToggleGroup {
    exclusive: true,
    value: selected_value,
    disabled: is_disabled,
    size: ToggleSize::Small,
    variant: ToggleVariant::Outline,
    orientation: ToggleGroupOrientation::Vertical,
    onchange: move |value| handle_change(value),
    // children...
}"#.to_string(),

            ShowcaseVariant {
                title: "Multi-Select Toolbar".to_string(),

                div { class: "space-y-4",
                    ToggleGroup {
                        exclusive: false,
                        value: toolbar_selection(),
                        size: ToggleSize::Small,

                        ToggleGroupItem {
                            value: "cut",
                            pressed: toolbar_selection().contains(&"cut".to_string()),
                            size: ToggleSize::Small,
                            onclick: move |_| {
                                let mut new_value = toolbar_selection();
                                if new_value.contains(&"cut".to_string()) {
                                    new_value.retain(|v| v != "cut");
                                } else {
                                    new_value.push("cut".to_string());
                                }
                                toolbar_selection.set(new_value);
                            },
                            "Cut"
                        }
                        ToggleGroupItem {
                            value: "copy",
                            pressed: toolbar_selection().contains(&"copy".to_string()),
                            size: ToggleSize::Small,
                            onclick: move |_| {
                                let mut new_value = toolbar_selection();
                                if new_value.contains(&"copy".to_string()) {
                                    new_value.retain(|v| v != "copy");
                                } else {
                                    new_value.push("copy".to_string());
                                }
                                toolbar_selection.set(new_value);
                            },
                            "Copy"
                        }
                        ToggleGroupItem {
                            value: "paste",
                            pressed: toolbar_selection().contains(&"paste".to_string()),
                            size: ToggleSize::Small,
                            onclick: move |_| {
                                let mut new_value = toolbar_selection();
                                if new_value.contains(&"paste".to_string()) {
                                    new_value.retain(|v| v != "paste");
                                } else {
                                    new_value.push("paste".to_string());
                                }
                                toolbar_selection.set(new_value);
                            },
                            "Paste"
                        }
                        ToggleGroupItem {
                            value: "delete",
                            pressed: toolbar_selection().contains(&"delete".to_string()),
                            disabled: true,
                            size: ToggleSize::Small,
                            "Delete"
                        }
                    }

                    div { class: "text-sm text-foreground",
                        "Selected: " {format!("[{}]", toolbar_selection().join(", "))}
                    }

                    p { class: "text-sm text-foreground",
                        "Multiple items can be selected simultaneously. The disabled item cannot be toggled."
                    }
                }
            }

            ShowcaseVariant {
                title: "Exclusive Mode (Radio-like)".to_string(),

                div { class: "space-y-4",
                    ToggleGroup {
                        exclusive: true,
                        value: view_mode(),
                        variant: ToggleVariant::Outline,

                        ToggleGroupItem {
                            value: "list",
                            pressed: view_mode().contains(&"list".to_string()),
                            variant: ToggleVariant::Outline,
                            onclick: move |_| {
                                if view_mode().contains(&"list".to_string()) {
                                    view_mode.set(vec![]);  // Deselect if already selected
                                } else {
                                    view_mode.set(vec!["list".to_string()]);  // Select only this one
                                }
                            },
                            "📄 List"
                        }
                        ToggleGroupItem {
                            value: "grid",
                            pressed: view_mode().contains(&"grid".to_string()),
                            variant: ToggleVariant::Outline,
                            onclick: move |_| {
                                if view_mode().contains(&"grid".to_string()) {
                                    view_mode.set(vec![]);
                                } else {
                                    view_mode.set(vec!["grid".to_string()]);
                                }
                            },
                            "⊞ Grid"
                        }
                        ToggleGroupItem {
                            value: "card",
                            pressed: view_mode().contains(&"card".to_string()),
                            variant: ToggleVariant::Outline,
                            onclick: move |_| {
                                if view_mode().contains(&"card".to_string()) {
                                    view_mode.set(vec![]);
                                } else {
                                    view_mode.set(vec!["card".to_string()]);
                                }
                            },
                            "🃏 Card"
                        }
                    }

                    div { class: "text-sm text-foreground",
                        "Selected: " {format!("[{}]", view_mode().join(", "))}
                    }

                    p { class: "text-sm text-foreground",
                        "Only one option can be selected at a time. Clicking the selected item deselects it."
                    }
                }
            }

            ShowcaseVariant {
                title: "Text Formatting (Multi-Select)".to_string(),

                div { class: "space-y-4",
                    div { class: "flex items-center space-x-4",
                        span { class: "text-sm font-medium text-foreground", "Format:" }
                        ToggleGroup {
                            exclusive: false,
                            value: text_format(),
                            size: ToggleSize::Small,

                            ToggleGroupItem {
                                value: "bold",
                                pressed: text_format().contains(&"bold".to_string()),
                                size: ToggleSize::Small,
                                onclick: move |_| {
                                    let mut new_value = text_format();
                                    if new_value.contains(&"bold".to_string()) {
                                        new_value.retain(|v| v != "bold");
                                    } else {
                                        new_value.push("bold".to_string());
                                    }
                                    text_format.set(new_value);
                                },
                                "B"
                            }
                            ToggleGroupItem {
                                value: "italic",
                                pressed: text_format().contains(&"italic".to_string()),
                                size: ToggleSize::Small,
                                onclick: move |_| {
                                    let mut new_value = text_format();
                                    if new_value.contains(&"italic".to_string()) {
                                        new_value.retain(|v| v != "italic");
                                    } else {
                                        new_value.push("italic".to_string());
                                    }
                                    text_format.set(new_value);
                                },
                                "I"
                            }
                            ToggleGroupItem {
                                value: "underline",
                                pressed: text_format().contains(&"underline".to_string()),
                                size: ToggleSize::Small,
                                onclick: move |_| {
                                    let mut new_value = text_format();
                                    if new_value.contains(&"underline".to_string()) {
                                        new_value.retain(|v| v != "underline");
                                    } else {
                                        new_value.push("underline".to_string());
                                    }
                                    text_format.set(new_value);
                                },
                                "U"
                            }
                            ToggleGroupItem {
                                value: "strikethrough",
                                pressed: text_format().contains(&"strikethrough".to_string()),
                                size: ToggleSize::Small,
                                onclick: move |_| {
                                    let mut new_value = text_format();
                                    if new_value.contains(&"strikethrough".to_string()) {
                                        new_value.retain(|v| v != "strikethrough");
                                    } else {
                                        new_value.push("strikethrough".to_string());
                                    }
                                    text_format.set(new_value);
                                },
                                "S"
                            }
                        }

                        span { class: "text-sm font-medium text-foreground", "Align:" }
                        ToggleGroup {
                            exclusive: true,
                            value: text_align(),
                            size: ToggleSize::Small,
                            variant: ToggleVariant::Outline,

                            ToggleGroupItem {
                                value: "left",
                                pressed: text_align().contains(&"left".to_string()),
                                size: ToggleSize::Small,
                                variant: ToggleVariant::Outline,
                                onclick: move |_| {
                                    if text_align().contains(&"left".to_string()) {
                                        text_align.set(vec![]);
                                    } else {
                                        text_align.set(vec!["left".to_string()]);
                                    }
                                },
                                "⇤"
                            }
                            ToggleGroupItem {
                                value: "center",
                                pressed: text_align().contains(&"center".to_string()),
                                size: ToggleSize::Small,
                                variant: ToggleVariant::Outline,
                                onclick: move |_| {
                                    if text_align().contains(&"center".to_string()) {
                                        text_align.set(vec![]);
                                    } else {
                                        text_align.set(vec!["center".to_string()]);
                                    }
                                },
                                "≡"
                            }
                            ToggleGroupItem {
                                value: "right",
                                pressed: text_align().contains(&"right".to_string()),
                                size: ToggleSize::Small,
                                variant: ToggleVariant::Outline,
                                onclick: move |_| {
                                    if text_align().contains(&"right".to_string()) {
                                        text_align.set(vec![]);
                                    } else {
                                        text_align.set(vec!["right".to_string()]);
                                    }
                                },
                                "⇥"
                            }
                            ToggleGroupItem {
                                value: "justify",
                                pressed: text_align().contains(&"justify".to_string()),
                                size: ToggleSize::Small,
                                variant: ToggleVariant::Outline,
                                onclick: move |_| {
                                    if text_align().contains(&"justify".to_string()) {
                                        text_align.set(vec![]);
                                    } else {
                                        text_align.set(vec!["justify".to_string()]);
                                    }
                                },
                                "⟷"
                            }
                        }
                    }

                    div { class: "p-4 bg-muted rounded-md border border-border",
                        p {
                            class: format!(
                                "text-foreground {} {} {} {} {}",
                                if text_format().contains(&"bold".to_string()) { "font-bold" } else { "" },
                                if text_format().contains(&"italic".to_string()) { "italic" } else { "" },
                                if text_format().contains(&"underline".to_string()) { "underline" } else { "" },
                                if text_format().contains(&"strikethrough".to_string()) { "line-through" } else { "" },
                                match text_align().first().map(|s| s.as_str()) {
                                    Some("left") => "text-left",
                                    Some("center") => "text-center",
                                    Some("right") => "text-right",
                                    Some("justify") => "text-justify",
                                    _ => "text-left"
                                }
                            ),
                            "This text demonstrates the formatting and alignment options selected above using the toggle groups."
                        }
                    }

                    div { class: "text-xs text-foreground space-y-1",
                        div { "Formatting: " {format!("[{}]", text_format().join(", "))} }
                        div { "Alignment: " {format!("[{}]", text_align().join(", "))} }
                    }
                }
            }

            ShowcaseVariant {
                title: "File Type Filter".to_string(),

                div { class: "space-y-4",
                    div { class: "flex items-center space-x-3",
                        span { class: "text-sm font-medium text-foreground", "Show files:" }
                        ToggleGroup {
                            exclusive: false,
                            value: file_types(),
                            size: ToggleSize::Small,
                            variant: ToggleVariant::Outline,

                            ToggleGroupItem {
                                value: "pdf",
                                pressed: file_types().contains(&"pdf".to_string()),
                                size: ToggleSize::Small,
                                variant: ToggleVariant::Outline,
                                onclick: move |_| {
                                    let mut new_value = file_types();
                                    if new_value.contains(&"pdf".to_string()) {
                                        new_value.retain(|v| v != "pdf");
                                    } else {
                                        new_value.push("pdf".to_string());
                                    }
                                    file_types.set(new_value);
                                },
                                ".pdf"
                            }
                            ToggleGroupItem {
                                value: "docx",
                                pressed: file_types().contains(&"docx".to_string()),
                                size: ToggleSize::Small,
                                variant: ToggleVariant::Outline,
                                onclick: move |_| {
                                    let mut new_value = file_types();
                                    if new_value.contains(&"docx".to_string()) {
                                        new_value.retain(|v| v != "docx");
                                    } else {
                                        new_value.push("docx".to_string());
                                    }
                                    file_types.set(new_value);
                                },
                                ".docx"
                            }
                            ToggleGroupItem {
                                value: "txt",
                                pressed: file_types().contains(&"txt".to_string()),
                                size: ToggleSize::Small,
                                variant: ToggleVariant::Outline,
                                onclick: move |_| {
                                    let mut new_value = file_types();
                                    if new_value.contains(&"txt".to_string()) {
                                        new_value.retain(|v| v != "txt");
                                    } else {
                                        new_value.push("txt".to_string());
                                    }
                                    file_types.set(new_value);
                                },
                                ".txt"
                            }
                            ToggleGroupItem {
                                value: "jpg",
                                pressed: file_types().contains(&"jpg".to_string()),
                                size: ToggleSize::Small,
                                variant: ToggleVariant::Outline,
                                onclick: move |_| {
                                    let mut new_value = file_types();
                                    if new_value.contains(&"jpg".to_string()) {
                                        new_value.retain(|v| v != "jpg");
                                    } else {
                                        new_value.push("jpg".to_string());
                                    }
                                    file_types.set(new_value);
                                },
                                ".jpg"
                            }
                            ToggleGroupItem {
                                value: "png",
                                pressed: file_types().contains(&"png".to_string()),
                                size: ToggleSize::Small,
                                variant: ToggleVariant::Outline,
                                onclick: move |_| {
                                    let mut new_value = file_types();
                                    if new_value.contains(&"png".to_string()) {
                                        new_value.retain(|v| v != "png");
                                    } else {
                                        new_value.push("png".to_string());
                                    }
                                    file_types.set(new_value);
                                },
                                ".png"
                            }
                        }
                    }

                    div { class: "p-3 bg-primary/20 border border-primary rounded-md",
                        p { class: "text-sm text-primary-foreground",
                            {
                                if file_types().is_empty() {
                                    "No file types selected. All files hidden.".to_string()
                                } else {
                                    format!("Showing {} file type(s): {}",
                                        file_types().len(),
                                        file_types().join(", ")
                                    )
                                }
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Vertical Orientation".to_string(),

                div { class: "space-y-4",
                    div { class: "flex items-start space-x-6",
                        div {
                            h4 { class: "text-sm font-medium text-foreground mb-3", "Priority:" }
                            ToggleGroup {
                                exclusive: true,
                                value: vertical_selection(),
                                orientation: ToggleGroupOrientation::Vertical,
                                variant: ToggleVariant::Outline,

                                ToggleGroupItem {
                                    value: "option1",
                                    pressed: vertical_selection().contains(&"option1".to_string()),
                                    variant: ToggleVariant::Outline,
                                    orientation: ToggleGroupOrientation::Vertical,
                                    onclick: move |_| {
                                        if vertical_selection().contains(&"option1".to_string()) {
                                            vertical_selection.set(vec![]);
                                        } else {
                                            vertical_selection.set(vec!["option1".to_string()]);
                                        }
                                    },
                                    "🔥 High"
                                }
                                ToggleGroupItem {
                                    value: "option2",
                                    pressed: vertical_selection().contains(&"option2".to_string()),
                                    variant: ToggleVariant::Outline,
                                    orientation: ToggleGroupOrientation::Vertical,
                                    onclick: move |_| {
                                        if vertical_selection().contains(&"option2".to_string()) {
                                            vertical_selection.set(vec![]);
                                        } else {
                                            vertical_selection.set(vec!["option2".to_string()]);
                                        }
                                    },
                                    "⚡ Medium"
                                }
                                ToggleGroupItem {
                                    value: "option3",
                                    pressed: vertical_selection().contains(&"option3".to_string()),
                                    variant: ToggleVariant::Outline,
                                    orientation: ToggleGroupOrientation::Vertical,
                                    onclick: move |_| {
                                        if vertical_selection().contains(&"option3".to_string()) {
                                            vertical_selection.set(vec![]);
                                        } else {
                                            vertical_selection.set(vec!["option3".to_string()]);
                                        }
                                    },
                                    "💤 Low"
                                }
                            }
                        }
                    }

                    div { class: "text-sm text-foreground",
                        "Selected: " {format!("[{}]", vertical_selection().join(", "))}
                    }

                    p { class: "text-sm text-foreground",
                        "Vertical orientation works well when you need to save horizontal space."
                    }
                }
            }

            ShowcaseVariant {
                title: "Sizes and Variants".to_string(),

                div { class: "space-y-6",
                    div { class: "space-y-3",
                        h4 { class: "font-medium text-foreground", "Sizes" }
                        div { class: "space-y-3",
                            div { class: "flex items-center space-x-4",
                                span { class: "text-sm text-foreground w-16", "Small:" }
                                ToggleGroup {
                                    exclusive: true,
                                    size: ToggleSize::Small,
                                    value: vec!["s2".to_string()],
                                    ToggleGroupItem { value: "s1", pressed: false, size: ToggleSize::Small, "One" }
                                    ToggleGroupItem { value: "s2", pressed: true, size: ToggleSize::Small, "Two" }
                                    ToggleGroupItem { value: "s3", pressed: false, size: ToggleSize::Small, "Three" }
                                }
                            }
                            div { class: "flex items-center space-x-4",
                                span { class: "text-sm text-foreground w-16", "Medium:" }
                                ToggleGroup {
                                    exclusive: true,
                                    size: ToggleSize::Medium,
                                    value: vec!["m2".to_string()],
                                    ToggleGroupItem { value: "m1", pressed: false, size: ToggleSize::Medium, "One" }
                                    ToggleGroupItem { value: "m2", pressed: true, size: ToggleSize::Medium, "Two" }
                                    ToggleGroupItem { value: "m3", pressed: false, size: ToggleSize::Medium, "Three" }
                                }
                            }
                            div { class: "flex items-center space-x-4",
                                span { class: "text-sm text-foreground w-16", "Large:" }
                                ToggleGroup {
                                    exclusive: true,
                                    size: ToggleSize::Large,
                                    value: vec!["l2".to_string()],
                                    ToggleGroupItem { value: "l1", pressed: false, size: ToggleSize::Large, "One" }
                                    ToggleGroupItem { value: "l2", pressed: true, size: ToggleSize::Large, "Two" }
                                    ToggleGroupItem { value: "l3", pressed: false, size: ToggleSize::Large, "Three" }
                                }
                            }
                        }
                    }

                    div { class: "space-y-3",
                        h4 { class: "font-medium text-foreground", "Variants" }
                        div { class: "space-y-3",
                            div { class: "flex items-center space-x-4",
                                span { class: "text-sm text-foreground w-16", "Default:" }
                                ToggleGroup {
                                    exclusive: false,
                                    variant: ToggleVariant::Default,
                                    value: vec!["d1".to_string()],
                                    ToggleGroupItem { value: "d1", pressed: true, variant: ToggleVariant::Default, "Selected" }
                                    ToggleGroupItem { value: "d2", pressed: false, variant: ToggleVariant::Default, "Normal" }
                                    ToggleGroupItem { value: "d3", pressed: false, disabled: true, variant: ToggleVariant::Default, "Disabled" }
                                }
                            }
                            div { class: "flex items-center space-x-4",
                                span { class: "text-sm text-foreground w-16", "Outline:" }
                                ToggleGroup {
                                    exclusive: false,
                                    variant: ToggleVariant::Outline,
                                    value: vec!["o1".to_string()],
                                    ToggleGroupItem { value: "o1", pressed: true, variant: ToggleVariant::Outline, "Selected" }
                                    ToggleGroupItem { value: "o2", pressed: false, variant: ToggleVariant::Outline, "Normal" }
                                    ToggleGroupItem { value: "o3", pressed: false, disabled: true, variant: ToggleVariant::Outline, "Disabled" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
