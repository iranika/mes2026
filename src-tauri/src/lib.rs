// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn mes_to_medo(text: &str) -> Result<String, String> {
    // Call into the workspace crate `mes-core` which exposes parsing helpers.
    // Return Err on panic or other failure.
    std::panic::catch_unwind(|| mes_core::parse_mes_to_json(text))
        .map_err(|e| format!("conversion failed: {:?}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, mes_to_medo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
