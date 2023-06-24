// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use obs_wrapper::source::video::VideoFormat;

pub mod obs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
  init_obs();
  format!("Hello, {}! You've been greeted from Rust!", name)
}

fn init_obs() {
  // https://github.com/lulzsun/libobs.NET/blob/main/obs_net.example/Program.cs
  // https://github.com/eyalcohen4/obs-headless-poc/blob/master/src/main.cpp
  println!("obs_version: {}", obs::get_version_string().unwrap());
  println!("obs_initalized: {}", obs::initialized());
  if !obs::initialized() {
    obs::startup("en_US", None);
    println!("obs_initalized: {}", obs::initialized());
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
  println!("inited");
}

fn main() {
  // init_obs();
  tauri::Builder::default()
    .plugin(tauri_plugin_store::Builder::default().build())
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
