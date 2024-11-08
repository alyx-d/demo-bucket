use id3::{Tag, TagLike};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::{read_dir, File};
use std::io::BufReader;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
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
            controller: Arc::new(Mutex::new(Controller {
                play_list: vec![],
                current_index: 0,
            })),
        }
    }

    pub fn add_source(&self, file_path: &str) {
        let mut controller = self.controller.lock().unwrap();
        if controller
            .play_list
            .iter()
            .any(|info| info.path == file_path)
        {
            return;
        }
        if let Ok(file) = File::open(file_path) {
            let size = file.metadata().unwrap().len();
            let source = Decoder::new(BufReader::new(file)).unwrap();
            let tag = id3::Tag::read_from_path(file_path).unwrap_or(Tag::default());
            let title = tag.title().unwrap_or("未知歌曲");
            let artist = tag.artist().unwrap_or("未知歌手");
            let album = tag.album().unwrap_or("未知专辑");
            controller.play_list.push(FileInfo {
                path: file_path.to_string(),
                total_duration: secs_to_string(source.total_duration().unwrap().as_secs()),
                title: title.to_string(),
                artist: artist.to_string(),
                album: album.to_string(),
                size,
                mb: byte_to_mb(size),
            });
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
                }
            } else {
                self.scan_dir(path.to_str().unwrap());
            }
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn play(&self) {
        let sink = self.sink.clone();
        let controller = self.controller.clone();
        let app_handle = self.app.clone();
        let is_running = self.is_running.clone();
        std::thread::spawn(move || {
            loop {
                {
                    let mut controller = controller.lock().unwrap();
                    if controller.play_list.is_empty() {
                        break;
                    }
                    let mut index = controller.current_index;
                    let file = File::open(controller.play_list[index].path.as_str());
                    let file_name = Path::new(&controller.play_list[index].path)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap();
                    println!("file_name: {}", file_name);
                    if let Ok(file) = file {
                        let source = Decoder::new(BufReader::new(file)).unwrap();
                        sink.append(source);
                        sink.play();
                        is_running.store(true, Ordering::SeqCst);
                    } else {
                        println!("file not exists {}", file_name);
                    }
                    app_handle.emit(PlayerEvents::Play.as_str(), index).unwrap();
                    index += 1;
                    controller.current_index = index % controller.play_list.len();
                }
                sink.sleep_until_end();
            }
            is_running.store(false, Ordering::SeqCst);
        });
    }

    pub fn play_index(&self, index: usize) {
        let mut controller = self.controller.lock().unwrap();
        controller.current_index = index;
        self.stop();
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
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.sink.play();
    }

    pub fn next(&self) {
        let mut controller = self.controller.lock().unwrap();
        controller.current_index += 1;
        self.stop();
    }

    pub fn prev(&self) {
        let mut controller = self.controller.lock().unwrap();
        controller.current_index = match controller.current_index {
            0 => controller.play_list.len() - 2,
            1 => controller.play_list.len() - 1,
            _ => controller.current_index - 2,
        };
        self.stop();
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
