use crate::error::Error;
use std::path::{Path, PathBuf};
use std::env::current_dir;
use std::fs::{read_to_string, File, write};
use std::io::prelude::*;

/// Builds an absolute path to `archive` with the `path` provided
/// by the request
pub fn get_full_path(from_request: PathBuf) -> Result<PathBuf, Error> {
  let mut cwd = current_dir()?;
  
  cwd.push(PathBuf::from("archive"));
  cwd.push(PathBuf::from(from_request));

  Ok(cwd)
}

/// Reads the file provided in the `path` and returns its contents
/// as an `String`
pub fn read_file(path: PathBuf) -> Result<String, Error> {
  let file_contents = read_to_string(path)?;


  // Use lossy as we want to provide as many information as possible
  Ok(file_contents)
}

/// Create a new file or overwrites it if already exists
pub fn create_file(filename: PathBuf, contents: String) -> Result<String, Error> {
  let path = get_full_path(filename)?;
  let mut file = File::create(path)?;
  let bytes = contents.as_bytes();

  file.write_all(bytes)?;

  Ok(String::from_utf8(bytes.to_vec())?)
}

/// Overwrites the file.
///
/// To avoid creating new files (an unexpected behaviour in case of an update),
/// the path provided is checked first. If the the file doesn't exists then the
/// `Result` will lead to `Err`, otherwise writes contents on the file and
/// leads to an `Ok` wrapping an `String` with the final contents of the file
pub fn update_file(filename: PathBuf, contents: String) -> Result<String, Error> {
  // check if the file exists
  if Path::new(&filename).exists() {
    let path = get_full_path(filename)?;

    write(&path, contents)?;

    return Ok(read_to_string(path)?);
  }

  Err(Error::new(404, &format!("File: \"{}\", doesn't exists", filename.to_str().unwrap())))
}
