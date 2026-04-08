# MeS 言語仕様（概要）

このドキュメントは `mes-core` クレートの実装に基づいた MeS 言語の仕様書です。
参照実装: [crates/mes-core/src/mes.rs](crates/mes-core/src/mes.rs#L1) と [crates/mes-core/src/mes/builder.rs](crates/mes-core/src/mes/builder.rs#L1)。

---

## 1. 概要

MeS はセリフ（ダイアログ）や注釈・メタ情報を含むテキスト形式で、次の目的で解析されます:
- 構造化された JSON (`Medo`) への変換
- VTT（簡易キャプション）出力
- キャラクター別ワードカウント

解析は `MeSBuilder` の設定（`mes_config` / `medo_piece_config`）に基づいて動作します。

---

## 2. ファイル全体の構造

- ヘッダ（任意）と本文（メイン）の2つに分割
- ヘッダと本文は `header_delimiter` により分割される（デフォルト `----\n`）

分割ロジック（擬似）:
- 改行コードは `\r\n` -> `\n` に正規化
- テキストを `header_delimiter` で最初に split
  - 区切りがない場合: header=""、body=全文
  - 区切りがある場合: header=最初のブロック、body=2番目のブロック（最初の区切りのみ）

---

## 3. 本文（MedoPiece）の分割ルール

- 本文は `medo_piece_config.block_delimiter`（デフォルト `"\n\n"`）でブロックに分割
- 各ブロックは1つの `MedoPiece` に変換される
- ブロック内の行はさらに分類:
  - 属性行: 先頭が特定のプレフィックス文字で始まる行
  - 対話行: 上記プレフィックスで始まらない行（発話テキスト）

### デフォルトのプレフィックス（`MedoPieceDecorator`）
- comments: `#`, `＃`
- sound_note: `$`, `＄`
- charactor: `@`, `＠`
- sound_position: `!`, `！`
- timing: `&`, `＆`
- dialogue: 既定は空（ダイアログはプレフィックス無しの行）

> これらは `crates/mes-core/src/mes/builder.rs` の `MedoPieceDecorator::default()` に定義されています。

---

## 4. フラットダイアログ（Flat dialogue）機能

- `flat_dialogue_config`（デフォルト: start_str=`「`, end_str=`」`）
- ログやフォーマットの違いを吸収するために、`toflat_dialogue_string` が実行される

処理のポイント:
- `\n` が3つ以上連続する箇所は `\n\n` に正規化
- 各行について `^.*「` パターンで開始するかを判定
  - マッチする場合：行先頭の「name」記法を検出し、内部の会話を `@name\n<dialogue>\n` の形式に変換
  - 例: `登場人物「発話文」` ->

```
@登場人物
発話文
```

これにより後続パーサーは `@` プレフィックスによるキャラクター指定を受け取れる。

---

## 5. パースアルゴリズム（高レベル）

1. 改行正規化（`\r\n` -> `\n`）
2. ヘッダ/本文を `header_delimiter` で分割
3. `toflat_dialogue` を適用し、フラット化された本文を得る
4. 本文を `block_delimiter` でブロック分割
5. 各ブロックを行単位で解析し、属性行と対話行を分離
6. `MedoPiece` を生成（fields: `dialogue`, `comments`, `sound_note`, `charactor`, `sound_position`, `timing`）
7. `Medo` 構造体（`header`, `body`）として出力

---

## 6. データモデル（JSON 出力）

主要構造体（JSON 例として）:

- `Medo`:
```json
{
  "header": { "raw": "..." },
  "body": { "pieces": [ { "dialogue": "...", "comments":"...", "sound_note":"...", "charactor":"...", "sound_position":"...", "timing":"..." } ] }
}
```

- `MedoPiece` 各フィールドは文字列（複数行は `\n` を含む）

---

## 7. 付加機能

### VTT 生成
- `get_vtt(text, conf)` は `timing` 属性を使い、各 `MedoPiece` を次の形で VTT 化:

```
<timing>
<dialogue>

<timing>
<dialogue>
```

- `timing` が空の場合、デフォルト `00:00:00.000 --> 00:00:00.000` を使用

### チャット出力
- `get_chat(text, conf)` は各 `MedoPiece` を HTML の `<span style="color:...">CHAR: DIALOGUE</span>` のリストとして返す
- カラー割り当ては未実装（空のハッシュに対して自動割当が示唆されている）

### ワードカウント
- `count_dialogue_word_to_json(text, conf)` は `unicode-segmentation` の `graphemes(true)` を使って文字（グラフェム）数をカウント
- 結果はキャラクターごとに集計した `HashMap<String, WordCount>` を JSON 化して返す
- `count_dialogue_word_to_json_with_conf(text, conf)` は `conf.count_config.ignore_char` に含まれる文字列を事前に削除してから集計する

---

## 8. 主要設定とデフォルト値

設定は `MeSBuilder`（`mes_config`, `count_config`, `chat_config`）により管理。主要デフォルト:

- `mes_config.header_delimiter`: `----\n`
- `mes_config.flat_dialogue_config.start_str`: `「`
- `mes_config.flat_dialogue_config.end_str`: `」`
- `medo_piece_config.block_delimiter`: `\n\n`
- `medo_piece_config.decorator`:
  - comments: `[#, ＃]`
  - sound_note: `[$, ＄]`
  - charactor: `[@, ＠]`
  - sound_position: `[!, ！]`
  - timing: `[&, ＆]`
- `count_config.ignore_char`: `[]`（空）

---

## 9. 入出力例

### 入力（例）
```
----
# header meta
----
@Alice
こんにちは
# 注釈
$ sfx:bell
& 00:00:01.000 --> 00:00:03.000

@Bob
やあ
```

### 出力（Medo JSON 抜粋）
```json
{
  "header": { "raw": "# header meta" },
  "body": {
    "pieces": [
      { "dialogue": "こんにちは", "comments":" 注釈", "sound_note":" sfx:bell", "charactor":"Alice", "sound_position":"", "timing":"00:00:01.000 --> 00:00:03.000" },
      { "dialogue": "やあ", "comments":"", "sound_note":"", "charactor":"Bob", "sound_position":"", "timing":"" }
    ]
  }
}
```

（実際の出力は `MedoPiece` のフィールド連結方法に依存します）

---

## 10. 実装上の注意点 / TODO

- `toflat_dialogue` の挙動は現状 `start_str`/`end_str` の組を前提に行われるため、他のフォーマットには注意が必要
- 空行トリムやブロック生成の処理は厳密ではない（TODO コメントあり）
- `MeSBuilder` の設定を JSON でロード・マージする機能は限定的（`set_json_conf` は完全置換）
- 文字列カウントはグラフェムベース（日本語対応）

---

## 11. 参考ソース
- 実装: [crates/mes-core/src/mes.rs](crates/mes-core/src/mes.rs#L1)
- 設定とデフォルト: [crates/mes-core/src/mes/builder.rs](crates/mes-core/src/mes/builder.rs#L1)


もし、この仕様書を README に追加したり、より詳細な構文定義（BNF 風）や JSON スキーマを生成したければ、次に作成します。