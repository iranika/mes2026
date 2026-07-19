// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn mes_to_medo(text: &str) -> Result<String, String> {
    mes_core::parse_mes_to_json(text).map_err(|e| e.to_string())
}

#[tauri::command]
fn mes_to_vtt(text: &str) -> Result<String, String> {
    mes_core::get_vtt(text).map_err(|e| e.to_string())
}

#[tauri::command]
fn mes_word_count(text: &str) -> Result<String, String> {
    mes_core::count_dialogue_word_to_json(text).map_err(|e| e.to_string())
}

#[tauri::command]
fn mes_to_chat(text: &str) -> Result<String, String> {
    mes_core::get_chat(text).map_err(|e| e.to_string())
}

#[tauri::command]
fn mes_emit(text: &str) -> Result<String, String> {
    mes_core::emit_mes(text).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            mes_to_medo,
            mes_to_vtt,
            mes_word_count,
            mes_to_chat,
            mes_emit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
