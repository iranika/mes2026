use mes_core::mes;
use mes_core::mes::builder;

#[test]
fn custom_crlf_header_delimiter_matches_normalized_input() {
    let conf = builder::merge_json_conf(
        r#"{"mes_config":{"header_delimiter":"====\r\n"}}"#,
    )
    .expect("custom config");
    let source = "title: windows\r\n====\r\n@Alice\r\nhello\r\n";

    let medo = mes::parse_mes(source, &conf).expect("parse CRLF document");

    assert_eq!(medo.header.raw, "title: windows\n");
    assert_eq!(medo.body.pieces.len(), 1);
    assert_eq!(medo.body.pieces[0].charactor, "Alice");
    assert_eq!(medo.body.pieces[0].dialogue, "hello");
}
