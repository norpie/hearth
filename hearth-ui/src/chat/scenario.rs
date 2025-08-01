//! Scenario dashboard components

use dioxus::prelude::*;
use super::conversation::NarratorMode;

#[derive(Props, Clone, PartialEq)]
pub struct ScenarioDashboardProps {
    pub scenario: Option<ScenarioData>,
    pub is_collapsed: bool,
    pub on_toggle_collapse: EventHandler<()>,
    pub on_action: EventHandler<String>,
    pub on_manual_narrator: EventHandler<()>,
}

#[derive(Props, Clone, PartialEq)]
pub struct ScenarioStatusProps {
    pub scenario: ScenarioData,
}

#[derive(Props, Clone, PartialEq)]
pub struct ActionSuggestionsProps {
    pub actions: Vec<ActionSuggestion>,
    pub on_action: EventHandler<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct NarratorControlsProps {
    pub is_enabled: bool,
    pub mode: NarratorMode,
    pub can_trigger: bool,
    pub on_manual_trigger: EventHandler<()>,
}

#[derive(Clone, PartialEq)]
pub struct ScenarioData {
    pub name: String,
    pub current_location: String,
    pub time_of_day: Option<String>,
    pub active_npcs: Vec<String>,
    pub objectives: Vec<String>,
    pub progress: f32, // 0.0 to 1.0
    pub world_state: Vec<WorldState>,
}

#[derive(Clone, PartialEq)]
pub struct WorldState {
    pub key: String,
    pub value: String,
    pub icon: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct ActionSuggestion {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub category: ActionCategory,
}

#[derive(Clone, PartialEq)]
pub enum ActionCategory {
    Social,
    Exploration,
    Combat,
    Investigation,
    Utility,
}


#[component]
pub fn ScenarioDashboard(props: ScenarioDashboardProps) -> Element {
    rsx! {
        div {
            class: format!(
                "bg-white dark:bg-gray-900 border-t border-gray-200 dark:border-gray-700 transition-all duration-300 {}",
                if props.is_collapsed { "h-12" } else { "h-96" },
            ),
            // Header
            div { class: "p-4 border-b border-gray-200 dark:border-gray-700",
                if props.is_collapsed {
                    button {
                        class: "w-full text-left flex items-center justify-between text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-100",
                        onclick: move |_| props.on_toggle_collapse.call(()),
                        div { class: "flex items-center space-x-2",
                            i { class: "fa-solid fa-map" }
                            if let Some(scenario) = &props.scenario {
                                span { class: "text-sm font-medium truncate", "{scenario.name}" }
                            } else {
                                span { class: "text-sm text-gray-500", "No scenario" }
                            }
                        }
                        span { "↑" }
                    }
                } else {
                    div { class: "flex items-center justify-between",
                        h3 { class: "font-medium text-gray-900 dark:text-gray-100",
                            "Scenario"
                        }
                        button {
                            class: "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-100",
                            onclick: move |_| props.on_toggle_collapse.call(()),
                            "↓"
                        }
                    }
                }
            }
            if !props.is_collapsed {
                if let Some(scenario) = &props.scenario {
                    div { class: "flex-1 overflow-y-auto",
                        // Scenario status
                        ScenarioStatus { scenario: scenario.clone() }
                        // Action suggestions
                        if !scenario.world_state.is_empty() {
                            div { class: "p-4 border-t border-gray-200 dark:border-gray-700",
                                ActionSuggestions {
                                    actions: generate_action_suggestions(scenario),
                                    on_action: props.on_action,
                                }
                            }
                        }
                        // Narrator controls
                        div { class: "p-4 border-t border-gray-200 dark:border-gray-700",
                            NarratorControls {
                                is_enabled: true,
                                mode: NarratorMode::Automatic,
                                can_trigger: true,
                                on_manual_trigger: props.on_manual_narrator,
                            }
                        }
                    }
                } else {
                    div { class: "flex-1 flex items-center justify-center p-4",
                        div { class: "text-center text-gray-500",
                            div { class: "text-4xl mb-2",
                                i { class: "fa-solid fa-map text-gray-400" }
                            }
                            div { class: "text-sm", "No scenario active" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ScenarioStatus(props: ScenarioStatusProps) -> Element {
    let scenario = &props.scenario;
    
    rsx! {
        div { class: "p-4 space-y-4",
            // Scenario name and progress
            div {
                div { class: "font-medium text-gray-900 dark:text-gray-100 mb-2", "{scenario.name}" }
                div { class: "w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2",
                    div {
                        class: "bg-chat-character h-2 rounded-full transition-all",
                        style: format!("width: {}%", scenario.progress * 100.0),
                    }
                }
            }
            // Current state
            div { class: "grid grid-cols-1 gap-3 text-sm",
                // Location
                div { class: "flex items-center space-x-2",
                    i { class: "fa-solid fa-location-dot text-gray-500" }
                    span { class: "font-medium text-gray-700 dark:text-gray-300",
                        "{scenario.current_location}"
                    }
                }
                // Time
                if let Some(time) = &scenario.time_of_day {
                    div { class: "flex items-center space-x-2",
                        i { class: "fa-solid fa-clock text-gray-500" }
                        span { class: "text-gray-600 dark:text-gray-400", "{time}" }
                    }
                }
                // Active NPCs
                if !scenario.active_npcs.is_empty() {
                    div { class: "flex items-start space-x-2",
                        i { class: "fa-solid fa-users text-gray-500" }
                        div { class: "flex flex-wrap gap-1",
                            for npc in &scenario.active_npcs {
                                span { class: "px-2 py-1 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded text-xs",
                                    "{npc}"
                                }
                            }
                        }
                    }
                }
            }
            // Objectives
            if !scenario.objectives.is_empty() {
                div {
                    div { class: "font-medium text-gray-700 dark:text-gray-300 mb-2 text-sm",
                        "Objectives"
                    }
                    div { class: "space-y-1",
                        for objective in &scenario.objectives {
                            div { class: "flex items-start space-x-2 text-sm text-gray-600 dark:text-gray-400",
                                span { "•" }
                                span { "{objective}" }
                            }
                        }
                    }
                }
            }
            // World state
            if !scenario.world_state.is_empty() {
                div {
                    div { class: "font-medium text-gray-700 dark:text-gray-300 mb-2 text-sm",
                        "World State"
                    }
                    div { class: "space-y-1",
                        for state in &scenario.world_state {
                            div { class: "flex items-center justify-between text-sm",
                                div { class: "flex items-center space-x-2",
                                    if let Some(icon) = &state.icon {
                                        i { class: "{icon}" }
                                    }
                                    span { class: "text-gray-600 dark:text-gray-400",
                                        "{state.key}"
                                    }
                                }
                                span { class: "text-gray-900 dark:text-gray-100 font-medium",
                                    "{state.value}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ActionSuggestions(props: ActionSuggestionsProps) -> Element {
    rsx! {
        div {
            div { class: "font-medium text-gray-700 dark:text-gray-300 mb-3 text-sm",
                "Suggested Actions"
            }
            div { class: "grid grid-cols-2 gap-2",
                for action in props.actions.iter() {
                    {
                        let action_id = action.id.clone();
                        rsx! {
                            button {
                                class: "p-2 text-left border border-gray-200 dark:border-gray-700 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors",
                                onclick: move |_| props.on_action.call(action_id.clone()),
                                div { class: "flex items-center space-x-2",
                                    if let Some(icon) = &action.icon {
                                        i { class: "text-lg {icon}" }
                                    }
                                    div {
                                        div { class: "text-sm font-medium text-gray-900 dark:text-gray-100", "{action.label}" }
                                        if let Some(description) = &action.description {
                                            div { class: "text-xs text-gray-500 mt-1", "{description}" }
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

#[component]
pub fn NarratorControls(props: NarratorControlsProps) -> Element {
    if !props.is_enabled {
        return rsx! {
            div {}
        };
    }

    rsx! {
        div {
            div { class: "font-medium text-gray-700 dark:text-gray-300 mb-3 text-sm",
                "Narrator"
            }
            div { class: "flex items-center justify-between",
                div { class: "text-sm text-gray-600 dark:text-gray-400",
                    match props.mode {
                        NarratorMode::Automatic => "Automatic mode",
                        NarratorMode::UserControlled => "User controlled",
                        NarratorMode::Guided => "Guided mode",
                        NarratorMode::Disabled => "Disabled",
                    }
                }
                if matches!(props.mode, NarratorMode::UserControlled) {
                    button {
                        class: format!(
                            "px-3 py-1 text-sm rounded {}",
                            if props.can_trigger {
                                "bg-chat-narrator text-white hover:bg-chat-narrator/90"
                            } else {
                                "bg-gray-200 dark:bg-gray-700 text-gray-500 cursor-not-allowed"
                            },
                        ),
                        disabled: !props.can_trigger,
                        onclick: move |_| {
                            if props.can_trigger {
                                props.on_manual_trigger.call(());
                            }
                        },
                        "Add Narration"
                    }
                }
            }
        }
    }
}

// Helper function to generate action suggestions based on scenario
fn generate_action_suggestions(_scenario: &ScenarioData) -> Vec<ActionSuggestion> {
    vec![
        ActionSuggestion {
            id: "explore".to_string(),
            label: "Explore".to_string(),
            description: Some("Look around the area".to_string()),
            icon: Some("fa-solid fa-magnifying-glass".to_string()),
            category: ActionCategory::Exploration,
        },
        ActionSuggestion {
            id: "interact".to_string(),
            label: "Interact".to_string(),
            description: Some("Talk to someone nearby".to_string()),
            icon: Some("fa-solid fa-message".to_string()),
            category: ActionCategory::Social,
        },
        ActionSuggestion {
            id: "investigate".to_string(),
            label: "Investigate".to_string(),
            description: Some("Examine something closely".to_string()),
            icon: Some("fa-solid fa-search".to_string()),
            category: ActionCategory::Investigation,
        },
        ActionSuggestion {
            id: "wait".to_string(),
            label: "Wait".to_string(),
            description: Some("See what happens".to_string()),
            icon: Some("fa-solid fa-hourglass-half".to_string()),
            category: ActionCategory::Utility,
        },
    ]
}