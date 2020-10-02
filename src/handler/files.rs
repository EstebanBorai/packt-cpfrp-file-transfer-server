use crate::utils::file_system;
use crate::error::Error;
use actix_web::{HttpResponse, Responder, Result};
use actix_web::web::{BytesMut, Path, Payload};
use futures::{Future, Stream};

pub fn read_file(path: Path<(String,)>) -> Result<impl Responder> {
  let path = path.0.as_str();
  let path = file_system::get_full_path(path);
  let contents = file_system::read_file(path?)?;

  Ok(HttpResponse::Ok()
    .content_type("text/plain")
    .body(contents))
}

pub fn create_file(path: Path<(String,)>, body: Payload) -> impl Future<Item = HttpResponse, Error = Error> {
  let path = path.0.as_str();
  let path = file_system::get_full_path(path);

  body.map_err(Error::from)
    .fold(BytesMut::new(), move |mut body, chunk| {
      body.extend_from_slice(&chunk);

      Ok::<_, Error>(body)
    })
    .and_then(|body| {
      let body = body.to_vec();
      let body = body.as_slice();
      let body = String::from_utf8_lossy(body);

      match file_system::new_file(path?, body.to_string()) {
        Ok(value) => Ok(HttpResponse::Ok().body(value)),
        Err(err) => Err(err)
      }
    })
}
