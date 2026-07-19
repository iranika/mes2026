//! Fixture-driven snapshot / edge-case tests under `tests/fixtures/`.

use std::fs;
use std::path::PathBuf;

use mes_core::mes;
use mes_core::mes::builder;

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture_dir().join(name)).unwrap_or_else(|e| {
        panic!("failed to read fixture {name}: {e}");
    })
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n").trim_end().to_string()
}

#[test]
fn sample_mes_json_matches_expected() {
    let source = read_fixture("sample.mes");
    let expected = normalize_newlines(&read_fixture("expected/sample.json"));
    let actual = normalize_newlines(
        &mes::parse_mes_to_json(&source, &builder::new()).expect("parse json"),
    );
    let expected_v: serde_json::Value = serde_json::from_str(&expected).unwrap();
    let actual_v: serde_json::Value = serde_json::from_str(&actual).unwrap();
    assert_eq!(actual_v, expected_v);
}

#[test]
fn sample_mes_vtt_matches_expected() {
    let source = read_fixture("sample.mes");
    let expected = normalize_newlines(&read_fixture("expected/sample.vtt"));
    let actual =
        normalize_newlines(&mes::get_vtt(&source, &builder::new()).expect("vtt"));
    assert_eq!(actual, expected);
}

#[test]
fn sample_mes_chat_matches_expected() {
    let source = read_fixture("sample.mes");
    let expected = normalize_newlines(&read_fixture("expected/sample.chat.html"));
    let actual =
        normalize_newlines(&mes::get_chat(&source, &builder::new()).expect("chat"));
    assert_eq!(actual, expected);
}

#[test]
fn sample_mes_count_matches_expected() {
    let source = read_fixture("sample.mes");
    let expected = normalize_newlines(&read_fixture("expected/sample.count.json"));
    let actual = normalize_newlines(
        &mes::count_dialogue_word_to_json(&source, &builder::new()).expect("count"),
    );
    // HashMap key order is unstable; compare as JSON values.
    let expected_v: serde_json::Value = serde_json::from_str(&expected).unwrap();
    let actual_v: serde_json::Value = serde_json::from_str(&actual).unwrap();
    assert_eq!(actual_v, expected_v);
}

#[test]
fn sample_mes_round_trips_through_emit() {
    let source = read_fixture("sample.mes");
    let conf = builder::new();
    let parsed = mes::parse_mes(&source, &conf).expect("parse");
    let emitted = mes::medo_to_mes(&parsed, &conf);
    let reparsed = mes::parse_mes(&emitted, &conf).expect("reparse");
    assert_eq!(parsed, reparsed);
}

#[test]
fn fullwidth_fixture_parses_attributes() {
    let source = read_fixture("fullwidth.mes");
    let medo = mes::parse_mes(&source, &builder::new()).expect("parse fullwidth");
    assert_eq!(medo.body.pieces.len(), 1);
    let piece = &medo.body.pieces[0];
    assert_eq!(piece.charactor, "Alice");
    assert_eq!(piece.dialogue, "こんにちは");
    assert_eq!(piece.comments, "メモ");
    assert_eq!(piece.sound_note, "chime");
    assert_eq!(piece.sound_position, "L");
    assert_eq!(piece.timing, "00:00:01.000 --> 00:00:02.000");
}

#[test]
fn no_header_fixture_parses_body_only() {
    let source = read_fixture("no_header.mes");
    let medo = mes::parse_mes(&source, &builder::new()).expect("parse no_header");
    assert!(medo.header.raw.is_empty());
    assert_eq!(medo.body.pieces.len(), 2);
    assert_eq!(medo.body.pieces[0].charactor, "Narrator");
    assert_eq!(medo.body.pieces[1].charactor, "Bob");
}

#[test]
fn fixtures_round_trip_table() {
    let conf = builder::new();
    for name in ["sample.mes", "fullwidth.mes", "no_header.mes"] {
        let source = read_fixture(name);
        let parsed = mes::parse_mes(&source, &conf)
            .unwrap_or_else(|e| panic!("{name} parse failed: {e}"));
        let emitted = mes::medo_to_mes(&parsed, &conf);
        let reparsed = mes::parse_mes(&emitted, &conf)
            .unwrap_or_else(|e| panic!("{name} reparse failed: {e}"));
        assert_eq!(parsed, reparsed, "{name} round-trip mismatch");
    }
}
