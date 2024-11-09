pub enum PlayerEvents {
    Play,
    FileNoExists,
}

impl PlayerEvents {
    pub fn as_str(&self) -> &'static str {
        match self {
            PlayerEvents::Play => "player_play",
            PlayerEvents::FileNoExists => "player_file_no_exists",
        }
    }
}
