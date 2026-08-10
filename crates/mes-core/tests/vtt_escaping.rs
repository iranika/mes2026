use mes_core::mes;
use mes_core::mes::builder;

#[test]
fn vtt_escapes_plain_text_that_looks_like_markup() {
    let source = "@Alice\n&00:00:01.000 --> 00:00:02.000\nFish & chips <b>literal</b>\n";

    let vtt = mes::get_vtt(source, &builder::new()).expect("generate vtt");

    assert!(vtt.contains("Fish &amp; chips &lt;b>literal&lt;/b>"));
    assert!(!vtt.contains("<b>"));
}
