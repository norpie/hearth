use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{
    Button, ButtonVariant, Card, CardContent, CardDescription, CardFooter, CardHeader, CardSize,
    CardTitle, CardVariant, Input, InputType, Label, Progress,
};
use dioxus::prelude::*;

#[component]
pub fn card_showcase() -> Element {
    rsx! {
        ComponentShowcase {
            name: "Card".to_string(),
            description: "Flexible container component for displaying content in a structured layout.".to_string(),
            basic_usage: r#"Card {
    CardHeader {
        CardTitle { "Card Title" }
        CardDescription { "Card description goes here" }
    }
    CardContent {
        p { "Your content goes here..." }
    }
    CardFooter {
        Button { "Action" }
    }
}"#.to_string(),
            with_props_usage: r#"Card {
    variant: CardVariant::Elevated,
    size: CardSize::Large,
    class: "max-w-md",
    
    CardHeader {
        CardTitle { "Advanced Card" }
        CardDescription { "With custom styling and props" }
    }
    CardContent {
        p { "Card content with custom variant and size" }
    }
    CardFooter {
        class: "justify-between",
        Button { variant: ButtonVariant::Secondary, "Cancel" }
        Button { "Confirm" }
    }
}"#.to_string(),

            ShowcaseVariant {
                title: "Variants".to_string(),
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    Card {
                        variant: CardVariant::Default,
                        CardHeader {
                            CardTitle { "Default Card" }
                            CardDescription { "Standard card with subtle shadow" }
                        }
                        CardContent {
                            p { "Default variant with border and light shadow for subtle elevation." }
                        }
                    }

                    Card {
                        variant: CardVariant::Outline,
                        CardHeader {
                            CardTitle { "Outline Card" }
                            CardDescription { "Card with prominent border" }
                        }
                        CardContent {
                            p { "Outline variant with thicker border and no shadow." }
                        }
                    }

                    Card {
                        variant: CardVariant::Elevated,
                        CardHeader {
                            CardTitle { "Elevated Card" }
                            CardDescription { "Card with strong shadow" }
                        }
                        CardContent {
                            p { "Elevated variant with larger shadow for prominent display." }
                        }
                    }

                    Card {
                        variant: CardVariant::Flat,
                        CardHeader {
                            CardTitle { "Flat Card" }
                            CardDescription { "Minimal card without border" }
                        }
                        CardContent {
                            p { "Flat variant with background color and no border or shadow." }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Sizes".to_string(),
                div { class: "space-y-4",
                    Card {
                        size: CardSize::Small,
                        CardHeader {
                            CardTitle { "Small Card" }
                            CardDescription { "Compact padding for tight layouts" }
                        }
                        CardContent {
                            p { "Small size with minimal padding." }
                        }
                    }

                    Card {
                        size: CardSize::Medium,
                        CardHeader {
                            CardTitle { "Medium Card" }
                            CardDescription { "Standard padding for most use cases" }
                        }
                        CardContent {
                            p { "Medium size (default) with standard padding." }
                        }
                    }

                    Card {
                        size: CardSize::Large,
                        CardHeader {
                            CardTitle { "Large Card" }
                            CardDescription { "Generous padding for prominent content" }
                        }
                        CardContent {
                            p { "Large size with generous padding for important content." }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Complete Examples".to_string(),
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    Card {
                        variant: CardVariant::Default,
                        class: "max-w-md",
                        CardHeader {
                            CardTitle { "User Profile" }
                            CardDescription { "Manage your account settings and preferences" }
                        }
                        CardContent {
                            div { class: "space-y-4",
                                div {
                                    Label { "Username" }
                                    Input {
                                        placeholder: "Enter username",
                                        disabled: true,
                                        value: "john_doe".to_string(),
                                        oninput: move |_: String| {},
                                    }
                                }
                                div {
                                    Label { "Email" }
                                    Input {
                                        placeholder: "Enter email",
                                        input_type: InputType::Email,
                                        value: "john@example.com".to_string(),
                                        oninput: move |_: String| {},
                                    }
                                }
                            }
                        }
                        CardFooter {
                            class: "justify-between",
                            Button {
                                variant: ButtonVariant::Ghost,
                                "Cancel"
                            }
                            Button {
                                "Save Changes"
                            }
                        }
                    }

                    Card {
                        variant: CardVariant::Elevated,
                        class: "max-w-md",
                        CardHeader {
                            CardTitle { "Statistics" }
                            CardDescription { "Your account performance metrics" }
                        }
                        CardContent {
                            div { class: "space-y-4",
                                div { class: "flex justify-between items-center",
                                    span { "Messages Sent" }
                                    span { class: "font-bold", "1,247" }
                                }
                                div { class: "space-y-2",
                                    Label { "Completion Rate" }
                                    Progress {
                                        value: 87.5,
                                        show_percentage: true,
                                    }
                                }
                                div { class: "space-y-2",
                                    Label { "Response Time" }
                                    Progress {
                                        value: 65.0,
                                        show_percentage: true,
                                    }
                                }
                            }
                        }
                        CardFooter {
                            Button {
                                variant: ButtonVariant::Outline,
                                class: "w-full",
                                "View Detailed Report"
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Layout Examples".to_string(),
                div { class: "space-y-6",
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                        Card {
                            variant: CardVariant::Default,
                            size: CardSize::Small,
                            CardContent {
                                div { class: "text-center",
                                    div { class: "text-2xl font-bold text-primary", "24" }
                                    div { class: "text-sm text-muted-foreground", "Active Users" }
                                }
                            }
                        }

                        Card {
                            variant: CardVariant::Default,
                            size: CardSize::Small,
                            CardContent {
                                div { class: "text-center",
                                    div { class: "text-2xl font-bold text-success", "156" }
                                    div { class: "text-sm text-muted-foreground", "Messages Today" }
                                }
                            }
                        }

                        Card {
                            variant: CardVariant::Default,
                            size: CardSize::Small,
                            CardContent {
                                div { class: "text-center",
                                    div { class: "text-2xl font-bold text-accent-foreground", "92%" }
                                    div { class: "text-sm text-muted-foreground", "Uptime" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
