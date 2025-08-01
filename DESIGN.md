# Hearth Architecture Design

## Overview

Hearth is a cross-platform LLM roleplay application with flexible deployment options. This document outlines the architectural decisions for supporting both local-only and centralized server deployments while maintaining a consistent user experience across web, desktop, and mobile platforms.

## Core Architecture Principles

1. **Platform Agnostic**: Shared business logic across all platforms
2. **Deployment Flexibility**: Works standalone or with optional central server
3. **API Abstraction**: Unified interface for different LLM providers
4. **Data Portability**: Easy migration between local and server deployments

## Backend Architecture

### Deployment Modes

Hearth supports two primary deployment modes:

#### 1. Local-Only Mode
- **Desktop App**: Standalone application with embedded backend
- **Mobile App**: Standalone mobile app with embedded backend
- **Local Data**: SQLite database for characters, conversations, settings
- **Direct LLM Access**: Uses llm-gateway for direct API connections

#### 2. Server Mode
- **Backend Server**: Central Rust server handling all data and LLM operations
- **Web App**: Dioxus web application served by the backend server
- **Mobile App**: Connects to backend API for either backend and LLM or just backend
- **Desktop App**: Connects to backend API for either backend and LLM or just backend

```
Server Mode Architecture:
┌─────────────┐    ┌─────────────────────────────┐
│ Mobile App  │    │      Backend Server         │
├─────────────┤    ├─────────────────────────────┤
│ API Client  │◄──►│ Web Server (serves web app) │
└─────────────┘    │ REST/WebSocket API          │
                   │ llm-gateway integration     │
┌─────────────┐    │ PostgreSQL Database         │
│Desktop App  │    │ User Authentication         │
├─────────────┤    └─────────────────────────────┘
│ API Client  │◄──►              ▲
└─────────────┘                  │
                                 ▼
┌─────────────┐           ┌─────────────┐
│   Web App   │           │ Web Browser │
│(served by   │◄─────────►│   Client    │
│ backend)    │           └─────────────┘
└─────────────┘
```

## LLM API Integration

### llm-gateway Integration

Hearth uses the existing `llm-gateway` library for LLM abstraction:

- **Provider Support**: Leverages llm-gateway's existing provider implementations
- **Configuration**: Uses llm-gateway's configuration system
- **Streaming**: Real-time response streaming through llm-gateway
- **Model Management**: Provider and model discovery via llm-gateway APIs

### Configuration Management

LLM provider configurations leverage llm-gateway's existing patterns:

- **Local Storage**: Configuration files and encrypted credentials
- **Server Storage**: Centralized provider configurations with user overrides
- **Runtime Configuration**: Dynamic provider switching and model selection

## Core Roleplay Concepts

### Entity (Generalized Character/Persona Model)
A unified model representing any roleplay identity - whether AI-controlled (character) or user-controlled (persona). This allows for a shared library where users can browse, import, and adapt entities for different roles.

**Core Components:**
- **Name**: Display name for the entity
- **Description**: Detailed background, appearance, and personality traits
- **Personality**: Core personality traits and behavioral patterns
- **Avatar**: Visual representation of the entity
- **Tags**: Searchable categories (e.g., "fantasy", "modern", "historical")
- **Template Macros**: Support for `{{char}}` and `{{user}}` placeholders

**Format Support:**
- **W++ Format**: Structured format using `[Character("Name") { ... }]` syntax
  ```
  [Character("Alice") {
    Species("Human")
    Age("25") 
    Personality("Cheerful" + "Curious" + "Intelligent")
    Body("Tall" + "Athletic" + "Blue eyes")
    Likes("Reading" + "Coffee" + "Rainy days")
    Dislikes("Crowds" + "Loud noises")
  }]
  ```
- **Plain Text**: Natural language descriptions
- **Structured Fields**: Separate fields for different aspects

### Character (Bot Instance)
An entity configured specifically for AI-controlled roleplay interactions.

