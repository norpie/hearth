# Layout Architecture Overview

## 1. **Shared UI Components** (`ui/` crate)

### Core Shared Components:
- **`PageHeader`** - Universal page header with optional back button
- **`SettingsSection`** & **`SettingsItem`** - Settings UI components
- **`FilterTab`** & **`CategoryChip`** - Navigation/filtering elements
- **`DarkModeToggle`** - Dark mode switch
- **`Logo`** & **`LogoWithText`** - Brand components
- **Chat components** - All messaging UI (`MessageList`, `ChatInput`, etc.)
- **Card components** - Content cards for conversations, characters, scenarios

### Shared Models:
- **`AppRoute` trait** - Common routing interface
- **`DarkModeContext`** - Theme state management
- **Data models** - `ConversationItem`, `CharacterItem`, `ScenarioItem`, etc.

## 2. **Mobile Layout** (`mobile/src/views/app_layout.rs`)

```
┌─────────────────┐
│   Page Content  │ ← Uses shared `PageHeader`
│                 │
│                 │
│    Main Area    │ ← Uses shared cards/components
│                 │
│                 │
├─────────────────┤
│ [💬][👤][🗺️][⚙️] │ ← `BottomNavigation` (mobile-specific)
└─────────────────┘
```

**Key Features:**
- **Bottom navigation** - 4 tabs, hidden during chat
- **Full-screen content** - Single column layout
- **Safe area classes** - Handles notched devices
- **Platform-specific wrapper**: `MobileAppLayout`
- **Reuses**: `PageHeader` as `MobilePageHeader` (alias)

## 3. **Web/Desktop Layout** (nearly identical)

```
┌──────┬────────────────┐
│ Logo │   Page Header  │ ← Uses shared `PageHeader` 
├──────┼────────────────┤
│ [💬] │                │
│ [👤] │                │ ← Uses shared cards/components
│ [🗺️] │   Main Area    │
│ [⚙️] │                │
│      │                │
├──────┼────────────────┤
│ Info │                │
└──────┴────────────────┘
```

**Key Features:**
- **Left sidebar** - Persistent navigation (264px wide)
- **Logo/brand section** - At top of sidebar
- **Two-column layout** - Sidebar + content
- **Platform wrappers**: `WebAppLayout`, `DesktopAppLayout`
- **Reuses**: `PageHeader` via `WebPageHeader`/`DesktopPageHeader` wrappers

## 4. **Chat Views - Special Layouts**

### Mobile Chat:
- **Full-screen** with custom header
- **Bottom sheets** for participants/scenario (overlay modals)
- **Hidden bottom nav** during chat

### Web/Desktop Chat:
- **Three-column layout**:
  - Left: Participants sidebar (320px)
  - Center: Chat messages
  - Right: Scenario sidebar (320px)
- **Collapsible sidebars** with toggle buttons
- **Persistent layout** (no overlay modals)

## 5. **Component Reuse Patterns**

### 100% Shared (ui/ crate):
- All **chat components** (MessageList, ChatInput, ParticipantCard, etc.)
- All **card components** (ConversationCard, CharacterCard, ScenarioCard)
- All **layout utilities** (PageHeader, SettingsSection, FilterTab, etc.)
- **Logo** and **brand components**
- **Data models** and **context providers**

### Platform-Specific:
- **Layout containers**: `MobileAppLayout`, `WebAppLayout`, `DesktopAppLayout`
- **Navigation**: `BottomNavigation` (mobile) vs `WebSidebar`/`DesktopSidebar`
- **Route enums**: `MobileRoute`, `WebRoute`, `DesktopRoute`
- **Header wrappers**: Platform-specific `PageHeader` wrappers

### Smart Reuse:
- **Chat views**: Same components, different layouts (stacked vs sidebar)
- **Settings**: Identical component usage, different container layouts
- **Page headers**: Shared component with platform-specific wrappers (back button logic)

## 6. **Layout Inheritance Chain**

```
App Root (mobile_app.rs, web_app.rs, desktop_app.rs)
  ↓
Platform Layout (MobileAppLayout, WebAppLayout, DesktopAppLayout)  
  ↓
Page Views (ConversationsView, CharactersView, etc.)
  ↓
Shared UI Components (PageHeader, Cards, etc.)
```

## 7. **Key Differences Summary**

| Aspect | Mobile | Web/Desktop |
|--------|--------|-------------|
| **Navigation** | Bottom tabs | Left sidebar |
| **Chat Layout** | Single column + modals | Three columns |
| **Screen Usage** | Full-screen, stacked | Multi-column, persistent |
| **Back Buttons** | Yes (PageHeader) | No (sidebar nav) |
| **Safe Areas** | Yes | No |
| **Component Reuse** | ~90% shared | ~90% shared |

The architecture maximizes code reuse (~90% shared components) while optimizing each platform's UX patterns through minimal platform-specific layout wrappers.