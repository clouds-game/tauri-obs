use obs_wrapper::{obs_sys::{obs_source_get_id, obs_source_get_name, obs_source_get_ref, obs_source_release, obs_source_t}, string::ObsString};

pub struct SourceRef {
  pub(crate) pointer: *mut obs_source_t,
}

impl std::fmt::Debug for SourceRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("SourceRef").field(&self.id().as_str()).field(&self.name().as_str()).field(&self.pointer).finish()
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
    let id = unsafe { std::ffi::CStr::from_ptr(id) };
    ObsString::Dynamic(id.to_owned())
  }

  pub fn name(&self) -> ObsString {
    let name = unsafe { obs_source_get_name(self.pointer) };
    let name = unsafe { std::ffi::CStr::from_ptr(name) };
    ObsString::Dynamic(name.to_owned())
  }
}