**Additional Components:**
- **Default History**: Pre-written conversation history that provides context, or empty for fresh start
- **Example Dialogue**: Sample conversations to reinforce character voice
- **Default Scenario**: Preferred context/setting for interactions
- **Response Style**: Preferred length, tone, and format for AI responses

**Default History Options:**
- **Empty Start**: No pre-existing context, conversation begins fresh
- **Contextual Opening**: Pre-written exchange that establishes relationship, setting, or situation
- **Mid-Conversation**: Ongoing dialogue that drops user into an existing scenario
- **Memory Fragments**: Scattered conversation pieces that suggest shared history without linear narrative

**Default History Benefits:**
- **Rich Context**: Provides immediate relationship dynamics and situational awareness
- **Character Consistency**: Shows character voice and personality through actual dialogue
- **Scenario Integration**: Seamlessly integrates character with specific scenarios
- **User Onboarding**: Helps users understand character personality and interaction style
- **Flexible Starting Points**: Supports various narrative entry points beyond simple introductions

**Implementation:**
- **Template Messages**: Pre-written message exchanges with configurable persona placeholders
- **Dynamic Context**: Default history can reference current scenario or persona details
- **Multiple Variants**: Characters can have several default history options for different scenarios
- **User Choice**: Users can choose to start with default history or begin fresh
- **Continuation Aware**: System knows whether messages are default history or live conversation

### Persona (User Instance)  
An entity configured specifically for user representation in roleplay.

**Additional Components:**
- **Locking Preferences**: Default, character-locked, chat-locked, or scenario-specific
- **Injection Settings**: How persona description gets injected into prompts
- **Privacy Settings**: Control over what information is shared in multi-user scenarios

### Entity Library
A shared collection of entities that can be used as templates for both characters and personas.

**Features:**
- **Browse & Search**: Find entities by name, tags, or description
- **Import & Adapt**: Copy entities and modify them for specific use cases
- **Community Sharing**: Public entities contributed by users
- **Version Control**: Track changes and updates to entities
- **Rating System**: User feedback on entity quality and effectiveness

### Scenario (Enhanced System)
An advanced scenario system designed to handle complex simulations and environments that traditionally cause LLM confusion when implemented as character cards.

**Core Problem Addressed:**
Current implementations (like SillyTavern) try to create complex scenarios through character descriptions, leading to:
- LLM confusion about role boundaries
- Conflicting instructions between character personality and scenario rules
- Context window bloat from scenario information mixed with character data
- Difficulty maintaining consistent world state and rules

**Enhanced Scenario Components:**

**Environment Definition:**
- **Setting**: Physical location, time period, cultural context
- **World Rules**: Physics, magic systems, social structures, technology levels
- **Atmosphere**: Mood, tone, themes, genre conventions
- **Scale**: Intimate personal scenes vs. complex institutional simulations

**System Mechanics:**
- **NPCs Management**: Background characters, crowds, institutional roles
- **Events System**: Random events, scheduled occurrences, consequence chains
- **State Tracking**: Resources, reputation, relationships, progress metrics
- **Response Templates**: Structured formats for different interaction types

**Complex Scenario Types:**

**Institutional Simulations:**
- **Schools**: Class schedules, social hierarchies, academic progress, faculty interactions
- **Workplaces**: Corporate structure, meetings, projects, office politics
- **Military**: Chain of command, missions, training, equipment management
- **Medical**: Hospital procedures, patient cases, staff rotations, emergency protocols

**World Simulations:**
- **Fantasy Kingdoms**: Politics, magic systems, guilds, economic systems
- **Sci-Fi Colonies**: Technology levels, alien encounters, resource management
- **Historical Periods**: Social norms, technological limitations, cultural practices
- **Modern Settings**: Social media, technology integration, contemporary issues

**Scenario Architecture:**

**Layered Structure:**
1. **World Layer**: Overall setting, rules, and physics
2. **Institution Layer**: Specific organizations, hierarchies, and procedures
3. **Scene Layer**: Current location, immediate context, active participants
4. **Interaction Layer**: Available actions, response templates, outcome possibilities

