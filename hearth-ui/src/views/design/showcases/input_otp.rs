use dioxus::prelude::*;
use crate::InputOTP;
use super::super::{ComponentShowcase, ShowcaseVariant};

#[derive(Props, Clone, PartialEq)]
pub struct InputOTPShowcaseProps {
    pub basic_otp: Signal<String>,
    pub custom_length_otp: Signal<String>,
    pub no_separator_otp: Signal<String>,
}

#[component]
pub fn input_otp_showcase(props: InputOTPShowcaseProps) -> Element {
    let InputOTPShowcaseProps { mut basic_otp, mut custom_length_otp, mut no_separator_otp } = props;
    rsx! {
        ComponentShowcase {
            name: "Input OTP".to_string(),
            description: "One-time password input component with automatic navigation and validation.".to_string(),
            basic_usage: r#"let mut otp_value = use_signal(|| String::new());

InputOTP {
    oncomplete: move |value: String| {
        otp_value.set(value);
    },
}"#.to_string(),
            with_props_usage: r#"let mut otp_value = use_signal(|| String::new());

InputOTP {
    length: 6,
    show_middle: true,
    disabled: false,
    class: "custom-class",
    oncomplete: move |value: String| {
        otp_value.set(value);
    },
}"#.to_string(),
            
            ShowcaseVariant {
                title: "Basic Usage".to_string(),
                div { class: "space-y-4",
                    div { class: "space-y-2",
                        p { class: "text-sm text-gray-600 dark:text-gray-400", "6-digit OTP with middle separator:" }
                        InputOTP {
                            oncomplete: move |value: String| {
                                basic_otp.set(value);
                            },
                        }
                        p { class: "text-xs text-gray-500", "Value: '{basic_otp}'" }
                    }
                }
            }
            
            ShowcaseVariant {
                title: "Different Lengths".to_string(),
                div { class: "space-y-4",
                    div { class: "space-y-2",
                        p { class: "text-sm text-gray-600 dark:text-gray-400", "4-digit OTP with middle separator:" }
                        InputOTP {
                            length: 4,
                            oncomplete: move |value: String| {
                                custom_length_otp.set(value);
                            },
                        }
                        p { class: "text-xs text-gray-500", "Value: '{custom_length_otp}'" }
                    }
                    div { class: "space-y-2",
                        p { class: "text-sm text-gray-600 dark:text-gray-400", "8-digit OTP with middle separator:" }
                        InputOTP {
                            length: 8,
                        }
                    }
                }
            }
            
            ShowcaseVariant {
                title: "Without Middle Separator".to_string(),
                div { class: "space-y-4",
                    div { class: "space-y-2",
                        p { class: "text-sm text-gray-600 dark:text-gray-400", "6-digit OTP without separator:" }
                        InputOTP {
                            show_middle: false,
                            oncomplete: move |value: String| {
                                no_separator_otp.set(value);
                            },
                        }
                        p { class: "text-xs text-gray-500", "Value: '{no_separator_otp}'" }
                    }
                    div { class: "space-y-2",
                        p { class: "text-sm text-gray-600 dark:text-gray-400", "5-digit OTP (odd length, no separator):" }
                        InputOTP {
                            length: 5,
                            show_middle: true,
                        }
                    }
                }
            }
            
            ShowcaseVariant {
                title: "States".to_string(),
                div { class: "space-y-4",
                    div { class: "space-y-2",
                        p { class: "text-sm text-gray-600 dark:text-gray-400", "Normal state:" }
                        InputOTP {
                            length: 4,
                        }
                    }
                    div { class: "space-y-2",
                        p { class: "text-sm text-gray-600 dark:text-gray-400", "Disabled state:" }
                        InputOTP {
                            length: 4,
                            disabled: true,
                            value: "12".to_string(),
                        }
                    }
                }
            }
        }
    }
}