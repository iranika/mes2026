#[cfg(test)]
mod mes_unit_tests {
    use crate::mes::{self, builder};

    const SAMPLE: &str = r#"meta: sample
----
@Alice
こんにちは
# メモ
$ bell

@Bob
&00:00:01.000 --> 00:00:02.000
やあ

Alice「フラット発話です」
"#;

    #[test]
    fn parse_mes_extracts_attributes() {
        let medo = mes::parse_mes(SAMPLE, &builder::new());
        assert_eq!(medo.header.raw.trim(), "meta: sample");
        assert!(medo.body.pieces.len() >= 3);

        let alice = &medo.body.pieces[0];
        assert_eq!(alice.charactor, "Alice");
        assert_eq!(alice.dialogue, "こんにちは");
        assert_eq!(alice.comments, " メモ");
        assert_eq!(alice.sound_note, " bell");

        let bob = &medo.body.pieces[1];
        assert_eq!(bob.charactor, "Bob");
        assert_eq!(bob.timing, "00:00:01.000 --> 00:00:02.000");
    }

    #[test]
    fn flat_dialogue_converts_bracket_style() {
        let medo = mes::parse_mes(SAMPLE, &builder::new());
        let flat = medo
            .body
            .pieces
            .iter()
            .find(|p| p.charactor == "Alice" && p.dialogue.contains("フラット"))
            .expect("flat dialogue piece");
        assert_eq!(flat.dialogue.trim(), "フラット発話です");
    }

    #[test]
    fn empty_blocks_are_filtered() {
        let text = "@A\nhello\n\n\n\n@B\nworld\n";
        let medo = mes::parse_mes(text, &builder::new());
        assert_eq!(medo.body.pieces.len(), 2);
        assert_eq!(medo.body.pieces[0].charactor, "A");
        assert_eq!(medo.body.pieces[1].charactor, "B");
    }

    #[test]
    fn get_vtt_includes_timing_and_dialogue() {
        let vtt = mes::get_vtt(SAMPLE, &builder::new());
        assert!(vtt.contains("00:00:01.000 --> 00:00:02.000"));
        assert!(vtt.contains("やあ"));
    }

    #[test]
    fn word_count_aggregates_by_character() {
        let json = mes::count_dialogue_word_to_json(SAMPLE, &builder::new());
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.get("Alice").is_some());
        assert!(parsed.get("Bob").is_some());
    }

    #[test]
    fn get_chat_assigns_stable_colors() {
        let chat = mes::get_chat(SAMPLE, &builder::new());
        assert!(chat.contains("Alice:"));
        assert!(chat.contains("color:#"));
    }

    #[test]
    fn medo_piece_default_is_empty() {
        let piece = mes::MedoPiece::default();
        assert!(piece.dialogue.is_empty());
        assert!(piece.charactor.is_empty());
    }
}