**Smart NPC System:**
- **Background Characters**: Crowd simulation without individual personality overhead
- **Procedural NPCs**: Generated on-demand with consistent traits and motivations
- **Institutional Roles**: Characters defined by function rather than personality
- **Relationship Mapping**: Dynamic connections between user persona and scenario inhabitants

**Context Management:**
- **Scenario Context**: Separate from character personality context
- **Progressive Revelation**: Information provided as needed rather than front-loaded
- **State Persistence**: Scenario remembers previous interactions and consequences
- **Context Switching**: Clean transitions between different scenario aspects

**Implementation Features:**

**Template System:**
- **Interaction Templates**: Standardized formats for common scenario interactions
- **Response Frameworks**: Structured approaches for different scenario types
- **Event Scripting**: Flexible event system for dynamic scenario development
- **Outcome Matrices**: Consequence mapping for user actions

**Dynamic Content:**
- **Procedural Generation**: NPCs, events, and complications generated as needed
- **Adaptive Difficulty**: Scenario complexity adjusts to user engagement level
- **Branch Management**: Multiple storylines and outcome paths
- **Continuity Tracking**: Maintaining consistent world state across sessions

**User Interface:**
- **Scenario Dashboard**: Current status, available actions, world state
- **Quick References**: Easy access to scenario rules, NPCs, and current objectives
- **Action Suggestions**: Context-aware prompts for appropriate scenario interactions
- **Progress Tracking**: Visual indicators of scenario advancement and achievements

**Example: Enhanced High School Simulation**

**World Layer:**
- Modern American high school setting
- Standard academic calendar and daily schedule
- Social hierarchy systems (cliques, popularity, academic performance)
- School rules and consequences framework

**Institution Layer:**
- Class schedule with specific subjects and teachers
- Extracurricular activities with requirements and benefits
- Faculty personalities and teaching styles
- Student body demographics and social groups

**Scene Layer:**
- Current time period (class, lunch, after school)
- Location within school (classroom, cafeteria, hallway)
- Active NPCs relevant to current scene
- Available interactions based on context

**Smart Features:**
- **Schedule Awareness**: Different NPCs and interactions available at different times
- **Reputation System**: Actions affect standing with different social groups
- **Academic Progress**: Grades and performance affect available story paths
- **Social Dynamics**: Relationships with NPCs influence scenario development

**Best Practices:**
- Separate scenario mechanics from character personalities
- Use templates for common institutional interactions
- Implement progressive complexity rather than front-loading everything
- Maintain clear boundaries between world rules and character autonomy
- Provide clear action frameworks without over-constraining creativity

### Conversation
The actual roleplay session combining character, persona, and scenario.

**Key Elements:**
- **Message History**: Complete conversation thread
- **Context Management**: Maintaining relevant information within token limits
- **Turn-based Interaction**: Alternating between user and character responses
- **Real-time Streaming**: Live response generation for natural flow

### Guiding
A non-intrusive story direction system that allows users to provide contextual guidance without breaking immersion or cluttering the conversation history.

**Core Concept:**
- **Per-Message Guidance**: Users can attach guidance to any message they send
- **Invisible to History**: Guidance is processed by the LLM but never appears in conversation logs
- **Story Direction**: Subtle hints about where the user wants the narrative to go
- **Character Behavior**: Suggestions for how the character should react or respond
- **Scene Setting**: Environmental or situational context to influence the scene

**Usage Examples:**
- *User Message*: "I look around the room nervously."
- *Guidance*: "The character should notice something is wrong and offer comfort"

- *User Message*: "What do you think we should do next?"  
- *Guidance*: "Steer toward the mystery plot, but don't make it obvious"

**Implementation:**
- **UI Integration**: Expandable guidance field below message input
- **Quick Access**: Keyboard shortcut or toggle button for easy guidance entry
- **Prompt Injection**: Guidance gets injected as system instructions before the LLM response
- **Persistence**: Guidance is stored with messages but marked as non-conversational
- **Privacy**: In multi-user scenarios, guidance remains private to the sender

