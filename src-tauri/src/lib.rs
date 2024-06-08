// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .plugin(db::setup_builder().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_app_dirs() -> directories::ProjectDirs {
    directories::ProjectDirs::from("com", env!("CARGO_PKG_AUTHORS"), env!("CARGO_PKG_NAME"))
        .unwrap()
}
