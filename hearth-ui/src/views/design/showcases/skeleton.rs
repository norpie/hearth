use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Skeleton, SkeletonVariant};
use dioxus::prelude::*;

#[component]
pub fn skeleton_showcase() -> Element {
    rsx! {
        ComponentShowcase {
            name: "Skeleton".to_string(),
            description: "Loading placeholder component that matches typography and avatar sizes.".to_string(),
            basic_usage: r#"Skeleton {
    variant: SkeletonVariant::Text
}"#.to_string(),
            with_props_usage: r#"Skeleton {
    variant: SkeletonVariant::Avatar,
    size: "w-16 h-16",
    class: "my-custom-class"
}"#.to_string(),

            ShowcaseVariant {
                title: "Typography Variants".to_string(),
                div { class: "space-y-4",
                    div { class: "space-y-2",
                        p { class: "text-sm font-semibold text-foreground", "H1 - Large Title" }
                        Skeleton { variant: SkeletonVariant::H1 }
                    }
                    div { class: "space-y-2",
                        p { class: "text-sm font-semibold text-foreground", "H2 - Section Header" }
                        Skeleton { variant: SkeletonVariant::H2 }
                    }
                    div { class: "space-y-2",
                        p { class: "text-sm font-semibold text-foreground", "H3 - Subsection" }
                        Skeleton { variant: SkeletonVariant::H3 }
                    }
                    div { class: "space-y-2",
                        p { class: "text-sm font-semibold text-foreground", "H4-H6 - Smaller Headers" }
                        Skeleton { variant: SkeletonVariant::H4 }
                        Skeleton { variant: SkeletonVariant::H5 }
                        Skeleton { variant: SkeletonVariant::H6 }
                    }
                    div { class: "space-y-2",
                        p { class: "text-sm font-semibold text-foreground", "Text - Paragraph Content" }
                        Skeleton { variant: SkeletonVariant::Text }
                        Skeleton { variant: SkeletonVariant::Text, width: "w-1/2" }
                        Skeleton { variant: SkeletonVariant::Text, width: "w-2/3" }
                    }
                }
            }

            ShowcaseVariant {
                title: "Avatar Variants".to_string(),
                div { class: "flex items-center gap-4",
                    div { class: "text-center",
                        p { class: "text-sm mb-2 text-foreground", "Small" }
                        Skeleton { variant: SkeletonVariant::Avatar, size: "w-6 h-6" }
                    }
                    div { class: "text-center",
                        p { class: "text-sm mb-2 text-foreground", "Medium" }
                        Skeleton { variant: SkeletonVariant::Avatar, size: "w-10 h-10" }
                    }
                    div { class: "text-center",
                        p { class: "text-sm mb-2 text-foreground", "Default" }
                        Skeleton { variant: SkeletonVariant::Avatar }
                    }
                    div { class: "text-center",
                        p { class: "text-sm mb-2 text-foreground", "Large" }
                        Skeleton { variant: SkeletonVariant::Avatar, size: "w-16 h-16" }
                    }
                    div { class: "text-center",
                        p { class: "text-sm mb-2 text-foreground", "XL" }
                        Skeleton { variant: SkeletonVariant::Avatar, size: "w-20 h-20" }
                    }
                }
            }

            ShowcaseVariant {
                title: "Custom Shapes".to_string(),
                div { class: "space-y-4",
                    div { class: "space-y-2",
                        p { class: "text-sm font-semibold text-foreground", "Custom Rectangle" }
                        Skeleton {
                            variant: SkeletonVariant::Custom,
                            width: "w-48",
                            height: "h-24"
                        }
                    }
                    div { class: "space-y-2",
                        p { class: "text-sm font-semibold text-foreground", "Custom Square" }
                        Skeleton {
                            variant: SkeletonVariant::Custom,
                            width: "w-32",
                            height: "h-32"
                        }
                    }
                    div { class: "space-y-2",
                        p { class: "text-sm font-semibold text-foreground", "Button-like" }
                        Skeleton {
                            variant: SkeletonVariant::Custom,
                            width: "w-24",
                            height: "h-10",
                            class: "rounded-md"
                        }
                    }
                }
            }

            ShowcaseVariant {
                title: "Loading Cards Example".to_string(),
                div { class: "space-y-6",
                    div { class: "border border-border rounded-lg p-4 space-y-3",
                        div { class: "flex items-center gap-3",
                            Skeleton { variant: SkeletonVariant::Avatar, size: "w-12 h-12" }
                            div { class: "space-y-2 flex-1",
                                Skeleton { variant: SkeletonVariant::H4 }
                                Skeleton { variant: SkeletonVariant::Text, width: "w-1/3" }
                            }
                        }
                        div { class: "space-y-2",
                            Skeleton { variant: SkeletonVariant::Text }
                            Skeleton { variant: SkeletonVariant::Text, width: "w-3/4" }
                            Skeleton { variant: SkeletonVariant::Text, width: "w-1/2" }
                        }
                    }
                }
            }
        }
    }
}
