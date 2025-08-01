//! Accordion component showcase

use dioxus::prelude::*;
use crate::{Accordion, AccordionItem};
use super::super::{ComponentShowcase, ShowcaseVariant};

#[component]
pub fn accordion_showcase() -> Element {
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
                                    class: "text-gray-700 dark:text-gray-300",
                                    "Welcome to the accordion component! This section contains basic information about getting started."
                                }
                                ul {
                                    class: "list-disc list-inside space-y-1 text-gray-600 dark:text-gray-400 ml-4",
                                    li { "Only one section can be open at a time" }
                                    li { "Click any section to expand it" }
                                    li { "Opening a section automatically closes others" }
                                }
                            }
                        }),
                        AccordionItem::new("basic2", "Features", rsx! {
                            div { class: "space-y-3",
                                p {
                                    class: "text-gray-700 dark:text-gray-300",
                                    "The accordion component comes with several built-in features:"
                                }
                                div {
                                    class: "bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-md p-3",
                                    h4 {
                                        class: "font-semibold text-blue-800 dark:text-blue-200 mb-2",
                                        "Key Features:"
                                    }
                                    ul {
                                        class: "list-disc list-inside space-y-1 text-blue-700 dark:text-blue-300 text-sm",
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
                                    class: "text-gray-700 dark:text-gray-300",
                                    "Implementing an accordion is straightforward using AccordionItem structs:"
                                }
                                div {
                                    class: "bg-gray-100 dark:bg-gray-800 rounded-md p-4 font-mono text-sm",
                                    code {
                                        class: "text-gray-800 dark:text-gray-200",
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
                                class: "text-gray-700 dark:text-gray-300",
                                "This section starts closed."
                            }
                        }),
                        AccordionItem::new("default2", "Initially Open Section", rsx! {
                            div { class: "space-y-2",
                                p {
                                    class: "text-gray-700 dark:text-gray-300",
                                    "This section starts open by setting the default_open prop to this item's ID."
                                }
                                div {
                                    class: "bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-md p-3",
                                    p {
                                        class: "text-green-800 dark:text-green-200 text-sm",
                                        "💡 Tip: Use default_open to highlight important content."
                                    }
                                }
                            }
                        }),
                        AccordionItem::new("default3", "Another Section", rsx! {
                            p {
                                class: "text-gray-700 dark:text-gray-300",
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
                                    class: "text-gray-700 dark:text-gray-300",
                                    "In non-collapsible mode, at least one section must always remain open."
                                }
                                p {
                                    class: "text-gray-600 dark:text-gray-400 text-sm italic",
                                    "Try clicking on this section's header - it won't close because this is the only open section."
                                }
                            }
                        }),
                        AccordionItem::new("nocollapse2", "Another Section", rsx! {
                            p {
                                class: "text-gray-700 dark:text-gray-300",
                                "Click this to open it and close the other section."
                            }
                        }),
                        AccordionItem::new("nocollapse3", "Third Section", rsx! {
                            p {
                                class: "text-gray-700 dark:text-gray-300",
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
                                class: "text-gray-700 dark:text-gray-300",
                                "This section is fully functional and can be opened."
                            }
                        }),
                        AccordionItem::new("disabled2", "Disabled Section", rsx! {
                            p {
                                class: "text-gray-700 dark:text-gray-300",
                                "This content cannot be accessed because the section is disabled."
                            }
                        }).disabled(),
                        AccordionItem::new("disabled3", "Coming Soon", rsx! {
                            p {
                                class: "text-gray-700 dark:text-gray-300",
                                "This feature is under development."
                            }
                        }).disabled(),
                        AccordionItem::new("disabled4", "Another Available Section", rsx! {
                            div { class: "space-y-2",
                                p {
                                    class: "text-gray-700 dark:text-gray-300",
                                    "This section works normally. Notice how disabled sections appear dimmed and cannot be clicked."
                                }
                                p {
                                    class: "text-gray-600 dark:text-gray-400 text-sm",
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
                                        label {
                                            class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                            "Project Name:"
                                        }
                                        input {
                                            r#type: "text",
                                            placeholder: "My Awesome Project",
                                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                        }
                                    }
                                    div {
                                        label {
                                            class: "block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1",
                                            "Description:"
                                        }
                                        textarea {
                                            placeholder: "Enter project description...",
                                            rows: "3",
                                            class: "w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
                                        }
                                    }
                                }
                                div { class: "flex items-center space-x-2",
                                    input {
                                        r#type: "checkbox",
                                        id: "accordion-public",
                                        class: "rounded"
                                    }
                                    label {
                                        r#for: "accordion-public",
                                        class: "text-sm text-gray-700 dark:text-gray-300",
                                        "Make this project public"
                                    }
                                }
                            }
                        }),
                        AccordionItem::new("rich2", "🔧 Build Configuration", rsx! {
                            div { class: "space-y-3",
                                p {
                                    class: "text-gray-700 dark:text-gray-300 text-sm",
                                    "Configure how your project is built and deployed:"
                                }
                                div { class: "space-y-3",
                                    div { class: "flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-md",
                                        div {
                                            h4 { class: "font-medium text-gray-900 dark:text-gray-100", "Auto-deploy" }
                                            p { class: "text-sm text-gray-600 dark:text-gray-400", "Deploy on every push to main branch" }
                                        }
                                        input { r#type: "checkbox", class: "rounded" }
                                    }
                                    div { class: "flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-md",
                                        div {
                                            h4 { class: "font-medium text-gray-900 dark:text-gray-100", "Run tests" }
                                            p { class: "text-sm text-gray-600 dark:text-gray-400", "Execute test suite before deployment" }
                                        }
                                        input { r#type: "checkbox", checked: true, class: "rounded" }
                                    }
                                }
                            }
                        }),
                        AccordionItem::new("rich3", "📊 Analytics & Monitoring", rsx! {
                            div { class: "space-y-4",
                                div { class: "bg-gradient-to-r from-purple-50 to-pink-50 dark:from-purple-900/20 dark:to-pink-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-4",
                                    h4 {
                                        class: "font-semibold text-purple-800 dark:text-purple-200 mb-2",
                                        "📈 Performance Metrics"
                                    }
                                    div { class: "grid grid-cols-3 gap-4 text-center",
                                        div {
                                            div { class: "text-2xl font-bold text-purple-600 dark:text-purple-400", "98%" }
                                            div { class: "text-sm text-purple-700 dark:text-purple-300", "Uptime" }
                                        }
                                        div {
                                            div { class: "text-2xl font-bold text-purple-600 dark:text-purple-400", "1.2s" }
                                            div { class: "text-sm text-purple-700 dark:text-purple-300", "Load Time" }
                                        }
                                        div {
                                            div { class: "text-2xl font-bold text-purple-600 dark:text-purple-400", "99%" }
                                            div { class: "text-sm text-purple-700 dark:text-purple-300", "Success Rate" }
                                        }
                                    }
                                }
                                p {
                                    class: "text-gray-600 dark:text-gray-400 text-sm",
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