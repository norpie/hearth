//! Sample data for demo purposes

use crate::models::*;
use std::collections::HashMap;

// Character sample data
pub fn sample_characters() -> Vec<CharacterItem> {
    vec![
        CharacterItem {
            id: "1".to_string(),
            name: "Alice".to_string(),
            description: "A cheerful tavern keeper with a mysterious past".to_string(),
            avatar_url: None,
            tags: vec![
                "Fantasy".to_string(),
                "Friendly".to_string(),
                "Tavern Keeper".to_string(),
                "Mysterious".to_string(),
                "Cheerful".to_string(),
                "Hospitality".to_string(),
                "Storyteller".to_string(),
                "Local Knowledge".to_string(),
                "Warm".to_string(),
                "Trustworthy".to_string(),
                "Helpful".to_string(),
                "Well-Connected".to_string(),
                "Social Hub".to_string(),
                "Ale Brewer".to_string(),
                "Town Center".to_string(),
                "Gossip Network".to_string(),
                "Safe Haven".to_string(),
                "Community Leader".to_string(),
                "Traditional".to_string(),
                "Maternal".to_string(),
                "Past Adventurer".to_string(),
                "Hidden Depths".to_string(),
                "Secret Keeper".to_string(),
                "Information Broker".to_string(),
                "Problem Solver".to_string(),
                "Wise Counsel".to_string(),
                "Protective".to_string(),
                "Observant".to_string(),
                "Well-Traveled".to_string(),
                "Cultural Bridge".to_string(),
            ],
            is_favorite: true,
            last_used: Some("Today".to_string()),
            story_count: 3,
        },
        CharacterItem {
            id: "2".to_string(),
            name: "Dr. Watson".to_string(),
            description: "Loyal companion and medical expert with keen observation skills"
                .to_string(),
            avatar_url: None,
            tags: vec![
                "Victorian".to_string(),
                "Detective".to_string(),
                "Medical".to_string(),
                "Loyal".to_string(),
                "Observant".to_string(),
                "Companion".to_string(),
            ],
            is_favorite: false,
            last_used: Some("Yesterday".to_string()),
            story_count: 1,
        },
        CharacterItem {
            id: "3".to_string(),
            name: "Captain Maya".to_string(),
            description:
                "Experienced starship captain leading dangerous missions across the galaxy"
                    .to_string(),
            avatar_url: None,
            tags: vec![
                "Sci-Fi".to_string(),
                "Leader".to_string(),
                "Space".to_string(),
                "Captain".to_string(),
                "Experienced".to_string(),
                "Starship".to_string(),
                "Dangerous".to_string(),
            ],
            is_favorite: true,
            last_used: Some("Last week".to_string()),
            story_count: 2,
        },
        CharacterItem {
            id: "4".to_string(),
            name: "Luna Blackwood".to_string(),
            description: "Enigmatic sorceress studying forbidden magic at the academy".to_string(),
            avatar_url: None,
            tags: vec![
                "Fantasy".to_string(),
                "Magic".to_string(),
                "Student".to_string(),
                "Sorceress".to_string(),
                "Enigmatic".to_string(),
                "Forbidden".to_string(),
                "Academy".to_string(),
                "Dark Arts".to_string(),
            ],
            is_favorite: true,
            last_used: Some("2 hours ago".to_string()),
            story_count: 5,
        },
        CharacterItem {
            id: "5".to_string(),
            name: "Detective Morgan".to_string(),
            description: "Hard-boiled detective solving crimes in the gritty city streets"
                .to_string(),
            avatar_url: None,
            tags: vec![
                "Modern".to_string(),
                "Crime".to_string(),
                "Investigator".to_string(),
                "Hard-boiled".to_string(),
                "Gritty".to_string(),
            ],
            is_favorite: false,
            last_used: Some("1 day ago".to_string()),
            story_count: 1,
        },
        CharacterItem {
            id: "6".to_string(),
            name: "Kai Nakamura".to_string(),
            description: "Cybernetic engineer navigating the neon-lit streets of Neo Tokyo"
                .to_string(),
            avatar_url: None,
            tags: vec![
                "Cyberpunk".to_string(),
                "Tech".to_string(),
                "Hacker".to_string(),
                "Engineer".to_string(),
                "Neo Tokyo".to_string(),
                "Neon".to_string(),
            ],
            is_favorite: true,
            last_used: Some("3 hours ago".to_string()),
            story_count: 4,
        },
        CharacterItem {
            id: "7".to_string(),
            name: "Princess Elara".to_string(),
            description: "Noble princess fighting to save her kingdom from ancient evil"
                .to_string(),
            avatar_url: None,
            tags: vec![
                "Fantasy".to_string(),
                "Royal".to_string(),
                "Brave".to_string(),
                "Princess".to_string(),
                "Noble".to_string(),
                "Kingdom".to_string(),
                "Ancient Evil".to_string(),
            ],
            is_favorite: false,
            last_used: Some("2 days ago".to_string()),
            story_count: 2,
        },
        CharacterItem {
            id: "8".to_string(),
            name: "Commander Vex".to_string(),
            description:
                "Battle-hardened military commander leading the resistance against alien invaders"
                    .to_string(),
            avatar_url: None,
            tags: vec![
                "Sci-Fi".to_string(),
                "Military".to_string(),
                "Leader".to_string(),
                "Commander".to_string(),
                "Resistance".to_string(),
                "Alien Invaders".to_string(),
            ],
            is_favorite: true,
            last_used: Some("5 hours ago".to_string()),
            story_count: 3,
        },
        CharacterItem {
            id: "9".to_string(),
            name: "Isabella Romano".to_string(),
            description: "Passionate artist seeking love and inspiration in the modern world"
                .to_string(),
            avatar_url: None,
            tags: vec![
                "Romance".to_string(),
                "Artist".to_string(),
                "Creative".to_string(),
                "Passionate".to_string(),
                "Inspiration".to_string(),
            ],
            is_favorite: false,
            last_used: Some("6 hours ago".to_string()),
            story_count: 1,
        },
        CharacterItem {
            id: "10".to_string(),
            name: "Zara Al-Rashid".to_string(),
            description: "Desert nomad and scholar of ancient mysteries".to_string(),
            avatar_url: None,
            tags: vec![
                "Historical".to_string(),
                "Scholar".to_string(),
                "Adventurer".to_string(),
                "Desert".to_string(),
                "Nomad".to_string(),
                "Ancient Mysteries".to_string(),
            ],
            is_favorite: true,
            last_used: Some("1 week ago".to_string()),
            story_count: 2,
        },
        CharacterItem {
            id: "11".to_string(),
            name: "Dr. Elizabeth Chen".to_string(),
            description: "Brilliant surgeon balancing life and death decisions in the ER"
                .to_string(),
            avatar_url: None,
            tags: vec![
                "Medical".to_string(),
                "Drama".to_string(),
                "Professional".to_string(),
                "Surgeon".to_string(),
                "Brilliant".to_string(),
                "Life and Death".to_string(),
                "ER".to_string(),
            ],
            is_favorite: false,
            last_used: Some("4 days ago".to_string()),
            story_count: 1,
        },
        CharacterItem {
            id: "12".to_string(),
            name: "Viktor Steele".to_string(),
            description: "Mysterious vampire lord with centuries of dark secrets".to_string(),
            avatar_url: None,
            tags: vec![
                "Supernatural".to_string(),
                "Dark".to_string(),
                "Ancient".to_string(),
                "Vampire".to_string(),
                "Mysterious".to_string(),
                "Centuries Old".to_string(),
                "Secrets".to_string(),
                "Lord".to_string(),
            ],
            is_favorite: true,
            last_used: Some("12 hours ago".to_string()),
            story_count: 6,
        },
    ]
}

