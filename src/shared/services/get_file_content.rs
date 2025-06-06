use crate::types::Errors;
use crate::types::Result;
use std::fs;

#[allow(dead_code)]
pub fn get_file_content(path: &str) -> Result<String> {
  fs::read_to_string(path).map_err(|error| Errors::new(Errors::Filesystem, Some(error.to_string())))
}