**Best Practices:**
- Keep guidance subtle and suggestive rather than directive
- Focus on emotional tone, pacing, or general direction
- Avoid overly specific plot points that might feel forced
- Use guidance to maintain narrative consistency and flow

### Narrator
A built-in story narrator system that provides environmental description, scene setting, story context, and third-person narrative elements without requiring a separate character entity.

**Core Concept:**
- **Built-in System**: Narrator functionality is a core feature, not a character in the entity library
- **Story Narrator**: Provides third-person narrative descriptions, environmental details, and story context
- **User Controllable**: Can be toggled on/off or set to user-controlled mode
- **Visual Distinction**: Displayed between messages in a different style (italics, different color, etc.)
- **Narrative Flow**: Bridges story elements, describes scenes, and provides context

**Narrator Modes:**
- **Automatic**: AI determines when narrator text is needed for story flow
- **User Controlled**: User can manually trigger narrator descriptions
- **Disabled**: Pure character-to-persona dialogue without narrative descriptions
- **Guided**: Narrator responds to guidance cues about scene or story changes

**Usage Examples:**
```
User: "I walk into the tavern and look around."
Character: "Welcome, stranger! What brings you to our humble establishment?"

*The ancient tavern creaks under the weight of decades of stories. Smoke curls from 
clay pipes as weathered patrons huddle over their drinks. Rain begins to patter 
against the diamond-paned windows.*

User: "I approach the bar and order a drink."
Character: "What'll it be? We've got ale, wine, or something stronger if you need it."

*The barkeep's eyes linger on the mud-stained cloak and the way the stranger's 
hand instinctively moves toward their weapon.*
```

**Implementation Features:**
- **Scene Setting**: Describes environments, atmosphere, and physical details
- **Story Progression**: Bridges time gaps, location changes, and story developments  
- **Contextual Awareness**: References established setting and previous narrative elements
- **Action Description**: Describes consequences of actions, environmental changes
- **Mood Enhancement**: Adds atmospheric details to enhance story immersion

**UI Integration:**
- **Toggle Control**: Easy on/off switch in conversation settings
- **Manual Trigger**: Button or command to invoke narrator when in user-controlled mode
- **Style Settings**: Options for narrative style (detailed vs. minimal, tone, perspective)
- **Visual Design**: Distinct formatting (italics, indentation, different color) to separate from dialogue

**Best Practices:**
- Focus on environmental details and story context rather than character thoughts
- Describe scenes, consequences, and atmosphere to enhance immersion
- Respond to major story moments, location changes, and time transitions
- Maintain consistent narrative perspective and tone

### Group Chat Deck System
A flexible participant management system that allows dynamic control over multi-character conversations without creating rigid "group chat" structures.

**Core Concept:**
- **Participant Deck**: A collection of available characters that can be easily enabled/disabled
- **Dynamic Participation**: Characters can be added or removed from active conversation mid-chat
- **Flexible Triggers**: Multiple modes for determining when characters participate
- **No Fixed Groups**: Same deck can be used across different conversations with different active participants

**Participant Deck Features:**
- **Character Pool**: Library of characters available for the conversation
- **Active/Inactive Toggle**: Easy on/off switches for each character
- **Priority Ordering**: Drag-and-drop reordering to influence participation likelihood
- **Quick Presets**: Save/load common participant combinations
- **Visual Overview**: Clear display of who's active and their trigger settings

**Trigger Modes per Character:**

**Manual Trigger:**
- Character only speaks when explicitly triggered by user
- User button/command to invoke specific character responses
- Full control over conversation flow and timing
- Ideal for structured scenes or specific character moments

**Random Trigger:**
- Character has chance to participate based on configurable probability
- Randomness adds natural conversation unpredictability
- Frequency settings (rare, occasional, frequent)
- Can be influenced by conversation context and character relevance

