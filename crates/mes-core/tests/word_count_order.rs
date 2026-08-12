use mes_core::mes;
use mes_core::mes::builder;

#[test]
fn word_count_json_orders_characters_stably() {
    let source = "@Charlie\nthree\n\n@Alice\none\n\n@Bob\ntwo\n";

    let json = mes::count_dialogue_word_to_json(source, &builder::new())
        .expect("generate word count");

    let alice = json.find("\"Alice\"").expect("Alice key");
    let bob = json.find("\"Bob\"").expect("Bob key");
    let charlie = json.find("\"Charlie\"").expect("Charlie key");
    assert!(alice < bob && bob < charlie, "unstable key order: {json}");
}
