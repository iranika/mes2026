# AGENTS.md

## Cursor Cloud specific instructions

This repo is a Tauri v2 desktop app (`mes-editor`) with a Vue 3 + TypeScript frontend (`src/`) and a Rust workspace: the Tauri shell (`src-tauri/`) and the `mes-core` crate (`crates/mes-core`, MeS parser + CLI). Standard commands live in `README.md`, `package.json` scripts, and `Cargo.toml`.

Services / entry points:
- `mes-core` (Rust lib + CLI): `cargo test -p mes-core`, `cargo run -p mes-core -- <parse|vtt|count|chat> <file.mes>`. This holds the core conversion logic. Optional `-c/--config` (default `./mes.json`) merges partial JSON over defaults for all subcommands.
- Frontend only (`pnpm dev`, Vite on port 1420): conversions run in-browser via the bundled WASM build under `src/wasm/mes-core/`. No Tauri runtime required for preview. File open/save/export fall back to the browser file picker / download APIs.
- Desktop app (`pnpm tauri dev`): compiles the Rust shell and launches the native window; uses Tauri commands for conversion and native dialogs for file I/O. Prefer this for end-to-end desktop testing.
- Frontend build + type-check: `pnpm build` (runs `vue-tsc --noEmit` then `vite build`). There is no separate lint script.
- Rebuild WASM after `mes-core` API changes: `pnpm run build:wasm` (needs `wasm-pack` and `wasm32-unknown-unknown`).

Non-obvious caveats:
- The GUI requires an X display; use `DISPLAY=:1` when launching `pnpm tauri dev` in the cloud VM.
- `libEGL warning: DRI3 error ...` on startup is harmless (software rendering fallback); the window still works.
- First `pnpm tauri dev` compiles ~550 crates and takes a few minutes; subsequent runs are cached in `target/`.
- Tauri needs system libraries (webkit2gtk-4.1, gtk-3, libsoup-3, ayatana appindicator, librsvg2, libxdo). These are installed at the OS level (not by the update script) and persist in the VM snapshot.
- `AGENTS.md` previously claimed Vite-only preview could not convert MeS; that is outdated — WASM fallback handles conversions in the browser.
