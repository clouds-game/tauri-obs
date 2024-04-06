use std::time::Duration;

use obs_wrapper::graphics::{display::DisplayRef, GraphicsColorFormat};
use raw_window_handle::WindowHandle;

use crate::obs::{display::DisplayInitInfo, Obs};

pub fn create_display(handle: WindowHandle<'_>) -> crate::Result<DisplayRef> {
  let mut obs = Obs::new();
  let info = DisplayInitInfo::new(1920, 1080)
    .with_color_format(GraphicsColorFormat::RGBA)
    .build(handle);
  let display = obs.create_display(&info, 0xFF00FF00)?;
  std::thread::sleep(Duration::from_millis(5000));
  Ok(display)
}
