//! Participant deck management components

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ParticipantDeckProps {
    pub participants: Vec<ParticipantData>,
    pub is_collapsed: bool,
    pub on_toggle_collapse: EventHandler<()>,
    pub on_participant_toggle: EventHandler<String>,
    pub on_trigger_mode_change: EventHandler<(String, TriggerMode)>,
    pub on_manual_trigger: EventHandler<String>,
    pub on_reorder: EventHandler<(String, usize)>,
}

#[derive(Props, Clone, PartialEq)]
pub struct ParticipantCardProps {
    pub participant: ParticipantData,
    pub on_toggle: EventHandler<()>,
    pub on_trigger_mode_change: EventHandler<TriggerMode>,
    pub on_manual_trigger: EventHandler<()>,
}

#[derive(Props, Clone, PartialEq)]
pub struct TriggerModeSelectorProps {
    pub current_mode: TriggerMode,
    pub is_active: bool,
    pub on_change: EventHandler<TriggerMode>,
}

#[derive(Clone, PartialEq)]
pub struct ParticipantData {
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub trigger_mode: TriggerMode,
    pub recent_activity: Option<String>,
    pub can_trigger_now: bool,
}

#[derive(Clone, PartialEq)]
pub enum TriggerMode {
    Manual,
    Random { frequency: RandomFrequency },
    LLM,
    Hybrid { llm_weight: f32, random_weight: f32 },
}

#[derive(Clone, PartialEq)]
pub enum RandomFrequency {
    Rare,
    Occasional,
    Frequent,
}

