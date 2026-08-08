use std::collections::BTreeSet;

use mes_core::mes::MedoPiece;

#[test]
fn schema_requires_every_serialized_medo_piece_field() {
    let serialized = serde_json::to_value(MedoPiece::default()).expect("serialize MedoPiece");
    let serialized_fields = serialized
        .as_object()
        .expect("MedoPiece serializes as an object")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();

    let schema: serde_json::Value =
        serde_json::from_str(include_str!("../MES_SCHEMA.json")).expect("valid schema JSON");
    let required_fields = schema
        .pointer("/properties/body/properties/pieces/items/required")
        .and_then(serde_json::Value::as_array)
        .expect("MedoPiece required fields")
        .iter()
        .map(|field| field.as_str().expect("required field name"))
        .collect::<BTreeSet<_>>();

    assert_eq!(required_fields, serialized_fields);
}
