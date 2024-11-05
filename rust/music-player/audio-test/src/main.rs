use audio_test::Player;

#[allow(dead_code)]
fn home_dir() -> String {
    let home_dir = std::env::var("USERPROFILE").unwrap();
    home_dir
}

#[allow(dead_code)]
fn read_audioes(player: &mut Player) {
    let dir = home_dir() + "\\Music\\mp3";
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let path = path.to_str().unwrap();
            if path.ends_with(".mp3") {
                player.add_source(path);
            }
        }
    }
}

#[allow(dead_code)]
fn play_test() {
    let mut player = Player::new();
    read_audioes(&mut player);
    player.set_on_per_seconds(|_delta| {
        println!("delta: {}", _delta);
    });
    player.play();

    loop {
        println!("Commands: quit, pause, resume, next");
        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf).unwrap();
        match buf.trim() {
            "quit" => break,
            "pause" => player.pause(),
            "resume" => player.resume(),
            "prev" => player.prev(),
            "next" => player.next(),
            _ => println!("unknown command"),
        }
    }
}

fn main() {
    play_test();
}
