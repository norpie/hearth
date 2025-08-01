use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{ToggleIcon, ToggleIconSize, ToastConfig, ToastManager, ToastType};
use dioxus::prelude::*;

#[derive(Props, Clone)]
pub struct ToggleIconShowcaseProps {
    pub toaster: ToastManager,
}

impl PartialEq for ToggleIconShowcaseProps {
    fn eq(&self, _other: &Self) -> bool {
        // ToastManager doesn't implement PartialEq, so we'll always return true
        // This is okay for component props since toaster manager is typically unique
        true
    }
}

#[component]
pub fn toggle_icon_showcase(props: ToggleIconShowcaseProps) -> Element {
    let ToggleIconShowcaseProps { toaster } = props;
    
    // State for interactive examples
    let mut favorite_toggle = use_signal(|| false);
    let mut bookmark_toggle = use_signal(|| true);
    let mut like_toggle = use_signal(|| false);
    let mut star_toggle = use_signal(|| true);
    let mut heart_toggle = use_signal(|| false);
    let mut quick_filter_favorite = use_signal(|| true);
    let mut quick_filter_previously_used = use_signal(|| false);

    rsx! {
        ComponentShowcase {
            name: "ToggleIcon".to_string(),
            description: "Icon-only toggle button with color-based state indication. Perfect for compact UIs like favorites, bookmarks, and likes.".to_string(),
            basic_usage: r#"ToggleIcon {
    icon: "fa-star",
    is_active: is_favorite(),
    onclick: move |_| is_favorite.set(!is_favorite()),
    aria_label: "Toggle favorite",
}"#.to_string(),
            with_props_usage: r#"ToggleIcon {
    icon: "fa-heart",
    is_active: is_liked(),
    active_color: "text-red-500",
    inactive_color: "text-gray-400",
    size: ToggleIconSize::Large,
    onclick: move |_| is_liked.set(!is_liked()),
    aria_label: "Toggle like",
    disabled: false,
}"#.to_string(),

            ShowcaseVariant {
                title: "Basic Examples".to_string(),
                div { class: "flex flex-wrap gap-4 items-center",
                    div { class: "flex flex-col items-center gap-2",
                        ToggleIcon {
                            icon: "fa-star",
                            is_active: favorite_toggle(),
                            active_color: "text-yellow-400",
                            inactive_color: "text-gray-500",
                            onclick: move |_| {
                                favorite_toggle.set(!favorite_toggle());
                                toaster.add_toast(ToastConfig {
                                    message: if favorite_toggle() { "Added to favorites!" } else { "Removed from favorites!" }.to_string(),
                                    toast_type: ToastType::Info,
                                    duration: Some(std::time::Duration::from_secs(2)),
                                    dismissible: true,
                                });
                            },
                            aria_label: "Toggle favorite",
                        }
                        span { class: "text-xs text-muted-foreground", "Favorite" }
                    }
                    
                    div { class: "flex flex-col items-center gap-2",
                        ToggleIcon {
                            icon: "fa-bookmark",
                            is_active: bookmark_toggle(),
                            active_color: "text-blue-500",
                            inactive_color: "text-gray-500",
                            onclick: move |_| bookmark_toggle.set(!bookmark_toggle()),
                            aria_label: "Toggle bookmark",
                        }
                        span { class: "text-xs text-muted-foreground", "Bookmark" }
                    }
                    
                    div { class: "flex flex-col items-center gap-2",
                        ToggleIcon {
                            icon: "fa-heart",
                            is_active: like_toggle(),
                            active_color: "text-red-500",
                            inactive_color: "text-gray-500",
                            onclick: move |_| like_toggle.set(!like_toggle()),
                            aria_label: "Toggle like",
                        }
                        span { class: "text-xs text-muted-foreground", "Like" }
                    }
                }
            }

            ShowcaseVariant {
                title: "Sizes".to_string(),
                div { class: "flex flex-wrap gap-4 items-center",
                    div { class: "flex flex-col items-center gap-2",
                        ToggleIcon {
                            icon: "fa-star",
                            is_active: true,
                            active_color: "text-yellow-400",
                            size: ToggleIconSize::Small,
                            onclick: move |_| {},
                            aria_label: "Small star",
                        }
                        span { class: "text-xs text-muted-foreground", "Small" }
                    }
                    
                    div { class: "flex flex-col items-center gap-2",
                        ToggleIcon {
                            icon: "fa-star",
                            is_active: true,
                            active_color: "text-yellow-400",
                            size: ToggleIconSize::Medium,
                            onclick: move |_| {},
                            aria_label: "Medium star",
                        }
                        span { class: "text-xs text-muted-foreground", "Medium" }
                    }
                    
                    div { class: "flex flex-col items-center gap-2",
                        ToggleIcon {
                            icon: "fa-star",
                            is_active: true,
                            active_color: "text-yellow-400",
                            size: ToggleIconSize::Large,
                            onclick: move |_| {},
                            aria_label: "Large star",
                        }
                        span { class: "text-xs text-muted-foreground", "Large" }
                    }
                }
            }

            ShowcaseVariant {
                title: "Color Variations".to_string(),
                div { class: "flex flex-wrap gap-4 items-center",
                    div { class: "flex flex-col items-center gap-2",
                        ToggleIcon {
                            icon: "fa-star",
                            is_active: star_toggle(),
                            active_color: "text-yellow-400",
                            inactive_color: "text-gray-500",
                            onclick: move |_| star_toggle.set(!star_toggle()),
                            aria_label: "Yellow star",
                        }
                        span { class: "text-xs text-muted-foreground", "Yellow" }
                    }
                    
                    div { class: "flex flex-col items-center gap-2",
                        ToggleIcon {
                            icon: "fa-heart",
                            is_active: heart_toggle(),
                            active_color: "text-red-500",
                            inactive_color: "text-gray-500",
                            onclick: move |_| heart_toggle.set(!heart_toggle()),
                            aria_label: "Red heart",
                        }
                        span { class: "text-xs text-muted-foreground", "Red" }
                    }
                    
                    div { class: "flex flex-col items-center gap-2",
                        ToggleIcon {
                            icon: "fa-clock",
                            is_active: true,
                            active_color: "text-amber-800",
                            inactive_color: "text-gray-500",
                            onclick: move |_| {},
                            aria_label: "Amber clock",
                        }
                        span { class: "text-xs text-muted-foreground", "Amber" }
                    }
                    
                    div { class: "flex flex-col items-center gap-2",
                        ToggleIcon {
                            icon: "fa-thumbs-up",
                            is_active: true,
                            active_color: "text-green-500",
                            inactive_color: "text-gray-500",
                            onclick: move |_| {},
                            aria_label: "Green thumbs up",
                        }
                        span { class: "text-xs text-muted-foreground", "Green" }
                    }
                    
                    div { class: "flex flex-col items-center gap-2",
                        ToggleIcon {
                            icon: "fa-bell",
                            is_active: true,
                            active_color: "text-blue-500",
                            inactive_color: "text-gray-500",
                            onclick: move |_| {},
                            aria_label: "Blue bell",
                        }
                        span { class: "text-xs text-muted-foreground", "Blue" }
                    }
                }
            }

            ShowcaseVariant {
                title: "States".to_string(),
                div { class: "flex flex-wrap gap-4 items-center",
                    div { class: "flex flex-col items-center gap-2",
                        ToggleIcon {
                            icon: "fa-star",
                            is_active: false,
                            active_color: "text-yellow-400",
                            inactive_color: "text-gray-500",
                            onclick: move |_| {},
                            aria_label: "Inactive star",
                        }
                        span { class: "text-xs text-muted-foreground", "Inactive" }
                    }
                    
                    div { class: "flex flex-col items-center gap-2",
                        ToggleIcon {
                            icon: "fa-star",
                            is_active: true,
                            active_color: "text-yellow-400",
                            inactive_color: "text-gray-500",
                            onclick: move |_| {},
                            aria_label: "Active star",
                        }
                        span { class: "text-xs text-muted-foreground", "Active" }
                    }
                    
                    div { class: "flex flex-col items-center gap-2",
                        ToggleIcon {
                            icon: "fa-star",
                            is_active: false,
                            active_color: "text-yellow-400",
                            inactive_color: "text-gray-500",
                            disabled: true,
                            onclick: move |_| {},
                            aria_label: "Disabled star",
                        }
                        span { class: "text-xs text-muted-foreground", "Disabled" }
                    }
                }
            }

            ShowcaseVariant {
                title: "Common Use Cases".to_string(),
                div { class: "space-y-4",
                    div { class: "p-4 border rounded-lg",
                        h4 { class: "text-sm font-medium mb-3", "Quick Filter Example" }
                        div { class: "flex gap-2",
                            ToggleIcon {
                                icon: "fa-star",
                                is_active: quick_filter_favorite(),
                                active_color: "text-yellow-400",
                                inactive_color: "text-gray-500",
                                class: "m-1",
                                onclick: move |_| {
                                    quick_filter_favorite.set(!quick_filter_favorite());
                                    toaster.add_toast(ToastConfig {
                                        message: if quick_filter_favorite() { "Favorites filter enabled!" } else { "Favorites filter disabled!" }.to_string(),
                                        toast_type: ToastType::Info,
                                        duration: Some(std::time::Duration::from_secs(2)),
                                        dismissible: true,
                                    });
                                },
                                aria_label: "Toggle favorites filter",
                            }
                            ToggleIcon {
                                icon: "fa-clock",
                                is_active: quick_filter_previously_used(),
                                active_color: "text-green-500",
                                inactive_color: "text-gray-500",
                                class: "m-1",
                                onclick: move |_| {
                                    quick_filter_previously_used.set(!quick_filter_previously_used());
                                    toaster.add_toast(ToastConfig {
                                        message: if quick_filter_previously_used() { "Previously used filter enabled!" } else { "Previously used filter disabled!" }.to_string(),
                                        toast_type: ToastType::Info,
                                        duration: Some(std::time::Duration::from_secs(2)),
                                        dismissible: true,
                                    });
                                },
                                aria_label: "Toggle previously used filter",
                            }
                        }
                    }
                    
                    div { class: "p-4 border rounded-lg",
                        h4 { class: "text-sm font-medium mb-3", "Social Actions" }
                        div { class: "flex gap-3",
                            ToggleIcon {
                                icon: "fa-heart",
                                is_active: false,
                                active_color: "text-red-500",
                                inactive_color: "text-gray-400",
                                onclick: move |_| {},
                                aria_label: "Like post",
                            }
                            ToggleIcon {
                                icon: "fa-bookmark",
                                is_active: true,
                                active_color: "text-blue-500",
                                inactive_color: "text-gray-400",
                                onclick: move |_| {},
                                aria_label: "Bookmark post",
                            }
                            ToggleIcon {
                                icon: "fa-share",
                                is_active: false,
                                active_color: "text-green-500",
                                inactive_color: "text-gray-400",
                                onclick: move |_| {},
                                aria_label: "Share post",
                            }
                        }
                    }
                }
            }
        }
    }
}