use std::sync::Mutex;

use tauri::{ipc::IpcResponse, State};

use crate::player::{default_dirs, music_player::FileInfoVec, Player};

#[tauri::command]
pub fn button1() -> String {
    "Hello, button1!".to_string()
}

#[tauri::command]
pub fn button2() -> String {
    default_dirs().join(",")
}

#[tauri::command]
pub fn get_default_dirs() -> String {
    default_dirs().join(",")
}

#[tauri::command]
pub fn player_scan_dirs(player: State<Mutex<Player>>, dirs: Vec<String>) -> usize {
    let player = player.lock().unwrap();
    let len = player.len();
    for dir in dirs {
        player.scan_dir(dir.as_str());
    }
    player.play();
    player.len() - len
}

#[tauri::command]
pub fn player_play(player: State<Mutex<Player>>) {
    player.lock().unwrap().play();
}

#[tauri::command]
pub fn player_pause(player: State<Mutex<Player>>) {
    player.lock().unwrap().pause();
}

#[tauri::command]
pub fn player_resume(player: State<Mutex<Player>>) {
    player.lock().unwrap().resume();
}

#[tauri::command]
pub fn player_next(player: State<Mutex<Player>>) {
    player.lock().unwrap().next();
}

#[tauri::command]
pub fn player_prev(player: State<Mutex<Player>>) {
    player.lock().unwrap().prev();
}

#[tauri::command]
pub fn player_set_volume(player: State<Mutex<Player>>, volume: f32) {
    player.lock().unwrap().set_volume(volume);
}

#[tauri::command]
pub fn player_set_speed(player: State<Mutex<Player>>, speed: f32) {
    player.lock().unwrap().set_speed(speed);
}

#[tauri::command]
pub fn player_list(player: State<Mutex<Player>>) -> FileInfoVec {
    let result = FileInfoVec(player.lock().unwrap().list());
    let clone = result.clone();
    if let tauri::ipc::InvokeResponseBody::Json(json) = clone.body().unwrap() {
        println!("json: {:?}", json);
    }
    result
}

#[tauri::command]
pub fn player_play_index(player: State<Mutex<Player>>, index: usize) {
    player.lock().unwrap().play_index(index);
}
