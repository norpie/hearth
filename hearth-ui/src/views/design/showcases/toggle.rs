//! Toggle component showcase

use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Toggle, ToggleSize, ToggleVariant};
use dioxus::prelude::*;

#[component]
pub fn toggle_showcase() -> Element {
    // State for interactive toggles
    let mut bold_pressed = use_signal(|| false);
    let mut italic_pressed = use_signal(|| false);
    let mut underline_pressed = use_signal(|| false);
    let mut strikethrough_pressed = use_signal(|| false);

    let mut default_toggle = use_signal(|| false);
    let mut outline_toggle = use_signal(|| true);

    let mut small_toggle = use_signal(|| false);
    let mut medium_toggle = use_signal(|| true);
    let mut large_toggle = use_signal(|| false);

    rsx! {
        ComponentShowcase {
            name: "Toggle".to_string(),
            description: "A button that can be toggled on or off, ideal for formatting controls.".to_string(),
            basic_usage: r#"Toggle {
    pressed: is_bold(),
    onclick: move |_| toggle_bold(),
    "B"
}"#.to_string(),
            with_props_usage: r#"Toggle {
    pressed: is_active,
    disabled: is_disabled,
    size: ToggleSize::Small,
    variant: ToggleVariant::Outline,
    onclick: move |_| handle_toggle(),
    "Toggle"
}"#.to_string(),

            ShowcaseVariant {
                title: "Text Formatting Example".to_string(),

                div { class: "space-y-4",
                    div { class: "flex items-center space-x-2",
                        Toggle {
                            pressed: bold_pressed(),
                            size: ToggleSize::Small,
                            onclick: move |_| bold_pressed.set(!bold_pressed()),
                            "B"
                        }
                        Toggle {
                            pressed: italic_pressed(),
                            size: ToggleSize::Small,
                            onclick: move |_| italic_pressed.set(!italic_pressed()),
                            "I"
                        }
                        Toggle {
                            pressed: underline_pressed(),
                            size: ToggleSize::Small,
                            onclick: move |_| underline_pressed.set(!underline_pressed()),
                            "U"
                        }
                        Toggle {
                            pressed: strikethrough_pressed(),
                            size: ToggleSize::Small,
                            onclick: move |_| strikethrough_pressed.set(!strikethrough_pressed()),
                            "S"
                        }
                    }

                    div { class: "p-4 bg-muted rounded-md border border-border",
                        p {
                            class: format!(
                                "text-foreground {}{}{}{}",
                                if bold_pressed() { " font-bold" } else { "" },
                                if italic_pressed() { " italic" } else { "" },
                                if underline_pressed() { " underline" } else { "" },
                                if strikethrough_pressed() { " line-through" } else { "" }
                            ),
                            "Sample text showing the formatting effects applied by the toggle buttons above."
                        }
                    }

                    p { class: "text-sm text-foreground",
                        "Click the formatting buttons above to see the text change. Each toggle maintains its own pressed state."
                    }
                }
            }

            ShowcaseVariant {
                title: "Variants".to_string(),

                div { class: "space-y-6",
                    div { class: "space-y-3",
                        h4 { class: "font-medium text-foreground", "Default" }
                        div { class: "flex items-center space-x-2",
                            Toggle {
                                pressed: default_toggle(),
                                onclick: move |_| default_toggle.set(!default_toggle()),
                                "Default"
                            }
                            Toggle {
                                pressed: true,
                                disabled: true,
                                "Pressed & Disabled"
                            }
                            Toggle {
                                pressed: false,
                                disabled: true,
                                "Disabled"
                            }
                        }
                    }

                    div { class: "space-y-3",
                        h4 { class: "font-medium text-foreground", "Outline" }
                        div { class: "flex items-center space-x-2",
                            Toggle {
                                variant: ToggleVariant::Outline,
                                pressed: outline_toggle(),
                                onclick: move |_| outline_toggle.set(!outline_toggle()),
                                "Outline"
                            }
                            Toggle {
                                variant: ToggleVariant::Outline,
                                pressed: true,
                                disabled: true,
                                "Pressed & Disabled"
                            }
                            Toggle {
                                variant: ToggleVariant::Outline,
                                pressed: false,
                                disabled: true,
                                "Disabled"
                            }
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Sizes".to_string(),

                div { class: "space-y-4",
                    div { class: "flex items-center space-x-2",
                        Toggle {
                            size: ToggleSize::Small,
                            pressed: small_toggle(),
                            onclick: move |_| small_toggle.set(!small_toggle()),
                            "Small"
                        }
                        Toggle {
                            size: ToggleSize::Medium,
                            pressed: medium_toggle(),
                            onclick: move |_| medium_toggle.set(!medium_toggle()),
                            "Medium"
                        }
                        Toggle {
                            size: ToggleSize::Large,
                            pressed: large_toggle(),
                            onclick: move |_| large_toggle.set(!large_toggle()),
                            "Large"
                        }
                    }

                    p { class: "text-sm text-foreground",
                        "Toggle buttons come in three sizes: Small, Medium (default), and Large."
                    }
                }
            }

            ShowcaseVariant {
                title: "Icon Toggles".to_string(),

                div { class: "space-y-4",
                    div { class: "flex items-center space-x-2",
                        Toggle {
                            pressed: false,
                            size: ToggleSize::Small,
                            "♡"
                        }
                        Toggle {
                            pressed: true,
                            size: ToggleSize::Small,
                            "♥"
                        }
                        Toggle {
                            pressed: false,
                            size: ToggleSize::Small,
                            "☆"
                        }
                        Toggle {
                            pressed: true,
                            size: ToggleSize::Small,
                            "★"
                        }
                        Toggle {
                            pressed: false,
                            size: ToggleSize::Small,
                            "👁"
                        }
                        Toggle {
                            pressed: true,
                            size: ToggleSize::Small,
                            "🙈"
                        }
                    }

                    p { class: "text-sm text-foreground",
                        "Toggle buttons work well with icons and symbols. Use different icons for pressed and unpressed states to provide clear visual feedback."
                    }
                }
            }

            ShowcaseVariant {
                title: "Settings Usage".to_string(),

                div { class: "space-y-4",
                    p { class: "text-foreground text-sm",
                        "Individual toggles work well for independent settings that can be turned on or off."
                    }

                    div { class: "space-y-3",
                        div { class: "flex items-center justify-between p-3 border border-border rounded-md",
                            div {
                                h4 { class: "font-medium text-foreground", "Dark Mode" }
                                p { class: "text-sm text-foreground", "Switch to dark theme" }
                            }
                            Toggle {
                                pressed: false,
                                size: ToggleSize::Small,
                                "🌙"
                            }
                        }
                        div { class: "flex items-center justify-between p-3 border border-border rounded-md",
                            div {
                                h4 { class: "font-medium text-foreground", "Notifications" }
                                p { class: "text-sm text-foreground", "Receive push notifications" }
                            }
                            Toggle {
                                pressed: true,
                                size: ToggleSize::Small,
                                "🔔"
                            }
                        }
                        div { class: "flex items-center justify-between p-3 border border-border rounded-md",
                            div {
                                h4 { class: "font-medium text-foreground", "Auto-save" }
                                p { class: "text-sm text-foreground", "Automatically save changes" }
                            }
                            Toggle {
                                pressed: true,
                                size: ToggleSize::Small,
                                "💾"
                            }
                        }
                    }

                    p { class: "text-sm text-foreground italic",
                        "For grouped toggles like toolbars or radio-like selections, use ToggleGroup instead."
                    }
                }
            }
        }
    }
}
