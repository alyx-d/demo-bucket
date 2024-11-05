use rodio::Sink;
use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};
pub struct Player {
    current_sink: Arc<Sink>,
    _stream: rodio::OutputStream,
    on_per_seconds: Option<fn(u64)>,
    singal_sender: Sender<u64>,
    singal_receiver: Receiver<u64>,
    controller: Arc<Mutex<Controller>>,
}

struct Controller {
    play_list: Vec<String>,
    current_index: usize,
}
impl Player {
    /**
     * create player instance <br/>
     * ```
     * use audio_test::Player;
     * let player = Player::new();
     * ```
     */
    pub fn new() -> Self {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        let (singal_sender, singal_receiver) = std::sync::mpsc::channel();
        Self {
            current_sink: Arc::new(sink),
            _stream, // 保存stream 确保它的生命周期与播放器对象相同，以保持音频输出设备的连接。
            on_per_seconds: None,
            singal_sender,
            singal_receiver,
            controller: Arc::new(Mutex::new(Controller {
                play_list: Vec::new(),
                current_index: 0,
            })),
        }
    }
    pub fn add_source(&self, path: &str) {
        self.controller
            .lock()
            .unwrap()
            .play_list
            .push(path.to_string());
    }

    pub fn len(&self) -> usize {
        return self.controller.lock().unwrap().play_list.len();
    }

    pub fn empty(&self) -> bool {
        return self.len() == 0;
    }
    pub fn play(&self) {
        if self.empty() {
            eprintln!("nothing to play");
            return;
        }
        let controller = self.controller.clone();
        let sink = self.current_sink.clone();
        let sender = self.singal_sender.clone();
        std::thread::spawn(move || loop {
            {
                let mut controller = controller.lock().unwrap();
                let play_list = controller.play_list.clone();
                let mut index = controller.current_index;
                let file = std::fs::File::open(play_list[index].as_str()).unwrap();
                let source = rodio::Decoder::new(std::io::BufReader::new(file)).unwrap();
                sink.append(source);
                sink.play();
                sender.send(1).unwrap();
                println!(
                    "Playing: {}",
                    std::path::Path::new(&play_list[index])
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                );
                index += 1;
                controller.current_index = index % play_list.len();
            }
            sink.sleep_until_end();
        });
    }
    pub fn pause(&self) {
        self.current_sink.pause();
    }
    pub fn resume(&self) {
        self.current_sink.play();
    }

    /**
     *  暂停播放 并移除
     */
    pub fn stop(&self) {
        self.current_sink.stop();
    }

    pub fn clear(&self) {
        let mut controller = self.controller.lock().unwrap();
        controller.play_list.clear();
    }

    pub fn set_on_per_seconds(&mut self, func: fn(u64)) {
        self.on_per_seconds = Some(func);
    }

    pub fn next(&mut self) {
        self.stop();
    }

    pub fn play_index(&self, index: usize) {
        let mut controller = self.controller.lock().unwrap();
        if index > controller.play_list.len() {
            println!("index out of range");
            return;
        }
        controller.current_index = index - 1;
        self.stop();
    }

    pub fn list(&self) {
        let controller = self.controller.lock().unwrap();
        for (i, path) in controller.play_list.iter().enumerate() {
            println!(
                "{}. {}",
                i + 1,
                std::path::Path::new(path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
            );
        }
    }

    pub fn prev(&self) {
        {
            let mut controller = self.controller.lock().unwrap();
            controller.current_index = if controller.current_index == 0 {
                controller.play_list.len() - 2
            } else if controller.current_index == 1 {
                controller.play_list.len() - 1
            } else {
                controller.current_index - 2
            };
        }
        self.stop();
    }

    pub fn set_speed(&self, speed: f32) {
        self.current_sink.set_speed(speed);
    }
    pub fn set_volume(&self, volume: f32) {
        self.current_sink.set_volume(volume);
    }

    pub fn get_pos(&self) -> u64 {
        self.current_sink.get_pos().as_secs()
    }

    pub fn wait(&self) {
        loop {
            self.singal_receiver.recv().unwrap();
            self.current_sink.sleep_until_end();
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
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
}