// Scenario sample data
pub fn sample_scenarios() -> Vec<ScenarioItem> {
    vec![
        ScenarioItem {
            id: "1".to_string(),
            name: "Tavern Adventure".to_string(),
            description: "A cozy tavern where travelers gather to share tales and begin new adventures".to_string(),
            avatar_url: None,
            tags: vec!["Medieval".to_string(), "Social".to_string(), "Mystery".to_string(), "Cozy".to_string(), "Travelers".to_string()],
            is_favorite: true,
            story_count: 5,
            last_used: Some("3 hours ago".to_string()),
        },
        ScenarioItem {
            id: "2".to_string(),
            name: "Victorian Mystery".to_string(),
            description: "Dark streets of London hide secrets waiting to be uncovered by keen detectives".to_string(),
            avatar_url: None,
            tags: vec!["Victorian".to_string(), "Investigation".to_string(), "Crime".to_string(), "London".to_string(), "Dark Streets".to_string(), "Secrets".to_string()],
            is_favorite: false,
            story_count: 2,
            last_used: Some("2 days ago".to_string()),
        },
        ScenarioItem {
            id: "3".to_string(),
            name: "Space Station Alpha".to_string(),
            description: "Life aboard a remote space station where every decision could mean survival or disaster".to_string(),
            avatar_url: None,
            tags: vec!["Space".to_string(), "Survival".to_string(), "Command".to_string(), "Remote".to_string(), "Life or Death".to_string(), "Decisions".to_string(), "Disaster".to_string()],
            is_favorite: true,
            story_count: 3,
            last_used: Some("1 day ago".to_string()),
        },
        ScenarioItem {
            id: "4".to_string(),
            name: "Magical Academy".to_string(),
            description: "Study arcane arts and uncover ancient secrets in a prestigious magical institution".to_string(),
            avatar_url: None,
            tags: vec!["Magic".to_string(), "School".to_string(), "Learning".to_string(), "Arcane Arts".to_string(), "Ancient Secrets".to_string(), "Prestigious".to_string()],
            is_favorite: true,
            story_count: 8,
            last_used: Some("30 minutes ago".to_string()),
        },
        ScenarioItem {
            id: "5".to_string(),
            name: "Urban Crime Scene".to_string(),
            description: "Navigate the dangerous underworld of crime in a modern metropolitan setting".to_string(),
            avatar_url: None,
            tags: vec!["Police".to_string(), "Investigation".to_string(), "Urban".to_string(), "Dangerous".to_string(), "Underworld".to_string()],
            is_favorite: false,
            story_count: 4,
            last_used: Some("1 week ago".to_string()),
        },
        ScenarioItem {
            id: "6".to_string(),
            name: "Cyberpunk 2087".to_string(),
            description: "High-tech low-life in a neon-soaked future where corporations rule everything".to_string(),
            avatar_url: None,
            tags: vec!["Cyberpunk".to_string(), "Hacking".to_string(), "Dystopia".to_string(), "High-tech".to_string(), "Low-life".to_string(), "Neon".to_string(), "Corporations".to_string(), "Future".to_string()],
            is_favorite: true,
            story_count: 6,
            last_used: Some("8 hours ago".to_string()),
        },
        ScenarioItem {
            id: "7".to_string(),
            name: "Medieval Kingdom".to_string(),
            description: "Rule a kingdom through political intrigue, war, and diplomacy in medieval times".to_string(),
            avatar_url: None,
            tags: vec!["Politics".to_string(), "War".to_string(), "Diplomacy".to_string(), "Kingdom".to_string(), "Intrigue".to_string(), "Medieval".to_string()],
            is_favorite: false,
            story_count: 3,
            last_used: Some("3 days ago".to_string()),
        },
        ScenarioItem {
            id: "8".to_string(),
            name: "Galactic War".to_string(),
            description: "Command fleets across the galaxy in an epic struggle for cosmic dominance".to_string(),
            avatar_url: None,
            tags: vec!["Space Battle".to_string(), "Strategy".to_string(), "Military".to_string(), "Fleets".to_string(), "Galaxy".to_string(), "Epic".to_string(), "Cosmic Dominance".to_string()],
            is_favorite: true,
            story_count: 7,
            last_used: Some("2 hours ago".to_string()),
        },
        ScenarioItem {
            id: "9".to_string(),
            name: "Modern Romance".to_string(),
            description: "Navigate the complexities of love and relationships in contemporary society".to_string(),
            avatar_url: None,
            tags: vec!["Dating".to_string(), "Emotions".to_string(), "City Life".to_string(), "Love".to_string(), "Relationships".to_string(), "Contemporary".to_string()],
            is_favorite: false,
            story_count: 12,
            last_used: None,
        },
        ScenarioItem {
            id: "10".to_string(),
            name: "Arabian Nights".to_string(),
            description: "Experience tales of magic, adventure, and wonder in the mystical Middle East".to_string(),
            avatar_url: None,
            tags: vec!["Desert".to_string(), "Magic".to_string(), "Exploration".to_string(), "Arabian".to_string(), "Tales".to_string(), "Wonder".to_string(), "Mystical".to_string(), "Middle East".to_string()],
            is_favorite: true,
            story_count: 5,
            last_used: Some("5 days ago".to_string()),
        },
        ScenarioItem {
            id: "11".to_string(),
            name: "Medical Emergency".to_string(),
            description: "High-stakes medical drama where every decision saves or costs lives".to_string(),
            avatar_url: None,
            tags: vec!["Hospital".to_string(), "Emergency".to_string(), "Medical".to_string(), "High-stakes".to_string(), "Drama".to_string()],
            is_favorite: false,
            story_count: 2,
            last_used: Some("2 weeks ago".to_string()),
        },
        ScenarioItem {
            id: "12".to_string(),
            name: "Vampire Court".to_string(),
            description: "Navigate deadly politics and ancient grudges in the shadowy world of vampires".to_string(),
            avatar_url: None,
            tags: vec!["Supernatural".to_string(), "Politics".to_string(), "Dark".to_string(), "Vampire".to_string(), "Court".to_string(), "Deadly".to_string(), "Ancient Grudges".to_string(), "Shadowy".to_string()],
            is_favorite: true,
            story_count: 9,
            last_used: Some("6 hours ago".to_string()),
        },
    ]
}

