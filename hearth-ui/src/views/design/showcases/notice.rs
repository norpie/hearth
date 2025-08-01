//! Notice component showcase

use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Notice, NoticeVariant};
use dioxus::prelude::*;

#[component]
pub fn notice_showcase() -> Element {
    rsx! {
        ComponentShowcase {
            name: "Notice".to_string(),
            description: "Informational boxes with icons and variants for different message types.".to_string(),
            basic_usage: r#"Notice {
    variant: NoticeVariant::Success,
    icon: "fas fa-check-circle".to_string(),
    "Your changes have been saved successfully!"
}"#.to_string(),
            with_props_usage: r#"Notice {
    variant: NoticeVariant::Warning,
    icon: "fas fa-exclamation-triangle".to_string(),
    class: "mb-4",
    "Please review your settings before continuing."
}"#.to_string(),

            ShowcaseVariant {
                title: "Variants".to_string(),

                div { class: "space-y-4",
                    Notice {
                        variant: NoticeVariant::Success,
                        icon: "fas fa-check-circle".to_string(),
                        "Success: Your changes have been saved successfully!"
                    }

                    Notice {
                        variant: NoticeVariant::Info,
                        icon: "fas fa-info-circle".to_string(),
                        "Info: This is some helpful information you should know."
                    }

                    Notice {
                        variant: NoticeVariant::Warning,
                        icon: "fas fa-exclamation-triangle".to_string(),
                        "Warning: Please review your settings before continuing."
                    }

                    Notice {
                        variant: NoticeVariant::Destructive,
                        icon: "fas fa-times-circle".to_string(),
                        "Error: Something went wrong. Please try again."
                    }
                }
            }

            ShowcaseVariant {
                title: "Different Icons".to_string(),

                div { class: "space-y-4",
                    Notice {
                        variant: NoticeVariant::Success,
                        icon: "fas fa-thumbs-up".to_string(),
                        "Great job! You completed the task."
                    }

                    Notice {
                        variant: NoticeVariant::Info,
                        icon: "fas fa-lightbulb".to_string(),
                        "Tip: You can customize the appearance using CSS classes."
                    }

                    Notice {
                        variant: NoticeVariant::Warning,
                        icon: "fas fa-clock".to_string(),
                        "Reminder: Your session will expire in 5 minutes."
                    }

                    Notice {
                        variant: NoticeVariant::Destructive,
                        icon: "fas fa-exclamation".to_string(),
                        "Critical: System maintenance is required."
                    }
                }
            }

            ShowcaseVariant {
                title: "Rich Content".to_string(),

                div { class: "space-y-4",
                    Notice {
                        variant: NoticeVariant::Info,
                        icon: "fas fa-download".to_string(),
                        div {
                            strong { "New Update Available" }
                            p { class: "mt-1",
                                "Version 2.1.0 is now available with bug fixes and new features. "
                                a {
                                    href: "#",
                                    class: "underline hover:no-underline",
                                    "View changelog"
                                }
                            }
                        }
                    }

                    Notice {
                        variant: NoticeVariant::Warning,
                        icon: "fas fa-shield-alt".to_string(),
                        div {
                            strong { "Security Notice" }
                            p { class: "mt-1",
                                "We recommend enabling two-factor authentication to secure your account. "
                                "This adds an extra layer of protection."
                            }
                            button {
                                class: "mt-2 px-3 py-1 text-xs bg-white/20 hover:bg-white/30 rounded border border-current transition-colors",
                                "Enable 2FA"
                            }
                        }
                    }

                    Notice {
                        variant: NoticeVariant::Success,
                        icon: "fas fa-rocket".to_string(),
                        div {
                            strong { "Deployment Successful" }
                            p { class: "mt-1",
                                "Your application has been deployed to production. "
                            }
                            ul { class: "mt-2 text-sm list-disc list-inside",
                                li { "Build completed in 2m 34s" }
                                li { "Tests passed: 127/127" }
                                li { "Live at: https://myapp.com" }
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Custom Styling".to_string(),

                div { class: "space-y-4",
                    Notice {
                        variant: NoticeVariant::Info,
                        icon: "fas fa-star".to_string(),
                        class: "border-l-4 border-l-blue-500",
                        "Notice with custom left border styling."
                    }

                    Notice {
                        variant: NoticeVariant::Warning,
                        icon: "fas fa-bell".to_string(),
                        class: "shadow-lg",
                        "Notice with custom shadow for emphasis."
                    }

                    Notice {
                        variant: NoticeVariant::Success,
                        icon: "fas fa-medal".to_string(),
                        class: "text-base font-medium",
                        "Notice with larger, bolder text."
                    }
                }
            }
        }
    }
}
