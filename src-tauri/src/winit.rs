use obs_wrapper::graphics::{display::DisplayRef, GraphicsColorFormat};
use raw_window_handle::WindowHandle;

use crate::obs::{display::DisplayInitInfo, Obs};

pub fn create_display(handle: WindowHandle<'_>, size: (u32, u32)) -> crate::Result<DisplayRef> {
  let mut obs = Obs::new();
  let info = DisplayInitInfo::new(size.0, size.1)
    .with_color_format(GraphicsColorFormat::RGBA)
    .build(handle);
  let display = obs.create_display(&info, 0xFF80FF80)?;
  Ok(display)
}
