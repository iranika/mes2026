use mes_core::mes::builder;

#[test]
fn partial_config_accepts_known_fields() {
    let result = builder::merge_json_conf(
        r#"{"mes_config":{"header_delimiter":"====\n"}}"#,
    );

    assert!(result.is_ok(), "known partial config should remain valid");
}

#[test]
fn partial_config_rejects_unknown_root_key() {
    let result = builder::merge_json_conf(r#"{"counting_config":{}}"#);

    assert!(result.is_err(), "unknown root key should not be ignored");
}

#[test]
fn partial_config_rejects_unknown_nested_key() {
    let result = builder::merge_json_conf(
        r#"{"mes_config":{"header_delimeter":"====\n"}}"#,
    );

    assert!(result.is_err(), "misspelled nested key should not be ignored");
}
