use crate::error::Error;
use std::path::PathBuf;
use std::env::current_dir;
use std::fs::{read, File};
use std::io::prelude::*;

/// Builds an absolute path to `archive` with the `path` provided
/// by the request
pub fn get_full_path(from_request: &str) -> Result<PathBuf, Error> {
  let mut file_path = current_dir()?;
  
  file_path.push(PathBuf::from("archive"));
  file_path.push(PathBuf::from(from_request));

  Ok(file_path)
}

/// Reads the file provided in the `path` and returns its contents
/// as an `String`
pub fn read_file(path: PathBuf) -> Result<String, Error> {
  let file_contents = read(path)?;

  // Use lossy as we want to provide as many information as possible
  Ok(String::from_utf8_lossy(file_contents.as_slice()).to_string())
}

/// Create a new file or overwrites it if already exists
pub fn new_file(filename: PathBuf, contents: String) -> Result<String, Error> {
  let path = get_full_path(filename.to_str().unwrap())?;
  let mut file = File::create(path)?;

  file.write_all(contents.as_bytes())?;

  Ok(String::from("ok"))
}
