# MeS Editor (mes2026)

Tauri + Vue 3 + TypeScript の MeS 言語エディタです。`mes-core` で MeS をパースし、Medo JSON / VTT / ワードカウント / チャット形式へ変換できます。

ブラウザのみ（Vite）でも、同梱の WASM ビルド経由でプレビューできます。Tauri 起動時はネイティブコマンドを優先します。

## 構成

| パス | 内容 |
|------|------|
| `crates/mes-core` | MeS パーサ・CLI・言語仕様（`wasm` feature でブラウザ向けバインディング） |
| `src/` | Vue フロントエンド（プレビュー UI） |
| `src/wasm/mes-core/` | `wasm-pack` 生成物（Vite 用） |
| `src-tauri/` | Tauri シェル（Rust コマンド経由で mes-core を呼び出し） |

## セットアップ

```bash
pnpm install
```

### フロントのみ（Vite + WASM）

```bash
pnpm dev
```

`pnpm dev` はブラウザ内 WASM で変換します（Tauri 不要）。WASM を再生成する場合:

```bash
pnpm run build:wasm
```

### デスクトップアプリ（Tauri）

```bash
pnpm tauri dev
```

### mes-core CLI

```bash
cargo run -p mes-core -- parse path/to/script.mes
cargo run -p mes-core -- vtt path/to/script.mes
cargo run -p mes-core -- count path/to/script.mes
cargo run -p mes-core -- chat path/to/script.mes
```

### テスト

```bash
cargo test -p mes-core
```

サンプル MeS: [`crates/mes-core/tests/fixtures/sample.mes`](crates/mes-core/tests/fixtures/sample.mes)

## MeS 言語

- 仕様書: [crates/mes-core/MES_LANGUAGE.md](crates/mes-core/MES_LANGUAGE.md)
- JSON スキーマ: [crates/mes-core/MES_SCHEMA.json](crates/mes-core/MES_SCHEMA.json)
- EBNF: [crates/mes-core/MES_LANGUAGE.bnf](crates/mes-core/MES_LANGUAGE.bnf)

### プレフィックス（デフォルト）

| 記号 | 意味 |
|------|------|
| `@` / `＠` | キャラクター |
| `#` / `＃` | コメント |
| `$` / `＄` | サウンドノート |
| `!` / `！` | サウンドポジション |
| `&` / `＆` | タイミング（VTT） |

ヘッダと本文は `----\n` で区切ります。`名前「セリフ」` 形式はフラット化されて `@名前` + セリフに変換されます。
