//! Shared data models and types

use serde::{Deserialize, Serialize};

// Tag data
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub count: u32,
}

impl Tag {
    pub fn new(id: String, name: String, count: u32) -> Self {
        Self { id, name, count }
    }
    
    pub fn display_with_count(&self) -> String {
        format!("{} ({})", self.name, self.count)
    }
}

// Story participant data
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct StoryParticipant {
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

// Story data - supports group conversations with multiple characters
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct StoryItem {
    pub id: String,
    pub title: String,
    pub characters: Vec<StoryParticipant>,
    pub user_character: Option<StoryParticipant>, // User's character persona
    pub last_message: String,
    pub last_speaker: String, // Name of who sent the last message
    pub timestamp: String,
    pub scenario_name: Option<String>,
    pub message_count: u32,
}

// Character data
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub avatar_url: Option<String>,
    pub tags: Vec<String>,
    pub is_favorite: bool,
    pub last_used: Option<String>,
    pub story_count: u32,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum CharacterFilter {
    All,
    Favorites,
    Recent,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum ViewMode {
    Grid,
    List,
}

// Scenario data
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ScenarioItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub avatar_url: Option<String>,
    pub tags: Vec<String>,
    pub is_favorite: bool,
    pub story_count: u32,
    pub last_used: Option<String>,
}

// Generic card item trait for characters, scenarios, etc.
pub trait CardItem {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn avatar_url(&self) -> &Option<String>;
    fn tags(&self) -> &Vec<String>;
    fn is_favorite(&self) -> bool;
    fn story_count(&self) -> u32;
}

impl CardItem for CharacterItem {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn avatar_url(&self) -> &Option<String> {
        &self.avatar_url
    }
    fn tags(&self) -> &Vec<String> {
        &self.tags
    }
    fn is_favorite(&self) -> bool {
        self.is_favorite
    }
    fn story_count(&self) -> u32 {
        self.story_count
    }
}

impl CardItem for ScenarioItem {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn avatar_url(&self) -> &Option<String> {
        &self.avatar_url
    }
    fn tags(&self) -> &Vec<String> {
        &self.tags
    }
    fn is_favorite(&self) -> bool {
        self.is_favorite
    }
    fn story_count(&self) -> u32 {
        self.story_count
    }
}

// Flexible metadata for card display
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CardMetadata {
    pub icon: &'static str,
    pub count: u32,
    pub label: &'static str,
}
