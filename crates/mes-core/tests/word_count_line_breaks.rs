use mes_core::mes;
use mes_core::mes::builder;

#[test]
fn word_count_excludes_dialogue_line_breaks() {
    let source = "@Alice\nfirst\nsecond\n";

    let json = mes::count_dialogue_word_to_json(source, &builder::new())
        .expect("generate word count");
    let counts: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");

    assert_eq!(counts["Alice"]["word_count"].as_u64(), Some(11));
}
