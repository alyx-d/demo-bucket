pub mod events;
pub mod music_player;

pub use music_player::Player;
use tauri::AppHandle;

pub fn default_dirs() -> Vec<String> {
    let home_dir = std::env::var("USERPROFILE").unwrap();
    vec![
        home_dir.to_string() + "\\Music",
        home_dir.to_string() + "\\Downloads",
    ]
}

pub fn init_player(app: AppHandle) -> Player {
    let player = Player::new(app);
    player.play();
    player
}

pub fn secs_to_string(secs: u64) -> String {
    let min = secs / 60;
    let sec = secs % 60;
    let result = match min {
        _ if min > 0 => format!("{:02}:{:02}", min, sec),
        _ => format!("{:02}", sec),
    };
    return result;
}

pub fn byte_to_mb(byte: u64) -> String {
    format!("{:.2}MB", byte as f64 / 1024.0 / 1024.0)
}
