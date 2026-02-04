# AGENTS.md - Coding Guidelines for Ukiji

This document provides guidelines for AI agents working on the Ukiji codebase.

## Project Overview

Ukiji is a cross-platform desktop keystroke visualizer built with:
- **Frontend:** SvelteKit 5 + TypeScript + Tailwind CSS v4
- **Backend:** Tauri v2 (Rust)
- **Package Manager:** pnpm (v10.24.0)
- **Node Version:** 24.11.1 (pinned)

## Build & Development Commands

```bash
# Install dependencies
pnpm install

# Start development server
pnpm dev
# OR for Tauri dev mode
pnpm tauri dev

# Build for production
pnpm build

# Type checking
pnpm check
pnpm check:watch

# Build Tauri app
pnpm tauri build

# Version bump (maintainers only)
pnpm version patch|minor|major
```

## Testing

**Note:** No testing framework is currently configured. When adding tests:
- Prefer Vitest for unit tests
- Use Playwright for E2E tests with Tauri

## Code Style Guidelines

### TypeScript / Svelte

- **Strict TypeScript:** Always enable strict mode (configured in tsconfig.json)
- **Type imports:** Use explicit type imports: `import type { Foo } from 'bar'`
- **Semicolons:** Required
- **Quotes:** Double quotes for strings
- **Indentation:** 4 spaces
- **Trailing commas:** Avoid in single-line, use in multi-line

### Svelte Components

- Use `<script lang="ts">` for TypeScript support
- Use Svelte 5 runes: `$props()`, `$state()`, etc.
- Props destructuring: `let { children } = $props()`
- Event handlers use camelCase: `onMount`, `onDestroy`
- Component props: camelCase

### Imports

```typescript
// Order: external libs → Tauri APIs → internal modules
import { onMount } from "svelte";
import { listen } from "@tauri-apps/api/event";
import { myUtil } from "../lib/utils";

// Type imports
import type { AppHandle } from "@tauri-apps/api/core";
```

### Naming Conventions

- **Components:** PascalCase (e.g., `KeyVisualizer.svelte`)
- **Functions/Variables:** camelCase (e.g., `getKeyLabel`, `displayState`)
- **Constants:** UPPER_SNAKE_CASE for true constants
- **Types/Interfaces:** PascalCase with descriptive names
- **Event handlers:** Prefix with "handle" (e.g., `handleKeyPress`)

### Rust

- **Functions:** snake_case (e.g., `spawn_listener`)
- **Types/Structs:** PascalCase (e.g., `AppHandle`)
- **Constants:** UPPER_SNAKE_CASE
- **Modules:** snake_case file names
- **Error handling:** Use `?` operator and `Result` types
- **Documentation:** Add doc comments for public APIs: `///`

### Tailwind CSS

- Use Tailwind v4 syntax
- Prefer utility classes over custom CSS
- Use arbitrary values sparingly: `w-[100px]`
- Color scheme: Use `neutral` for grays, semantic colors for UI states

### Error Handling

**TypeScript:**
```typescript
try {
    const result = await riskyOperation();
} catch (error) {
    console.error("Descriptive error message:", error);
}
```

**Rust:**
```rust
if let Err(error) = listen(callback) {
    eprintln!("Error in global listener: {:?}", error);
}
```

## Project Structure

```
├── src/                    # SvelteKit frontend
│   ├── routes/            # SvelteKit routes
│   ├── app.css            # Global styles (Tailwind)
│   └── app.html           # HTML template
├── src-tauri/             # Rust backend
│   ├── src/              # Rust source files
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
├── scripts/               # Build scripts
├── static/               # Static assets
└── build/                # Production build output
```

## Version Management

- Version is managed in `package.json`
- Running `pnpm version` syncs all files via `scripts/sync-version.js`
- Updates: `package.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`
- Creates git commit and tag automatically

## Git Conventions

- **Commits:** Clear, descriptive messages
- **Tags:** Use `v` prefix (e.g., `v0.1.1`)
- **Branch naming:** kebab-case (e.g., `feature/key-combo`)

## Important Notes

- **SPA Mode:** SvelteKit is configured as SPA (ssr: false)
- **Tauri Window:** Configured as overlay (transparent, frameless, always on top)
- **Platform:** Currently Windows-focused (global key listener)
- **No secrets:** Never commit `.env` files or API keys
- **Lint/Format:** No ESLint/Prettier configured yet - follow existing code style
