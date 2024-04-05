use obs_wrapper::{obs_sys::{obs_scene_add, obs_scene_get_ref, obs_scene_get_source, obs_scene_release, obs_scene_t, obs_sceneitem_release, obs_sceneitem_t, obs_source_get_ref}, string::ObsString};

use super::source::SourceRef;

pub struct SceneRef {
  pointer: *mut obs_scene_t
}

impl std::fmt::Debug for SceneRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("SceneRef").field(&self.name().as_str()).field(&self.pointer).finish()
  }
}

impl Clone for SceneRef {
  fn clone(&self) -> Self {
    Self { pointer: unsafe { obs_scene_get_ref(self.pointer) } }
  }
}

impl Drop for SceneRef {
  fn drop(&mut self) {
    unsafe { obs_scene_release(self.pointer) }
  }
}

impl SceneRef {
  pub fn from_raw(pointer: *mut obs_scene_t) -> Self {
    Self { pointer }
  }

  pub fn name(&self) -> ObsString {
    self.as_source().name()
  }

  pub fn as_source(&self) -> SourceRef {
    let ptr = unsafe {
      // as doc said "The scene’s source. Does not increment the reference"
      // we should manually add_ref for it
      let ptr = obs_scene_get_source(self.pointer);
      obs_source_get_ref(ptr)
    };
    SourceRef::from_raw(ptr)
  }

  pub fn add_source(&self, source: &SourceRef) -> SceneItemRef {
    let ptr = unsafe { obs_scene_add(self.pointer, source.pointer) };
    SceneItemRef::from_raw(ptr)
  }
}

pub struct SceneItemRef {
  pointer: *mut obs_sceneitem_t
}

impl SceneItemRef {
  pub fn from_raw(pointer: *mut obs_sceneitem_t) -> Self {
    Self { pointer }
  }
}

impl Drop for SceneItemRef {
  fn drop(&mut self) {
    unsafe { obs_sceneitem_release(self.pointer) }
  }
}