// Available tags for search/filtering with usage counts
pub fn sample_character_tags() -> HashMap<String, u32> {
    let characters = sample_characters();
    let mut tag_counts = HashMap::new();

    // Count tag occurrences across all characters
    for character in characters {
        for tag in character.tags {
            *tag_counts.entry(tag).or_insert(0) += 1;
        }
    }

    tag_counts
}

pub fn sample_character_tags_sorted() -> Vec<(String, u32)> {
    let tag_counts = sample_character_tags();
    let mut sorted_tags: Vec<(String, u32)> = tag_counts.into_iter().collect();

    // Sort by count (descending), then by name (ascending)
    sorted_tags.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    sorted_tags
}

// Generate proper tag ID from tag name
fn generate_tag_id(name: &str) -> String {
    name.to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

pub fn sample_character_tags_with_ids() -> Vec<Tag> {
    let tag_counts = sample_character_tags();
    let mut tags: Vec<Tag> = tag_counts.into_iter()
        .map(|(name, count)| Tag::new(generate_tag_id(&name), name, count))
        .collect();

    // Sort by count (descending), then by name (ascending)
    tags.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.name.cmp(&b.name)));

    tags
}

pub fn sample_scenario_tags() -> HashMap<String, u32> {
    let scenarios = sample_scenarios();
    let mut tag_counts = HashMap::new();

    // Count tag occurrences across all scenarios
    for scenario in scenarios {
        for tag in scenario.tags {
            *tag_counts.entry(tag).or_insert(0) += 1;
        }
    }

    tag_counts
}

