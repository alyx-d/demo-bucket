use rodio::{Decoder, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::thread;
pub struct Player {
    play_list: Vec<String>,
    current_sink: Arc<Mutex<Sink>>,
    _stream: rodio::OutputStream,
}
impl Player {
    pub fn new() -> Self {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        Self {
            play_list: Vec::new(),
            current_sink: Arc::new(Mutex::new(sink)),
            _stream,
        }
    }
    pub fn add_source(&mut self, path: &str) {
        self.play_list.push(path.to_string());
    }
    pub fn play(&self) {
        let play_list = self.play_list.clone();
        let sink_clone = Arc::clone(&self.current_sink);
        thread::spawn(move || {
            let sink = sink_clone.lock().unwrap();
            for path in play_list {
                let file = File::open(path).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();
                sink.append(source);
            }
            sink.play();
            println!("Playing...");
        })
        .join()
        .unwrap();
    }
    pub fn pause(&self) {
        self.current_sink.lock().unwrap().pause();
    }
    pub fn resume(&self) {
        self.current_sink.lock().unwrap().play();
    }

    pub fn wait(&self) {
        let sink = self.current_sink.lock().unwrap();
        sink.sleep_until_end();
    }
}
#[allow(dead_code)]
pub fn play() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let file = std::fs::File::open(
        "C:\\dev\\java\\demo\\rust\\music-player\\audio\\素颜-许嵩何曼婷.320.mp3",
    )
    .unwrap();
    let source = rodio::Decoder::new(std::io::BufReader::new(file)).unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    sink.append(source);
    sink.play();
    sink.sleep_until_end();
}
