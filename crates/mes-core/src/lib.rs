pub mod mes;

#[cfg(test)]
mod test_mes;

use mes::Medo;

#[inline(always)]
pub fn parse_mes_to_json(text: &str) -> String {
    let conf = mes::builder::new();
    mes::parse_mes_to_json(text, &conf)
}

pub fn parse_mes(text: &str) -> Medo {
    let conf = mes::builder::new();
    mes::parse_mes(text, &conf)
}

#[inline(always)]
pub fn get_vtt(mes_text: &str) -> String {
    mes::get_vtt(mes_text, &mes::builder::new())
}

#[inline(always)]
pub fn get_chat(mes_text: &str) -> String {
    mes::get_chat(mes_text, &mes::builder::new())
}

#[inline(always)]
pub fn count_dialogue_word_to_json(text: &str) -> String {
    let conf = mes::builder::new();
    mes::count_dialogue_word_to_json(text, &conf)
}

/// Browser / WASM bindings for Vite-only preview (no Tauri required).
#[cfg(feature = "wasm")]
mod wasm_exports {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn parse_mes_to_json(text: &str) -> String {
        crate::parse_mes_to_json(text)
    }

    #[wasm_bindgen]
    pub fn get_vtt(text: &str) -> String {
        crate::get_vtt(text)
    }

    #[wasm_bindgen]
    pub fn get_chat(text: &str) -> String {
        crate::get_chat(text)
    }

    #[wasm_bindgen]
    pub fn count_dialogue_word_to_json(text: &str) -> String {
        crate::count_dialogue_word_to_json(text)
    }

    #[wasm_bindgen]
    pub fn get_default_config_json() -> String {
        crate::mes::get_default_config_json()
    }
}
