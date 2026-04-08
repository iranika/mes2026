use serde::{Deserialize, Serialize};

use super::{RawMedo, parse_raw_medo};
use crate::mes::{Medo};

/* MeS Config関連のコード */
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MeSConfig{
    pub name: String,
    /// Default is "----\n"
    pub header_delimiter: String,
    pub flat_dialogue_config: FlatDialogueConfig,
    pub medo_piece_config: MedoPieceConfig
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MedoPieceConfig {
    pub block_delimiter: String,
    //以下、アトリビュートのメンバ
    pub decorator: MedoPieceDecorator,
}

impl Default for MedoPieceConfig{
    fn default() -> Self {
        Self { 
            block_delimiter: "\n\n".to_string(),
            decorator: Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MedoPieceDecorator {
    pub dialogue: Vec<char>,
    pub comments: Vec<char>,
    pub sound_note: Vec<char>,
    pub charactor: Vec<char>,
    pub sound_position: Vec<char>,
    pub timing: Vec<char>
}

impl Default for MedoPieceDecorator {
    fn default() -> Self {
        Self {
            //以下、アトリビュートのメンバ
            dialogue: vec![],
            comments: vec!['#','＃'], 
            sound_note: vec!['$','＄'],
            charactor: vec!['@','＠'],
            sound_position: vec!['!','！'],
            timing: vec!['&','＆'],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct FlatDialogueConfig{
    pub start_str: String,
    pub end_str: String,
    
}


#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CountConfig{
    pub ignore_char: Vec<String>
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ChatConfig{

}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeSBuilder{
    pub mes_config: MeSConfig,
    pub count_config: CountConfig,
    pub chat_config: ChatConfig
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

            }
        }
    }
}

impl MeSBuilder {
    /// CamelCase legacy wrapper kept for compatibility.
    #[deprecated(note = "use `parse_raw_medo` instead")]
    pub fn parseRawMedo(self: &Self, text: &str) -> RawMedo {
        self.parse_raw_medo(text)
    }

    /// Modern snake_case API
    pub fn parse_raw_medo(self: &Self, text: &str) -> RawMedo {
        parse_raw_medo(text, self)
    }
}

impl MeSBuilder {
    pub fn parse(self: &Self, mes_text: &str) -> Medo{
        let mut raw_medo = self.parse_raw_medo(mes_text);
        raw_medo.doflat(self);
        Medo{
            header: raw_medo.parse_header().unwrap(),
            body: raw_medo.parse_body(self).unwrap()
        }

    }
    pub fn parse_to_jsonstr(self: &Self, mes_text: &str) -> String{
        let medo = self.parse(mes_text);
        serde_json::to_string(&medo).unwrap()
    }
}

pub fn new()->MeSBuilder{
    let builder: MeSBuilder = Default::default();
    builder
}

pub fn set_json_conf(json: &str)->MeSBuilder{
    //TODO: JsonStringとのマージを実行できるようにする
    let builder: MeSBuilder = serde_json::from_str(json).unwrap();
    builder
}


#[cfg(test)]
mod builder_test{

    use crate::mes::{RawMedo, self};

    use super::MeSBuilder;

    //TODO: メジャーバージョンリリース時にテストデータを固定していく

    #[test]
    fn test_parseRawMedo(){
        let text = std::fs::read_to_string("tests/SampleCommonScript.txt").unwrap();
        let rawmedo: RawMedo = crate::mes::builder::new().parseRawMedo(&text);

        println!("<header>{}</header>", rawmedo.header);
        println!("<body>{}</body>", rawmedo.body);
        
    }

    #[test]
    fn test_parse(){
        let text = std::fs::read_to_string("tests/SampleCommonScript.txt").unwrap();
        let medo = crate::mes::builder::new().parse(&text);

        println!("<header>{:?}</header>", medo.header);
        println!("<body>{:?}</body>", medo.body);
        
    }

    #[test]
    fn test_parse_to_jsonstr(){
        let text = std::fs::read_to_string("tests/SampleCommonScript.txt").unwrap();
        let json = crate::mes::builder::new().parse_to_jsonstr(&text);

        println!("{}", json);
    }

    #[test]
    fn test_parse_to_jsonstr_withconf(){
        let text = std::fs::read_to_string("tests/SampleCommonScript.txt").unwrap();
        let json = r#"
{
    "mes_config": {
        "name": "",
        "header_delimiter": "----\n",
        "flat_dialogue_config": {
        "start_str": "「",
        "end_str": "」"
        },
    }
}"#;
        let djson = serde_json::to_string(&mes::builder::new()).unwrap();
        let result = mes::builder::set_json_conf(&djson).parse_to_jsonstr(&text);
        println!("{}", result);
    }

}