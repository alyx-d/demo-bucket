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
    format!("{:02}:{:02}", min, sec)
}

pub fn string_to_secs(time: String) -> u32 {
    let arr: Vec<&str> = time.split(":").collect();
    let min = arr[0].parse::<u32>().unwrap();
    let sec = arr[1].parse::<u32>().unwrap();
    min * 60 + sec
}
pub fn byte_to_mb(byte: u64) -> String {
    format!("{:.2}MB", byte as f64 / 1024.0 / 1024.0)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_secs_to_string() {
        assert_eq!(secs_to_string(0), "00:00");
        assert_eq!(secs_to_string(60), "01:00");
        assert_eq!(secs_to_string(120), "02:00");
        assert_eq!(secs_to_string(3600), "60:00");
    }
}
