use mes_core::mes;
use mes_core::mes::builder;

#[test]
fn whitespace_only_line_separates_blocks() {
    let source = "@Alice\r\nhello\r\n \t \r\n@Bob\r\nworld\r\n";

    let medo = mes::parse_mes(source, &builder::new()).expect("parse whitespace separator");

    assert_eq!(medo.body.pieces.len(), 2);
    assert_eq!(medo.body.pieces[0].charactor, "Alice");
    assert_eq!(medo.body.pieces[0].dialogue, "hello");
    assert_eq!(medo.body.pieces[1].charactor, "Bob");
    assert_eq!(medo.body.pieces[1].dialogue, "world");
}
