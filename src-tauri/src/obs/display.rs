use obs_wrapper::{graphics::GraphicsColorFormat, obs_sys::gs_zstencil_format};

use self::sys::gs_init_data;

#[allow(non_camel_case_types)]
pub mod sys {
  use obs_wrapper::obs_sys::{gs_color_format, gs_zstencil_format};

  #[repr(C)]
  #[derive(Debug, Copy, Clone)]
  pub struct gs_window {
    #[cfg(target_os = "windows")]
    /// void *hwnd;
    pub hwnd: *mut ::std::os::raw::c_void,
    #[cfg(target_os = "macos")]
    /// ```c
    /// pub type id = *mut objc_object;
    /// __unsafe_unretained id view;
    /// ```
    pub view: *mut ::std::os::raw::c_void,
    #[cfg(target_os = "linux")]
    /// uint32_t id;
    pub id: u32,
    #[cfg(target_os = "linux")]
    /// void *display;
    pub display: *mut ::std::os::raw::c_void,
  }

  #[repr(C)]
  #[derive(Debug, Copy, Clone)]
  pub struct gs_init_data {
      pub window: gs_window,
      pub cx: u32,
      pub cy: u32,
      pub num_backbuffers: u32,
      pub format: gs_color_format,
      pub zsformat: gs_zstencil_format,
      pub adapter: u32,
  }
  impl Default for gs_init_data {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
  }
}


pub struct DisplayInitInfo {
  pub inner: gs_init_data
}

impl DisplayInitInfo {
  pub fn new(cx: u32, cy: u32) -> Self  {
    let mut inner = gs_init_data::default();
    inner.cx = cx;
    inner.cy = cy;
    Self { inner }
  }

  pub fn with_color_format(mut self, format: GraphicsColorFormat) -> Self {
    self.inner.format = format.as_raw();
    self
  }

  pub fn with_zstencil_format(mut self, format: gs_zstencil_format) -> Self {
    self.inner.zsformat = format;
    self
  }

  pub fn build(mut self, handle: raw_window_handle::WindowHandle<'_>) -> Self {
    // static DisplayContext CreateDisplay(NSView *view)
    // info.window.view = view;
    #[cfg(target_os = "macos")] {
      self.inner.window.view = match handle.as_raw() {
        raw_window_handle::RawWindowHandle::AppKit(mut view) => unsafe { view.ns_view.as_mut() as *mut _ },
        _ => panic!("invalid handle"),
      };
      return self
    }
    #[allow(unreachable_code)] {
      unreachable!("unsupported platform")
    }
  }
}
