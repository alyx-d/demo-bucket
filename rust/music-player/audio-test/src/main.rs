use audio_test::Player;

#[allow(dead_code)]
fn home_dir() -> String {
    let home_dir = std::env::var("USERPROFILE").unwrap();
    home_dir
}

#[allow(dead_code)]
fn read_audios(player: &mut Player) {
    player.clear();
    let dirs = vec![home_dir() + "\\Music\\mp3", "c:\\CloudMusic".to_string()];
    fn add_source(player: &mut Player, dir: &str) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().unwrap() == "mp3" {
                player.add_source(path.to_str().unwrap());
            }
        }
    }
    for dir in dirs {
        add_source(player, dir.as_str());
    }
}

#[allow(dead_code)]
fn play_test() {
    let mut player = Player::new();
    read_audios(&mut player);
    player.set_on_per_seconds(|_delta| {
        println!("delta: {}", _delta);
    });
    player.play();

    loop {
        println!("Commands: quit, pause, resume, next, scan, list, play <index>");
        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf).unwrap();
        match buf.trim() {
            "quit" => break,
            "pause" => player.pause(),
            "resume" => player.resume(),
            "prev" => player.prev(),
            "next" => player.next(),
            "scan" => {
                read_audios(&mut player);
                player.list();
            }
            "list" => player.list(),
            _ if buf.starts_with("play ") => {
                let it = buf
                    .split(" ")
                    .nth(1)
                    .unwrap()
                    .trim()
                    .parse::<usize>()
                    .unwrap_or_default();
                player.play_index(it);
            }
            _ => println!("unknown command"),
        }
    }
}

fn main() {
    play_test();
}
