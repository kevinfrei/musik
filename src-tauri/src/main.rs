// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Look here: https://github.com/tauri-apps/tauri/issues/3543
// And maybe here: https://medium.com/@marm.nakamura/practice-rust-and-tauri-make-an-image-viewer-4-39623547b06d
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        // .register_uri_scheme_protocol(uri_scheme, protocol)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
