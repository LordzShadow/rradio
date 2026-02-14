# RRadio - Copilot Agent Instructions

## Project Overview

**RRadio** is a lightweight internet radio streaming desktop application built with:
- **Frontend**: SvelteKit 2 + Svelte 5 + TypeScript + Tailwind CSS 4 + Vite 6
- **Backend**: Tauri 2 (Rust 2021 edition) for cross-platform desktop functionality
- **Audio**: Rodio (Rust audio library) with ICY metadata support for radio streams

The app allows users to browse radio stations, stream audio (AAC/MP3), and view track metadata with system tray integration.

## Project Structure

```
rradio/
├── src/                          # Frontend (SvelteKit)
│   ├── routes/                   # SvelteKit routes
│   │   ├── +page.svelte          # Main page (radio UI)
│   │   ├── +layout.svelte        # Layout component
│   │   ├── +layout.ts            # Universal layout logic
│   │   └── layout.css            # Styles
│   └── app.html                  # HTML shell
├── src-tauri/                    # Tauri desktop backend (Rust)
│   ├── src/
│   │   ├── main.rs               # Entry point
│   │   ├── lib.rs                # Commands & app state
│   │   ├── player.rs             # Audio player logic
│   │   ├── radios.rs             # Radio stations data
│   │   └── tray.rs               # System tray integration
│   ├── Cargo.toml                # Rust dependencies
│   ├── tauri.conf.json           # Tauri configuration
│   └── icons/                    # App icons (PNG, ICO, ICNS)
├── static/                       # Static assets
├── build/                        # Built frontend (generated, gitignored)
└── Configuration Files (see below)
```

## Key Configuration Files

| File | Purpose |
|------|---------|
| `package.json` | Node.js dependencies and scripts |
| `pnpm-lock.yaml` | pnpm lockfile (DO NOT modify manually) |
| `pnpm-workspace.yaml` | Monorepo config; specifies built dependencies (esbuild, @tailwindcss/oxide) |
| `vite.config.js` | Vite config; dev server on port 1420, ignores src-tauri |
| `svelte.config.js` | Uses `@sveltejs/adapter-static` for SSG (no Node.js server in Tauri) |
| `tsconfig.json` | TypeScript config; strict mode enabled |
| `src-tauri/Cargo.toml` | Rust dependencies and crate configuration |
| `src-tauri/tauri.conf.json` | Tauri app config; window size (900x700), build commands, icons |
| `.gitignore` | Excludes: node_modules, /build, /.svelte-kit, /package, .env files |

## Package Manager

**ALWAYS use `pnpm`** for JavaScript/Node.js dependencies:
- ✅ `pnpm install` - Install dependencies
- ✅ `pnpm add <package>` - Add dependency
- ✅ `pnpm dev` - Start Vite dev server
- ❌ **NEVER** use npm or yarn

If pnpm is not installed, run: `npm install -g pnpm`

## Development Workflow

### Initial Setup

1. **Install pnpm** (if not available):
   ```bash
   npm install -g pnpm
   ```

2. **Install Node.js dependencies**:
   ```bash
   pnpm install
   ```

3. **Install Rust toolchain** (if not available):
   Follow instructions at https://rustup.rs/

4. **Install system dependencies** (Linux/Ubuntu):
   ```bash
   sudo apt-get update
   sudo apt-get install -y libwebkit2gtk-4.1-dev \
     libgtk-3-dev \
     libayatana-appindicator3-dev \
     librsvg2-dev \
     patchelf \
     libasound2-dev \
     pkg-config \
     build-essential \
     libssl-dev
   ```
   
   **Note**: These GTK/GLib dependencies are REQUIRED for Tauri on Linux. Without them, `cargo check` or `cargo build` will fail with errors like:
   ```
   The system library `glib-2.0` required by crate `glib-sys` was not found.
   The file `glib-2.0.pc` needs to be installed and the PKG_CONFIG_PATH environment variable must contain its parent directory.
   ```

### Development Commands

```bash
# Frontend development (SvelteKit + Vite)
pnpm dev              # Start Vite dev server on http://localhost:1420

# Type checking
pnpm check            # Type-check with svelte-kit
pnpm check:watch      # Watch mode type-checking

# Full Tauri development (frontend + backend)
pnpm tauri dev        # Runs 'pnpm dev' + Tauri backend, opens desktop app window

# Production build
pnpm build            # Build SvelteKit frontend → /build directory
pnpm tauri build      # Build full desktop app executable

# Rust backend only (for testing)
cd src-tauri
cargo check           # Type-check Rust code (fast)
cargo build           # Build Rust code
cargo fmt             # Format Rust code (rustfmt available)
cargo clippy          # Lint Rust code (if clippy installed)
```

