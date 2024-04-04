// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use] extern crate tracing;

use std::{fs::DirEntry, path::Path};

use obs_wrapper::media::video::VideoFormat;

pub mod obs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
  init_obs();
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg(target_os = "macos")]
const OBS_SETTING_FOLDER: &str = "$HOME/Library/Application Support/obs-studio";

fn obs_setting_folder() -> String {
  OBS_SETTING_FOLDER.replace("$HOME", &dirs::home_dir().unwrap().display().to_string())
}

fn ignore_file(i: &DirEntry) -> bool {
  i.file_name().eq_ignore_ascii_case(".DS_Store")
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct KV {
  name: String,
  value: serde_json::Value,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProfileResult {
  pub scenes: Vec<KV>,
  pub profiles: Vec<KV>,
}

#[tauri::command]
async fn list_profile(folder: Option<&str>) -> Result<ProfileResult, String> {
  let setting_dir = folder.map(str::to_string).unwrap_or_else(||obs_setting_folder());
  info!(setting_dir);
  let mut result = ProfileResult::default();
  if let Ok(read_dir) = std::fs::read_dir(Path::new(&setting_dir).join("basic/profiles")) {
    for i in read_dir {
      let Ok(i) = i else { continue };
      if ignore_file(&i) { continue }
      result.profiles.push(KV {
        name: i.file_name().to_string_lossy().to_string(),
        value: serde_json::Value::Null,
      });
    }
  }
  if let Ok(read_dir) = std::fs::read_dir(Path::new(&setting_dir).join("basic/scenes")) {
    for i in read_dir {
      let Ok(i) = i else { continue };
      if ignore_file(&i) { continue }
      if i.file_name().to_string_lossy().ends_with(".json") || i.file_name().to_string_lossy().ends_with(".json.bak") {
        let Ok(content) = std::fs::read_to_string(i.path()) else { continue };
        let Ok(data) = serde_json::from_str(&content) else { continue };
        result.scenes.push(KV {
          name: i.file_name().to_string_lossy().to_string(),
          value: data,
        });
      }
    }
  }
  Ok(result)
}

fn init_obs() {
  // https://github.com/lulzsun/libobs.NET/blob/main/obs_net.example/Program.cs
  // https://github.com/eyalcohen4/obs-headless-poc/blob/master/src/main.cpp
  info!(obs_version=obs::get_version_string().unwrap());
  info!(obs_initalized=obs::initialized());
  if !obs::initialized() {
    obs::startup("en_US", None);
    info!(obs_initalized=obs::initialized());
    // let data_path = std::env::current_dir().unwrap().join("../target/Frameworks/libobs.framework");
    // println!("resource exists: {} -> {}", data_path.to_string_lossy(), data_path.exists());
    // obs::add_data_path(data_path);
  }
  let mut video_info = obs::VideoInfo::new()
    .set_graphics_module(obs::GraphicsModule::OpenGL)
    .set_fps(30000, 1000)
    .set_base_size(1920, 1080)
    .set_output_size(1920, 1080)
    .set_output_format(VideoFormat::I420);
  obs::reset_video(&mut video_info).unwrap();
  info!("inited");
}

fn main() {
  tracing_subscriber::fmt().with_file(true).with_line_number(true).compact().init();
  // init_obs();
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_store::Builder::default().build())
    .invoke_handler(tauri::generate_handler![
      greet,
      list_profile,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
