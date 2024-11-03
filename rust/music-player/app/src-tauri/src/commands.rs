#[tauri::command]
pub fn button1() -> String {
    "Hello, button1!".to_string()
}

#[tauri::command]
pub fn button2() -> String {
    "Hello, button2!".to_string()
}
