use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{RawMedo, parse_raw_medo};
use crate::error::{MesError, MesResult};
use crate::mes::Medo;

/* MeS Config関連のコード */
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct MeSConfig {
    pub name: String,
    /// Default is "----\n"
    pub header_delimiter: String,
    pub flat_dialogue_config: FlatDialogueConfig,
    pub medo_piece_config: MedoPieceConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct MedoPieceConfig {
    pub block_delimiter: String,
    //以下、アトリビュートのメンバ
    pub decorator: MedoPieceDecorator,
}

impl Default for MedoPieceConfig {
    fn default() -> Self {
        Self {
            block_delimiter: "\n\n".to_string(),
            decorator: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct MedoPieceDecorator {
    pub dialogue: Vec<char>,
    pub comments: Vec<char>,
    pub sound_note: Vec<char>,
    pub charactor: Vec<char>,
    pub sound_position: Vec<char>,
    pub timing: Vec<char>,
}

impl Default for MedoPieceDecorator {
    fn default() -> Self {
        Self {
            //以下、アトリビュートのメンバ
            dialogue: vec![],
            comments: vec!['#', '＃'],
            sound_note: vec!['$', '＄'],
            charactor: vec!['@', '＠'],
            sound_position: vec!['!', '！'],
            timing: vec!['&', '＆'],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct FlatDialogueConfig {
    pub start_str: String,
    pub end_str: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct CountConfig {
    pub ignore_char: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct ChatConfig {}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct MeSBuilder {
    pub mes_config: MeSConfig,
    pub count_config: CountConfig,
    pub chat_config: ChatConfig,
}

impl Default for MeSBuilder {
    fn default() -> Self {
        //デフォルト設定の定義
        Self {
            mes_config: MeSConfig {
                header_delimiter: "----\n".to_string(),
                flat_dialogue_config: FlatDialogueConfig {
                    start_str: "「".to_string(),
                    end_str: "」".to_string(),
                    ..Default::default()
                },
                ..Default::default() //残りはDefaultのデフォルトをセットする
            },
            count_config: CountConfig {
                ignore_char: vec![],
                ..Default::default()
            },
            chat_config: ChatConfig {
                ..Default::default()
            },
        }
    }
}

impl MeSBuilder {
    /// CamelCase legacy wrapper kept for compatibility.
    #[deprecated(note = "use `parse_raw_medo` instead")]
    #[allow(non_snake_case)]
    pub fn parseRawMedo(self: &Self, text: &str) -> RawMedo {
        self.parse_raw_medo(text)
    }

    /// Modern snake_case API
    pub fn parse_raw_medo(self: &Self, text: &str) -> RawMedo {
        parse_raw_medo(text, self)
    }
}

impl MeSBuilder {
    pub fn parse(self: &Self, mes_text: &str) -> MesResult<Medo> {
        let mut raw_medo = self.parse_raw_medo(mes_text);
        raw_medo.doflat(self)?;
        Ok(Medo {
            header: raw_medo.parse_header(),
            body: raw_medo.parse_body(self),
        })
    }

    pub fn parse_to_jsonstr(self: &Self, mes_text: &str) -> MesResult<String> {
        let medo = self.parse(mes_text)?;
        Ok(serde_json::to_string_pretty(&medo)?)
    }

    /// Parse then emit a canonical MeS script.
    pub fn emit(self: &Self, mes_text: &str) -> MesResult<String> {
        let medo = self.parse(mes_text)?;
        Ok(medo.to_mes_string(self))
    }
}

pub fn new() -> MeSBuilder {
    Default::default()
}

/// Fully replace config from JSON (all required fields must be present).
pub fn replace_json_conf(json: &str) -> MesResult<MeSBuilder> {
    Ok(serde_json::from_str(json)?)
}

/// Merge a partial JSON config over the default `MeSBuilder`.
///
/// Nested objects are deep-merged; arrays and scalars in the overlay replace
/// the corresponding default values.
pub fn merge_json_conf(json: &str) -> MesResult<MeSBuilder> {
    let mut base = serde_json::to_value(new())?;
    let overlay: Value = serde_json::from_str(json)?;
    if !overlay.is_object() {
        return Err(MesError::new("config JSON must be an object"));
    }
    merge_values(&mut base, &overlay);
    Ok(serde_json::from_value(base)?)
}

/// Backward-compatible entry point: prefer merge over defaults so partial
/// configs work (previously this fully replaced and required every field).
pub fn set_json_conf(json: &str) -> MesResult<MeSBuilder> {
    merge_json_conf(json)
}

fn merge_values(base: &mut Value, overlay: &Value) {
    match (base, overlay) {
        (Value::Object(base_map), Value::Object(overlay_map)) => {
            for (key, overlay_val) in overlay_map {
                match base_map.get_mut(key) {
                    Some(base_val) => merge_values(base_val, overlay_val),
                    None => {
                        base_map.insert(key.clone(), overlay_val.clone());
                    }
                }
            }
        }
        (base_slot, overlay_val) => {
            *base_slot = overlay_val.clone();
        }
    }
}

#[cfg(test)]
mod builder_test {
    use super::MeSBuilder;

    const SAMPLE: &str = "title: demo\n----\n@Alice\nこんにちは\n\n@Bob\nやあ\n";

    #[test]
    fn test_parse_raw_medo_splits_header_and_body() {
        let raw = crate::mes::builder::new().parse_raw_medo(SAMPLE);
        assert!(raw.header.contains("title: demo"));
        assert!(raw.body.contains("@Alice"));
        assert!(raw.body.contains("@Bob"));
    }

    #[test]
    fn test_parse_keeps_header_and_pieces() {
        let medo = crate::mes::builder::new().parse(SAMPLE).unwrap();
        assert_eq!(medo.header.raw.trim(), "title: demo");
        assert_eq!(medo.body.pieces.len(), 2);
        assert_eq!(medo.body.pieces[0].charactor, "Alice");
        assert_eq!(medo.body.pieces[0].dialogue, "こんにちは");
        assert_eq!(medo.body.pieces[1].charactor, "Bob");
    }

    #[test]
    fn test_parse_to_jsonstr_is_valid_json() {
        let json = crate::mes::builder::new()
            .parse_to_jsonstr(SAMPLE)
            .unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        assert_eq!(
            parsed["header"]["raw"].as_str().unwrap().trim(),
            "title: demo"
        );
        assert_eq!(parsed["body"]["pieces"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_parse_to_jsonstr_with_default_conf() {
        let djson = serde_json::to_string(&crate::mes::builder::new()).unwrap();
        let result = crate::mes::builder::set_json_conf(&djson)
            .unwrap()
            .parse_to_jsonstr(SAMPLE)
            .unwrap();
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
    }

    #[test]
    fn test_builder_default_exists() {
        let _: MeSBuilder = crate::mes::builder::new();
    }

    #[test]
    fn merge_json_conf_applies_partial_overlay() {
        let conf = crate::mes::builder::merge_json_conf(
            r#"{"count_config":{"ignore_char":["、","。"]},"mes_config":{"name":"custom"}}"#,
        )
        .unwrap();
        assert_eq!(conf.mes_config.name, "custom");
        assert_eq!(conf.count_config.ignore_char, vec!["、", "。"]);
        // Defaults preserved
        assert_eq!(conf.mes_config.header_delimiter, "----\n");
        assert_eq!(conf.mes_config.flat_dialogue_config.start_str, "「");
    }

    #[test]
    fn replace_json_conf_rejects_partial() {
        let err = crate::mes::builder::replace_json_conf(r#"{"mes_config":{"name":"x"}}"#);
        assert!(err.is_err());
    }
}
