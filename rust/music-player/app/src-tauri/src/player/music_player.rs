use id3::{Tag, TagLike};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::{read_dir, File};
use std::io::BufReader;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::ipc::{InvokeResponseBody, IpcResponse};
use tauri::{AppHandle, Emitter};

use crate::player::events::PlayerEvents;

use super::{byte_to_mb, secs_to_string};

pub struct Player {
    app: AppHandle,
    sink: Arc<Sink>,
    _output_stream: OutputStream,
    controller: Arc<Mutex<Controller>>,
    is_running: Arc<AtomicBool>,
    add_len: Arc<AtomicUsize>,
    signal: Arc<Mutex<Option<Sender<usize>>>>,
}

struct Controller {
    play_list: Vec<FileInfo>,
    current_index: usize,
}

#[derive(Clone, Debug)]
pub struct FileInfo {
    path: String,
    total_duration: String,
    size: u64,
    mb: String,
    title: String,
    artist: String,
    album: String,
}

impl IpcResponse for FileInfo {
    fn body(self) -> tauri::Result<tauri::ipc::InvokeResponseBody> {
        // remove control char
        let re = regex::Regex::new(r"[\u0000-\u001F]").unwrap();
        Ok(tauri::ipc::InvokeResponseBody::Json(format!(
            "{{\"path\":\"{}\",\"totalDuration\":\"{}\",
            \"size\":{},\"mb\":\"{}\",\"title\":\"{}\",\"artist\":\"{}\",\"album\":\"{}\"}}",
            re.replace_all(&self.path, "").replace("\\", "\\\\"),
            self.total_duration,
            self.size,
            self.mb,
            re.replace_all(&self.title, ""),
            re.replace_all(&self.artist, ""),
            re.replace_all(&self.album, "")
        )))
    }
}

#[derive(Clone)]
pub struct FileInfoVec(pub Vec<FileInfo>);

impl IpcResponse for FileInfoVec {
    fn body(self) -> tauri::Result<tauri::ipc::InvokeResponseBody> {
        Ok(tauri::ipc::InvokeResponseBody::Json(format!(
            "[{}]",
            self.0
                .iter()
                .map(|info| match info.clone().body() {
                    Ok(InvokeResponseBody::Json(json)) => json,
                    _ => "null".to_string(),
                })
                .collect::<Vec<String>>()
                .join(",")
        )))
    }
}

unsafe impl Send for Player {}

impl Player {
    pub fn new(app: AppHandle) -> Self {
        let (_output_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self {
            app,
            sink: Arc::new(sink),
            _output_stream,
            is_running: Arc::new(AtomicBool::new(false)),
            add_len: Arc::new(AtomicUsize::new(0)),
            controller: Arc::new(Mutex::new(Controller {
                play_list: vec![],
                current_index: 0,
            })),
            signal: Arc::new(Mutex::new(None)),
        }
    }

    pub fn add_source(&self, file_path: &str) {
        let mut controller = self.controller.lock().unwrap();
        if controller.play_list.iter().any(|info| {
            return info.path == file_path;
        }) {
            return;
        }
        if let Ok(file) = File::open(file_path) {
            println!("add source {}", file_path);
            let size = file.metadata().unwrap().len();
            let source = Decoder::new(BufReader::new(file)).unwrap();
            let tag = id3::Tag::read_from_path(file_path).unwrap_or(Tag::default());
            let title = tag.title().unwrap_or("未知歌曲");
            let artist = tag.artist().unwrap_or("未知歌手");
            let album = tag.album().unwrap_or("未知专辑");
            controller.play_list.push(FileInfo {
                path: file_path.to_string(),
                // FIXME: total_duration 读取的总时长不准确
                total_duration: secs_to_string(source.total_duration().unwrap().as_secs()),
                title: title.to_string(),
                artist: artist.to_string(),
                album: album.to_string(),
                size,
                mb: byte_to_mb(size),
            });
            self.add_len.fetch_add(1, Ordering::SeqCst);
        } else {
            println!("{} file_path not exists", file_path);
        }
    }

    pub fn scan_dir(&self, dir_path: &str) {
        for entry in read_dir(dir_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                if path.extension().unwrap() == "mp3" {
                    self.add_source(path.to_str().unwrap());
                    println!("add source done");
                }
            } else {
                self.scan_dir(path.to_str().unwrap());
            }
        }
    }

