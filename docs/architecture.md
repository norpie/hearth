# Architecture Overview

## Project Overview

Hearth is a cross-platform LLM roleplay application competing with SillyTavern and TavernAI. Features:
- Cross-platform support (web, desktop, mobile)
- Flexible deployment options:
  - Self-hostable server for multi-user, multi-device syncing
  - Self-contained local operation
- Bring your own LLM API approach

## Project Structure

**Dioxus multi-platform workspace** with unified UI architecture:

### Core Architecture
- `hearth-ui/` - **Unified UI library** containing all shared components, views, routing, and application logic
- `hearth-core/` - Core data models, settings management, logging, markdown processing, and storage utilities
- `hearth-web/` - Web platform entry point (minimal wrapper around hearth-ui)
- `hearth-desktop/` - Desktop platform entry point (minimal wrapper around hearth-ui)
- `hearth-mobile/` - Mobile platform entry point with platform-specific asset handling

### Current Implementation Details

**Platform Entry Points:**
- All platforms use simple `main.rs` files that launch `hearth_ui::App`
- `hearth-mobile/` includes platform-specific CSS asset loading
- `hearth-web/` and `hearth-desktop/` are minimal wrappers

**Unified UI Structure (`hearth-ui/`):**
```
hearth-ui/src/
├── lib.rs                 # Crate root and main exports
├── app.rs                 # Main application component with routing
├── routes.rs              # Route definitions and navigation
├── layout.rs              # Adaptive layout system (mobile/desktop)
├── models.rs              # UI data models, platform detection, and state
├── sample.rs              # Sample/mock data for development
├── settings.rs            # Settings management and configuration
├── logo.rs                # Logo components (SVG/PNG variants)
├── viewport.rs            # Cross-platform viewport abstraction
├── views/                 # Main application views/pages
│   ├── mod.rs             # View module exports
│   ├── design/            # Design system showcase
│   │   ├── mod.rs         # Main design view with component selector
│   │   ├── showcase.rs    # ComponentShowcase wrapper components
│   │   └── showcases/     # Individual component showcases (20+ components)
│   ├── characters.rs      # Character management view
│   ├── scenarios.rs       # Scenario management view  
│   ├── stories.rs         # Story management view
│   ├── story.rs           # Interactive storytelling view with narrator/character/user messages
│   └── settings.rs        # Settings configuration view
└── components/            # Two-tier component architecture
    ├── mod.rs             # Component module organization
    ├── ui/                # Generic reusable UI components (25+ components)
    │   ├── accordion.rs   # Expandable/collapsible content sections
    │   ├── aspect_ratio.rs # Maintain aspect ratios for responsive layouts
    │   ├── avatar.rs      # User/character profile images and initials
    │   ├── avatar_group.rs # Multiple avatar displays for groups
    │   ├── badge.rs       # Status indicators and labels
    │   ├── button.rs      # Interactive buttons with variants and sizes
    │   ├── calendar.rs    # Date picker and calendar widgets
    │   ├── card.rs        # Content containers with elevation and borders
    │   ├── carousel.rs    # Image and content slider/rotation
    │   ├── checkbox.rs    # Boolean selection input
    │   ├── collapsible.rs # Show/hide content with animation
    │   ├── gesture_detector.rs # Touch gesture recognition (swipe, tap, etc.)
    │   ├── input.rs       # Text input fields with validation
    │   ├── input_otp.rs   # One-time password/code entry
    │   ├── label.rs       # Form field labels and descriptions
    │   ├── layout.rs      # Layout helper components and containers
    │   ├── markdown_content.rs # Render markdown with custom styling
    │   ├── modal.rs       # Overlay dialogs and popups
    │   ├── notice.rs      # Alert and notification displays
    │   ├── popover.rs     # Contextual overlay content
    │   ├── progress.rs    # Progress bars and loading indicators
    │   ├── radio.rs       # Single-choice selection from options
    │   ├── range_calendar.rs # Date range selection widget
    │   ├── scroll_area.rs # Custom scrollable containers with fade effects
    │   ├── select.rs      # Dropdown selection menus
    │   ├── separator.rs   # Visual dividers between content
    │   ├── sheet.rs       # Bottom sheets and slide-out panels
    │   ├── skeleton.rs    # Loading state placeholders
    │   ├── slider.rs      # Range input sliders
    │   ├── switch.rs      # Toggle switches for boolean states
    │   ├── table.rs       # Data display tables
    │   ├── tabs.rs        # Tabbed interface navigation
    │   ├── textarea.rs    # Multi-line text input
    │   ├── toaster.rs     # Toast notification system
    │   ├── toggle.rs      # Toggle buttons with pressed states
    │   ├── toggle_group.rs # Grouped toggle button controls
    │   └── toggle_icon.rs # Icon-based toggle buttons
    └── features/          # Feature-specific page sections
        ├── log_viewer.rs  # Development and debug log display
        └── universal_search/ # Global search and filtering system
            ├── search_dropdown.rs # Search results dropdown
            ├── search_toggle.rs   # Search toggle button
            ├── tag.rs             # Tag display components
            ├── tag_modal.rs       # Tag management modal
            └── tag_section.rs     # Tag organization sections
```

**Asset Management:**
- `hearth-ui/assets/` - Structured asset organization with fonts, icons, and modular CSS
  - `fonts/` - Inter Variable and FontAwesome font files (woff2 format)
  - `icons/` - Complete icon set from 32px to 1024px PNG plus SVG
  - `components/` - Component-specific CSS (scrollbar, toast, tooltip)
  - `fontawesome.css` - FontAwesome icon integration
  - `fonts.css` - Font face declarations
  - `theme.css` - Color definitions and dark mode
  - `typography.css` - Text styling and prose utilities
- Platform-specific `assets/` directories are mostly empty or contain platform-specific compiled CSS

**Layout Architecture:**
The adaptive layout system in `layout.rs` provides cross-platform UI organization:
- `AdaptiveLayout` - Main layout switcher that detects platform and renders appropriate layout
- `MobileLayout` - Bottom navigation bar layout for mobile devices
- `DesktopLayout` - Sidebar navigation layout for web/desktop
- `Platform::current()` - Detects platform at compile time using feature flags
- Layout components handle responsive behavior, navigation state, and platform-specific UI patterns

```
Layout Flow:
app.rs -> AdaptiveLayout -> [MobileLayout | DesktopLayout] -> view content
│                           │                │
│                           └─ BottomNavigation  └─ AppSidebar + content
└─ Route detection & state management
```

## Development State
- Application in active development with unified UI architecture
- All major UI components and views implemented in `hearth-ui/`
- Platform entry points are minimal and primarily handle asset loading
- Tailwind CSS properly configured and compiled across all platforms
- Icon assets available in multiple resolutions for different platforms