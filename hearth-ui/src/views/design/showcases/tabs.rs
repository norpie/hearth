use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{
    Button, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Checkbox, Input,
    InputType, Label, Switch, Tabs, TabsContent, TabsList, TabsSize, TabsTrigger, TabsVariant,
};
use dioxus::prelude::*;

#[component]
pub fn tabs_showcase() -> Element {
    rsx! {
        ComponentShowcase {
            name: "Tabs".to_string(),
            description: "Organize content into switchable panels with keyboard navigation support.".to_string(),
            basic_usage: r#"Tabs {
    default_value: 0,
    TabsList {
        TabsTrigger { value: 0, "Tab 1" }
        TabsTrigger { value: 1, "Tab 2" }
        TabsTrigger { value: 2, "Tab 3" }
    }
    TabsContent { value: 0, 
        p { "Content for Tab 1" }
    }
    TabsContent { value: 1,
        p { "Content for Tab 2" }
    }
    TabsContent { value: 2,
        p { "Content for Tab 3" }
    }
}"#.to_string(),
            with_props_usage: r#"Tabs {
    variant: TabsVariant::Pills,
    size: TabsSize::Large,
    orientation: TabsOrientation::Horizontal,
    default_value: 0,
    onchange: move |index| {
        // Handle tab change
    },
    
    TabsList {
        TabsTrigger { value: 0, "Settings" }
        TabsTrigger { value: 1, disabled: true, "Advanced" }
    }
    TabsContent { value: 0,
        div { "Settings content..." }
    }
}"#.to_string(),

            ShowcaseVariant {
                title: "Variants".to_string(),
                div { class: "space-y-8",
                    div {
                        h4 { class: "text-sm font-medium text-foreground mb-3", "Default" }
                        Tabs {
                            variant: TabsVariant::Default,
                            default_value: 0,
                            TabsList {
                                TabsTrigger { value: 0, "Account" }
                                TabsTrigger { value: 1, "Password" }
                                TabsTrigger { value: 2, "Settings" }
                            }
                            TabsContent { value: 0,
                                Card {
                                    CardContent {
                                        p { "Manage your account information and preferences." }
                                    }
                                }
                            }
                            TabsContent { value: 1,
                                Card {
                                    CardContent {
                                        p { "Change your password and security settings." }
                                    }
                                }
                            }
                            TabsContent { value: 2,
                                Card {
                                    CardContent {
                                        p { "Configure application settings and preferences." }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        h4 { class: "text-sm font-medium text-foreground mb-3", "Pills" }
                        Tabs {
                            variant: TabsVariant::Pills,
                            default_value: 1,
                            TabsList {
                                TabsTrigger { value: 0, "Overview" }
                                TabsTrigger { value: 1, "Analytics" }
                                TabsTrigger { value: 2, "Reports" }
                            }
                            TabsContent { value: 0,
                                Card {
                                    CardContent {
                                        p { "Overview of your dashboard with key metrics." }
                                    }
                                }
                            }
                            TabsContent { value: 1,
                                Card {
                                    CardContent {
                                        p { "Detailed analytics and performance data." }
                                    }
                                }
                            }
                            TabsContent { value: 2,
                                Card {
                                    CardContent {
                                        p { "Generate and view detailed reports." }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        h4 { class: "text-sm font-medium text-foreground mb-3", "Underlined" }
                        Tabs {
                            variant: TabsVariant::Underlined,
                            default_value: 2,
                            TabsList {
                                TabsTrigger { value: 0, "Home" }
                                TabsTrigger { value: 1, "Products" }
                                TabsTrigger { value: 2, "About" }
                            }
                            TabsContent { value: 0,
                                Card {
                                    CardContent {
                                        p { "Welcome to our homepage with featured content." }
                                    }
                                }
                            }
                            TabsContent { value: 1,
                                Card {
                                    CardContent {
                                        p { "Browse our product catalog and offerings." }
                                    }
                                }
                            }
                            TabsContent { value: 2,
                                Card {
                                    CardContent {
                                        p { "Learn more about our company and mission." }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Sizes".to_string(),
                div { class: "space-y-8",
                    div {
                        h4 { class: "text-sm font-medium text-foreground mb-3", "Small" }
                        Tabs {
                            variant: TabsVariant::Pills,
                            size: TabsSize::Small,
                            default_value: 0,
                            TabsList {
                                TabsTrigger { value: 0, "Tab 1" }
                                TabsTrigger { value: 1, "Tab 2" }
                                TabsTrigger { value: 2, "Tab 3" }
                            }
                            TabsContent { value: 0,
                                Card {
                                    size: crate::CardSize::Small,
                                    CardContent {
                                        p { class: "text-sm", "Small tabs with compact content." }
                                    }
                                }
                            }
                            TabsContent { value: 1,
                                Card {
                                    size: crate::CardSize::Small,
                                    CardContent {
                                        p { class: "text-sm", "Perfect for sidebars and small spaces." }
                                    }
                                }
                            }
                            TabsContent { value: 2,
                                Card {
                                    size: crate::CardSize::Small,
                                    CardContent {
                                        p { class: "text-sm", "Minimal padding and text size." }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        h4 { class: "text-sm font-medium text-foreground mb-3", "Medium (Default)" }
                        Tabs {
                            variant: TabsVariant::Default,
                            size: TabsSize::Medium,
                            default_value: 1,
                            TabsList {
                                TabsTrigger { value: 0, "Medium 1" }
                                TabsTrigger { value: 1, "Medium 2" }
                                TabsTrigger { value: 2, "Medium 3" }
                            }
                            TabsContent { value: 0,
                                Card {
                                    CardContent {
                                        p { "Standard size tabs for most use cases." }
                                    }
                                }
                            }
                            TabsContent { value: 1,
                                Card {
                                    CardContent {
                                        p { "Balanced padding and text size for readability." }
                                    }
                                }
                            }
                            TabsContent { value: 2,
                                Card {
                                    CardContent {
                                        p { "Good for main content areas and forms." }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        h4 { class: "text-sm font-medium text-foreground mb-3", "Large" }
                        Tabs {
                            variant: TabsVariant::Pills,
                            size: TabsSize::Large,
                            default_value: 0,
                            TabsList {
                                TabsTrigger { value: 0, "Large Tab 1" }
                                TabsTrigger { value: 1, "Large Tab 2" }
                            }
                            TabsContent { value: 0,
                                Card {
                                    size: crate::CardSize::Large,
                                    CardContent {
                                        p { class: "text-lg", "Large tabs for prominent navigation." }
                                    }
                                }
                            }
                            TabsContent { value: 1,
                                Card {
                                    size: crate::CardSize::Large,
                                    CardContent {
                                        p { class: "text-lg", "Great for hero sections and landing pages." }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "States & Features".to_string(),
                div { class: "space-y-8",
                    div {
                        h4 { class: "text-sm font-medium text-foreground mb-3", "With Disabled Tab" }
                        Tabs {
                            variant: TabsVariant::Default,
                            default_value: 0,
                            TabsList {
                                TabsTrigger { value: 0, "Enabled" }
                                TabsTrigger { value: 1, disabled: true, "Disabled" }
                                TabsTrigger { value: 2, "Also Enabled" }
                            }
                            TabsContent { value: 0,
                                Card {
                                    CardContent {
                                        p { "This tab is accessible and functional." }
                                    }
                                }
                            }
                            TabsContent { value: 1,
                                Card {
                                    CardContent {
                                        p { "This content won't be shown as the tab is disabled." }
                                    }
                                }
                            }
                            TabsContent { value: 2,
                                Card {
                                    CardContent {
                                        p { "Another accessible tab with content." }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        h4 { class: "text-sm font-medium text-foreground mb-3", "Rich Content" }
                        Tabs {
                            variant: TabsVariant::Pills,
                            default_value: 0,
                            TabsList {
                                TabsTrigger { value: 0, "Profile" }
                                TabsTrigger { value: 1, "Settings" }
                                TabsTrigger { value: 2, "Notifications" }
                            }
                            TabsContent { value: 0,
                                Card {
                                    CardHeader {
                                        CardTitle { "Profile Information" }
                                        CardDescription { "Manage your profile details" }
                                    }
                                    CardContent {
                                        div { class: "space-y-4",
                                            div {
                                                Label { "Name" }
                                                Input {
                                                    value: "John Doe".to_string(),
                                                    oninput: move |_: String| {},
                                                }
                                            }
                                            div {
                                                Label { "Email" }
                                                Input {
                                                    input_type: InputType::Email,
                                                    value: "john@example.com".to_string(),
                                                    oninput: move |_: String| {},
                                                }
                                            }
                                        }
                                    }
                                    CardFooter {
                                        Button { "Save Changes" }
                                    }
                                }
                            }
                            TabsContent { value: 1,
                                Card {
                                    CardHeader {
                                        CardTitle { "Application Settings" }
                                        CardDescription { "Configure your preferences" }
                                    }
                                    CardContent {
                                        div { class: "space-y-4",
                                            div { class: "flex items-center justify-between",
                                                Label { "Dark Mode" }
                                                Switch {
                                                    checked: false,
                                                    onchange: move |_| {},
                                                }
                                            }
                                            div { class: "flex items-center justify-between",
                                                Label { "Notifications" }
                                                Switch {
                                                    checked: true,
                                                    onchange: move |_| {},
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            TabsContent { value: 2,
                                Card {
                                    CardHeader {
                                        CardTitle { "Notification Preferences" }
                                        CardDescription { "Choose what notifications you receive" }
                                    }
                                    CardContent {
                                        div { class: "space-y-3",
                                            div { class: "flex items-center space-x-2",
                                                Checkbox {
                                                    checked: true,
                                                    onchange: move |_| {},
                                                }
                                                Label { "Email notifications" }
                                            }
                                            div { class: "flex items-center space-x-2",
                                                Checkbox {
                                                    checked: false,
                                                    onchange: move |_| {},
                                                }
                                                Label { "Push notifications" }
                                            }
                                            div { class: "flex items-center space-x-2",
                                                Checkbox {
                                                    checked: true,
                                                    onchange: move |_| {},
                                                }
                                                Label { "SMS notifications" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
