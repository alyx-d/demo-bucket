#![allow(dead_code)]

use std::{path::PathBuf, sync::Mutex};

use tauri::Manager;

mod commands;
mod player;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Folder {
                        path: PathBuf::from("./logs"),
                        file_name: Some("logs".to_string()),
                    },
                ))
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app.manage(Mutex::new(player::init_player(app.handle().clone())));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::button1,
            commands::button2,
            commands::get_default_dirs,
            commands::player_scan_dirs,
            commands::player_play_index,
            commands::player_pause,
            commands::player_resume,
            commands::player_next,
            commands::player_prev,
            commands::player_set_volume,
            commands::player_set_speed,
            commands::player_list,
            commands::player_seek,
            commands::player_get_pos,
            commands::player_is_paused,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
