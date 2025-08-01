# Component Development Guidelines

## Component Development Workflow

The design system uses a **ComponentShowcase system** with:
- **Single Component View**: Dropdown selector to choose which component to showcase
- **Standardized Structure**: ComponentShowcase wrapper provides consistent styling
- **ShowcaseVariant containers**: Organize different aspects (variants, states, sizes)

When implementing new UI components, follow this systematic approach:

1. **Create the component** in `hearth-ui/src/components/` with all standard variants, sizes, and states
2. **Add to module exports** in `hearth-ui/src/components/mod.rs`
3. **Create showcase file** in `hearth-ui/src/views/design/showcases/` following the pattern:
   - Import ComponentShowcase and ShowcaseVariant from the parent module
   - Wrap content in ComponentShowcase with name, description, and code examples
   - Use ShowcaseVariant containers for different component aspects
   - Follow established patterns from other showcase files
4. **Add to showcase exports** in `hearth-ui/src/views/design/showcases/mod.rs`
5. **Add to main design view** in `hearth-ui/src/views/design/mod.rs`:
   - Import the showcase component
   - Add to the components dropdown list
   - **IMPORTANT**: Update the default selected component to showcase the new component
6. **Cross-platform testing** - build desktop and web to verify functionality
7. **Verification** - ensure the new component displays correctly and all interactions work

This workflow ensures consistent implementation and immediate visibility of new components.

## Component Architecture

### Two-Tier Component System

**UI Components (`hearth-ui/src/components/ui/`):**
Generic reusable UI components (25+ components) including:
- Form controls (button, input, textarea, checkbox, radio, select, slider, switch)
- Layout helpers (card, separator, aspect_ratio, layout)
- Navigation (tabs, modal, sheet, popover)
- Data display (table, badge, avatar, avatar_group)
- Feedback (progress, skeleton, notice, toaster)
- Interactive (accordion, collapsible, carousel, toggle variants)
- Platform-specific (gesture_detector for mobile)

**Feature Components (`hearth-ui/src/components/features/`):**
Feature-specific page sections including:
- `log_viewer.rs` - Development and debug log display
- `universal_search/` - Global search and filtering system with multiple sub-components

### Component Standards

- All components should support standard variants, sizes, and states
- Use consistent prop naming and structure
- Implement responsive behavior where appropriate
- Follow established patterns from existing components
- Support both light and dark themes
- Include proper accessibility attributes

### Showcase Development

When creating showcases:
- Use ComponentShowcase wrapper with clear name and description
- Include code examples showing usage
- Organize variants with ShowcaseVariant containers
- Show different states, sizes, and configurations
- Follow patterns from existing showcase files
- Test showcase displays correctly in design view dropdown