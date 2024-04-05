use obs_wrapper::obs_sys::{obs_data_t, obs_source_get_id, obs_source_get_name, obs_source_get_ref, obs_source_release, obs_source_t};

use super::{settings, string::{DisplayExt, ObsString}};

#[allow(non_camel_case_types)]
pub enum SourceSettings {
  Win_GameCapture(settings::win_game_capture::Setting),
}

impl SourceSettings {
  pub fn build(self) -> obs_data_t {
    todo!()
  }
}

pub struct SourceRef {
  pub(crate) pointer: *mut obs_source_t,
}

impl std::fmt::Debug for SourceRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("SourceRef").field(&self.id().display()).field(&self.name().display()).field(&self.pointer).finish()
  }
}

impl Clone for SourceRef {
  fn clone(&self) -> Self {
    Self { pointer: unsafe { obs_source_get_ref(self.pointer) } }
  }
}

impl Drop for SourceRef {
  fn drop(&mut self) {
    unsafe { obs_source_release(self.pointer) }
  }
}

impl SourceRef {
  pub fn from_raw(pointer: *mut obs_source_t) -> Option<Self> {
    if pointer.is_null() {
      None
    } else {
      Some(Self { pointer })
    }
  }

  pub fn id(&self) -> ObsString {
    let id = unsafe { obs_source_get_id(self.pointer) };
    ObsString::from_raw(id).expect("obs_source_get_id")
  }

  pub fn name(&self) -> ObsString {
    let name = unsafe { obs_source_get_name(self.pointer) };
    ObsString::from_raw(name).expect("obs_source_get_name")
  }
}
