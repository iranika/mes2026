extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
pub use crate::mes::*; 

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(getter_with_clone)]
pub struct WasmMedo {
    medo: crate::mes::Medo
}
#[wasm_bindgen]
pub struct WasmMeSBuilder {
    builder: crate::mes::builder::MeSBuilder
}

#[wasm_bindgen]
pub fn parseMeSToJson(text: &str) -> String {
    crate::mes::builder::new().parse_to_jsonstr(text)
}

#[wasm_bindgen]
pub fn countDialogueWordToJson(text: &str) -> String {
    crate::mes::countDialogueWordToJson(text)
}

#[wasm_bindgen]
pub fn echo(text: &str) -> String {
    text.to_string()
}