#[component]
pub fn ParticipantDeck(props: ParticipantDeckProps) -> Element {
    rsx! {
        div {
            class: format!(
                "bg-white dark:bg-gray-900 border-l border-gray-200 dark:border-gray-700 transition-all duration-300 {}",
                if props.is_collapsed { "w-12" } else { "w-80" },
            ),
            // Header
            div { class: "p-4 border-b border-gray-200 dark:border-gray-700",
                if props.is_collapsed {
                    button {
                        class: "w-full text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-100",
                        onclick: move |_| props.on_toggle_collapse.call(()),
                        i { class: "fa-solid fa-users" }
                    }
                } else {
                    div { class: "flex items-center justify-between",
                        h3 { class: "font-medium text-gray-900 dark:text-gray-100",
                            "Participants"
                        }
                        button {
                            class: "text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-100",
                            onclick: move |_| props.on_toggle_collapse.call(()),
                            "←"
                        }
                    }
                }
            }
            if !props.is_collapsed {
                // Participant list
                div { class: "flex-1 overflow-y-auto p-4 space-y-3",
                    for participant in props.participants.iter() {
                        {
                            let participant_id = participant.id.clone();
                            let participant_id_2 = participant.id.clone();
                            let participant_id_3 = participant.id.clone();
                            rsx! {
                                ParticipantCard {
                                    participant: participant.clone(),
                                    on_toggle: move |_| {
                                        props.on_participant_toggle.call(participant_id.clone());
                                    },
                                    on_trigger_mode_change: move |mode| {
                                        props.on_trigger_mode_change.call((participant_id_2.clone(), mode));
                                    },
                                    on_manual_trigger: move |_| {
                                        props.on_manual_trigger.call(participant_id_3.clone());
                                    },
                                }
                            }
                        }
                    }
                }
                // Quick actions
                div { class: "p-4 border-t border-gray-200 dark:border-gray-700",
                    div { class: "flex space-x-2",
                        button { class: "flex-1 px-3 py-2 text-sm bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700",
                            "Enable All"
                        }
                        button { class: "flex-1 px-3 py-2 text-sm bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700",
                            "Disable All"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ParticipantCard(props: ParticipantCardProps) -> Element {
    let participant = props.participant.clone();
    
    rsx! {
        div {
            class: format!(
                "border rounded-lg p-3 transition-all {}",
                if participant.is_active {
                    "border-chat-character bg-chat-character/5"
                } else {
                    "border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 opacity-60"
                },
            ),
            // Header row
            div { class: "flex items-center justify-between mb-2",
                div { class: "flex items-center space-x-2",
                    // Avatar
                    if let Some(avatar_url) = &participant.avatar_url {
                        img {
                            class: "w-8 h-8 rounded-full",
                            src: "{avatar_url}",
                            alt: "{participant.name}",
                        }
                    } else {
                        div { class: "w-8 h-8 rounded-full bg-chat-character flex items-center justify-center text-white text-sm font-medium",
                            "{participant.name.chars().next().unwrap_or('?').to_uppercase()}"
                        }
                    }
                    // Name and status
                    div {
                        div { class: "font-medium text-sm text-gray-900 dark:text-gray-100",
                            "{participant.name}"
                        }
                        if let Some(activity) = &participant.recent_activity {
                            div { class: "text-xs text-gray-500", "{activity}" }
                        }
                    }
                }
                // Toggle switch
                button {
                    class: format!(
                        "w-10 h-6 rounded-full transition-colors relative {}",
                        if participant.is_active {
                            "bg-chat-character"
                        } else {
                            "bg-gray-300 dark:bg-gray-600"
                        },
                    ),
                    onclick: move |_| props.on_toggle.call(()),
                    div {
                        class: format!(
                            "w-4 h-4 bg-white rounded-full transition-transform absolute top-1 {}",
                            if participant.is_active { "translate-x-5" } else { "translate-x-1" },
                        ),
                    }
                }
            }
            if participant.is_active {
                // Trigger mode selector
                TriggerModeSelector {
                    current_mode: participant.trigger_mode.clone(),
                    is_active: participant.is_active,
                    on_change: props.on_trigger_mode_change,
                }
                // Manual trigger button (if applicable)
                if matches!(participant.trigger_mode, TriggerMode::Manual)
                    || participant.can_trigger_now
                {
                    div { class: "mt-2",
                        button {
                            class: format!(
                                "w-full px-3 py-1 text-sm rounded {}",
                                if participant.can_trigger_now {
                                    "bg-chat-character text-white hover:bg-chat-character/90"
                                } else {
                                    "bg-gray-200 dark:bg-gray-700 text-gray-500 cursor-not-allowed"
                                },
                            ),
                            disabled: !participant.can_trigger_now,
                            onclick: move |_| {
                                if participant.can_trigger_now {
                                    props.on_manual_trigger.call(());
                                }
                            },
                            "Trigger Response"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TriggerModeSelector(props: TriggerModeSelectorProps) -> Element {
    if !props.is_active {
        return rsx! {
            div {}
        };
    }

    rsx! {
        div { class: "mt-2",
            label { class: "block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1",
                "Trigger Mode"
            }
            select {
                class: "w-full px-2 py-1 text-xs border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100",
                onchange: move |evt| {
                    let mode = match evt.value().as_str() {
                        "manual" => TriggerMode::Manual,
                        "random_rare" => {
                            TriggerMode::Random {
                                frequency: RandomFrequency::Rare,
                            }
                        }
                        "random_occasional" => {
                            TriggerMode::Random {
                                frequency: RandomFrequency::Occasional,
                            }
                        }
                        "random_frequent" => {
                            TriggerMode::Random {
                                frequency: RandomFrequency::Frequent,
                            }
                        }
                        "llm" => TriggerMode::LLM,
                        "hybrid" => {
                            TriggerMode::Hybrid {
                                llm_weight: 0.7,
                                random_weight: 0.3,
                            }
                        }
                        _ => TriggerMode::Manual,
                    };
                    props.on_change.call(mode);
                },
                option {
                    value: "manual",
                    selected: matches!(props.current_mode, TriggerMode::Manual),
                    "Manual"
                }
                option {
                    value: "random_rare",
                    selected: matches!(
                        props.current_mode,
                        TriggerMode::Random { frequency: RandomFrequency::Rare }
                    ),
                    "Random (Rare)"
                }
                option {
                    value: "random_occasional",
                    selected: matches!(
                        props.current_mode,
                        TriggerMode::Random { frequency: RandomFrequency::Occasional }
                    ),
                    "Random (Occasional)"
                }
                option {
                    value: "random_frequent",
                    selected: matches!(
                        props.current_mode,
                        TriggerMode::Random { frequency: RandomFrequency::Frequent }
                    ),
                    "Random (Frequent)"
                }
                option {
                    value: "llm",
                    selected: matches!(props.current_mode, TriggerMode::LLM),
                    "AI Controlled"
                }
                option {
                    value: "hybrid",
                    selected: matches!(props.current_mode, TriggerMode::Hybrid { .. }),
                    "Hybrid (AI + Random)"
                }
            }
        }
    }
}