### Build Process Flow

1. **Development**: `pnpm tauri dev`
   - Runs `pnpm dev` (beforeDevCommand in tauri.conf.json)
   - Starts Vite dev server at http://localhost:1420
   - Tauri serves frontend from dev server with hot-reload
   - Backend runs with `cargo run` in watch mode

2. **Production**: `pnpm tauri build`
   - Runs `pnpm build` (beforeBuildCommand in tauri.conf.json)
   - SvelteKit builds static files to `/build` directory
   - Cargo compiles Rust backend with bundled frontend
   - Creates platform-specific executable (Windows .exe, macOS .app, Linux AppImage/deb)

## Architecture & Code Organization

### Frontend (SvelteKit)

- **Framework**: Svelte 5 with runes (`$state`, `$props`, `$derived`)
- **Routing**: File-based routing in `src/routes/`
- **Styling**: Tailwind CSS 4 (utility-first, configured in Vite)
- **Tauri API**: Use `@tauri-apps/api` to invoke Rust commands
  ```typescript
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  
  // Call Rust command
  const result = await invoke("play", { uuid: "station-id" });
  
  // Listen to events from Rust
  listen("title", (event) => {
    console.log(event.payload);
  });
  ```

### Backend (Rust/Tauri)

- **Entry point**: `src-tauri/src/main.rs` (calls `radio_app_lib::run()`)
- **Library**: `src-tauri/src/lib.rs` (defines Tauri commands and app setup)
- **Commands** (callable from frontend):
  - `play(uuid)` - Start playing a radio station
  - `pause()` - Pause playback
  - `stations()` - Get list of available stations
- **Modules**:
  - `player.rs` - Audio streaming with rodio, ICY metadata extraction
  - `radios.rs` - Station data structure and hardcoded station list
  - `tray.rs` - System tray icon and menu

### Data Flow

```
Frontend (Svelte)
    ↓ invoke("play", {uuid})
Tauri Command Handler (lib.rs)
    ↓ calls Player::play()
Player (player.rs)
    ↓ streams audio via stream-download + rodio
    ↓ parses ICY metadata
    ↓ emits "title" event
Frontend (listens to event)
    ↓ updates UI with track info
```

## Code Style & Conventions

### TypeScript/Svelte
- **Strict mode enabled** in tsconfig.json
- Use **Svelte 5 runes**: `$state`, `$props`, `$derived` (NOT stores)
- Use **TypeScript** for all `.ts` and `<script lang="ts">` blocks
- **Tailwind CSS**: Use utility classes, avoid custom CSS when possible
- **Formatting**: No explicit formatter configured; follow existing style
  - Indentation: 4 spaces (see existing .svelte files)
  - Use double quotes for strings

### Rust
- **Edition**: 2021
- **Formatting**: Use `cargo fmt` (rustfmt available)
- **Conventions**:
  - Use `Result<T, E>` for fallible operations
  - Prefer `async/await` for I/O operations
  - Use `Arc<Mutex<T>>` for shared state across threads
  - Keep modules small and focused (player, radios, tray)

## Testing

**No test infrastructure currently exists in this project.**

If adding tests:
- Frontend: Consider using Vitest (already compatible with Vite)
- Rust: Use standard `#[cfg(test)]` modules and `cargo test`

## Common Issues & Workarounds

### Issue 1: Missing System Dependencies (Linux)

**Error**:
```
The system library `glib-2.0` required by crate `glib-sys` was not found.
```

**Solution**: Install GTK/GLib development libraries:
```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev patchelf \
  libasound2-dev pkg-config build-essential libssl-dev
```

### Issue 2: pnpm Not Found

**Error**: `bash: pnpm: command not found`

**Solution**: Install pnpm globally:
```bash
npm install -g pnpm
```

### Issue 3: Port 1420 Already in Use

**Error**: Vite fails to start because port is in use

**Solution**: Kill process on port 1420:
```bash
# Linux/macOS
lsof -ti:1420 | xargs kill -9

# Or change port in vite.config.js (requires also updating tauri.conf.json)
```

### Issue 4: TypeScript Warning about .svelte-kit/tsconfig.json

**Warning during build**:
```
Cannot find base config file "./.svelte-kit/tsconfig.json"
```

**Workaround**: This is a harmless warning. The file is generated during the build process. Run `pnpm dev` or `pnpm check` first to generate it, or ignore the warning.

## Dependencies & Security

### Checking for Vulnerabilities

**JavaScript/Node.js**:
```bash
pnpm audit              # Check for known vulnerabilities
pnpm audit --fix        # Attempt to fix vulnerabilities
```

