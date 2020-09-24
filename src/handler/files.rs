use crate::utils::file_system;
use actix_web::{HttpResponse, Responder, Result};
use actix_web::web::Path;

pub fn read_file(path: Path<(String,)>) -> Result<impl Responder> {
  let path: String = path.0.to_string();
  let path = file_system::get_full_path(path);
  let contents = file_system::read_file(path?)?;

  Ok(HttpResponse::Ok()
    .content_type("text/plain")
    .body(contents))
}
