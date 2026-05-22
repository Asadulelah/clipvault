# ClipVault

> **A modern, native clipboard manager for Windows.** Win+V on steroids — unlimited history, search, pinning, auto-paste, dark mode. No cloud, no accounts, no limits.

---

## Why ClipVault?

The default Windows clipboard history is limited to 25 items, doesn't persist across restarts, and loses images quickly. ClipVault fixes all of that:

| Windows Clipboard | ClipVault |
|-------------------|-----------|
| 25-item limit | Unlimited history (configurable) |
| Cleared on restart | Persistent across reboots |
| No search | Full-text search with instant filtering |
| No pinning | Pin important items so they never get deleted |
| Basic text only | Text, images, URLs, and HTML with smart detection |
| No source tracking | Shows which app you copied from |
| No auto-paste | Click an item and it pastes directly into your active app |

---

## Install

Grab the latest installer from [Releases](https://github.com/Asadulelah/clipvault/releases):

- **`ClipVault_x.y.z_x64-setup.exe`** — NSIS installer, ~2 MB
- **`ClipVault_x.y.z_x64_en-US.msi`** — MSI installer, ~2.7 MB

Run it. ClipVault launches automatically and lives in your system tray. Press **Win+V** to open.

No configuration needed. No accounts. No internet connection required. Everything stays on your machine.

---

## Key Features

### Instant access with Win+V
ClipVault registers a global hotkey and opens a sleek, frameless panel in the bottom-right corner of your screen — exactly where the Windows clipboard appears. It feels native because it is.

### Smart content detection
Every clipboard entry is automatically categorized:
- **Text** — plain text with a preview
- **URL** — detected via pattern matching, displayed with a link icon
- **Image** — screenshots and copied images are stored with auto-generated thumbnails
- **HTML** — rich text content from browsers and editors

### Deduplication
Copy the same thing twice? ClipVault won't create a duplicate. It uses SHA-256 hashing to detect identical content and bumps the timestamp instead.

### Pin important items
Pin frequently-used snippets — email signatures, code blocks, addresses — and they stay locked at the top of your history, even when you clear everything else.

### Full-text search
Start typing in the search bar and your entire clipboard history is filtered instantly. Find that link you copied three days ago in seconds.

### Auto-paste
Click any item in ClipVault and it doesn't just copy — it **pastes directly** into whatever app has focus. The panel hides, the content goes to your clipboard, and a simulated Ctrl+V keystroke delivers it where your cursor is. All under 200ms.

### Source app tracking
Every clipboard entry records which application it came from. See at a glance whether that snippet was from VS Code, Chrome, or Slack.

### Dark and light themes
Full theme support with a Windows 11-inspired design. Dark, Light, or System (follows your Windows theme automatically).

### Starts with Windows
ClipVault registers itself to launch on startup. It sits quietly in your system tray, always ready. Right-click the tray icon for quick access to settings or to quit.

---

## Technical Architecture

ClipVault is built with a modern, performance-focused stack:

- **[Tauri v2](https://tauri.app/)** — Rust-based framework for building desktop apps. Unlike Electron, Tauri produces tiny binaries (~2 MB installer) and uses the system's built-in WebView2 instead of bundling an entire browser.
- **React 18 + TypeScript** — Responsive component-based UI with full type safety.
- **Rust** — All system-level operations (clipboard access, hotkeys, tray icon, database, window management, paste simulation) run in Rust for maximum performance and reliability.
- **SQLite** — Clipboard history stored locally at `%APPDATA%\ClipVault\`. Fast, reliable, zero-config.
- **Tailwind CSS v4** — Utility-first CSS for a clean, consistent UI without bloat.
- **Zustand** — Lightweight state management for the React frontend.

### How auto-paste works

When you click an item:

1. The ClipVault panel hides itself (so the target app regains focus)
2. A 50ms delay lets the OS transfer focus
3. The selected content is written to the system clipboard
4. A background thread waits 100ms, then uses the Windows `SendInput` API to simulate a physical Ctrl+V keypress
5. The target app receives the paste as if you pressed the keys yourself

All under 200ms — it feels instant.

### How clipboard watching works

A background Rust thread polls the system clipboard every 500ms using the `arboard` crate. On each change:

1. Content type is detected (text, image, URL, or HTML)
2. A SHA-256 hash is computed for deduplication
3. The source application is identified via the Windows `GetForegroundWindow` API
4. If the hash is new, a database row is inserted; if it exists, the timestamp is updated
5. A `clipboard-updated` event is emitted to the React frontend
6. The UI updates in real-time without polling

A skip flag prevents ClipVault from re-capturing its own paste operations.

---

## The Numbers

| Metric | Value |
|--------|-------|
| Installer size | 2.0 MB (NSIS) / 2.7 MB (MSI) |
| Binary size | ~5.3 MB |
| Memory usage | ~15–25 MB |
| Clipboard poll interval | 500 ms |
| Auto-paste latency | < 200 ms |
| Supported content types | 4 (text, image, URL, HTML) |
| Frontend bundle | < 500 KB |
| Database | SQLite, local, zero-config |

---

## Develop

Requires **Node.js 18+**, **pnpm**, and the **Rust toolchain** (with the Tauri prerequisites for your OS — see [tauri.app/start/prerequisites](https://tauri.app/start/prerequisites)).

```bash
git clone https://github.com/Asadulelah/clipvault.git
cd clipvault
pnpm install
pnpm tauri dev
```

To build a release installer:

```bash
pnpm tauri build
```

Installers land in `src-tauri/target/release/bundle/`.

---

## Settings

Access settings from the gear icon in the panel footer or from the system tray:

- **Theme** — Light / Dark / System
- **Launch on startup** — Enabled by default
- **Show source app** — Toggle source app labels on clipboard items
- **History limit** — Set a maximum number of items to keep (0 = unlimited)
- **Hotkey** — Currently Win+V (displayed in settings)

---

## Roadmap

Things on the table for v2:

- Customizable hotkeys
- Snippet templates with placeholders
- Category/tag organization
- Export/import clipboard history
- Rich text preview for HTML content
- Favorites and collections
- macOS and Linux builds

---

## License

[MIT](LICENSE) — use it, fork it, ship your own version.

---

## Built by

[**Asadulelah**](https://github.com/Asadulelah) — framework designer + AI-leveraged builder. This entire app — from the Rust backend to the React frontend, from the SQLite schema to the Windows installer — was designed and built in a single intense development session using AI agentic tools as the pair programmer. The frameworks behind that workflow are documented across [my other repos](https://github.com/Asadulelah).

---

*ClipVault — built for Windows 10/11 (x64).*
