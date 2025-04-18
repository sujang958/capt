mod captures;

#[tauri::command]
fn get_displays() -> String {
    serde_json::to_string(&captures::video::get_displays()).unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_displays])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
