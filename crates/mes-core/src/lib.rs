pub mod error;
pub mod mes;

#[cfg(test)]
mod test_mes;

pub use error::{MesError, MesResult};
use mes::Medo;

#[inline(always)]
pub fn parse_mes_to_json(text: &str) -> MesResult<String> {
    let conf = mes::builder::new();
    mes::parse_mes_to_json(text, &conf)
}

pub fn parse_mes(text: &str) -> MesResult<Medo> {
    let conf = mes::builder::new();
    mes::parse_mes(text, &conf)
}

#[inline(always)]
pub fn get_vtt(mes_text: &str) -> MesResult<String> {
    mes::get_vtt(mes_text, &mes::builder::new())
}

#[inline(always)]
pub fn get_chat(mes_text: &str) -> MesResult<String> {
    mes::get_chat(mes_text, &mes::builder::new())
}

#[inline(always)]
pub fn count_dialogue_word_to_json(text: &str) -> MesResult<String> {
    let conf = mes::builder::new();
    mes::count_dialogue_word_to_json(text, &conf)
}

/// Browser / WASM bindings for Vite-only preview (no Tauri required).
#[cfg(feature = "wasm")]
mod wasm_exports {
    use wasm_bindgen::prelude::*;

    fn to_js_err(err: crate::MesError) -> JsValue {
        JsValue::from_str(err.message())
    }

    #[wasm_bindgen]
    pub fn parse_mes_to_json(text: &str) -> Result<String, JsValue> {
        crate::parse_mes_to_json(text).map_err(to_js_err)
    }

    #[wasm_bindgen]
    pub fn get_vtt(text: &str) -> Result<String, JsValue> {
        crate::get_vtt(text).map_err(to_js_err)
    }

    #[wasm_bindgen]
    pub fn get_chat(text: &str) -> Result<String, JsValue> {
        crate::get_chat(text).map_err(to_js_err)
    }

    #[wasm_bindgen]
    pub fn count_dialogue_word_to_json(text: &str) -> Result<String, JsValue> {
        crate::count_dialogue_word_to_json(text).map_err(to_js_err)
    }

    #[wasm_bindgen]
    pub fn get_default_config_json() -> Result<String, JsValue> {
        crate::mes::get_default_config_json().map_err(to_js_err)
    }

    #[wasm_bindgen]
    pub fn merge_config_json(partial: &str) -> Result<String, JsValue> {
        let conf = crate::mes::builder::merge_json_conf(partial).map_err(to_js_err)?;
        serde_json::to_string_pretty(&conf).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
