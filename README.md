# Ukiji - keystrokes Visualizer

**Ukiji** is a beautiful, cross-platform desktop application that visualizes your global keystrokes in real-time. Designed for screen casters, tutorial creators, and streamers, it runs invisibly in the background and overlays stylish key bubbles on your desktop without obstructing your mouse clicks.

Built with **Tauri v2**, **Svelte**, **Rust**.

---

## Tech Stack

- **Core:** Tauri v2 (Rust backend + Web frontend)
- **UI:** Svelte (TypeScript), Tailwind CSS
- **Input Handling:** Rust backend for global key listening (even when the app is unfocused)
- **State Management:** Svelte Stores + Tauri Store Plugin (Persistence)

---

## Development Roadmap

### Phase 1: Foundation & Configuration

- [x] **Initialize Project:** Initialize a new Tauri app with Svelte + TypeScript.
- [x] **Configure Overlay Window:** Define the main visualizer window as:
  - [x] Transparent
  - [x] Frameless (No title bar)
  - [x] Always On Top
  - [x] Fixed position (Bottom-Center)
  - [x] Shadowless
  - [x] Click-through behavior
  - [x] Not in taskbar
- [x] **Version Bump Script:** Create a script to automate version bumping and tagging.
- [x] **Setup Tag Protection:** Configure tag protection rules for authorized maintainers.
- [x] **Release Workflow:** GitHub Actions workflow for automated Windows builds and releases.
- [x] **Labeler Workflow:** GitHub Actions workflow for automated label assignment.

### Phase 2: The Backend (Rust)

- [ ] **Global Listener Thread:** Spawn a separate thread in Rust that listens for key events system-wide (even when the app is unfocused).
- [ ] **Event Bridge:** Implement a callback that filters raw inputs and emits a simplified `keypress` event to the frontend via Tauri's Event system.

### Phase 3: The Visualizer (Frontend)

- [ ] **Key Store:** Create a Svelte writable store to manage the queue of active keys.
- [ ] **Auto-Expiration:** Remove keys from store after a set duration.
- [ ] **Visual Component:** Key UI components.
- [ ] **Animations:** Svelte transitions to animate keys entering and leaving the screen.

### Phase 4: Window Behavior & Polish

- [ ] **Click-Through:** Main window with click-through behavior.
- [ ] **Cross-Platform Check:** Ensure features work smoothly on Windows, macOS, and Linux.

### Phase 5: User Control (Settings & Tray)

- [ ] **System Tray:** Implement a system tray icon (Menu Bar / Taskbar) with context menu options:
  - Open Settings
  - Pause Visualizer
  - Quit App
- [ ] **Settings Window:** Create a second settings window.
- [ ] **Settings UI:** Settings window should contain:
  - Color pickers for text/background.
  - Sliders for key duration and size.
  - Position toggles (Bottom-Left, Center, Right).
  - Typography and key styles.
  - ...
- [ ] **State Persistence:** Save user preferences to a JSON file so settings are remembered on restart.
- [ ] **Live Sync:** Ensure changes in the Settings Window instantly update the Overlay Window via event listeners.

---

## How to Run

1. **Install Dependencies**

```bash
pnpm install
```

2. Start Development Mode

```bash
pnpm run tauri dev
```

3. Build for Production

```bash
pnpm run tauri build
```

---

## Version Management & Releases (Maintainers Only)

```
Note: Only authorized maintainers can create releases. Tag protection rules prevent unauthorized contributors from pushing version tags.
```

### Bump Version

Use standard npm version commands to bump the version across all files (`package.json`, `tauri.conf.json`, `Cargo.toml`, `Cargo.lock`):

```bash
# Bump patch version (0.1.0 → 0.1.1)
pnpm version patch

# Bump minor version (0.1.0 → 0.2.0)
pnpm version minor

# Bump major version (0.1.0 → 1.0.0)
pnpm version major

# Or specify exact version
pnpm version 1.5.2
```

This will:

- Update all version files
- Create a git commit with the version changes
- Create a git tag (e.g., `v0.1.1`)

### Create a Release

Push the tag to trigger the automated release workflow:

```bash
git push && git push --tags
```

**Note:** If you're not an authorized maintainer, the `git push --tags` command will be rejected by GitHub's tag protection rules.

Once the tag is pushed to GitHub, a GitHub Action will automatically:

- Build the Windows distributable (.exe, .msi)
- Create a draft release on GitHub
- Attach the installers to the release

Review the draft release on GitHub and publish it when ready.
