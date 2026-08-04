//! CLI exit-code / error-path smoke tests for the `mes-core` binary.

use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mes-core"))
}

fn tmp_path(name: &str) -> std::path::PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push(format!(
        "mes-core-cli-{}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
        name
    ));
    dir
}

#[test]
fn missing_input_file_exits_nonzero() {
    let missing = tmp_path("does-not-exist.mes");
    let output = bin()
        .args(["parse", missing.to_str().unwrap()])
        .output()
        .expect("spawn mes-core");

    assert!(
        !output.status.success(),
        "expected failure for missing input, got {:?}",
        output.status
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("could not read") || stderr.contains("error:"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn invalid_config_exits_nonzero() {
    let conf_path = tmp_path("bad.json");
    fs::write(&conf_path, "{ not valid json").expect("write bad config");

    let mes_path = tmp_path("ok.mes");
    fs::write(&mes_path, "@Alice\nこんにちは\n").expect("write mes");

    let output = bin()
        .args([
            "-c",
            conf_path.to_str().unwrap(),
            "parse",
            mes_path.to_str().unwrap(),
        ])
        .output()
        .expect("spawn mes-core");

    let _ = fs::remove_file(&conf_path);
    let _ = fs::remove_file(&mes_path);

    assert!(
        !output.status.success(),
        "expected failure for invalid config, got {:?}",
        output.status
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid config"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn missing_config_falls_back_to_defaults() {
    let missing_conf = tmp_path("absent-mes.json");
    let mes_path = tmp_path("ok.mes");
    fs::write(&mes_path, "title: t\n----\n@Alice\nこんにちは\n").expect("write mes");

    let output = bin()
        .args([
            "-c",
            missing_conf.to_str().unwrap(),
            "parse",
            mes_path.to_str().unwrap(),
        ])
        .output()
        .expect("spawn mes-core");

    let _ = fs::remove_file(&mes_path);

    assert!(
        output.status.success(),
        "missing config should use defaults; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Alice"), "stdout={stdout}");
}

#[test]
fn valid_partial_config_is_applied() {
    let conf_path = tmp_path("partial.json");
    fs::write(
        &conf_path,
        r#"{"mes_config":{"header_delimiter":"====\n"}}"#,
    )
    .expect("write config");

    let mes_path = tmp_path("custom-delim.mes");
    fs::write(
        &mes_path,
        "title: custom\n====\n@Alice\nこんにちは\n",
    )
    .expect("write mes");

    let output = bin()
        .args([
            "-c",
            conf_path.to_str().unwrap(),
            "parse",
            mes_path.to_str().unwrap(),
        ])
        .output()
        .expect("spawn mes-core");

    let _ = fs::remove_file(&conf_path);
    let _ = fs::remove_file(&mes_path);

    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Alice"), "stdout={stdout}");
    assert!(stdout.contains("title: custom"), "stdout={stdout}");
}

#[test]
fn config_create_can_replace_invalid_config() {
    let conf_path = tmp_path("invalid-to-recreate.json");
    fs::write(&conf_path, "{ not valid json").expect("write invalid config");

    let mut child = bin()
        .args([
            "-c",
            conf_path.to_str().unwrap(),
            "config",
            "create",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn mes-core");
    child
        .stdin
        .take()
        .expect("child stdin")
        .write_all(b"y\n")
        .expect("confirm overwrite");

    let output = child.wait_with_output().expect("wait for mes-core");
    assert!(
        output.status.success(),
        "config create should replace invalid config; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );

    let recreated = fs::read_to_string(&conf_path).expect("read recreated config");
    let parsed: serde_json::Value = serde_json::from_str(&recreated).expect("valid config JSON");
    let _ = fs::remove_file(&conf_path);

    assert_eq!(parsed["mes_config"]["header_delimiter"], "----\n");
}
