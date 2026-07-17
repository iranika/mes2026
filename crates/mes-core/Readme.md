# mes-core

MeS（セリフ／注釈スクリプト）の参照実装クレートです。

## 主な API

いずれも `MesResult`（`Result<T, MesError>`）を返します。

- `parse_mes` / `parse_mes_to_json` — Medo 構造体 / JSON
- `get_vtt` — WebVTT 風テキスト
- `count_dialogue_word_to_json` — キャラ別文字数
- `get_chat` — 色付き HTML チャット形式
- `builder::merge_json_conf` — 部分設定 JSON をデフォルトへディープマージ

## CLI

```bash
cargo run -p mes-core -- parse script.mes
cargo run -p mes-core -- vtt script.mes
cargo run -p mes-core -- count script.mes
cargo run -p mes-core -- chat script.mes
cargo run -p mes-core -- config show
cargo run -p mes-core -- config create
```

## 仕様

- [MES_LANGUAGE.md](./MES_LANGUAGE.md)
- [MES_SCHEMA.json](./MES_SCHEMA.json)
- [MES_LANGUAGE.bnf](./MES_LANGUAGE.bnf)
