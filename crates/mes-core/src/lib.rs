use mes::Medo;

macro_rules! if_wasm {
    ($($item:item)*) => {$(
        #[cfg(target_arch = "wasm32")]
        $item
    )*}
}

macro_rules! if_hyper {
    ($($item:item)*) => {$(
        #[cfg(not(target_arch = "wasm32"))]
        $item
    )*}
}

if_hyper! {
    pub mod mes;

    mod test_mes;

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
    pub fn count_dialogue_word_to_json(text: &str) -> String {
        let conf = mes::builder::new();
        mes::count_dialogue_word_to_json(text, &conf)
    }
}

if_wasm! {
    use wasm_bindgen::prelude::*;
    pub use crate::mes::*;
    pub mod mes;

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
    #[derive(Debug, Deserialize, Serialize)]
    pub struct WasmMeSBuilder {
        builder: crate::mes::builder::MeSBuilder
    }



    #[wasm_bindgen]
    impl WasmMeSBuilder {
        #[wasm_bindgen]
        pub fn get_default_config() -> String {
            serde_json::to_string(&mes::builder::new()).expect("cannot serialize config")
        }

        pub fn show_config(&self) -> String {
            serde_json::to_string(self).expect("cannot serialize config")
        }
    }

    
    #[wasm_bindgen]
    pub fn parse_mes_to_json(text: &str) -> String {
        mes::builder::new().parse_to_jsonstr(text)
    }

    #[wasm_bindgen]
    pub fn parse_mes_to_json_with_conf(text: &str, json: &str) -> String {
        mes::builder::set_json_conf(json).parse_to_jsonstr(text)
    }

    #[wasm_bindgen]
    pub fn get_default_config_json() -> String {
        serde_json::to_string(&mes::builder::new()).expect("cannot serialize config")
    }

    #[wasm_bindgen]
    pub fn count_dialogue_word_to_json(text: &str) -> String {
        mes::count_dialogue_word_to_json(text, &mes::builder::new())
    }

    #[wasm_bindgen]
    pub fn get_vtt(text: &str) -> String {
        mes::get_vtt(text, &mes::builder::new())
    }
    
    #[wasm_bindgen]
    pub fn echo(text: &str) -> String {
        text.to_string()
    }
    
}