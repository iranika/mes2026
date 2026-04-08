use std::path::PathBuf;

use clap::{Parser, Subcommand};
use config::{Config, File};
use mes_core::mes::builder::MeSBuilder;
use mes_core::mes;
use question::{Question, Answer};

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
    Create
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// MeSをパースしてMedo型のJSON文字列として出力します.
    Parse{
        #[clap(value_parser)]
        path: std::path::PathBuf,
    },
    Vtt{
        #[clap(value_parser)]
        path: std::path::PathBuf,
    },
    /// WIP:チャット形式で出力します.
    Chat {
        #[clap(value_parser)]
        path: std::path::PathBuf,
    },
    /// コンフィグ関連のサブコマンドです
    Config{
        #[clap(subcommand)]
        conf: ConfigCommand
    },
    /// キャラ毎にセリフの文字数を集計します
    Count {
        #[clap(value_parser)]
        path: std::path::PathBuf,
    }
}

fn main() {
    let cli = Cli::parse();
    // コンフィグの初期化
    let mes_conf = if std::path::Path::new(&cli.conf).exists() {
        Config::builder()
            .add_source(File::with_name(&cli.conf))
            .build()
            .unwrap()
            .try_deserialize::<MeSBuilder>()
            .unwrap()
    } else {
        mes::builder::new()
    };

    // サブコマンドの解析
    match cli.command {
        Commands::Chat { path } => do_chat(path),
        Commands::Parse { path } => do_parse(path),
        Commands::Vtt { path } => do_vtt(path),
        Commands::Count { path } => do_count(path, mes_conf),
        Commands::Config { conf } => match conf {
            ConfigCommand::Create => do_config_create(cli.conf),
            ConfigCommand::Show => do_config_show(cli.conf, mes_conf),
        },
    }
}

fn do_parse(path: PathBuf) {
    let content = std::fs::read_to_string(path).expect("could not read file");
    let conf = mes::builder::new();
    let json = mes::parse_mes_to_json(&content, &conf);
    print!("{json}");
}

fn do_vtt(path: PathBuf) {
    let content = std::fs::read_to_string(path).expect("could not read file");
    let conf = mes::builder::new();
    let text = mes::get_vtt(&content, &conf);
    print!("{text}");
}

fn do_chat(_path: PathBuf) {
    println!("chat はまだ実装されていません");
}
fn do_count(path: PathBuf, conf: MeSBuilder) {
    let content = std::fs::read_to_string(path).expect("could not read file");
    let json = mes::count_dialogue_word_to_json_with_conf(content, &conf);
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

fn do_config_show(_path: String, mesconf: MeSBuilder) {
    let output = serde_json::to_string_pretty(&mesconf).expect("cannot serialize config");
    println!("{output}");
}