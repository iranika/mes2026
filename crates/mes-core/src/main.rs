use std::path::PathBuf;

use clap::{Parser, Subcommand};
use config::{Config, File};
use mes_core::mes;
use mes_core::mes::builder::MeSBuilder;
use question::{Answer, Question};

#[derive(Debug, Parser)]
#[clap(name = "mes", author, about, version)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(short = 'c', long = "config", default_value_t = String::from("./mes.json"), value_parser)]
    conf: String,
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
    /// コンフィグを表示します
    Show,
    /// 初期設定のコンフィグを作成します
    Create,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// MeSをパースしてMedo型のJSON文字列として出力します.
    Parse {
        #[clap(value_parser)]
        path: std::path::PathBuf,
    },
    Vtt {
        #[clap(value_parser)]
        path: std::path::PathBuf,
    },
    /// チャット形式（HTML span）で出力します.
    Chat {
        #[clap(value_parser)]
        path: std::path::PathBuf,
    },
    /// コンフィグ関連のサブコマンドです
    Config {
        #[clap(subcommand)]
        conf: ConfigCommand,
    },
    /// キャラ毎にセリフの文字数を集計します
    Count {
        #[clap(value_parser)]
        path: std::path::PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    // コンフィグの初期化（ファイルがあればデフォルトへマージ）
    let mes_conf = load_config(&cli.conf);

    // サブコマンドの解析
    match cli.command {
        Commands::Chat { path } => do_chat(path, &mes_conf),
        Commands::Parse { path } => do_parse(path, &mes_conf),
        Commands::Vtt { path } => do_vtt(path, &mes_conf),
        Commands::Count { path } => do_count(path, &mes_conf),
        Commands::Config { conf } => match conf {
            ConfigCommand::Create => do_config_create(cli.conf),
            ConfigCommand::Show => do_config_show(&mes_conf),
        },
    }
}

fn load_config(path: &str) -> MeSBuilder {
    if !std::path::Path::new(path).exists() {
        return mes::builder::new();
    }

    // Prefer deep-merge of the file JSON over defaults so partial configs work.
    match std::fs::read_to_string(path) {
        Ok(raw) => match mes::builder::merge_json_conf(&raw) {
            Ok(conf) => conf,
            Err(err) => {
                eprintln!("warning: failed to merge config ({err}); trying typed deserialize");
                Config::builder()
                    .add_source(File::with_name(path))
                    .build()
                    .ok()
                    .and_then(|c| c.try_deserialize::<MeSBuilder>().ok())
                    .unwrap_or_else(mes::builder::new)
            }
        },
        Err(_) => mes::builder::new(),
    }
}

fn do_parse(path: PathBuf, conf: &MeSBuilder) {
    let content = std::fs::read_to_string(path).expect("could not read file");
    let json = mes::parse_mes_to_json(&content, conf).expect("parse failed");
    print!("{json}");
}

fn do_vtt(path: PathBuf, conf: &MeSBuilder) {
    let content = std::fs::read_to_string(path).expect("could not read file");
    let text = mes::get_vtt(&content, conf).expect("vtt failed");
    print!("{text}");
}

fn do_chat(path: PathBuf, conf: &MeSBuilder) {
    let content = std::fs::read_to_string(path).expect("could not read file");
    let text = mes::get_chat(&content, conf).expect("chat failed");
    print!("{text}");
}

fn do_count(path: PathBuf, conf: &MeSBuilder) {
    let content = std::fs::read_to_string(path).expect("could not read file");
    let json = mes::count_dialogue_word_to_json_with_conf(content, conf).expect("count failed");
    println!("{json}");
}

fn do_config_create(path: String) {
    let def_conf = mes::builder::new();
    let json = serde_json::to_string_pretty(&def_conf).expect("cannot serialize config");
    let filepath = std::path::Path::new(&path);
    if filepath.exists() {
        let answer = Question::new("すでにファイルが存在します。上書きしますか？").confirm();
        if answer == Answer::NO {
            return;
        }
    }
    std::fs::write(path, json).expect("cannot write config");
}

fn do_config_show(mesconf: &MeSBuilder) {
    let output = serde_json::to_string_pretty(mesconf).expect("cannot serialize config");
    println!("{output}");
}
