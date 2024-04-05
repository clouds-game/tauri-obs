use std::{borrow::Cow, ffi::CString, path::Path};

use super::Result;

pub struct DisplayStr<'a, T>(&'a T);

impl<'a> std::fmt::Display for DisplayStr<'a, ObsString> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.to_string_lossy())
  }
}
impl<'a> std::fmt::Display for DisplayStr<'a, Option<ObsString>> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.0 {
      Some(s) => write!(f, "{}", s.to_string_lossy()),
      None => write!(f, "<none>"),
    }
  }
}
impl<'a> std::fmt::Debug for DisplayStr<'a, ObsString> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.0.to_string_lossy())
  }
}
impl<'a> std::fmt::Debug for DisplayStr<'a, Option<ObsString>> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.0 {
      Some(s) => write!(f, "{:?}", s.to_string_lossy()),
      None => write!(f, "<none>"),
    }
  }
}

pub trait DisplayExt: Sized {
  fn display(&self) -> DisplayStr<'_, Self> {
    DisplayStr(self)
  }
}

impl DisplayExt for ObsString {}
impl DisplayExt for Option<ObsString> {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObsString {
  String(String),
  CString(std::ffi::CString),
}

impl From<String> for ObsString {
  fn from(s: String) -> Self {
    Self::String(s)
  }
}

impl From<CString> for ObsString {
  fn from(s: CString) -> Self {
    Self::CString(s)
  }
}

impl ObsString {
  pub fn as_str(&self) -> Result<&str> {
    Ok(match self {
      Self::String(s) => s,
      Self::CString(s) => s.to_str()?,
    })
  }

  pub fn to_string_lossy(&self) -> Cow<'_, str> {
    match self {
      Self::String(s) => Cow::Borrowed(s),
      Self::CString(s) => s.to_string_lossy(),
    }
  }

  pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
    let path = path.as_ref();
    let s = path.to_string_lossy().to_string();
    Ok(Self::String(s))
  }

  pub fn from_raw(ptr: *const std::os::raw::c_char) -> Option<Self> {
    if ptr.is_null() {
      None
    } else {
      let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };
      Some(Self::CString(cstr.to_owned()))
    }
  }

  pub fn from_str(s: &str) -> Self {
    Self::String(s.to_string())
  }

  pub fn to_cstring(self) -> Result<CString> {
    match self {
      Self::String(s) => Ok(CString::new(s)?),
      Self::CString(s) => Ok(s),
    }
  }
}
