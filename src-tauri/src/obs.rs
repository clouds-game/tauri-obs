use std::{ffi::{CStr, CString}, path::Path};

use obs_wrapper::{obs_sys::{obs_initialized, obs_get_version_string, obs_startup, obs_video_info, obs_reset_video, OBS_VIDEO_SUCCESS, obs_add_data_path}, source::video::VideoFormat};


pub struct VideoInfo(obs_video_info);

impl VideoInfo {
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

  pub fn set_graphics_module(mut self, value: &'static str) -> Self {
    self.0.graphics_module = value.as_ptr() as *const _;
    self
  }

  pub fn set_fps(mut self, num: u32, den: u32) -> Self {
    self.0.fps_num = num;
    self.0.fps_den = den;
    self
  }

  pub fn set_base_size(mut self, width: u32, height: u32) -> Self {
    self.0.base_width = width;
    self.0.base_height = height;
    self
  }

  pub fn set_output_size(mut self, width: u32, height: u32) -> Self {
    self.0.output_width = width;
    self.0.output_height = height;
    self
  }

  pub fn set_output_format(mut self, format: VideoFormat) -> Self {
    self.0.output_format = match format {
      VideoFormat::Unknown => todo!(),
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
    };
    self
  }
}

pub fn initialized() -> bool {
  unsafe { obs_initialized() }
}

pub fn get_version_string() -> Result<&'static str, std::str::Utf8Error> {
  unsafe { CStr::from_ptr(obs_get_version_string()) }.to_str()
}

pub fn startup(locale: &str, module_config_path: Option<&str>) -> bool {
  let locale = CString::new(locale).unwrap();
  let path = match module_config_path {
    Some(s) => Some(CString::new(s).unwrap()),
    None => None,
  };
  unsafe { obs_startup(
    locale.as_ptr(),
    path.map(|i| i.as_ptr()).unwrap_or(std::ptr::null()),
    std::ptr::null_mut(),
  )}
}

pub fn add_data_path<P: AsRef<Path>>(path: P) {
  let path = CString::new(path.as_ref().to_string_lossy().to_string()).unwrap();
  unsafe { obs_add_data_path(path.as_ptr()) }
}

pub fn reset_video(info: &mut VideoInfo) -> Result<(), i32> {
  let ret = unsafe { obs_reset_video(&mut info.0) };
  if ret == OBS_VIDEO_SUCCESS as i32 {
    return Ok(())
  }
  Err(ret)
}
