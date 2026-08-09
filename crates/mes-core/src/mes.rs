pub mod builder;

use std::{
    collections::HashMap,
    sync::LazyLock,
};

use regex::Regex;
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use self::builder::MeSBuilder;
use crate::error::{MesError, MesResult};

/* MeSのコア処理 */
//NOTE: メンバを増減するときは、builder.rsのMedoPieceConfigも編集すること
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MedoPiece {
    pub dialogue: String,
    pub comments: String,
    pub sound_note: String,
    pub charactor: String,
    pub sound_position: String,
    pub timing: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MedoBody {
    pub pieces: Vec<MedoPiece>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MedoHeader {
    pub raw: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Medo {
    pub header: MedoHeader,
    pub body: MedoBody,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RawMedo {
    pub header: String,
    pub body: String,
}

static MULTI_BLANK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\n{3,}").expect("valid static regex"));

impl RawMedo {
    pub fn doflat(&mut self, conf: &builder::MeSBuilder) -> MesResult<RawMedo> {
        //NOTE:　フラットレイヤー
        self.toflat_dialogue(conf)?;
        Ok(self.clone())
    }

    pub fn toflat_dialogue(&mut self, conf: &builder::MeSBuilder) -> MesResult<RawMedo> {
        self.body = RawMedo::toflat_dialogue_string(self.body.as_str(), conf)?;
        Ok(self.clone())
    }

    pub fn toflat_dialogue_string(text: &str, conf: &builder::MeSBuilder) -> MesResult<String> {
        let flat_dialogue_config = &conf.mes_config.flat_dialogue_config;
        let name_re = Regex::new(&format!(
            "{}{}",
            r"^.*",
            regex::escape(&flat_dialogue_config.start_str)
        ))
        .map_err(MesError::from)?;
        let raw = MULTI_BLANK_RE.replace_all(text, "\n\n").to_string();
        let line: Vec<&str> = raw.split('\n').collect();
        let body = line
            .into_iter()
            .map(|x| -> String {
                match name_re.captures(x) {
                    Some(val) => {
                        let Some(matched) = val.get(0) else {
                            return x.to_string();
                        };
                        let name = matched
                            .as_str()
                            .replace(flat_dialogue_config.start_str.as_str(), "");
                        let rep_name = name.clone() + flat_dialogue_config.start_str.as_str();
                        let dialogue = x
                            .replace(&rep_name, "")
                            .replace(flat_dialogue_config.end_str.as_str(), "");

                        format!("@{}\n{}\n", name, &dialogue)
                    }
                    None => x.to_string(),
                }
            })
            .collect::<Vec<String>>()
            .join("\n");
        Ok(body)
    }

    /// Reconstruct a MeS script from header/body text.
    pub fn to_mes_string(&self, conf: &builder::MeSBuilder) -> String {
        let delimiter = &conf.mes_config.header_delimiter;
        if self.header.is_empty() {
            return self.body.clone();
        }
        format!("{}{}{}", self.header, delimiter, self.body)
    }
}

impl Medo {
    /// Serialize Medo back to a canonical MeS script using the builder decorators.
    pub fn to_mes_string(&self, conf: &builder::MeSBuilder) -> String {
        medo_to_mes(self, conf)
    }
}

impl RawMedo {
    pub fn parse_header(&self) -> MedoHeader {
        MedoHeader {
            raw: self.header.clone(),
        }
    }

    pub fn parse_body(&self, conf: &MeSBuilder) -> MedoBody {
        parse_medo_body(self.body.as_str(), conf)
    }

    pub fn parse_to_medo(&mut self, conf: &MeSBuilder) -> MesResult<Medo> {
        let mut raw = self.clone();
        raw.doflat(conf)?;
        Ok(Medo {
            header: raw.parse_header(),
            body: parse_medo_body(&raw.body, conf),
        })
    }
}

impl MedoBody {
    fn get_attribute(block: Vec<&str>, prefix: &[char]) -> Vec<String> {
        block
            .into_iter()
            .filter(|x| {
                prefix.iter().any(|&p| match x.chars().next() {
                    Some(v) => v == p,
                    None => false,
                })
            })
            .map(|v| {
                let mut text = v.to_string();
                text.remove(0);
                text
            })
            .collect()
    }

    fn get_dialogue(block: Vec<&str>, ignore_prefix: &[char]) -> Vec<String> {
        block
            .into_iter()
            .filter(|x| {
                ignore_prefix.iter().all(|&p| match x.chars().next() {
                    Some(v) => v != p,
                    None => false,
                })
            })
            .map(|v| v.to_string())
            .collect()
    }
}

fn primary_decorator(chars: &[char]) -> Option<char> {
    chars.first().copied()
}

fn push_prefixed_parts(lines: &mut Vec<String>, value: &str, prefix: Option<char>) {
    let Some(prefix) = prefix else {
        return;
    };
    if value.is_empty() {
        return;
    }
    for part in value.split(',') {
        lines.push(format!("{prefix}{part}"));
    }
}

fn piece_to_mes_lines(piece: &MedoPiece, conf: &MeSBuilder) -> Vec<String> {
    let decorator = &conf.mes_config.medo_piece_config.decorator;
    let mut lines = Vec::new();

    push_prefixed_parts(
        &mut lines,
        &piece.charactor,
        primary_decorator(&decorator.charactor),
    );
    push_prefixed_parts(
        &mut lines,
        &piece.timing,
        primary_decorator(&decorator.timing),
    );
    push_prefixed_parts(
        &mut lines,
        &piece.sound_position,
        primary_decorator(&decorator.sound_position),
    );

    if !piece.dialogue.is_empty() {
        for dialogue_line in piece.dialogue.split('\n') {
            lines.push(dialogue_line.to_string());
        }
    }

    push_prefixed_parts(
        &mut lines,
        &piece.comments,
        primary_decorator(&decorator.comments),
    );
    push_prefixed_parts(
        &mut lines,
        &piece.sound_note,
        primary_decorator(&decorator.sound_note),
    );

    lines
}

/// Serialize a parsed [`Medo`] document back to MeS text.
pub fn medo_to_mes(medo: &Medo, conf: &MeSBuilder) -> String {
    let block_delimiter = &conf.mes_config.medo_piece_config.block_delimiter;
    let body = medo
        .body
        .pieces
        .iter()
        .map(|piece| piece_to_mes_lines(piece, conf).join("\n"))
        .filter(|block| !block.is_empty())
        .collect::<Vec<_>>()
        .join(block_delimiter);

    let raw = RawMedo {
        header: medo.header.raw.clone(),
        body,
    };
    raw.to_mes_string(conf)
}

/* パース関連 */
pub fn parse_mes_to_json(text: &str, conf: &MeSBuilder) -> MesResult<String> {
    let medo = parse_mes(text, conf)?;
    Ok(serde_json::to_string_pretty(&medo)?)
}

pub fn parse_mes_to_json_with_conf(text: &str, json: &str) -> MesResult<String> {
    let conf = builder::merge_json_conf(json)?;
    conf.parse_to_jsonstr(text)
}

pub fn get_default_config_json() -> MesResult<String> {
    Ok(serde_json::to_string(&builder::new())?)
}

pub fn get_vtt(text: &str, conf: &MeSBuilder) -> MesResult<String> {
    let medo = conf.parse(text)?;
    let vtt_list = medo
        .body
        .pieces
        .into_iter()
        .map(|v| -> String {
            let timing = if !v.timing.is_empty() {
                v.timing
            } else {
                "00:00:00.000 --> 00:00:00.000".to_string()
            };
            format!("{}\n{}", timing, v.dialogue)
        })
        .collect::<Vec<String>>();

    Ok(vtt_list.join("\n\n"))
}

fn escape_html(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(c),
        }
    }
    escaped
}

pub fn get_chat(text: &str, conf: &MeSBuilder) -> MesResult<String> {
    let medo = conf.parse(text)?;
    // キャラクター名から決定的に色を割り当てる（同一キャラは常に同色）
    let palette = [
        "#e11d48", "#2563eb", "#059669", "#d97706", "#7c3aed", "#0891b2", "#db2777",
    ];
    let mut color_map: HashMap<String, String> = HashMap::new();
    let mut next_color = 0usize;

    let chat = medo
        .body
        .pieces
        .into_iter()
        .map(|v| {
            let name = if v.charactor.is_empty() {
                "???".to_string()
            } else {
                v.charactor.clone()
            };
            let color = color_map
                .entry(name.clone())
                .or_insert_with(|| {
                    let c = palette[next_color % palette.len()].to_string();
                    next_color += 1;
                    c
                })
                .clone();
            format!(
                "<span style=\"color:{}\">{}: {}</span>",
                color,
                escape_html(&name),
                escape_html(&v.dialogue)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    Ok(chat)
}

pub fn parse_mes(text: &str, conf: &MeSBuilder) -> MesResult<Medo> {
    let mut raw_medo = parse_raw_medo(text, conf);
    //CommonScript等の差異を均す
    raw_medo.doflat(conf)?;

    Ok(Medo {
        header: raw_medo.parse_header(),
        body: raw_medo.parse_body(conf),
    })
}

pub fn parse_raw_medo(text: &str, conf: &MeSBuilder) -> RawMedo {
    let tmp = text.replace("\r\n", "\n");
    let delimiter = conf.mes_config.header_delimiter.as_str();
    match tmp.split_once(delimiter) {
        Some((header, body)) => RawMedo {
            header: header.to_string(),
            body: body.to_string(),
        },
        None => RawMedo {
            header: String::new(),
            body: tmp,
        },
    }
}

pub fn parse_medo_body(_text: &str, conf: &builder::MeSBuilder) -> MedoBody {
    let tmp = _text.replace("\r\n", "\n");
    let block_delimiter = conf.mes_config.medo_piece_config.block_delimiter.as_str();
    let normalized = if block_delimiter == "\n\n" {
        tmp.split('\n')
            .map(|line| if line.trim().is_empty() { "" } else { line })
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        tmp
    };
    let blocks: Vec<&str> = normalized
        .split(block_delimiter)
        .filter(|block| !block.trim().is_empty())
        .collect();

    //設定を破壊されたくないので一旦コピーしてしまう
    let decorator = conf.mes_config.medo_piece_config.decorator.clone();
    let ignore_prefix = [
        decorator.comments,
        decorator.sound_note,
        decorator.charactor,
        decorator.sound_position,
        decorator.timing,
    ]
    .concat();
    let mpc = &conf.mes_config.medo_piece_config;

    let pieces = blocks
        .into_iter()
        .map(|x| -> MedoPiece {
            let lines: Vec<&str> = x
                .split('\n')
                .map(|line| line.trim_end())
                .filter(|line| !line.is_empty())
                .collect();
            let dialogue = MedoBody::get_dialogue(lines.clone(), &ignore_prefix).join("\n");
            let comments = MedoBody::get_attribute(lines.clone(), &mpc.decorator.comments).join(",");
            let sound_note =
                MedoBody::get_attribute(lines.clone(), &mpc.decorator.sound_note).join(",");
            let charactor =
                MedoBody::get_attribute(lines.clone(), &mpc.decorator.charactor).join(",");
            let sound_position =
                MedoBody::get_attribute(lines.clone(), &mpc.decorator.sound_position).join(",");
            let timing = MedoBody::get_attribute(lines.clone(), &mpc.decorator.timing).join(",");

            MedoPiece {
                dialogue,
                comments,
                sound_note,
                charactor,
                sound_position,
                timing,
            }
        })
        .collect();

    MedoBody { pieces }
}

/* WordCount関連のコード */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WordCount {
    charactor: String,
    word_count: usize,
}

pub fn count_dialogue_word_to_json_with_conf(mut text: String, conf: &MeSBuilder) -> MesResult<String> {
    for c in &conf.count_config.ignore_char {
        text = text.replace(c, "");
    }
    count_dialogue_word_to_json(&text, conf)
}

pub fn count_dialogue_word_to_json(text: &str, conf: &MeSBuilder) -> MesResult<String> {
    let medo = parse_mes(text, conf)?;
    //キャラクター毎にワード数を集計する
    let mut word_counter: HashMap<String, WordCount> = HashMap::new();
    medo.body.pieces.into_iter().for_each(|piece: MedoPiece| {
        match word_counter.get_mut(&piece.charactor) {
            Some(x) => {
                //既存のきゃらの集計追加
                x.word_count += piece.dialogue.graphemes(true).count();
            }
            None => {
                //新規キャラの集計追加
                word_counter.insert(
                    piece.charactor.clone(),
                    WordCount {
                        charactor: piece.charactor.clone(),
                        word_count: piece.dialogue.graphemes(true).count(),
                    },
                );
            }
        }
    });
    Ok(serde_json::to_string_pretty(&word_counter)?)
}
