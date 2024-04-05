use obs_wrapper::obs_sys::{obs_get_module_name, obs_module_t};

use super::string::{DisplayExt, ObsString};

pub struct ModuleRef {
  pub(crate) pointer: *mut obs_module_t,
}

impl std::fmt::Debug for ModuleRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("ModuleRef").field(&self.name().display()).field(&self.pointer).finish()
  }
}

impl ModuleRef {
  pub fn from_raw(pointer: *mut obs_module_t) -> Option<Self> {
    if pointer.is_null() {
      None
    } else {
      Some(Self { pointer })
    }
  }

  pub fn name(&self) -> Option<ObsString> {
    unsafe {
      let name = obs_get_module_name(self.pointer);
      ObsString::from_raw(name)
    }
  }
}