**Rust**:
```bash
cargo audit             # Requires cargo-audit: cargo install cargo-audit
```

### Adding New Dependencies

**Frontend (pnpm)**:
```bash
pnpm add <package>           # Add runtime dependency
pnpm add -D <package>        # Add dev dependency
```

**Backend (Cargo)**:
Edit `src-tauri/Cargo.toml` and run:
```bash
cd src-tauri
cargo update
```

**IMPORTANT**: For both ecosystems, ensure new dependencies:
- Are actively maintained
- Have acceptable licenses (project is MIT)
- Don't have known security vulnerabilities

## Git Workflow

### Files to NEVER Commit
Already in `.gitignore`:
- `node_modules/`
- `/build/` (generated frontend)
- `/.svelte-kit/` (SvelteKit build cache)
- `/package/` (generated packages)
- `.env` files (secrets)
- `vite.config.*.timestamp-*` (Vite cache)

### Rust Build Artifacts
Add to `.gitignore` if not already present:
- `src-tauri/target/` (Cargo build output)

## Environment Variables

- **TAURI_DEV_HOST**: Optional; sets Vite dev server host for mobile development
- **PKG_CONFIG_PATH**: May be needed if system libraries are in non-standard locations

## Editor Recommendations

Recommended VSCode extensions (see `.vscode/extensions.json`):
- `svelte.svelte-vscode` - Svelte language support
- `tauri-apps.tauri-vscode` - Tauri development tools
- `rust-lang.rust-analyzer` - Rust language server

VSCode settings (see `.vscode/settings.json`):
- TypeScript plugin enabled for Svelte files
- `.css` files treated as Tailwind CSS

## Debugging

### Frontend Debugging
- Open DevTools in the Tauri window (Ctrl+Shift+I / Cmd+Option+I)
- Console logs and network requests visible in DevTools
- Vite HMR provides instant feedback during development

### Backend Debugging
- Use `println!()` or `eprintln!()` for logging (visible in terminal)
- Tauri errors appear in both terminal and DevTools console
- For deep debugging: Use `rust-lldb` or `rust-gdb`

## Performance Considerations

- **Audio Buffering**: Player buffers 5 seconds of audio (see `get_prefetch_bytes` in player.rs)
- **Memory Storage**: Uses in-memory storage for audio streaming (see `MemoryStorageProvider`)
- **Frontend Bundle**: Vite bundles are optimized for production builds
- **Tauri Binary Size**: Typically 5-15MB depending on platform and features

## Additional Resources

- **Tauri Docs**: https://v2.tauri.app/
- **SvelteKit Docs**: https://kit.svelte.dev/
- **Svelte 5 Docs**: https://svelte.dev/docs/svelte/overview
- **Vite Docs**: https://vitejs.dev/
- **Tailwind CSS**: https://tailwindcss.com/
- **Rodio (Rust Audio)**: https://docs.rs/rodio/

## Making Changes

### General Guidelines
1. **Frontend changes**: Edit files in `src/`, test with `pnpm dev`
2. **Backend changes**: Edit files in `src-tauri/src/`, test with `pnpm tauri dev`
3. **Styling**: Use Tailwind utilities; add custom CSS only if necessary
4. **Type safety**: Ensure TypeScript strict mode compliance
5. **Rust safety**: Ensure all `cargo check` and `cargo clippy` warnings are addressed

### Adding a New Radio Station
1. Edit `src-tauri/src/radios.rs`
2. Add new `Station::new(url, name, uuid)` to `get_stations()` vector
3. No frontend changes needed (stations fetched dynamically)

### Changing Window Size/Title
1. Edit `src-tauri/tauri.conf.json`
2. Modify `app.windows[0]` properties (width, height, title, etc.)

### Adding a New Tauri Command
1. Define function in `src-tauri/src/lib.rs` with `#[tauri::command]`
2. Add to `invoke_handler!` macro in `run()` function
3. Call from frontend using `invoke("command_name", { args })`

## Summary

RRadio is a **Tauri 2 + SvelteKit** desktop app for streaming internet radio. Development requires:
- **pnpm** for frontend dependencies
- **Rust toolchain** for backend
- **System libraries** (GTK, GLib, WebKit) on Linux
- **Port 1420** for Vite dev server

Key commands:
- `pnpm install` → Install dependencies
- `pnpm tauri dev` → Run development build
- `pnpm tauri build` → Create production executable
- `cargo fmt` → Format Rust code
- `pnpm check` → Type-check TypeScript/Svelte

**Always use pnpm (NOT npm or yarn)** and be aware of system dependency requirements on Linux.
