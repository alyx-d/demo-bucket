#[allow(dead_code)]
pub fn greet() -> String {
    "Hello, music_player!".to_string()
}

pub fn play() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = std::io::BufReader::new(
        std::fs::File::open(
            "C:\\dev\\java\\demo\\rust\\music-player\\audio\\素颜-许嵩何曼婷.320.mp3",
        )
        .unwrap(),
    );

    let source = rodio::Decoder::new(file).unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.set_volume(1.0);
    sink.play();
}
