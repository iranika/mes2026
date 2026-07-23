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
        let medo = mes::parse_mes(SAMPLE, &builder::new()).unwrap();
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
        let medo = mes::parse_mes(SAMPLE, &builder::new()).unwrap();
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
        let medo = mes::parse_mes(text, &builder::new()).unwrap();
        assert_eq!(medo.body.pieces.len(), 2);
        assert_eq!(medo.body.pieces[0].charactor, "A");
        assert_eq!(medo.body.pieces[1].charactor, "B");
    }

    #[test]
    fn get_vtt_includes_timing_and_dialogue() {
        let vtt = mes::get_vtt(SAMPLE, &builder::new()).unwrap();
        assert!(vtt.contains("00:00:01.000 --> 00:00:02.000"));
        assert!(vtt.contains("やあ"));
    }

    #[test]
    fn word_count_aggregates_by_character() {
        let json = mes::count_dialogue_word_to_json(SAMPLE, &builder::new()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.get("Alice").is_some());
        assert!(parsed.get("Bob").is_some());
    }

    #[test]
    fn get_chat_assigns_stable_colors() {
        let chat = mes::get_chat(SAMPLE, &builder::new()).unwrap();
        assert!(chat.contains("Alice:"));
        assert!(chat.contains("color:#"));
    }

    #[test]
    fn medo_piece_default_is_empty() {
        let piece = mes::MedoPiece::default();
        assert!(piece.dialogue.is_empty());
        assert!(piece.charactor.is_empty());
    }

    #[test]
    fn ignore_char_config_affects_count() {
        let conf = builder::merge_json_conf(r#"{"count_config":{"ignore_char":["ん"]}}"#).unwrap();
        let with_ignore =
            mes::count_dialogue_word_to_json_with_conf(SAMPLE.to_string(), &conf).unwrap();
        let without = mes::count_dialogue_word_to_json(SAMPLE, &builder::new()).unwrap();
        assert_ne!(with_ignore, without);
    }

    #[test]
    fn medo_to_mes_round_trips_parsed_document() {
        let conf = builder::new();
        let parsed = mes::parse_mes(SAMPLE, &conf).unwrap();
        let emitted = parsed.to_mes_string(&conf);
        let reparsed = mes::parse_mes(&emitted, &conf).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn parse_raw_medo_keeps_body_after_second_delimiter() {
        let text = "meta: demo\n----\n@A\nhello\n----\nstill body\n";
        let raw = mes::parse_raw_medo(text, &builder::new());
        assert_eq!(raw.header.trim(), "meta: demo");
        assert!(raw.body.contains("still body"));
        assert!(raw.body.contains("----"));
    }

    #[test]
    fn fullwidth_decorators_are_recognized() {
        let text = "----\n＠Alice\nこんにちは\n＃メモ\n＄bell\n！L\n＆00:00:01.000 --> 00:00:02.000\n";
        let medo = mes::parse_mes(text, &builder::new()).unwrap();
        assert_eq!(medo.body.pieces.len(), 1);
        let piece = &medo.body.pieces[0];
        assert_eq!(piece.charactor, "Alice");
        assert_eq!(piece.dialogue, "こんにちは");
        assert_eq!(piece.comments, "メモ");
        assert_eq!(piece.sound_note, "bell");
        assert_eq!(piece.sound_position, "L");
        assert_eq!(piece.timing, "00:00:01.000 --> 00:00:02.000");
    }

    #[test]
    fn emit_preserves_attribute_and_dialogue_order() {
        let text = "----\n@Alice\nこんにちは\n#メモ\n$bell\n!L\n&00:00:01.000 --> 00:00:02.000\n";
        let conf = builder::new();
        let emitted = mes::parse_mes(text, &conf)
            .unwrap()
            .to_mes_string(&conf);
        let expected = "@Alice\nこんにちは\n#メモ\n$bell\n!L\n&00:00:01.000 --> 00:00:02.000";
        assert!(
            emitted.contains(expected),
            "expected source field order in emit:\n{emitted}"
        );
    }

    #[test]
    fn emit_uses_canonical_order_without_field_layout() {
        let conf = builder::new();
        let medo = mes::Medo {
            header: mes::MedoHeader {
                raw: String::new(),
            },
            body: mes::MedoBody {
                pieces: vec![mes::MedoPiece {
                    charactor: "Alice".into(),
                    dialogue: "hi".into(),
                    comments: "note".into(),
                    timing: "00:00:01.000 --> 00:00:02.000".into(),
                    sound_position: "L".into(),
                    sound_note: "bell".into(),
                    field_order: Vec::new(),
                }],
            },
        };
        let emitted = medo.to_mes_string(&conf);
        assert_eq!(
            emitted,
            "@Alice\n&00:00:01.000 --> 00:00:02.000\n!L\nhi\n#note\n$bell"
        );
    }

    #[test]
    fn crlf_input_normalizes() {
        let text = "meta: crlf\r\n----\r\n@Alice\r\nhello\r\n";
        let medo = mes::parse_mes(text, &builder::new()).unwrap();
        assert_eq!(medo.header.raw.trim(), "meta: crlf");
        assert_eq!(medo.body.pieces[0].charactor, "Alice");
        assert_eq!(medo.body.pieces[0].dialogue, "hello");
    }

    #[test]
    fn missing_header_delimiter_treats_all_as_body() {
        let text = "@Solo\nline\n";
        let medo = mes::parse_mes(text, &builder::new()).unwrap();
        assert!(medo.header.raw.is_empty());
        assert_eq!(medo.body.pieces[0].charactor, "Solo");
    }
}
