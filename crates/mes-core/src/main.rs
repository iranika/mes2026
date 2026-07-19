use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use config::{Config, File};
use mes_core::error::{MesError, MesResult};
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
    /// パース結果を正規化した MeS として再出力します（ラウンドトリップ）.
    Emit {
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

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> MesResult<()> {
    let cli = Cli::parse();
    let mes_conf = load_config(&cli.conf)?;

    match cli.command {
        Commands::Chat { path } => do_chat(path, &mes_conf),
        Commands::Parse { path } => do_parse(path, &mes_conf),
        Commands::Vtt { path } => do_vtt(path, &mes_conf),
        Commands::Emit { path } => do_emit(path, &mes_conf),
        Commands::Count { path } => do_count(path, &mes_conf),
        Commands::Config { conf } => match conf {
            ConfigCommand::Create => do_config_create(cli.conf),
            ConfigCommand::Show => do_config_show(&mes_conf),
        },
    }
}

fn load_config(path: &str) -> MesResult<MeSBuilder> {
    if !std::path::Path::new(path).exists() {
        return Ok(mes::builder::new());
    }

    // Prefer deep-merge of the file JSON over defaults so partial configs work.
    let raw = std::fs::read_to_string(path)?;
    match mes::builder::merge_json_conf(&raw) {
        Ok(conf) => Ok(conf),
        Err(err) => {
            eprintln!("warning: failed to merge config ({err}); trying typed deserialize");
            Ok(Config::builder()
                .add_source(File::with_name(path))
                .build()
                .ok()
                .and_then(|c| c.try_deserialize::<MeSBuilder>().ok())
                .unwrap_or_else(mes::builder::new))
        }
    }
}

fn read_input(path: PathBuf) -> MesResult<String> {
    std::fs::read_to_string(&path).map_err(|err| {
        MesError::new(format!("could not read {}: {err}", path.display()))
    })
}

fn do_parse(path: PathBuf, conf: &MeSBuilder) -> MesResult<()> {
    let content = read_input(path)?;
    let json = mes::parse_mes_to_json(&content, conf)?;
    print!("{json}");
    Ok(())
}

fn do_vtt(path: PathBuf, conf: &MeSBuilder) -> MesResult<()> {
    let content = read_input(path)?;
    let text = mes::get_vtt(&content, conf)?;
    print!("{text}");
    Ok(())
}

fn do_chat(path: PathBuf, conf: &MeSBuilder) -> MesResult<()> {
    let content = read_input(path)?;
    let text = mes::get_chat(&content, conf)?;
    print!("{text}");
    Ok(())
}

fn do_emit(path: PathBuf, conf: &MeSBuilder) -> MesResult<()> {
    let content = read_input(path)?;
    let medo = mes::parse_mes(&content, conf)?;
    print!("{}", medo.to_mes_string(conf));
    Ok(())
}

fn do_count(path: PathBuf, conf: &MeSBuilder) -> MesResult<()> {
    let content = read_input(path)?;
    let json = mes::count_dialogue_word_to_json_with_conf(content, conf)?;
    println!("{json}");
    Ok(())
}

fn do_config_create(path: String) -> MesResult<()> {
    let def_conf = mes::builder::new();
    let json = serde_json::to_string_pretty(&def_conf)?;
    let filepath = std::path::Path::new(&path);
    if filepath.exists() {
        let answer = Question::new("すでにファイルが存在します。上書きしますか？").confirm();
        if answer == Answer::NO {
            return Ok(());
        }
    }
    std::fs::write(&path, json).map_err(|err| {
        MesError::new(format!("cannot write config {}: {err}", filepath.display()))
    })?;
    Ok(())
}

fn do_config_show(mesconf: &MeSBuilder) -> MesResult<()> {
    let output = serde_json::to_string_pretty(mesconf)?;
    println!("{output}");
    Ok(())
}
