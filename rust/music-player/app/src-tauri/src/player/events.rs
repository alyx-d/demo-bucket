pub enum PlayerEvents {
    Play,
    Pause,
    Resume,
    // per audio tragger
    PlayEnd,
    FileNoExists,
}

impl PlayerEvents {
    pub fn as_str(&self) -> &'static str {
        match self {
            PlayerEvents::Play => "player_play",
            PlayerEvents::Pause => "player_pause",
            PlayerEvents::Resume => "player_resume",
            PlayerEvents::PlayEnd => "player_play_end",
            PlayerEvents::FileNoExists => "player_file_no_exists",
        }
    }
}