pub fn sample_scenario_tags_sorted() -> Vec<(String, u32)> {
    let tag_counts = sample_scenario_tags();
    let mut sorted_tags: Vec<(String, u32)> = tag_counts.into_iter().collect();

    // Sort by count (descending), then by name (ascending)
    sorted_tags.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    sorted_tags
}

pub fn sample_scenario_tags_with_ids() -> Vec<Tag> {
    let tag_counts = sample_scenario_tags();
    let mut tags: Vec<Tag> = tag_counts.into_iter()
        .map(|(name, count)| Tag::new(generate_tag_id(&name), name, count))
        .collect();

    // Sort by count (descending), then by name (ascending)
    tags.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.name.cmp(&b.name)));

    tags
}

// Story sample data - mix of 1-on-1 and group conversations
pub fn sample_stories() -> Vec<StoryItem> {
    vec![
        StoryItem {
            id: "1".to_string(),
            title: "Tavern Tales with Alice".to_string(),
            characters: vec![
                StoryParticipant {
                    id: "1".to_string(),
                    name: "Alice".to_string(),
                    avatar_url: None,
                }
            ],
            user_character: Some(StoryParticipant {
                id: "user_1".to_string(),
                name: "Theron the Traveler".to_string(),
                avatar_url: None,
            }),
            last_message: "The evening crowd is gathering, and I have some interesting stories to share...".to_string(),
            last_speaker: "Alice".to_string(),
            timestamp: "2 minutes ago".to_string(),
            scenario_name: Some("Medieval Tavern".to_string()),
            message_count: 47,
        },
        StoryItem {
            id: "2".to_string(),
            title: "Academy Investigation Squad".to_string(),
            characters: vec![
                StoryParticipant {
                    id: "4".to_string(),
                    name: "Luna".to_string(),
                    avatar_url: None,
                },
                StoryParticipant {
                    id: "2".to_string(),
                    name: "Detective Marcus".to_string(),
                    avatar_url: None,
                },
                StoryParticipant {
                    id: "8".to_string(),
                    name: "Professor Eliza".to_string(),
                    avatar_url: None,
                }
            ],
            user_character: Some(StoryParticipant {
                id: "user_2".to_string(),
                name: "Detective Sage".to_string(),
                avatar_url: None,
            }),
            last_message: "The ancient library holds more secrets than we initially thought. We need to investigate the restricted section tonight.".to_string(),
            last_speaker: "Professor Eliza".to_string(),
            timestamp: "15 minutes ago".to_string(),
            scenario_name: Some("Magical Academy".to_string()),
            message_count: 124,
        },
        StoryItem {
            id: "3".to_string(),
            title: "Space Station Crisis Command".to_string(),
            characters: vec![
                StoryParticipant {
                    id: "3".to_string(),
                    name: "Captain Nova".to_string(),
                    avatar_url: None,
                },
                StoryParticipant {
                    id: "7".to_string(),
                    name: "Engineer Knox".to_string(),
                    avatar_url: None,
                },
                StoryParticipant {
                    id: "6".to_string(),
                    name: "Dr. Sarah Chen".to_string(),
                    avatar_url: None,
                }
            ],
            user_character: Some(StoryParticipant {
                id: "user_3".to_string(),
                name: "Commander Riley".to_string(),
                avatar_url: None,
            }),
            last_message: "Hull breach in sector 7! All personnel to emergency stations immediately!".to_string(),
            last_speaker: "Captain Nova".to_string(),
            timestamp: "1 hour ago".to_string(),
            scenario_name: Some("Space Station Alpha".to_string()),
            message_count: 89,
        },
        StoryItem {
            id: "4".to_string(),
            title: "Cyberpunk Heist Crew".to_string(),
            characters: vec![
                StoryParticipant {
                    id: "5".to_string(),
                    name: "Zara".to_string(),
                    avatar_url: None,
                },
                StoryParticipant {
                    id: "9".to_string(),
                    name: "Ghost".to_string(),
                    avatar_url: None,
                },
                StoryParticipant {
                    id: "10".to_string(),
                    name: "Neon".to_string(),
                    avatar_url: None,
                },
                StoryParticipant {
                    id: "11".to_string(),
                    name: "Cipher".to_string(),
                    avatar_url: None,
                }
            ],
            user_character: Some(StoryParticipant {
                id: "user_4".to_string(),
                name: "Phoenix".to_string(),
                avatar_url: None,
            }),
            last_message: "The security feeds are looped. We have a 10-minute window to get in and out. Ghost, you're up.".to_string(),
            last_speaker: "Zara".to_string(),
            timestamp: "3 hours ago".to_string(),
            scenario_name: Some("Cyberpunk 2087".to_string()),
            message_count: 203,
        },
        StoryItem {
            id: "5".to_string(),
            title: "Coffee Date with Emma".to_string(),
            characters: vec![
                StoryParticipant {
                    id: "12".to_string(),
                    name: "Emma".to_string(),
                    avatar_url: None,
                }
            ],
            user_character: Some(StoryParticipant {
                id: "user_5".to_string(),
                name: "Alex".to_string(),
                avatar_url: None,
            }),
            last_message: "I had such a wonderful time today. Same place tomorrow?".to_string(),
            last_speaker: "Emma".to_string(),
            timestamp: "Yesterday".to_string(),
            scenario_name: Some("Modern Romance".to_string()),
            message_count: 32,
        },
        StoryItem {
            id: "6".to_string(),
            title: "Royal Court Intrigue".to_string(),
            characters: vec![
                StoryParticipant {
                    id: "13".to_string(),
                    name: "King Aldric".to_string(),
                    avatar_url: None,
                },
                StoryParticipant {
                    id: "14".to_string(),
                    name: "Lady Morgana".to_string(),
                    avatar_url: None,
                },
                StoryParticipant {
                    id: "15".to_string(),
                    name: "Sir Gareth".to_string(),
                    avatar_url: None,
                }
            ],
            user_character: Some(StoryParticipant {
                id: "user_6".to_string(),
                name: "Lord Castellan".to_string(),
                avatar_url: None,
            }),
            last_message: "The nobles grow restless. We must act swiftly before they rally against the crown.".to_string(),
            last_speaker: "Lady Morgana".to_string(),
            timestamp: "2 days ago".to_string(),
            scenario_name: Some("Medieval Kingdom".to_string()),
            message_count: 156,
        }
    ]
}