**LLM Trigger:**
- AI decides when character should participate based on context
- Considers character personality, current conversation topic, relationships
- Most natural feeling but less user control
- Smart enough to avoid over-participation or awkward interruptions

**Hybrid Modes:**
- **LLM + Random**: AI decision weighted by random chance
- **Manual Override**: Any mode can be manually triggered by user
- **Context Aware**: Trigger sensitivity based on keywords, topics, or character mentions

**Implementation Features:**

**UI/UX:**
- **Deck Panel**: Collapsible sidebar showing all available characters
- **Quick Toggle**: One-click enable/disable for each character
- **Trigger Mode Selector**: Dropdown or icons for each character's mode
- **Participation History**: Visual indicators of recent character activity
- **Bulk Controls**: Enable/disable multiple characters at once

**Conversation Flow:**
- **Turn Management**: Smart ordering when multiple characters want to respond
- **Response Queuing**: Handle multiple simultaneous character responses
- **Conflict Resolution**: Prevent character response conflicts or over-talking
- **Natural Pacing**: Avoid overwhelming conversation flow

**Advanced Features:**
- **Character Relationships**: Characters more likely to respond to specific other characters
- **Topic Affinity**: Characters triggered more often by relevant conversation topics
- **Energy Levels**: Characters can have participation energy that depletes/recharges
- **Scene Awareness**: Trigger sensitivity based on current scenario context

**Usage Examples:**

**Tavern Scene with 4-Character Deck:**
- **Bartender** (LLM Trigger): Responds to drink orders and tavern-related topics
- **Regular Patron** (Random Trigger - Rare): Occasional background comments
- **Mysterious Stranger** (Manual Trigger): User controls dramatic timing
- **Bard** (LLM + Random): Sings/tells stories when atmosphere is right

**Meeting Scene with 6-Character Deck:**
- **Meeting Leader** (LLM Trigger): Responds to agenda items and questions
- **Allies** (LLM Trigger): Support user's character
- **Opposition** (Manual Trigger): User controls conflict timing
- **Neutral Parties** (Random Trigger): Occasional input
- **Secretary** (Manual Trigger): Takes notes, provides information when asked

**Best Practices:**
- Start with 2-3 active characters to avoid overwhelming conversation
- Use manual triggers for important story moments
- Mix trigger modes to create natural conversation dynamics
- Consider character personalities when setting trigger frequencies
- Adjust active participants based on scene changes

## Data Architecture

### Core Data Entities

**Entities**: Shared library of roleplay identities with name, description, personality, avatar, tags, and format support (W++, plain text, structured). Can be used as templates for both characters and personas.

**Characters**: Entity instances configured for AI-controlled roleplay with additional default history (pre-written context or empty start), example dialogue, default scenario, and response style settings.

**Personas**: Entity instances configured for user representation with locking preferences, injection settings, and privacy controls.

**Scenarios**: Enhanced roleplay contexts with layered architecture (world/institution/scene/interaction), smart NPC systems, state tracking, event systems, and template frameworks for complex simulations.

**Conversations**: Sessions linking character, persona, and scenario with message history, narrator settings (mode, style preferences, enabled/disabled), and participant deck configuration (active characters, trigger modes, presets).

**Messages**: Individual chat messages with role ('user', 'character', 'narrator'), content, timestamp, metadata, edit history, and optional guidance (stored separately, not in conversation flow).

**LLM Configs**: Provider configurations with encrypted credentials and settings.

### Multi-Device Strategy

In server mode, all clients operate through the backend API continuously:

1. **Real-Time Operations**: All data operations go through backend API
2. **No Local Caching**: Clients are thin, backend is source of truth
3. **WebSocket Updates**: Real-time notifications for multi-device updates
4. **Offline Handling**: Graceful degradation when server unavailable

```
Multi-Device Flow:
Device A ──► Backend API ──► Database
              │
              ├─► WebSocket ──► Device B (live updates)
              │
              └─► WebSocket ──► Device C (live updates)
```


