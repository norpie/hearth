# Cross-Platform Development

## Cross-Platform Architecture

### Platform-Specific Features

**Mobile Platforms:**
- **Gesture Detection**: Touch gesture recognition (swipe, tap, pan) via `GestureDetector` component
- **Mobile Navbar Context**: Context-based navigation state management for mobile layouts
- **Bottom Navigation**: Tab-based navigation optimized for thumb interaction
- **Touch-Optimized Components**: Larger touch targets and mobile-appropriate spacing

**Desktop/Web Platforms:**
- **Sidebar Navigation**: Traditional sidebar layout with collapsible sections
- **Hover States**: Mouse hover interactions and tooltip positioning
- **Keyboard Navigation**: Full keyboard accessibility support
- **Right-Click Context**: Context menu support where appropriate

**Shared Across Platforms:**
- **Adaptive Layouts**: Components automatically adjust to platform capabilities
- **Responsive Design**: Tailwind-based responsive breakpoints work across all platforms
- **Toast Notifications**: Consistent feedback system across all platforms

### Platform Detection Rules

- **NEVER use runtime platform detection** - platform is determined at compile time
- Use conditional compilation: `#[cfg(target_arch = "wasm32")]` for web, `#[cfg(feature = "desktop")]` for desktop, etc.

### Viewport System
- Use `use_viewport()` hook to get viewport dimensions in components

## Platform Async Utilities

The Platform struct provides unified async utilities that abstract cross-platform differences:

**Platform::spawn()**
```rust
use crate::Platform;

// Cross-platform async task spawning
Platform::spawn(async move {
    // Your async code here
});
```

**Platform::sleep()**
```rust
use crate::Platform;
use std::time::Duration;

// Cross-platform async delay
Platform::sleep(Duration::from_millis(100)).await;
Platform::sleep(Duration::from_secs(1)).await;
```

**IMPORTANT RULES:**
- **ALWAYS use Platform utilities** instead of conditional compilation for async operations
- **Import Platform**: Add `use crate::Platform;` or include it in your crate imports
- **NEVER manually use `spawn`/`spawn_local`** - use `Platform::spawn()` instead
- **NEVER manually use timer crates** - use `Platform::sleep()` instead
- **Duration works everywhere**: `std::time::Duration` is available on all platforms

**Before (DON'T DO THIS):**
```rust
#[cfg(target_arch = "wasm32")]
{
    spawn_local(async move { ... });
    gloo_timers::future::sleep(Duration::from_millis(100)).await;
}
#[cfg(not(target_arch = "wasm32"))]
{
    spawn(async move { ... });
    tokio::time::sleep(Duration::from_millis(100)).await;
}
```

**After (CORRECT):**
```rust
Platform::spawn(async move { ... });
Platform::sleep(Duration::from_millis(100)).await;
```

This approach eliminates repetitive conditional compilation and provides a clean, unified API for async operations across web, desktop, and mobile platforms.

## Layout Architecture

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