    pub fn set_add_len(&self, len: usize) {
        self.add_len.store(len, Ordering::SeqCst);
    }

    pub fn add_len(&self) -> usize {
        self.add_len.load(Ordering::SeqCst)
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn play(&self) {
        let sink = self.sink.clone();
        let controller = self.controller.clone();
        let app_handle = self.app.clone();
        let is_running = self.is_running.clone();
        let signal = self.signal.clone();
        std::thread::spawn(move || {
            is_running.store(true, Ordering::SeqCst);
            let (sender, receiver) = channel::<usize>();
            {
                signal.lock().unwrap().replace(sender.clone());
            }
            loop {
                let mut index;
                {
                    println!("wait signal");
                    index = receiver.recv().unwrap();
                    println!("got signal {}", index);
                }
                {
                    let mut controller = controller.lock().unwrap();
                    if controller.play_list.is_empty() {
                        println!("play_list is empty");
                        continue;
                    }
                    index %= controller.play_list.len();
                    controller.current_index = index;
                    let file = File::open(controller.play_list[index].path.as_str());
                    let file_name = Path::new(&controller.play_list[index].path)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap();
                    println!("is playing: {}", file_name);
                    if let Ok(file) = file {
                        let source = Decoder::new(BufReader::new(file)).unwrap();
                        sink.append(source);
                        sink.play();
                        app_handle.emit(PlayerEvents::Play.as_str(), index).unwrap();
                    } else {
                        println!("file not exists {}", file_name);
                        app_handle
                            .emit(PlayerEvents::FileNoExists.as_str(), index)
                            .unwrap();
                        controller.play_list.remove(index);
                    }
                }
                sink.sleep_until_end();
                println!("play sec pos {}", sink.get_pos().as_secs());
                println!("play end");
                app_handle
                    .emit(PlayerEvents::PlayEnd.as_str(), index)
                    .unwrap();
            }
        });
    }

    pub fn play_index(&self, index: usize) {
        println!("play_index {}", index);
        if !self.sink.empty() {
            self.stop();
        }
        let signal = self.signal.lock().unwrap();
        signal.clone().unwrap().send(index).unwrap();
        println!("send signal {}", index);
    }

    pub fn seek(&self, pos: Duration) {
        self.sink.try_seek(pos).unwrap();
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn get_pos(&self) -> Duration {
        self.sink.get_pos()
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn pause(&self) {
        let controller = self.controller.lock().unwrap();
        let index = controller.current_index;
        self.sink.pause();
        self.app.emit(PlayerEvents::Pause.as_str(), index).unwrap();
        println!("player pause");
    }

    pub fn resume(&self) {
        self.sink.play();
        let controller = self.controller.lock().unwrap();
        self.app
            .emit(PlayerEvents::Resume.as_str(), controller.current_index)
            .unwrap();
        println!("player resume");
    }

    pub fn next(&self) {
        let controller = self.controller.lock().unwrap();
        self.play_index(controller.current_index + 1);
    }

    pub fn prev(&self) {
        let mut controller = self.controller.lock().unwrap();
        controller.current_index = match controller.current_index {
            0 => controller.play_list.len() - 1,
            _ => controller.current_index - 1,
        };
        self.play_index(controller.current_index);
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn set_speed(&self, speed: f32) {
        self.sink.set_speed(speed);
    }

    pub fn len(&self) -> usize {
        self.controller.lock().unwrap().play_list.len()
    }

    pub fn empty(&self) -> bool {
        self.len() == 0
    }

    pub fn list(&self) -> Vec<FileInfo> {
        let controller = self.controller.lock().unwrap();
        let result = controller.play_list.iter().cloned().collect();
        // println!("list: {:?}", result);
        result
    }
}
