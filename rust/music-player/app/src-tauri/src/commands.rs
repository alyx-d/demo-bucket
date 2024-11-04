use crate::player;

#[tauri::command]
pub fn button1() -> String {
    "Hello, button1!".to_string()
}

#[tauri::command]
pub fn button2() -> String {
    player::music_player::play();
    return "start play".to_string();
}
