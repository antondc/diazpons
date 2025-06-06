use crate::infrastructure::http::types::HttpError;
use crate::types::Errors;
use rocket::catch;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[catch(404)]
pub async fn not_found_error() -> HttpError {
  let error = Errors::new(Errors::NotFound, Some(String::from("Endpoint not found")));

  status::Custom(Status::new(error.status), Json(error))
}

// Catching rest of errors
#[catch(default)]
pub fn default_error() -> HttpError {
  let error = Errors::new(Errors::UnprocessableEntity, Some(String::from("Unknown error")));

  status::Custom(Status::new(error.status), Json(error))
}
