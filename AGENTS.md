# AGENTS.md

## Cursor Cloud specific instructions

This repo is a Tauri v2 desktop app (`mes-editor`) with a Vue 3 + TypeScript frontend (`src/`) and a Rust workspace: the Tauri shell (`src-tauri/`) and the `mes-core` crate (`crates/mes-core`, MeS parser + CLI). Standard commands live in `README.md`, `package.json` scripts, and `Cargo.toml`.

Services / entry points:
- `mes-core` (Rust lib + CLI): `cargo test -p mes-core`, `cargo run -p mes-core -- <parse|vtt|count|chat> <file.mes>`. This holds the core conversion logic.
- Frontend only (`pnpm dev`, Vite on port 1420): renders the UI but the `invoke(...)` calls in `src/App.vue` only resolve inside the Tauri runtime, so conversions will error in a plain browser. Use the full desktop app to exercise conversions.
- Desktop app (`pnpm tauri dev`): compiles the Rust shell and launches the native window; this is the way to test the full product end to end.
- Frontend build + type-check: `pnpm build` (runs `vue-tsc --noEmit` then `vite build`). There is no separate lint script.

Non-obvious caveats:
- The GUI requires an X display; use `DISPLAY=:1` when launching `pnpm tauri dev` in the cloud VM.
- `libEGL warning: DRI3 error ...` on startup is harmless (software rendering fallback); the window still works.
- First `pnpm tauri dev` compiles ~550 crates and takes a few minutes; subsequent runs are cached in `target/`.
- Tauri needs system libraries (webkit2gtk-4.1, gtk-3, libsoup-3, ayatana appindicator, librsvg2, libxdo). These are installed at the OS level (not by the update script) and persist in the VM snapshot.
