use std::{
  ffi::OsString,
  path::{Path},
};

pub fn name(name: OsString) -> String {
  name.to_str().unwrap().to_string()
}

pub fn path(path: &Path, parent_path: Option<&str>) -> String {
  if let Some(parent) = parent_path {
    return path
      .strip_prefix(parent)
      .unwrap()
      .to_str()
      .unwrap()
      .to_string();
  }
  path.to_str().unwrap().to_string()
}
