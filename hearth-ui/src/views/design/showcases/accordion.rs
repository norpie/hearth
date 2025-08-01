//! Accordion component showcase

use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Accordion, AccordionItem, Checkbox, Input, Label, Textarea};
use dioxus::prelude::*;

#[component]
pub fn accordion_showcase() -> Element {
    let mut project_name = use_signal(|| "My Awesome Project".to_string());
    let mut project_description = use_signal(String::new);
    let mut make_public = use_signal(|| false);
    let mut auto_deploy = use_signal(|| false);
    let mut run_tests = use_signal(|| true);

    rsx! {
        ComponentShowcase {
            name: "Accordion".to_string(),
            description: "Exclusive collapsible sections where only one can be open at a time.".to_string(),
            basic_usage: r#"Accordion {
    items: vec![
        AccordionItem::new("item1", "Section 1", rsx! { p { "Content 1" } }),
        AccordionItem::new("item2", "Section 2", rsx! { p { "Content 2" } }),
    ]
}"#.to_string(),
            with_props_usage: r#"Accordion {
    items: accordion_items,
    default_open: Some("item2".to_string()),
    collapsible: false
}"#.to_string(),

            ShowcaseVariant {
                title: "Basic Usage".to_string(),

                Accordion {
                    items: vec![
                        AccordionItem::new("basic1", "Getting Started", rsx! {
                            div { class: "space-y-3",
                                p {
                                    class: "text-foreground",
                                    "Welcome to the accordion component! This section contains basic information about getting started."
                                }
                                ul {
                                    class: "list-disc list-inside space-y-1 text-foreground ml-4",
                                    li { "Only one section can be open at a time" }
                                    li { "Click any section to expand it" }
                                    li { "Opening a section automatically closes others" }
                                }
                            }
                        }),
                        AccordionItem::new("basic2", "Features", rsx! {
                            div { class: "space-y-3",
                                p {
                                    class: "text-foreground",
                                    "The accordion component comes with several built-in features:"
                                }
                                div {
                                    class: "bg-primary/20 border border-primary rounded-md p-3",
                                    h4 {
                                        class: "font-semibold text-primary-foreground mb-2",
                                        "Key Features:"
                                    }
                                    ul {
                                        class: "list-disc list-inside space-y-1 text-primary-foreground text-sm",
                                        li { "Exclusive expansion - only one open at a time" }
                                        li { "Smooth CSS animations" }
                                        li { "Keyboard accessible" }
                                        li { "Customizable styling" }
                                        li { "Disabled state support" }
                                    }
                                }
                            }
                        }),
                        AccordionItem::new("basic3", "Implementation", rsx! {
                            div { class: "space-y-3",
                                p {
                                    class: "text-foreground",
                                    "Implementing an accordion is straightforward using AccordionItem structs:"
                                }
                                div {
                                    class: "bg-muted rounded-md p-4 font-mono text-sm",
                                    code {
                                        class: "text-foreground",
                                        "let items = vec![\n  AccordionItem::new(\"id\", \"Title\", content),\n  // more items...\n];"
                                    }
                                }
                            }
                        }),
                    ]
                }
            }

            ShowcaseVariant {
                title: "Default Open".to_string(),

                Accordion {
                    items: vec![
                        AccordionItem::new("default1", "Closed Section", rsx! {
                            p {
                                class: "text-foreground",
                                "This section starts closed."
                            }
                        }),
                        AccordionItem::new("default2", "Initially Open Section", rsx! {
                            div { class: "space-y-2",
                                p {
                                    class: "text-foreground",
                                    "This section starts open by setting the default_open prop to this item's ID. "
                                    span { class: "text-success font-bold", "GREEN TEST" }
                                }
                                div {
                                    class: "bg-success/10 border border-success/20 rounded-md p-3",
                                    p {
                                        class: "text-success-foreground text-sm",
                                        "💡 Tip: Use default_open to highlight important content."
                                    }
                                }
                            }
                        }),
                        AccordionItem::new("default3", "Another Section", rsx! {
                            p {
                                class: "text-foreground",
                                "This section is also closed by default."
                            }
                        }),
                    ],
                    default_open: Some("default2".to_string())
                }
            }

            ShowcaseVariant {
                title: "Non-Collapsible Mode".to_string(),

                Accordion {
                    items: vec![
                        AccordionItem::new("nocollapse1", "Always One Open", rsx! {
                            div { class: "space-y-2",
                                p {
                                    class: "text-foreground",
                                    "In non-collapsible mode, at least one section must always remain open."
                                }
                                p {
                                    class: "text-foreground text-sm italic",
                                    "Try clicking on this section's header - it won't close because this is the only open section."
                                }
                            }
                        }),
                        AccordionItem::new("nocollapse2", "Another Section", rsx! {
                            p {
                                class: "text-foreground",
                                "Click this to open it and close the other section."
                            }
                        }),
                        AccordionItem::new("nocollapse3", "Third Section", rsx! {
                            p {
                                class: "text-foreground",
                                "This ensures one section is always visible for important navigation."
                            }
                        }),
                    ],
                    default_open: Some("nocollapse1".to_string()),
                    collapsible: false
                }
            }

            ShowcaseVariant {
                title: "Disabled Items".to_string(),

                Accordion {
                    items: vec![
                        AccordionItem::new("disabled1", "Available Section", rsx! {
                            p {
                                class: "text-foreground",
                                "This section is fully functional and can be opened."
                            }
                        }),
                        AccordionItem::new("disabled2", "Disabled Section", rsx! {
                            p {
                                class: "text-foreground",
                                "This content cannot be accessed because the section is disabled."
                            }
                        }).disabled(),
                        AccordionItem::new("disabled3", "Coming Soon", rsx! {
                            p {
                                class: "text-foreground",
                                "This feature is under development."
                            }
                        }).disabled(),
                        AccordionItem::new("disabled4", "Another Available Section", rsx! {
                            div { class: "space-y-2",
                                p {
                                    class: "text-foreground",
                                    "This section works normally. Notice how disabled sections appear dimmed and cannot be clicked."
                                }
                                p {
                                    class: "text-foreground text-sm",
                                    "Disabled items are useful for showing unavailable features or work-in-progress content."
                                }
                            }
                        }),
                    ]
                }
            }

            ShowcaseVariant {
                title: "Rich Content Example".to_string(),

                Accordion {
                    items: vec![
                        AccordionItem::new("rich1", "📋 Project Settings", rsx! {
                            div { class: "space-y-4",
                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                    div {
                                        Label { "Project Name:" }
                                        Input {
                                            placeholder: "My Awesome Project",
                                            value: project_name.read().clone(),
                                            oninput: move |value: String| project_name.set(value),
                                        }
                                    }
                                    div {
                                        Label { "Description:" }
                                        Textarea {
                                            placeholder: "Enter project description...",
                                            rows: 3,
                                            value: project_description.read().clone(),
                                            oninput: move |value: String| project_description.set(value),
                                        }
                                    }
                                }
                                div { class: "flex items-center space-x-2",
                                    Checkbox {
                                        checked: *make_public.read(),
                                        onchange: move |checked| make_public.set(checked),
                                    }
                                    Label { "Make this project public" }
                                }
                            }
                        }),
                        AccordionItem::new("rich2", "🔧 Build Configuration", rsx! {
                            div { class: "space-y-3",
                                p {
                                    class: "text-foreground text-sm",
                                    "Configure how your project is built and deployed:"
                                }
                                div { class: "space-y-3",
                                    div { class: "flex items-center justify-between p-3 bg-muted rounded-md",
                                        div {
                                            h4 { class: "font-medium text-foreground", "Auto-deploy" }
                                            p { class: "text-sm text-foreground", "Deploy on every push to main branch" }
                                        }
                                        Checkbox {
                                            checked: *auto_deploy.read(),
                                            onchange: move |checked| auto_deploy.set(checked),
                                        }
                                    }
                                    div { class: "flex items-center justify-between p-3 bg-muted rounded-md",
                                        div {
                                            h4 { class: "font-medium text-foreground", "Run tests" }
                                            p { class: "text-sm text-foreground", "Execute test suite before deployment" }
                                        }
                                        Checkbox {
                                            checked: *run_tests.read(),
                                            onchange: move |checked| run_tests.set(checked),
                                        }
                                    }
                                }
                            }
                        }),
                        AccordionItem::new("rich3", "📊 Analytics & Monitoring", rsx! {
                            div { class: "space-y-4",
                                div { class: "bg-gradient-to-r from-accent/5 to-accent/10 border border-accent/20 rounded-lg p-4",
                                    h4 {
                                        class: "font-semibold text-accent-foreground mb-2",
                                        "📈 Performance Metrics"
                                    }
                                    div { class: "grid grid-cols-3 gap-4 text-center",
                                        div {
                                            div { class: "text-2xl font-bold text-accent-foreground", "98%" }
                                            div { class: "text-sm text-accent-foreground", "Uptime" }
                                        }
                                        div {
                                            div { class: "text-2xl font-bold text-accent-foreground", "1.2s" }
                                            div { class: "text-sm text-accent-foreground", "Load Time" }
                                        }
                                        div {
                                            div { class: "text-2xl font-bold text-accent-foreground", "99%" }
                                            div { class: "text-sm text-accent-foreground", "Success Rate" }
                                        }
                                    }
                                }
                                p {
                                    class: "text-foreground text-sm",
                                    "Monitor your application's performance, user engagement, and system health in real-time."
                                }
                            }
                        }),
                    ],
                    default_open: Some("rich1".to_string())
                }
            }
        }
    }
}
