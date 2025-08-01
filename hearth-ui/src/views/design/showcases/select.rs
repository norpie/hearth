use super::super::{ComponentShowcase, ShowcaseVariant};
use crate::{Select, SelectOption, SelectVariant};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SelectShowcaseProps {
    pub select_theme: Signal<String>,
    pub select_size: Signal<String>,
    pub select_framework: Signal<String>,
}

#[component]
pub fn select_showcase(props: SelectShowcaseProps) -> Element {
    let SelectShowcaseProps {
        mut select_theme,
        mut select_size,
        select_framework: _,
    } = props;
    let mut select_country = use_signal(String::new);
    let mut select_language = use_signal(String::new);
    rsx! {
        ComponentShowcase {
            name: "Select".to_string(),
            description: "Dropdown select component for choosing from multiple options.".to_string(),
            basic_usage: r#"Select {
    options: vec![
        SelectOption::new("option1", "Option 1"),
        SelectOption::new("option2", "Option 2"),
    ],
    value: selected_value.read().clone(),
    onchange: move |value| selected_value.set(value),
}"#.to_string(),
            with_props_usage: r#"Select {
    variant: SelectVariant::Default,
    size: SelectSize::Medium,
    placeholder: "Select an option",
    searchable: true,
    disabled: false,
    class: "custom-class",
}"#.to_string(),

            ShowcaseVariant {
                title: "Variants".to_string(),
                div { class: "space-y-4",
                    Select {
                        options: vec![
                            SelectOption::new("light", "Light Theme"),
                            SelectOption::new("dark", "Dark Theme"),
                            SelectOption::new("system", "System Theme"),
                        ],
                        placeholder: "Select theme",
                        value: select_theme.read().clone(),
                        onchange: move |value| select_theme.set(value),
                    }
                    Select {
                        variant: SelectVariant::Filled,
                        options: vec![
                            SelectOption::new("xs", "Extra Small"),
                            SelectOption::new("sm", "Small"),
                            SelectOption::new("md", "Medium"),
                            SelectOption::new("lg", "Large"),
                        ],
                        placeholder: "Select size (filled)",
                        value: select_size.read().clone(),
                        onchange: move |value| select_size.set(value),
                    }
                }
            }

            ShowcaseVariant {
                title: "Searchable Functionality".to_string(),
                div { class: "space-y-4",
                    p { class: "text-sm text-foreground",
                        "Enable fuzzy search to filter options by typing. Perfect for long lists of options."
                    }

                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        div { class: "space-y-2",
                            label { class: "block text-sm font-medium text-foreground", "Country (Searchable)" }
                            Select {
                                searchable: true,
                                options: vec![
                                    SelectOption::new("us", "United States"),
                                    SelectOption::new("ca", "Canada"),
                                    SelectOption::new("uk", "United Kingdom"),
                                    SelectOption::new("fr", "France"),
                                    SelectOption::new("de", "Germany"),
                                    SelectOption::new("it", "Italy"),
                                    SelectOption::new("es", "Spain"),
                                    SelectOption::new("au", "Australia"),
                                    SelectOption::new("jp", "Japan"),
                                    SelectOption::new("kr", "South Korea"),
                                    SelectOption::new("cn", "China"),
                                    SelectOption::new("in", "India"),
                                    SelectOption::new("br", "Brazil"),
                                    SelectOption::new("mx", "Mexico"),
                                    SelectOption::new("ar", "Argentina"),
                                    SelectOption::new("za", "South Africa"),
                                    SelectOption::new("eg", "Egypt"),
                                    SelectOption::new("ng", "Nigeria"),
                                    SelectOption::new("ma", "Morocco"),
                                    SelectOption::new("et", "Ethiopia"),
                                ],
                                placeholder: "Search countries...",
                                value: select_country.read().clone(),
                                onchange: move |value| select_country.set(value),
                            }
                        }

                        div { class: "space-y-2",
                            label { class: "block text-sm font-medium text-foreground", "Programming Language" }
                            Select {
                                searchable: true,
                                variant: SelectVariant::Filled,
                                options: vec![
                                    SelectOption::new("rust", "Rust"),
                                    SelectOption::new("javascript", "JavaScript"),
                                    SelectOption::new("typescript", "TypeScript"),
                                    SelectOption::new("python", "Python"),
                                    SelectOption::new("java", "Java"),
                                    SelectOption::new("csharp", "C#"),
                                    SelectOption::new("cpp", "C++"),
                                    SelectOption::new("c", "C"),
                                    SelectOption::new("go", "Go"),
                                    SelectOption::new("php", "PHP"),
                                    SelectOption::new("ruby", "Ruby"),
                                    SelectOption::new("swift", "Swift"),
                                    SelectOption::new("kotlin", "Kotlin"),
                                    SelectOption::new("dart", "Dart"),
                                    SelectOption::new("scala", "Scala"),
                                    SelectOption::new("haskell", "Haskell"),
                                    SelectOption::new("elixir", "Elixir"),
                                    SelectOption::new("clojure", "Clojure"),
                                    SelectOption::new("lua", "Lua"),
                                    SelectOption::new("r", "R"),
                                ],
                                placeholder: "Search languages...",
                                value: select_language.read().clone(),
                                onchange: move |value| select_language.set(value),
                            }
                        }
                    }

                    div { class: "mt-4 p-4 bg-primary/20 rounded-lg",
                        h4 { class: "font-medium text-primary-foreground mb-2", "Search Features" }
                        ul { class: "text-sm text-primary-foreground space-y-1 list-disc list-inside",
                            li { "Substring matching (e.g., 'script' matches 'JavaScript')" }
                            li { "Fuzzy matching (e.g., 'ts' matches 'TypeScript')" }
                            li { "Case-insensitive search" }
                            li { "Real-time filtering as you type" }
                            li { "No results message when no matches found" }
                        }
                    }
                }
            }
        }
    }
}
