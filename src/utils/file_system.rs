use crate::error::Error;
use std::path::PathBuf;
use std::env::current_dir;
use std::fs::{read, write};

/// Builds an absolute path to `archive` with the `path` provided
/// by the request
pub fn get_full_path(from_request: String) -> Result<PathBuf, Error> {
  let mut file_path = current_dir()?;
  
  file_path.push(PathBuf::from("archive"));
  file_path.push(PathBuf::from(from_request.as_str()));

  Ok(file_path)
}

/// Reads the file provided in the `path` and returns its contents
/// as an `String`
pub fn read_file(path: PathBuf) -> Result<String, Error> {
  let file_contents = read(path)?;

  // Use lossy as we want to provide as many information as possible
  Ok(String::from_utf8_lossy(file_contents.as_slice()).to_string())
}

pub fn new_file(filename: String, contents: String) -> Result<String, Error> {
  let path = get_full_path(filename)?;
  
  write(path, contents.as_bytes())?;

  Err(Error::new(400, "Unable to write a new file @{}"))
}
