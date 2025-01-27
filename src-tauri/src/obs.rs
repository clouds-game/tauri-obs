pub mod settings;
pub mod display;

use std::{ffi::{CStr, CString}, path::Path};

use obs_wrapper::{
  data::DataObj, graphics::display::{Color, DisplayRef}, media::video::VideoFormat, module::ModuleRef, obs_sys::{obs_add_data_path, obs_add_module_path, obs_add_safe_module, obs_display_create, obs_get_module, obs_get_output_source, obs_get_version_string, obs_initialized, obs_load_all_modules, obs_post_load_modules, obs_reset_video, obs_scene_create, obs_set_output_source, obs_source_create, obs_startup, obs_video_info, MAX_CHANNELS, OBS_VIDEO_SUCCESS}, source::{scene::SceneRef, SourceRef}, string::TryIntoObsString as _, wrapper::PtrWrapper as _
};

use self::display::DisplayInitInfo;

pub type Result<T, E=Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("convert string error")]
  String(#[from] std::str::Utf8Error),
  #[error("convert cstring error")]
  CString(#[from] std::ffi::NulError),
  #[error("convert json error")]
  Json(#[from] serde_json::Error),
  #[error("error code: {0}")]
  Code(i32),
  #[error("ffi error: {0}")]
  NulPointer(&'static str),
  #[error("obs error: {0}")]
  Obs(#[from] obs_wrapper::Error),
}

macro_rules! try_with {
  ($expr:expr) => {
    {
      let ret: i32 = unsafe { $expr };
      match ret as _ {
        OBS_VIDEO_SUCCESS => Ok(()),
        _ => Err(Error::Code(ret))
      }
    }
  };
}

#[derive(Debug)]
pub struct Obs {
  scenes: Vec<SceneRef>,
  marker: std::marker::PhantomData<*mut std::ffi::c_void>,
}

pub enum GraphicsModule {
  OpenGL, D3D11,
}

impl GraphicsModule {
  #[cfg(target_os = "macos")]
  pub fn as_str(self) -> &'static str {
    match self {
      GraphicsModule::OpenGL => "libobs-opengl.dylib\0",
      GraphicsModule::D3D11 => todo!(),
    }
  }

  #[cfg(not(target_os = "macos"))]
  pub fn as_str(self) -> &'static str {
    match self {
      GraphicsModule::OpenGL => "libobs-opengl\0",
      GraphicsModule::D3D11 => "libobs-d3d11\0",
    }
  }
}

pub struct VideoSetting(obs_video_info);

impl VideoSetting {
  pub fn new() -> Self {
    Self(obs_video_info {
      graphics_module: std::ptr::null(),
      fps_num: 0,
      fps_den: 0,
      base_width: 0,
      base_height: 0,
      output_width: 0,
      output_height: 0,
      output_format: 0,
      adapter: 0,
      gpu_conversion: false,
      colorspace: 0,
      range: 0,
      scale_type: 0,
    })
  }

  pub fn with_graphics_module(mut self, value: GraphicsModule) -> Self {
    self.0.graphics_module = value.as_str().as_ptr() as *const _;
    self
  }

  pub fn with_fps(mut self, num: u32, den: u32) -> Self {
    self.0.fps_num = num;
    self.0.fps_den = den;
    self
  }

  pub fn with_base_size(mut self, width: u32, height: u32) -> Self {
    self.0.base_width = width;
    self.0.base_height = height;
    self
  }

  pub fn with_output_size(mut self, width: u32, height: u32) -> Self {
    self.0.output_width = width;
    self.0.output_height = height;
    self
  }

  pub fn with_output_format(mut self, format: VideoFormat) -> Self {
    self.0.output_format = match format {
      VideoFormat::None => 0,
      VideoFormat::I420 => 1,
      VideoFormat::NV12 => 2,
      VideoFormat::YVYU => 3,
      VideoFormat::YUY2 => 4,
      VideoFormat::UYVY => 5,
      VideoFormat::RGBA => 6,
      VideoFormat::BGRA => 7,
      VideoFormat::BGRX => 8,
      VideoFormat::Y800 => 9,
      VideoFormat::I444 => 10,
      VideoFormat::BGR3 => 11,
      VideoFormat::I422 => 12,
      VideoFormat::I40A => 13,
      VideoFormat::I42A => 14,
      VideoFormat::YUVA => 15,
      VideoFormat::AYUV => 16,
      VideoFormat::I010 => todo!(),
      VideoFormat::P010 => todo!(),
      VideoFormat::I210 => todo!(),
      VideoFormat::I412 => todo!(),
      VideoFormat::YA2L => todo!(),
    };
    self
  }
}

impl Obs {
  pub fn new() -> Self {
    // the inner ptr is nothing but a marker that
    // Obs is not `Send`
    Self {
      scenes: Vec::new(),
      marker: std::marker::PhantomData::default(),
    }
  }

  pub fn ready(&self) -> bool {
    unsafe { obs_initialized() }
  }

  pub fn version() -> Result<&'static str> {
    Ok(unsafe { CStr::from_ptr(obs_get_version_string()) }.to_str()?)
  }

  pub fn init_with_config<P: AsRef<Path>>(&mut self, locale: &str, module_config_path: P) -> Result<()> {
    self.init_internal(locale, Some(module_config_path))
  }

  pub fn init(&mut self, locale: &str) -> Result<()> {
    self.init_internal(locale, None::<&str>)
  }

  pub fn init_internal<P: AsRef<Path>>(&mut self, locale: &str, module_config_path: Option<P>) -> Result<()> {
    let locale = CString::new(locale)?;
    let path = match module_config_path {
      Some(s) => Some(s.as_ref().try_into_obs_string()?),
      None => None,
    };
    let result = unsafe { obs_startup(
      locale.as_ptr(),
      path.map(|i| i.as_ptr()).unwrap_or(std::ptr::null()),
      std::ptr::null_mut(),
    )};
    if result {
      Ok(())
    } else {
      Err(Error::Code(-1))
    }
  }

  pub fn add_data_path<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
    let path = path.as_ref().try_into_obs_string()?;
    unsafe { obs_add_data_path(path.as_ptr()) };
    Ok(())
  }

  /// Add default module path for OBS.
  /// prefix for
  ///   macOS: `/path/to/OBS.app/Contents`, as you could found `OBS.app/Contents/MacOS/OBS`
  ///   windows: `path/to/obs-studio`, as you could found `obs-studio/bin/64bit/obs64.exe`
  ///   linux: `/path/to/prefix`, as you could found `bin/obs`
  ///
  /// in macOS: https://github.com/obsproject/obs-studio/blob/80ad63a6da6a932c04364b30173b880cd765d5ec/libobs/obs-cocoa.m#L43
  /// path     = `OBS.app/Contents/PlugIns/%module%.plugin/Contents/MacOS/`
  /// data_dir = `OBS.app/Contents/PlugIns/%module%.plugin/Contents/Resources/`
  ///
  /// in windows:
  /// `../../obs-plugins/64bit`, `../../data/obs-plugins/%module%`
  ///
  /// in linux: https://github.com/obsproject/obs-studio/blob/80ad63a6da6a932c04364b30173b880cd765d5ec/libobs/obs-nix.c#L74
  /// OBS_INSTALL_PREFIX: `{OBS_INSTALL_PREFIX}/obs-plugins`, `{OBS_INSTALL_PREFIX}/../../data/obs-plugins/%module%`
  /// OBS_RELATIVE_PREFIX: `../../obs-plugins/64bit`, `../../data/obs-plugins/%module%`
  /// FLATPAK_PLUGIN_PATH: `/app/plugins/obs-plugins`, `/app/plugins/share/obs/obs-plugins/%module%`
  ///
  pub fn add_default_module_path<P: AsRef<Path>>(&mut self, prefix: P) -> Result<()> {
    let prefix = prefix.as_ref().to_string_lossy();
    let prefix = match prefix.strip_prefix("~/") {
      Some(s) => format!("{}/{}", dirs::home_dir().unwrap().to_string_lossy(), s),
      _ => prefix.to_string()
    }.replace("$HOME", &dirs::home_dir().unwrap().to_string_lossy());
    #[cfg(target_os = "macos")] {
      let bin_dir = format!("{}/{}", prefix, "PlugIns/%module%.plugin/Contents/MacOS");
      let data_dir = format!("{}/{}", prefix, "PlugIns/%module%.plugin/Contents/Resources");
      self.add_module_path(bin_dir, data_dir)?;
    }
    #[cfg(target_os = "windows")] {
      let bin_dir = format!("{}/{}", prefix, "obs-plugins/64bit");
      let data_dir = format!("{}/{}", prefix, "data/obs-plugins/%module%");
      self.add_module_path(bin_dir, data_dir)?;
    }
    #[cfg(target_os = "linux")] {
      let bin_dir = format!("{}/{}", prefix, "lib/obs-plugins");
      let bin_dir = format!("{}/{}", prefix, "share/obs/obs-plugins/%module%");
      self.add_module_path(bin_dir, data_dir)?;
      let bin_dir = format!("{}/{}", prefix, "obs-plugins/64bit");
      let bin_dir = format!("{}/{}", prefix, "data/obs-plugins/%module%");
      self.add_module_path(bin_dir, data_dir)?;
    }
    Ok(())
  }
  pub fn add_module_path<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, bin_dir: P, data_dir: Q) -> Result<()> {
    let bin_dir = CString::new(bin_dir.as_ref().to_string_lossy().as_bytes())?;
    let data_dir = CString::new(data_dir.as_ref().to_string_lossy().as_bytes())?;
    unsafe { obs_add_module_path(bin_dir.as_ptr(), data_dir.as_ptr()) }
    Ok(())
  }
  /// this won't work since it loads frontend-tools
  pub fn load_modules<S: AsRef<str>, I: IntoIterator<Item = S>>(&self, names: I) -> Result<Vec<ModuleRef>> {
    let names = names.into_iter().map(|i| i.as_ref().to_string()).collect::<Vec<_>>();
    // trick: add safe module first
    // but we could not delete module
    // TODO: use find_modules
    for name in &names {
      let name_cstr = CString::new(name.to_string())?;
      unsafe {
        obs_add_safe_module(name_cstr.as_ptr());
      }
    }
    unsafe {
      // `obs_load_all_modules` would first `obs_open_module` then call `obs_init_module`
      // if init failed, it would `free_module`
      obs_load_all_modules();
      // `obs_post_load_modules` would call `module->post_load()`
      obs_post_load_modules();
    };
    names.iter().map(|i|
      self.get_module(i)
    ).collect()
  }
  pub fn get_module(&self, name: &str) -> Result<ModuleRef> {
    let name = CString::new(name.to_string())?;
    let ptr = unsafe { obs_get_module(name.as_ptr()) };
    Ok(ModuleRef::from_raw(ptr)?)
  }

  /// You should first call `.init()`.
  /// `reset_video` would fail if any output is active.
  /// According to the doc, there's no way "`reset_audio`"
  /// but a fully shutdown is required.
  pub fn reset_video(&mut self, mut info: VideoSetting) -> Result<()> {
    try_with!{ obs_reset_video(&mut info.0) }
  }

  pub fn create_scene(&mut self, name: &str) -> Result<SceneRef> {
    let name_c = CString::new(name.to_string()).unwrap();
    let scene = unsafe {
      let ptr = obs_scene_create(name_c.as_ptr());
      SceneRef::from_raw_unchecked(ptr).ok_or(Error::NulPointer("obs_scene_create"))?
    };
    self.scenes.push(scene.clone());
    Ok(scene)
  }

  pub fn create_source(&mut self, name: &str, type_: &str, settings: DataObj) -> Result<SourceRef> {
    let name = CString::new(name.to_string())?;
    let type_ = CString::new(type_.to_string())?;
    unsafe {
      let ptr = obs_source_create(type_.as_ptr(), name.as_ptr(), settings.as_ptr_mut(), std::ptr::null_mut());
      // TODO: check ptr valid since create failed won't return null
      // https://github.com/obsproject/obs-studio/blob/80ad63a6da6a932c04364b30173b880cd765d5ec/libobs/obs-source.c#L400-L401
      SourceRef::from_raw_unchecked(ptr).ok_or(Error::NulPointer("obs_source_create"))
    }
  }

  pub fn set_channel_source(&mut self, channel: usize, source: Option<SourceRef>) {
    if channel >= MAX_CHANNELS as usize {
      return
    }
    unsafe {
      let source = source.map(|i| i.as_ptr_mut()).unwrap_or_else(std::ptr::null_mut);
      // auto release the old source, add_ref for the new source
      obs_set_output_source(channel as _, source)
    }
  }
  pub fn get_channel_source(&self, channel: usize) -> Option<SourceRef> {
    if channel >= MAX_CHANNELS as usize {
      return None
    }
    // Use `obs_source_release` to release.
    unsafe {
      SourceRef::from_raw_unchecked(obs_get_output_source(channel as _))
    }
  }

  pub fn create_display(&mut self, info: &DisplayInitInfo, color: Color) -> Result<DisplayRef> {
    let color = color.as_format(info.color_format());
    unsafe {
      let ptr = obs_display_create(&info.inner as *const display::sys::gs_init_data as *const _, color);
      DisplayRef::from_raw_unchecked(ptr).ok_or(Error::NulPointer("obs_display_create"))
    }
  }
}
