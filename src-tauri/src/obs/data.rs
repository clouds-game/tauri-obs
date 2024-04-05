use std::ffi::{CStr, CString};

use obs_wrapper::obs_sys::{obs_data_addref, obs_data_create, obs_data_create_from_json, obs_data_get_bool, obs_data_get_int, obs_data_get_json, obs_data_get_string, obs_data_release, obs_data_set_bool, obs_data_set_int, obs_data_set_string, obs_data_t};

use super::Result;

pub struct DataRef {
  pointer: *mut obs_data_t,
}

impl std::fmt::Debug for DataRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("DataRef").field(&self.pointer).finish()
  }
}

impl Clone for DataRef {
  fn clone(&self) -> Self {
    unsafe {
      obs_data_addref(self.pointer)
    };
    Self { pointer: self.pointer }
  }
}

impl Drop for DataRef {
  fn drop(&mut self) {
    unsafe { obs_data_release(self.pointer) }
  }
}

impl DataRef {
  pub fn from_raw(pointer: *mut obs_data_t) -> Option<Self> {
    if pointer.is_null() {
      None
    } else {
      Some(Self { pointer })
    }
  }

  pub fn new() -> Self {
    let pointer = unsafe { obs_data_create() };
    Self::from_raw(pointer).expect("obs_data_create")
  }

  pub fn set_string(&self, key: &str, value: &str) -> Result<()> {
    let key = CString::new(key)?;
    let value = CString::new(value)?;
    unsafe {
      obs_data_set_string(self.pointer, key.as_ptr(), value.as_ptr());
    }
    Ok(())
  }

  pub fn get_string(&self, key: &str) -> Result<String> {
    let key = CString::new(key).expect("CString::new");
    let result = unsafe {
      obs_data_get_string(self.pointer, key.as_ptr())
    };
    Ok(unsafe { CStr::from_ptr(result) }.to_str()?.to_string())
  }

  pub fn set_int(&self, key: &str, value: i64) -> Result<()> {
    let key = CString::new(key)?;
    unsafe {
      obs_data_set_int(self.pointer, key.as_ptr(), value)
    };
    Ok(())
  }
  pub fn get_int(&self, key: &str) -> Result<i64> {
    let key = CString::new(key)?;
    let result = unsafe {
      obs_data_get_int(self.pointer, key.as_ptr())
    };
    Ok(result)
  }

  pub fn set_bool(&self, key: &str, value: bool) -> Result<()> {
    let key = CString::new(key)?;
    unsafe {
      obs_data_set_bool(self.pointer, key.as_ptr(), value)
    }
    Ok(())
  }
  pub fn get_bool(&self, key: &str) -> Result<bool> {
    let key = CString::new(key)?;
    let result = unsafe {
      obs_data_get_bool(self.pointer, key.as_ptr())
    };
    Ok(result)
  }

  pub fn from_value<T: serde::Serialize>(value: &T) -> Result<Self> {
    let json = serde_json::to_value(&value)?;
    let data = Self::new();
    let Some(map) = json.as_object() else {
      unimplemented!("unknown type {}", json)
    };
    for (key, value) in map {
      match value {
        serde_json::Value::String(s) => {
          data.set_string(key, s)?;
        }
        serde_json::Value::Number(n) => {
          data.set_int(key, n.as_i64().expect("as_i64"))?;
        }
        serde_json::Value::Bool(b) => {
          data.set_bool(key, *b)?;
        }
        serde_json::Value::Null => {
          debug!(key, "null value skipped");
        }
        _ => todo!("unknown type {}", value)
      }
    }
    Ok(data)
  }

  pub fn load(json: &str) -> Option<Self> {
    let json = CString::new(json).expect("CString::new");
    let pointer = unsafe { obs_data_create_from_json(json.as_ptr()) };
    Self::from_raw(pointer)
  }

  pub fn dump(&self) -> Result<String> {
    unsafe {
      // this buffer is managed by `self.pointer`, no need for addref/release
      let buffer = obs_data_get_json(self.pointer);
      let result = CStr::from_ptr(buffer).to_string_lossy().to_string();
      Ok(result)
    }
  }
}
