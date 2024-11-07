pub enum PlayerEvents {
    Play,
}

impl PlayerEvents {
    pub fn as_str(&self) -> &'static str {
        match self {
            PlayerEvents::Play => "player_play",
        }
    }
}
