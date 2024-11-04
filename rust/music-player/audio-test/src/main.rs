use std::thread;

use audio_test::Player;

#[allow(dead_code)]
fn play_test() {
    let mut player = Player::new();
    player.add_source("C:\\dev\\java\\demo\\rust\\music-player\\audio\\牧马城市-毛不易.mp3");
    player.add_source("C:\\dev\\java\\demo\\rust\\music-player\\audio\\素颜-许嵩何曼婷.320.mp3");
    player.play();
    player.wait();
}

fn main() {
    play_test();
}
