//! Fixture-driven snapshot tests against `tests/fixtures/sample.mes`.

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
