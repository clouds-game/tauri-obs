use obs_wrapper::{obs_sys::{obs_scene_add, obs_scene_get_ref, obs_scene_get_source, obs_scene_release, obs_scene_t, obs_sceneitem_addref, obs_sceneitem_release, obs_sceneitem_t, obs_sceneitem_visible}, source::SourceRef, string::{DisplayExt as _, ObsString}, wrapper::PtrWrapper as _};

use super::Result;

pub struct SceneRef {
  pointer: *mut obs_scene_t
}

impl std::fmt::Debug for SceneRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("SceneRef").field(&self.name().display()).field(&self.pointer).finish()
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
  pub fn from_raw(pointer: *mut obs_scene_t) -> Option<Self> {
    if pointer.is_null() {
      None
    } else {
      Some(Self { pointer })
    }
  }

  pub fn name(&self) -> Result<ObsString> {
    Ok(self.as_source().name()?)
  }

  pub fn as_source(&self) -> SourceRef {
    let ptr = unsafe {
      // as doc said "The scene’s source. Does not increment the reference"
      // we should manually add_ref for it
      obs_scene_get_source(self.pointer)
    };
    SourceRef::from_raw(ptr).expect("obs_scene_get_source")
  }

  pub fn add_source(&self, source: SourceRef) -> SceneItemRef {
    let ptr = unsafe {
      let ptr = obs_scene_add(self.pointer, source.as_ptr_mut());
      // add ref for source, Docs said "A new scene item for a source within a scene.  Does not
      // increment the reference"
      obs_sceneitem_addref(ptr);
      ptr
    };
    SceneItemRef::from_raw(ptr).expect("obs_scene_add")
  }
}

pub struct SceneItemRef {
  pointer: *mut obs_sceneitem_t
}

impl Drop for SceneItemRef {
  fn drop(&mut self) {
    unsafe { obs_sceneitem_release(self.pointer) }
  }
}

impl SceneItemRef {
  pub fn from_raw(pointer: *mut obs_sceneitem_t) -> Option<Self> {
    if pointer.is_null() {
      None
    } else {
      Some(Self { pointer })
    }
  }

  pub fn visible(&self) -> bool {
    unsafe { obs_sceneitem_visible(self.pointer) }
  }
}
