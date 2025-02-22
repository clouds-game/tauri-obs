// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use] extern crate tracing;

use std::{fs::DirEntry, path::Path};

use obs_wrapper::{data::DataObj, graphics::display::DisplayRef, media::video::VideoFormat};
use raw_window_handle::HasWindowHandle;
use tauri::AppHandle;

use crate::obs::Obs;

pub mod obs;
pub mod winit;

pub type Result<T, E=Error> = std::result::Result<T, E>;

#[derive(Debug, serde::Serialize)]
pub struct Error(String);

impl<T: ToString> From<T> for Error {
  fn from(e: T) -> Self {
    let message = e.to_string();
    warn!(ty=std::any::type_name::<T>(), %message, "error");
    Self(message)
  }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(_name: &str) -> String {
  match init_obs().inspect_err(|e| error!(error=%e, "when init_obs")) {
    Ok(_) => format!("obs inited"),
    _ => format!("init failed"),
  }
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
async fn list_profile(folder: Option<&str>) -> Result<ProfileResult> {
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

fn init_obs() -> Result<Obs, obs::Error> {
  // https://github.com/lulzsun/libobs-sharp/blob/main/libobs-sharp.example/Program.cs
  // https://github.com/eyalcohen4/obs-headless-poc/blob/master/src/main.cpp
  // https://docs.obsproject.com/frontends
  info!(obs_version=Obs::version()?);
  let mut obs = Obs::new();
  info!(obs_initalized=obs.ready());
  if !obs.ready() {
    obs.init("en_US")?;
    info!(obs_initalized=obs.ready());
    obs.add_default_module_path("~/Applications/OBS-test.app/Contents")?;
    obs.add_default_module_path("~/Library/Application Support/obs-studio")?;
    let module = obs.load_modules([
      "mac-capture",
      "image-source",
    ])?;
    //obs_scene_add
    info!(?module, "module loaded");
    // let data_path = std::env::current_dir().unwrap().join("../target/Frameworks/libobs.framework");
    // println!("resource exists: {} -> {}", data_path.to_string_lossy(), data_path.exists());
    // obs::add_data_path(data_path);
  }
  let video_info = obs::VideoSetting::new()
    .with_graphics_module(obs::GraphicsModule::OpenGL)
    .with_fps(30000, 1000)
    .with_base_size(1920, 1080)
    .with_output_size(1920, 1080)
    .with_output_format(VideoFormat::I420);
  obs.set_channel_source(0, None);
  obs.reset_video(video_info)?;

  let scene = obs.create_scene("main")?;
  debug!(?obs, scene=?scene.as_source());
  obs.set_channel_source(0, Some(scene.as_source()));
  debug!(source=?obs.get_channel_source(0));
  let setting = obs::settings::image_source::ColorSetting::default();
  let setting_data = DataObj::from_json(serde_json::to_string(&setting)?).unwrap();
  debug!(?setting, data=%setting_data.get_json().unwrap());
  let source = obs.create_source("capture 1", "color_source_v3", setting_data)?;
  scene.add_source(source.clone());
  info!(?source, "inited");
  Ok(obs)
}

#[tauri::command]
fn create_display(app: AppHandle) -> Result<()> {
  // let mut obs = Obs::new();
  // let display = obs.create_display(info)?;
  let window = app.create_tao_window(|| ("display".to_string(), {
    tao::window::WindowBuilder::new()
      .with_title("Hello World")
      .with_inner_size(tao::dpi::LogicalSize::new(400, 320))
  }))?.upgrade().unwrap();
  let window_id = window.id();
  info!(?window_id, "window created");
  let handle = window.window_handle()?;

  let display = crate::winit::create_display(handle, (window.inner_size().width, window.inner_size().height))?;
  struct DisplayBox(std::sync::Arc<std::sync::Mutex<Option<DisplayRef>>>);
  unsafe impl Send for DisplayBox {}
  impl DisplayBox { fn drop(&self) { self.0.lock().unwrap().take(); } }
  let display = DisplayBox(std::sync::Arc::new(std::sync::Mutex::new(Some(display))));
  app.send_tao_window_event(window_id, tauri_runtime_wry::WindowMessage::AddEventListener(100, Box::new(move |event| {
    match event {
      tauri_runtime::window::WindowEvent::Destroyed => { display.drop(); },
      tauri_runtime::window::WindowEvent::Resized(size) => {
        let mut display = display.0.lock().unwrap();
        if let Some(display) = display.as_mut() {
          display.set_size(size.width, size.height);
        }
      },
      tauri_runtime::window::WindowEvent::Moved(_) => {},
      _ => {
        debug!(?event, "event");
      },
    }
  })))?;
  Ok(())
}

fn main() {
  tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_file(true).with_line_number(true).compact().init();

  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_store::Builder::default().build())
    .invoke_handler(tauri::generate_handler![
      greet,
      list_profile,
      create_display,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
