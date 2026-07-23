# MeS Editor (mes2026)

Tauri + Vue 3 + TypeScript の MeS 言語エディタです。`mes-core` で MeS をパースし、Medo JSON / VTT / ワードカウント / チャット形式へ変換できます。

ブラウザのみ（Vite）でも、同梱の WASM ビルド経由でプレビューできます。Tauri 起動時はネイティブコマンドを優先します。

## 構成

| パス | 内容 |
|------|------|
| `crates/mes-core` | MeS パーサ・CLI・言語仕様（`wasm` feature でブラウザ向けバインディング） |
| `src/` | Vue フロントエンド（ハイライト付きエディタ / ファイル I/O / プレビュー） |
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

`pnpm dev` はブラウザ内 WASM で変換します（Tauri 不要）。開く／保存／書き出しはブラウザのファイルピッカーとダウンロードにフォールバックします。WASM を再生成する場合:

```bash
pnpm run build:wasm
```

### デスクトップアプリ（Tauri）

```bash
pnpm tauri dev
```

ネイティブダイアログで `.mes` の開く／保存、プレビューの書き出しが使えます。

### mes-core CLI

```bash
cargo run -p mes-core -- parse path/to/script.mes
cargo run -p mes-core -- vtt path/to/script.mes
cargo run -p mes-core -- count path/to/script.mes
cargo run -p mes-core -- chat path/to/script.mes
cargo run -p mes-core -- emit path/to/script.mes
```

`emit` はパース結果を正規化した MeS として再出力します（Medo → MeS ラウンドトリップ）。

任意の `-c/--config path/to/mes.json` で部分設定をデフォルトへマージできます（全サブコマンド共通）。

### テスト

```bash
cargo test -p mes-core
pnpm build
```

サンプル MeS: [`crates/mes-core/tests/fixtures/sample.mes`](crates/mes-core/tests/fixtures/sample.mes)  
期待出力スナップショット: [`crates/mes-core/tests/fixtures/expected/`](crates/mes-core/tests/fixtures/expected/)

## エディタ機能

- ライブプレビュー（JSON / VTT / Count / Chat）
- MeS プレフィックス（`@` `#` `$` `!` `&`）とヘッダ区切り `----` のシンタックスハイライト
- ファイルの開く／保存／別名で保存
- 現在のプレビュー形式の書き出し
- 正規化（パース結果を MeS としてエディタへ反映。属性・対話の出現順を保持）

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
