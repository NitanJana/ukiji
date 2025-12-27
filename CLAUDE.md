# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Ukiji is a cross-platform desktop application built with **Tauri v2**, **Svelte 5**, and **Rust** that visualizes global keystrokes in real-time. It runs as an always-on-top, click-through transparent overlay window designed for screencasters and tutorial creators.

## Development Commands

### Setup
```bash
pnpm install
```

### Development
```bash
pnpm run tauri dev
```
This starts the Vite dev server at `http://localhost:1420` and launches the Tauri development window.

### Build
```bash
pnpm run tauri build
```
Creates production builds for the current platform (Windows .exe/.msi, macOS .app/.dmg, Linux .deb/.AppImage).

### Type Checking
```bash
pnpm check           # Run once
pnpm check:watch     # Watch mode
```

### Version Management (Maintainers Only)
```bash
pnpm version patch|minor|major    # Bump version
git push && git push --tags       # Trigger release workflow
```

The `postversion` script (`scripts/sync-version.js`) automatically syncs versions across:
- `package.json`
- `src-tauri/tauri.conf.json`
- `src-tauri/Cargo.toml`
- `src-tauri/Cargo.lock` (via `cargo check`)

## Architecture

### Tauri Backend (Rust)

**Entry Point**: `src-tauri/src/lib.rs`
- `lib.rs` contains the main `run()` function that initializes the Tauri app
- `main.rs` is just a thin wrapper that calls `ukiji_lib::run()`

**Module Structure**:
- `input.rs`: Global keyboard listener using the `rdev` crate
  - Spawns a background thread that listens for system-wide key events
  - Converts raw `rdev::Key` events to readable labels (e.g., "Enter", "Shift", "Ctrl")
  - Emits `global-keypress` events to the frontend with the key label as payload
  - Currently handles individual keys; modifier combinations are not yet implemented

- `window.rs`: Window configuration and positioning
  - Sets up the main window as click-through (`set_ignore_cursor_events(true)`)
  - Positions the window at bottom-center of the primary monitor
  - Handles DPI scaling using `scale_factor`

**Window Configuration** (`src-tauri/tauri.conf.json`):
- Transparent, frameless, always-on-top overlay
- Not shown in taskbar (`skipTaskbar: true`)
- Shadow disabled, not resizable
- Initial size: 400x100

### Frontend (Svelte 5 + SvelteKit)

**Framework Setup**:
- Uses SvelteKit in SPA mode (`export const ssr = false` in `src/routes/+layout.ts`)
- Configured with `@sveltejs/adapter-static` with fallback to `index.html`
- No server-side rendering since Tauri doesn't have a Node.js server

**Current Implementation** (`src/routes/+page.svelte`):
- Listens for `global-keypress` events from the Rust backend
- Displays the most recent key in a semi-transparent overlay
- Uses Tailwind CSS for styling

**Event Flow**:
1. User presses a key (anywhere on the system)
2. Rust `input.rs` background thread captures it via `rdev::listen`
3. Event is converted to a readable label
4. Emitted as `global-keypress` event via Tauri's event system
5. Svelte frontend receives event via `@tauri-apps/api/event`
6. UI updates to display the key

### Known Limitations & TODOs

Based on the roadmap in README.md, the following are **not yet implemented**:
- **Combination Detection**: Modifier + key shortcuts (e.g., Ctrl+C) are not tracked as combinations
- **Key Store & Queue**: Currently only shows one key at a time; no queue or auto-expiration
- **Settings Window**: No UI for customization (colors, duration, position, etc.)
- **System Tray**: No tray icon or pause/quit controls
- **State Persistence**: User preferences are not saved

## Key Technical Details

### Cross-Platform Considerations
- `rdev` crate handles OS-specific key listening (Windows, macOS, Linux)
- The library name is `ukiji_lib` (not just `ukiji`) to avoid name conflicts on Windows (see `Cargo.toml` comments)

### Window Behavior
- The window is initially hidden and shown only after positioning is calculated
- Click-through is enabled immediately to prevent obstructing mouse interactions
- Position is recalculated based on monitor size and DPI scaling

### Event Communication
- Uses Tauri's built-in event system (not commands/invokes)
- One-way communication: Rust → Frontend
- Event name: `"global-keypress"`, payload: `String`

## Development Patterns

### Adding New Key Mappings
Edit `get_key_label()` in `src-tauri/src/input.rs` to add custom labels for specific keys.

### Modifying Window Appearance
- For Tauri window properties: Edit `src-tauri/tauri.conf.json` → `app.windows[0]`
- For runtime behavior: Edit `src-tauri/src/window.rs`
- For visual styling: Edit Tailwind classes in `src/routes/+page.svelte`

### Testing Releases
The GitHub Actions release workflow (`.github/workflows/release.yml`) is triggered by pushing version tags. Only authorized maintainers can push tags due to tag protection rules.
