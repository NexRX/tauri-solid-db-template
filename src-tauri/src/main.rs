// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{fs::create_dir_all, path::PathBuf};

use once_cell::sync::Lazy;
use tracing::info;

extern crate tracing;
mod db;

static CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| match cfg!(debug_assertions) {
    true => PathBuf::from(format!( 
        "{}/target/debug/config",
        env!("CARGO_MANIFEST_DIR")
    )),
    false => get_app_dirs().config_dir().to_path_buf(),
});

// Learn more about Tauri commands at https://v2.tauri.app/develop/plugins/#adding-commands
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // Setup logging and Config Folder
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!(CONFIG_DIR = ?&*CONFIG_DIR, "Starting app");
    create_dir_all(&*CONFIG_DIR).expect("Could not create config dir");

    // Create DB and Pool
    let db_pool = tauri::async_runtime::block_on(async {
        db::create_if_none().await.unwrap();
        db::new_pool()
    });

    // Build Tauri Application
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(db::setup_builder().build())
        .invoke_handler({
            tauri_specta::ts::builder()
                .commands(tauri_specta::collect_commands![
                    greet,
                    db::greetings::get_all_grettings,
                    db::greetings::create_greeting,
                    db::greetings::delete_greeting
                ]) // <- Your commands here
                .path("../src/bindings.ts")
                .build()
                .unwrap()
        })
        .manage(db_pool)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_app_dirs() -> directories::ProjectDirs {
    directories::ProjectDirs::from("com", env!("CARGO_PKG_AUTHORS"), env!("CARGO_PKG_NAME"))
        .unwrap()
}
