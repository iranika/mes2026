// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn mes_to_medo(text: &str) -> Result<String, String> {
    std::panic::catch_unwind(|| mes_core::parse_mes_to_json(text))
        .map_err(|e| format!("conversion failed: {:?}", e))
}

#[tauri::command]
fn mes_to_vtt(text: &str) -> Result<String, String> {
    std::panic::catch_unwind(|| mes_core::get_vtt(text))
        .map_err(|e| format!("vtt conversion failed: {:?}", e))
}

#[tauri::command]
fn mes_word_count(text: &str) -> Result<String, String> {
    std::panic::catch_unwind(|| mes_core::count_dialogue_word_to_json(text))
        .map_err(|e| format!("word count failed: {:?}", e))
}

#[tauri::command]
fn mes_to_chat(text: &str) -> Result<String, String> {
    std::panic::catch_unwind(|| mes_core::get_chat(text))
        .map_err(|e| format!("chat conversion failed: {:?}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            mes_to_medo,
            mes_to_vtt,
            mes_word_count,
            mes_to_chat
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
