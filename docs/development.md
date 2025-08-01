# Development Commands and Workflow

## Validation Workflow

**PRIMARY: Always start with cargo check**
```bash
cargo check -p hearth-ui   # Fast syntax and type checking (~2-3 seconds)
```

**SECONDARY: Full builds only when needed**
```bash
# MUST filter output - raw dx build is unusable due to truncation
dx build --package hearth-web --platform web 2>&1 | grep -v "INFO.*Compiled" | head -50
dx build --package hearth-desktop --platform linux 2>&1 | tail -30
```

## Build Commands (Filtered Required)

**From root directory (RECOMMENDED):**
```bash
# Always check first, then build with filtering
cargo check -p hearth-ui
dx build --package hearth-web --platform web 2>&1 | tail -30

cargo check -p hearth-ui  
dx build --package hearth-desktop --platform linux 2>&1 | tail -30

cargo check -p hearth-ui
dx build --package hearth-mobile --platform android 2>&1 | tail -30
```

**Alternative: From platform directories:**
```bash
cd hearth-web && cargo check -p hearth-ui && dx build 2>&1 | tail -30
cd hearth-desktop && cargo check -p hearth-ui && dx build 2>&1 | tail -30
cd hearth-mobile && cargo check -p hearth-ui && dx build --platform android 2>&1 | tail -30
```

**CRITICAL NOTES:**
- **NEVER run raw `dx build`** - output gets truncated, making errors invisible
- **Always `cargo check -p hearth-ui` first** - catches 80% of errors in 2-3 seconds
- **Always filter `dx build` output** - use `| tail -30` or `| grep -v "INFO.*Compiled" | head-50`
- **Platform names**: `web`, `linux`, `android`
- **Package names**: `hearth-web`, `hearth-desktop`, `hearth-mobile`

## Build Scripts and Automation
```bash
# CSS Management
./tailwind.sh          # Watch for changes and rebuild CSS automatically
./tailwind.sh --build  # Build CSS once for all subcrates

# Mobile Development
./build_android_with_icons.sh  # Android build with proper icon setup
./generate_icons.sh            # Generate icons for different platforms
./watch_android.sh             # Monitor Android build processes
```

## Serving Applications (User Only)
```bash
cd hearth-web     # or hearth-desktop, hearth-mobile
dx serve   # Start development server with hot reload (user testing only)
```

## Code Quality

**Recommended Workflow for Validation:**

1. **First: Quick validation with `cargo check`**
   ```bash
   cargo check -p hearth-ui   # Fast syntax and type checking (~2-3 seconds)
   ```

2. **Only if errors: Full build for complete verification**
   ```bash
   # Problem: dx build produces massive output (590+ packages) that gets truncated
   # Solution: Filter to show only errors/warnings
   dx build --package hearth-web --platform web 2>&1 | grep -v "INFO.*Compiled" | head -50
   dx build --package hearth-desktop --platform linux 2>&1 | grep -v "INFO.*Compiled" | head -50
   
   # Alternative: Show just the end where errors would appear
   dx build --package hearth-web --platform web 2>&1 | tail -30
   ```

3. **Formatting (anytime)**
   ```bash
   dx fmt     # Format RSX code
   ```

**Why This Workflow:**
- **`dx check` is ineffective** in our library-focused workspace (`hearth-ui/` is a library crate, not an app)
- **`cargo check` provides fast feedback** for basic syntax and type errors
- **`dx build` output gets truncated** in Claude's environment, making it hard to see specific errors
- **`cargo check` shows clear, concise error messages** that fit in the output window
- **Full builds are only needed** when `cargo check` passes but you need complete verification

**Output Management:**
- `cargo check` produces concise, readable error messages
- `dx build` produces verbose output (590+ package compilation logs) that gets truncated
- **Filtering is essential** for `dx build` to see actual errors/warnings
- Use `grep -v "INFO.*Compiled"` to filter out compilation noise  
- Use `tail -30` to see the end where errors typically appear
- Prioritize `cargo check` for debugging, filtered `dx build` for complete verification

**IMPORTANT**: `cargo check` won't catch all Dioxus/RSX-specific issues, but it catches 80% of common errors quickly. Only move to `dx build` when `cargo check` passes or when you need complete platform-specific validation.

## Development Best Practices

### General Guidelines
- **Always put yourself into the repository root after working in a directory**
  - This ensures consistent starting point for subsequent operations  
  - Prevents potential path-related issues in future commands
- **Focus Development on hearth-ui/**: All application features, views, components, and logic should be implemented in the `hearth-ui/` crate since it's the unified UI library
- **Minimal Platform Changes**: Platform-specific directories (`hearth-web/`, `hearth-desktop/`, `hearth-mobile/`) should remain minimal wrappers unless platform-specific functionality is required
- **Test Cross-Platform**: When making changes to `hearth-ui/`, verify builds work across all platforms since they all depend on it
- **NEVER ASSUME FIXES WORK**: Never assume something was fixed without testing or direct feedback from the user. Always verify functionality through testing, build verification, or explicit user confirmation before considering an issue resolved.
- **Look for Examples When Stuck**: If you find yourself repeatedly making errors or encountering compilation issues, stop guessing and look for inspiration from existing code in the project:
  - Search for similar patterns or components already implemented
  - Check how existing showcases are structured (in `hearth-ui/src/views/design/showcases/`)
  - Look at how similar APIs or props are used elsewhere in the codebase
  - Use existing working code as a template rather than guessing at implementation details
  - This approach saves time and ensures consistency with project patterns

### Important Constraints
- **Never run commands that hang or block** - AI agents cannot interact with blocking processes. This includes:
  - `dx serve` - Use filtered `dx build` instead and let the user test
  - Commands with `&` backgrounding - These hang Claude's session
  - Interactive commands requiring user input
  - Long-running watchers or servers (except when explicitly requested for setup)
- Never manually generate Tailwind CSS - use `./tailwind.sh` which handles all subcrates
- **Validate Frequently**: After making significant changes, run `cargo check -p hearth-ui` first, then filtered `dx build` only if needed
- **Skip Full Builds for Trivial Changes**: Don't run `dx build` for simple, non-breaking changes such as:
  - Adding new items to vectors (e.g., adding routes to navigation arrays)
  - Changing string literals or constants
  - Adding comments or documentation
  - Simple configuration changes that don't affect compilation
  - Use `cargo check -p hearth-ui` for quick verification instead
- **WASM Build Issues**: If encountering wasm-bindgen errors or similar build failures (e.g., "Failed to read file: web.js; No such file or directory"), this is usually because the user has `dx serve` running in the background which is watching for changes. **DO NOT DELETE BUILD ARTIFACTS (`rm -rf target/dx`) TO "FIX" THIS** - you're just racing against a background build process! Simply retry the build command and it will typically resolve the issue.
- **Nix Shell Available**: Use `nix-shell -p <package>` for one-off tools not installed on the system (e.g., `nix-shell -p imagemagick --run "magick ..."`).
- **Avoid Multi-Command Bash Tools**: Don't use multi bash tools `cd .. && cd some_dir && dx build whatever` because they may need manual approval from the user
- **Primary Development in hearth-ui/**: Since all application logic is in `hearth-ui/`, most development work happens there. Platform-specific directories are minimal wrappers.