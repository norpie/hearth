use crate::components::{
    Badge, BadgeVariant, Table, TableBody, TableCell, TableHead, TableHeader, TableRow, TableSize,
    TableVariant,
};
use crate::views::design::showcase::{ComponentShowcase, ShowcaseVariant};
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct Employee {
    name: String,
    email: String,
    role: String,
    status: String,
}

#[derive(Clone, Copy, PartialEq)]
enum SortBy {
    Name,
    Email,
    Role,
    Status,
}

#[derive(Clone, Copy, PartialEq)]
enum SortOrder {
    Asc,
    Desc,
}

#[component]
pub fn TableShowcase() -> Element {
    // Sortable data state
    let mut employees = use_signal(|| {
        vec![
            Employee {
                name: "Alice Johnson".to_string(),
                email: "alice@example.com".to_string(),
                role: "Admin".to_string(),
                status: "Active".to_string(),
            },
            Employee {
                name: "Bob Smith".to_string(),
                email: "bob@example.com".to_string(),
                role: "User".to_string(),
                status: "Pending".to_string(),
            },
            Employee {
                name: "Carol Wilson".to_string(),
                email: "carol@example.com".to_string(),
                role: "Editor".to_string(),
                status: "Inactive".to_string(),
            },
            Employee {
                name: "David Brown".to_string(),
                email: "david@example.com".to_string(),
                role: "User".to_string(),
                status: "Active".to_string(),
            },
        ]
    });

    let mut sort_by = use_signal(|| SortBy::Name);
    let mut sort_order = use_signal(|| SortOrder::Asc);

    // Sort function
    let mut sort_employees = move |field: SortBy| {
        let current_sort = *sort_by.read();
        let current_order = *sort_order.read();

        // Toggle order if clicking the same column, otherwise default to Asc
        let new_order = if current_sort == field {
            match current_order {
                SortOrder::Asc => SortOrder::Desc,
                SortOrder::Desc => SortOrder::Asc,
            }
        } else {
            SortOrder::Asc
        };

        sort_by.set(field);
        sort_order.set(new_order);

        let mut sorted_employees = employees.read().clone();
        sorted_employees.sort_by(|a, b| {
            let comparison = match field {
                SortBy::Name => a.name.cmp(&b.name),
                SortBy::Email => a.email.cmp(&b.email),
                SortBy::Role => a.role.cmp(&b.role),
                SortBy::Status => a.status.cmp(&b.status),
            };

            match new_order {
                SortOrder::Asc => comparison,
                SortOrder::Desc => comparison.reverse(),
            }
        });

        employees.set(sorted_employees);
    };

    // Helper to get sort indicator
    let get_sort_indicator = move |field: SortBy| {
        if *sort_by.read() == field {
            match *sort_order.read() {
                SortOrder::Asc => " ↑",
                SortOrder::Desc => " ↓",
            }
        } else {
            " ↕"
        }
    };
    rsx! {
        ComponentShowcase {
            name: "Table".to_string(),
            description: "Display structured data in rows and columns".to_string(),
            basic_usage: r#"rsx! {
    Table {
        TableHeader {
            TableRow {
                TableHead { "Name" }
                TableHead { "Status" }
                TableHead { "Actions" }
            }
        }
        TableBody {
            TableRow {
                TableCell { "John Doe" }
                TableCell { Badge { variant: BadgeVariant::Success, "Active" } }
                TableCell { "Edit" }
            }
        }
    }
}"#.to_string(),
            with_props_usage: r#"rsx! {
    Table {
        variant: TableVariant::Minimal,
        size: TableSize::Large,
        TableHeader {
            TableRow {
                TableHead { 
                    onclick: move |_| sort_employees(SortBy::Name),
                    "Sortable Name ↕"
                }
                TableHead { "Status" }
            }
        }
        TableBody {
            TableRow {
                onclick: |_| { /* Row click handler */ },
                TableCell { "John Doe" }
                TableCell { Badge { variant: BadgeVariant::Success, "Active" } }
            }
        }
    }
}"#.to_string(),

            ShowcaseVariant {
                title: "Sortable Table".to_string(),
                Table {
                    TableHeader {
                        TableRow {
                            TableHead {
                                onclick: move |_| sort_employees(SortBy::Name),
                                "Name{get_sort_indicator(SortBy::Name)}"
                            }
                            TableHead {
                                onclick: move |_| sort_employees(SortBy::Email),
                                "Email{get_sort_indicator(SortBy::Email)}"
                            }
                            TableHead {
                                onclick: move |_| sort_employees(SortBy::Role),
                                "Role{get_sort_indicator(SortBy::Role)}"
                            }
                            TableHead {
                                onclick: move |_| sort_employees(SortBy::Status),
                                "Status{get_sort_indicator(SortBy::Status)}"
                            }
                        }
                    }
                    TableBody {
                        {employees.read().iter().map(|employee| {
                            let status_variant = match employee.status.as_str() {
                                "Active" => BadgeVariant::Success,
                                "Pending" => BadgeVariant::Warning,
                                "Inactive" => BadgeVariant::Error,
                                _ => BadgeVariant::Default,
                            };

                            rsx! {
                                TableRow {
                                    key: "{employee.name}",
                                    TableCell { "{employee.name}" }
                                    TableCell { "{employee.email}" }
                                    TableCell { "{employee.role}" }
                                    TableCell { Badge { variant: status_variant, "{employee.status}" } }
                                }
                            }
                        })}
                    }
                }
            }

            ShowcaseVariant {
                title: "Striped Table".to_string(),
                Table {
                    variant: TableVariant::Striped,
                    TableHeader {
                        TableRow {
                            TableHead { "Product" }
                            TableHead { "Price" }
                            TableHead { "Stock" }
                            TableHead { "Category" }
                        }
                    }
                    TableBody {
                        TableRow {
                            TableCell { "Laptop" }
                            TableCell { "$999.99" }
                            TableCell { "15" }
                            TableCell { Badge { variant: BadgeVariant::Info, "Electronics" } }
                        }
                        TableRow {
                            TableCell { "Mouse" }
                            TableCell { "$29.99" }
                            TableCell { "50" }
                            TableCell { Badge { variant: BadgeVariant::Info, "Electronics" } }
                        }
                        TableRow {
                            TableCell { "Keyboard" }
                            TableCell { "$79.99" }
                            TableCell { "25" }
                            TableCell { Badge { variant: BadgeVariant::Info, "Electronics" } }
                        }
                        TableRow {
                            TableCell { "Monitor" }
                            TableCell { "$299.99" }
                            TableCell { "8" }
                            TableCell { Badge { variant: BadgeVariant::Warning, "Low Stock" } }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Bordered Table".to_string(),
                Table {
                    variant: TableVariant::Bordered,
                    TableHeader {
                        TableRow {
                            TableHead { "Quarter" }
                            TableHead { "Revenue" }
                            TableHead { "Growth" }
                            TableHead { "Target" }
                        }
                    }
                    TableBody {
                        TableRow {
                            TableCell { "Q1 2024" }
                            TableCell { "$125,000" }
                            TableCell { Badge { variant: BadgeVariant::Success, "+12%" } }
                            TableCell { "$120,000" }
                        }
                        TableRow {
                            TableCell { "Q2 2024" }
                            TableCell { "$138,500" }
                            TableCell { Badge { variant: BadgeVariant::Success, "+8%" } }
                            TableCell { "$135,000" }
                        }
                        TableRow {
                            TableCell { "Q3 2024" }
                            TableCell { "$145,200" }
                            TableCell { Badge { variant: BadgeVariant::Success, "+5%" } }
                            TableCell { "$145,000" }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Hoverable & Clickable Table".to_string(),
                Table {
                    variant: TableVariant::Hoverable,
                    TableHeader {
                        TableRow {
                            TableHead {
                                onclick: move |_| {
                                    #[cfg(target_arch = "wasm32")]
                                    web_sys::console::log_1(&"Sort by Name clicked!".into());
                                    #[cfg(not(target_arch = "wasm32"))]
                                    println!("Sort by Name clicked!");
                                },
                                "Name ↕"
                            }
                            TableHead {
                                onclick: move |_| {
                                    #[cfg(target_arch = "wasm32")]
                                    web_sys::console::log_1(&"Sort by Department clicked!".into());
                                    #[cfg(not(target_arch = "wasm32"))]
                                    println!("Sort by Department clicked!");
                                },
                                "Department ↕"
                            }
                            TableHead { "Actions" }
                        }
                    }
                    TableBody {
                        TableRow {
                            onclick: move |_| {
                                #[cfg(target_arch = "wasm32")]
                                web_sys::console::log_1(&"Row 1 clicked!".into());
                                #[cfg(not(target_arch = "wasm32"))]
                                println!("Row 1 clicked!");
                            },
                            TableCell { "Emma Davis" }
                            TableCell { "Engineering" }
                            TableCell { "Edit • Delete" }
                        }
                        TableRow {
                            onclick: move |_| {
                                #[cfg(target_arch = "wasm32")]
                                web_sys::console::log_1(&"Row 2 clicked!".into());
                                #[cfg(not(target_arch = "wasm32"))]
                                println!("Row 2 clicked!");
                            },
                            TableCell { "Michael Brown" }
                            TableCell { "Design" }
                            TableCell { "Edit • Delete" }
                        }
                        TableRow {
                            onclick: move |_| {
                                #[cfg(target_arch = "wasm32")]
                                web_sys::console::log_1(&"Row 3 clicked!".into());
                                #[cfg(not(target_arch = "wasm32"))]
                                println!("Row 3 clicked!");
                            },
                            TableCell { "Sarah Connor" }
                            TableCell { "Marketing" }
                            TableCell { "Edit • Delete" }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Different Sizes".to_string(),
                div { class: "space-y-6",
                    div {
                        h4 { class: "text-sm font-medium mb-2", "Small Table" }
                        Table {
                            size: TableSize::Small,
                            variant: TableVariant::Striped,
                            TableHeader {
                                TableRow {
                                    TableHead { "Item" }
                                    TableHead { "Qty" }
                                    TableHead { "Price" }
                                }
                            }
                            TableBody {
                                TableRow {
                                    TableCell { "Widget A" }
                                    TableCell { "5" }
                                    TableCell { "$25.00" }
                                }
                                TableRow {
                                    TableCell { "Widget B" }
                                    TableCell { "3" }
                                    TableCell { "$15.00" }
                                }
                            }
                        }
                    }

                    div {
                        h4 { class: "text-sm font-medium mb-2", "Large Table" }
                        Table {
                            size: TableSize::Large,
                            variant: TableVariant::Bordered,
                            TableHeader {
                                TableRow {
                                    TableHead { "Project" }
                                    TableHead { "Status" }
                                    TableHead { "Deadline" }
                                }
                            }
                            TableBody {
                                TableRow {
                                    TableCell { "Website Redesign" }
                                    TableCell { Badge { variant: BadgeVariant::Success, "Completed" } }
                                    TableCell { "Dec 15, 2024" }
                                }
                                TableRow {
                                    TableCell { "Mobile App" }
                                    TableCell { Badge { variant: BadgeVariant::Warning, "In Progress" } }
                                    TableCell { "Jan 30, 2025" }
                                }
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Minimal Table".to_string(),
                Table {
                    variant: TableVariant::Minimal,
                    TableHeader {
                        TableRow {
                            TableHead { "Feature" }
                            TableHead { "Basic" }
                            TableHead { "Pro" }
                            TableHead { "Enterprise" }
                        }
                    }
                    TableBody {
                        TableRow {
                            TableCell { "Users" }
                            TableCell { "5" }
                            TableCell { "50" }
                            TableCell { "Unlimited" }
                        }
                        TableRow {
                            TableCell { "Storage" }
                            TableCell { "1GB" }
                            TableCell { "100GB" }
                            TableCell { "1TB" }
                        }
                        TableRow {
                            TableCell { "Support" }
                            TableCell { Badge { variant: BadgeVariant::Default, "Email" } }
                            TableCell { Badge { variant: BadgeVariant::Info, "Priority" } }
                            TableCell { Badge { variant: BadgeVariant::Success, "24/7" } }
                        }
                        TableRow {
                            TableCell { "API Access" }
                            TableCell { Badge { variant: BadgeVariant::Error, "No" } }
                            TableCell { Badge { variant: BadgeVariant::Success, "Yes" } }
                            TableCell { Badge { variant: BadgeVariant::Success, "Yes" } }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "With Colspan".to_string(),
                Table {
                    variant: TableVariant::Bordered,
                    TableHeader {
                        TableRow {
                            TableHead { "Item" }
                            TableHead { colspan: 2, "Details" }
                            TableHead { "Total" }
                        }
                    }
                    TableBody {
                        TableRow {
                            TableCell { "Product A" }
                            TableCell { "Qty: 2" }
                            TableCell { "Price: $50" }
                            TableCell { "$100" }
                        }
                        TableRow {
                            TableCell { "Product B" }
                            TableCell { colspan: 2, "Out of Stock" }
                            TableCell { "-" }
                        }
                        TableRow {
                            TableCell { colspan: 3, class: "font-semibold", "Subtotal" }
                            TableCell { class: "font-semibold", "$100" }
                        }
                    }
                }
            }
        }
    }
}
