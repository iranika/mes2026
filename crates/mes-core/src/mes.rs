pub mod builder;

use std::{iter::Iterator, vec, collections::HashMap};
use regex::Regex;
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use self::builder::MeSBuilder;

/* MeSのコア処理 */
//NOTE: メンバを増減するときは、builder.rsのMedoPieceConfigも編集すること
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MedoPiece {
    pub dialogue: String,
    pub comments: String,
    pub sound_note: String,
    pub charactor: String,
    pub sound_position: String,
    pub timing: String
}

impl Default for MedoPiece {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug,PartialEq,Serialize, Deserialize)]
pub struct MedoBody {
    pub pieces: Vec<MedoPiece>,
}

#[derive(Debug,PartialEq,Serialize, Deserialize)]
pub struct MedoHeader {
    pub raw: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Medo {
    pub header: MedoHeader,
    pub body: MedoBody
}


#[derive(Debug,PartialEq,Clone,Serialize, Deserialize)]
pub struct RawMedo {
    pub header: String,
    pub body: String
}

impl RawMedo {
    //TODO: toflat->common スクリプトへの再変換ができるようにする

    pub fn doflat(&mut self, conf: &builder::MeSBuilder) -> RawMedo{
        //NOTE:　フラットレイヤー
        self.toflat_dialogue(conf);
        
        self.clone()
    }
    pub fn toflat_dialogue(&mut self, conf: &builder::MeSBuilder)-> RawMedo{
        self.body = RawMedo::toflat_dialogue_string(&self.body.as_str(), conf);
        return self.clone()
    }
    //TODO: toflat_Dialogueとロジックを共有する
    pub fn toflat_dialogue_string(text: &str, conf: &builder::MeSBuilder)-> String{
        let flat_dialogue_config = &conf.mes_config.flat_dialogue_config;
        let re = Regex::new(r"\n{3,}").unwrap();
        let name_re = Regex::new(format!("{}{}", r"^.*", flat_dialogue_config.start_str).as_str()).unwrap();
        let raw = re.replace_all(text, "\n\n").to_string();
        let line: Vec<&str> = raw.split("\n").collect();
        let body = line
            .into_iter()
            .map(|x| -> String
                {
                    match name_re.captures(x){
                        Some(val) => {
                            let name = val.get(0).unwrap().as_str().replace(flat_dialogue_config.start_str.as_str(), "");
                            let rep_name = name.clone() + flat_dialogue_config.start_str.as_str();
                            let dialogue = x.replace(&rep_name, "").replace(flat_dialogue_config.end_str.as_str(), "");

                            return format!("@{}\n{}\n", name, &dialogue)
                        },
                        None => x.to_string()
                    }
                }
            )
            .collect::<Vec<String>>()
            .join("\n");        
        return body
    }

}

impl RawMedo {
    pub fn parse_header(&self) -> Option<MedoHeader>{
        return Some(MedoHeader{
            raw : "".to_string()
        })
    }
    pub fn parse_body(&self, conf: &MeSBuilder) -> Option<MedoBody>{
        let result = parse_medo_body(self.body.as_str(), conf);
        //println!("{:?}", result);
        return Some(result)
    }
    pub fn parse_to_medo(&mut self, conf: builder::MeSBuilder) -> Medo{
        Medo{
            header: MedoHeader { raw: self.header.to_string() },
            body: parse_medo_body(&self.body, &conf)
        }
    }
}

impl MedoBody {
    fn get_attribute(block: Vec<&str>, prefix: &Vec<char>) -> Vec<String> {
        let attrs: Vec<String> = block
            .into_iter()
            .filter(|x| prefix.iter().any(|&p| {
                match x.chars().nth(0){
                    Some(v) => v == p,
                    None => false
                }
            }))
            .map(|v| -> String {
                let mut text = v.to_string().clone();
                text.remove(0);
                text
            })
            .collect();
        //println!("{:?}", attrs);
        return attrs;
    }
    fn get_dialogue(block: Vec<&str>, ignore_prefix: &Vec<char>) -> Vec<String> {
        let dialogue = block
            .into_iter()
            .filter(|x| ignore_prefix.iter().all(|&p| {
                match x.chars().nth(0){
                    Some(v) => v != p,
                    None => false
                }
            }))
            .map(|v| v.to_string())
            .collect();
        //println!("{:?}", dialogue);
        return dialogue;
    }
}

/* パース関連 */
pub fn parse_mes_to_json(text: &str, conf: &MeSBuilder) -> String{
    let medo = parse_mes(text, conf);
    let json = serde_json::to_string(&medo).unwrap();
    json
}

pub fn parse_mes_to_json_with_conf(text: &str, json: &str) -> String {
    builder::set_json_conf(json).parse_to_jsonstr(text)
}

pub fn get_default_config_json() -> String {
    serde_json::to_string(&builder::new()).unwrap()
}

pub fn get_vtt(text: &str, conf: &MeSBuilder) -> String{
    let medo = conf.parse(text);
    let vtt_list = medo.body.pieces.into_iter()
    .map(|v|->String{
        let timing = if v.timing != "" { v.timing }else{ "00:00:00.000 --> 00:00:00.000".to_string() };
        let text = format!("{}\n{}", timing, v.dialogue).to_string();
        text
    })
    .collect::<Vec<String>>();

    vtt_list.join("\n\n")

}

pub fn get_chat(text: &str, conf: &MeSBuilder) -> String {
    let medo = conf.parse(text);
    let _colorHash:HashMap<String, String> = HashMap::new();
    let chat_list = medo.body.pieces.into_iter()
        .map(|v|->String{
            //自動で色を割り振る
            format!("<span style=\"color:{}\">{}: {}</span>", &v.charactor, &v.charactor, &v.dialogue)
        }).collect::<Vec<String>>();
    
    chat_list.join("\n")
}

pub fn parse_mes(text: &str, conf: &MeSBuilder) -> Medo {
    //HeaderとBodyに分離

    let mut rawMedo = parse_raw_medo(text, conf);
    //CommonScript等の差異を均す
    rawMedo.doflat(conf);

    //println!("{}",rawMedo.body);
    //rawMedo.body = RawMedo::toflat_DialogueString(&(rawMedo.body));
    //Headerのパース
    //Bodyのパース
    //HeaderとBodyをMedoに結合

    return Medo {
        header: rawMedo.parse_header().unwrap(),
        body: rawMedo.parse_body(conf).unwrap()
    }
}

pub fn parse_raw_medo(text: &str, conf: &MeSBuilder) -> RawMedo {
    let tmp = text.replace("\r\n", "\n");
    let blocks: Vec<&str> = tmp.split(conf.mes_config.header_delimiter.as_str()).collect();
    if blocks.len() == 1 {
        return RawMedo {
            header: "".to_string(),
            body: blocks[0].to_string()
        }
    }
    return RawMedo {
        header: blocks[0].to_string(),
        body: blocks[1].to_string()
    }
}

pub fn parse_medo_body(_text: &str, conf: &builder::MeSBuilder) -> MedoBody {
    
    let tmp = _text.replace("\r\n", "\n");
    //TODO: blocksに不要な空白行から生成されているblockは削除するようにする
    //TODO: 空白行にスペース等があった場合のためにトリミングをする
    let blocks: Vec<&str> = tmp.split(conf.mes_config.medo_piece_config.block_delimiter.as_str()).collect();
    //println!("blocks{:?}", blocks);        
    //設定を破壊されたくないので一旦コピーしてしまう
    let decorator = conf.mes_config.medo_piece_config.decorator.clone();
    let ignore_prefix = vec![
        decorator.comments,
        decorator.sound_note,
        decorator.charactor,
        decorator.sound_position,
        decorator.timing
    ].concat();
    let mpc = &conf.mes_config.medo_piece_config;

    let block = blocks
        .into_iter()
        .map(|x| -> MedoPiece {
            let lines: Vec<&str> = x.split("\n").collect::<Vec<&str>>();
            let dialogue = MedoBody::get_dialogue(
                lines.clone(),
                &ignore_prefix
            ).join("\n"); //MedoBody::get_comments(lines).join(",");
            let comments = MedoBody::get_attribute(
                    lines.clone(),
                    &mpc.decorator.comments).join(","); //MedoBody::get_comments(lines).join(",");
            let sound_note = MedoBody::get_attribute(lines.clone(), &mpc.decorator.sound_note).join(",");
            let charactor = MedoBody::get_attribute(lines.clone(), &mpc.decorator.charactor).join(",");
            let sound_position = MedoBody::get_attribute(lines.clone(), &mpc.decorator.sound_position).join(",");
            let timing = MedoBody::get_attribute(lines.clone(), &mpc.decorator.timing).join(",");

            return MedoPiece {
                dialogue: dialogue,
                comments: comments,
                sound_note: sound_note,
                charactor: charactor,
                sound_position: sound_position,
                timing: timing,
            };
            //println!("{:?}",&result);
        })
        .collect();
    //println!("{:?}", block);

    let result: MedoBody = MedoBody { pieces: block };

    return result;
}




/* WordCount関連のコード */
#[derive(Debug,PartialEq,Serialize, Deserialize)]
pub struct WordCount{
    charactor: String,
    word_count: usize
}

pub fn count_dialogue_word_to_json_with_conf(mut text: String, conf: &MeSBuilder) -> String{
    conf.count_config.ignore_char.iter().for_each(|c|{
        text = text.replace(c, "");
    });
    let result = count_dialogue_word_to_json(&text, conf);
    result
}

pub fn count_dialogue_word_to_json(text: &str, conf: &MeSBuilder) -> String{
    let medo = parse_mes(text, conf);
    //キャラクター毎にワード数を集計する
    let mut word_counter: HashMap<String, WordCount> = HashMap::new(); 
    medo.body.pieces.into_iter().for_each(|piece: MedoPiece|{
        match word_counter.get_mut(&piece.charactor) {
            Some(x) => {
                //既存のきゃらの集計追加
                x.word_count += piece.dialogue.graphemes(true).count();
            }
            None => {
                //新規キャラの集計追加
                word_counter.insert(piece.charactor.clone(), WordCount { charactor: piece.charactor.clone(), word_count: piece.dialogue.graphemes(true).count() });
            }
        }
    });
    let json = serde_json::to_string(&word_counter).unwrap();
    return json;
}