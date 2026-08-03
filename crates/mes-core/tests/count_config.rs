use mes_core::mes;

#[test]
fn ignored_decorators_do_not_change_mes_structure() {
    let conf = mes::builder::merge_json_conf(
        r##"{"count_config":{"ignore_char":["@","#","ん"]}}"##,
    )
    .expect("valid count config");
    let source = "@Alice\nこんにちは\n#注釈\n";

    let json = mes::count_dialogue_word_to_json_with_conf(source.to_string(), &conf)
        .expect("count dialogue");
    let counts: serde_json::Value = serde_json::from_str(&json).expect("valid JSON");

    assert_eq!(counts["Alice"]["word_count"].as_u64(), Some(4));
    assert!(counts.get("").is_none());
}
