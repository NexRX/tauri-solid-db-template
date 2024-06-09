// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::path::PathBuf;

mod db;

lazy_static::lazy_static! {
    static ref CONFIG_DIR: PathBuf = get_app_dirs().config_dir().to_path_buf();
}

// Learn more about Tauri commands at https://v2.tauri.app/develop/plugins/#adding-commands
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(db::setup_builder().build())
        .invoke_handler({
            let builder = tauri_specta::ts::builder().commands(tauri_specta::collect_commands![
                greet,
                db::greetings::get_all_grettings,
                db::greetings::create_greeting,
                db::greetings::delete_greeting
            ]); // <- Each of your commands

            #[cfg(debug_assertions)]
            let builder = builder.path("../src/bindings.ts");

            builder.build().unwrap()
        })
        .manage(db::new_pool())
        .setup(|_| {
            tauri::async_runtime::block_on(db::create_if_none())?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_app_dirs() -> directories::ProjectDirs {
    directories::ProjectDirs::from("com", env!("CARGO_PKG_AUTHORS"), env!("CARGO_PKG_NAME"))
        .unwrap()
}
