pub mod builder;

use std::{
    collections::HashMap,
    sync::LazyLock,
};

use regex::Regex;
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use self::builder::{MeSBuilder, MedoPieceDecorator};
use crate::error::{MesError, MesResult};

/// Logical fields of a [`MedoPiece`], used to preserve source attribute/dialogue order on emit.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MedoPieceField {
    Charactor,
    Timing,
    SoundPosition,
    Dialogue,
    Comments,
    SoundNote,
}

/// Canonical emit order when no parse-time layout was recorded (e.g. JSON-imported Medo).
const DEFAULT_FIELD_ORDER: &[MedoPieceField] = &[
    MedoPieceField::Charactor,
    MedoPieceField::Timing,
    MedoPieceField::SoundPosition,
    MedoPieceField::Dialogue,
    MedoPieceField::Comments,
    MedoPieceField::SoundNote,
];

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
    /// Source field order observed while parsing. Skipped in JSON interchange.
    #[serde(skip)]
    pub field_order: Vec<MedoPieceField>,
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

fn push_dialogue_lines(lines: &mut Vec<String>, dialogue: &str) {
    if dialogue.is_empty() {
        return;
    }
    for dialogue_line in dialogue.split('\n') {
        lines.push(dialogue_line.to_string());
    }
}

fn classify_line(first: char, decorator: &MedoPieceDecorator) -> MedoPieceField {
    if decorator.charactor.iter().any(|&p| p == first) {
        MedoPieceField::Charactor
    } else if decorator.timing.iter().any(|&p| p == first) {
        MedoPieceField::Timing
    } else if decorator.sound_position.iter().any(|&p| p == first) {
        MedoPieceField::SoundPosition
    } else if decorator.comments.iter().any(|&p| p == first) {
        MedoPieceField::Comments
    } else if decorator.sound_note.iter().any(|&p| p == first) {
        MedoPieceField::SoundNote
    } else {
        MedoPieceField::Dialogue
    }
}

fn push_unique_field(order: &mut Vec<MedoPieceField>, field: MedoPieceField) {
    if !order.contains(&field) {
        order.push(field);
    }
}

fn append_csv(target: &mut String, value: &str) {
    if target.is_empty() {
        target.push_str(value);
    } else {
        target.push(',');
        target.push_str(value);
    }
}

fn append_dialogue(target: &mut String, line: &str) {
    if target.is_empty() {
        target.push_str(line);
    } else {
        target.push('\n');
        target.push_str(line);
    }
}

fn piece_to_mes_lines(piece: &MedoPiece, conf: &MeSBuilder) -> Vec<String> {
    let decorator = &conf.mes_config.medo_piece_config.decorator;
    let mut lines = Vec::new();
    let order = if piece.field_order.is_empty() {
        DEFAULT_FIELD_ORDER
    } else {
        piece.field_order.as_slice()
    };

    for field in order {
        match field {
            MedoPieceField::Charactor => push_prefixed_parts(
                &mut lines,
                &piece.charactor,
                primary_decorator(&decorator.charactor),
            ),
            MedoPieceField::Timing => push_prefixed_parts(
                &mut lines,
                &piece.timing,
                primary_decorator(&decorator.timing),
            ),
            MedoPieceField::SoundPosition => push_prefixed_parts(
                &mut lines,
                &piece.sound_position,
                primary_decorator(&decorator.sound_position),
            ),
            MedoPieceField::Dialogue => push_dialogue_lines(&mut lines, &piece.dialogue),
            MedoPieceField::Comments => push_prefixed_parts(
                &mut lines,
                &piece.comments,
                primary_decorator(&decorator.comments),
            ),
            MedoPieceField::SoundNote => push_prefixed_parts(
                &mut lines,
                &piece.sound_note,
                primary_decorator(&decorator.sound_note),
            ),
        }
    }

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
                color, name, v.dialogue
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
    let blocks: Vec<&str> = tmp
        .split(conf.mes_config.medo_piece_config.block_delimiter.as_str())
        .filter(|block| !block.trim().is_empty())
        .collect();

    let mpc = &conf.mes_config.medo_piece_config;

    let pieces = blocks
        .into_iter()
        .map(|x| -> MedoPiece {
            let lines: Vec<&str> = x
                .split('\n')
                .map(|line| line.trim_end())
                .filter(|line| !line.is_empty())
                .collect();

            let mut piece = MedoPiece::default();
            let mut field_order = Vec::new();

            for line in lines {
                let mut chars = line.chars();
                let Some(first) = chars.next() else {
                    continue;
                };
                let field = classify_line(first, &mpc.decorator);
                push_unique_field(&mut field_order, field);
                match field {
                    MedoPieceField::Dialogue => append_dialogue(&mut piece.dialogue, line),
                    MedoPieceField::Charactor => append_csv(&mut piece.charactor, chars.as_str()),
                    MedoPieceField::Timing => append_csv(&mut piece.timing, chars.as_str()),
                    MedoPieceField::SoundPosition => {
                        append_csv(&mut piece.sound_position, chars.as_str())
                    }
                    MedoPieceField::Comments => append_csv(&mut piece.comments, chars.as_str()),
                    MedoPieceField::SoundNote => append_csv(&mut piece.sound_note, chars.as_str()),
                }
            }

            piece.field_order = field_order;
            piece
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
