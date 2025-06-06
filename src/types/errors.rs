use schemars::JsonSchema;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub enum Errors {
  General,
  Database,
  Parsing,
  Filesystem,
  #[allow(dead_code)]
  HttpClient,
  NotFound, // Resorce not found
  #[allow(dead_code)]
  Authentication, // Wrong credentials to authenticate
  #[allow(dead_code)]
  AlreadyLogged, // User already logged
  #[allow(dead_code)]
  Unauthorized, // Insufficient credentials to access resource
  UnprocessableEntity,
  #[allow(dead_code)]
  Conflict,
}

#[derive(Debug, Serialize, Clone, JsonSchema, PartialEq, Eq)]
pub struct CustomError {
  pub status: u16,
  pub category: String,
  pub message: Option<String>,
}

impl fmt::Display for CustomError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "status: {}\n category: {}\n message: {:?}",
      self.status,
      self.category,
      self.message.as_ref().get_or_insert(&String::from(""))
    )
  }
}

impl Errors {
  pub fn new(error: Errors, message: Option<String>) -> CustomError {
    match error {
      Errors::General => CustomError {
        status: 500,
        category: String::from("General error"),
        message,
      },
      Errors::NotFound => CustomError {
        status: 404,
        category: String::from("Not Found"),
        message,
      },
      Errors::Database => CustomError {
        status: 500,
        category: String::from("Database error"),
        message,
      },
      Errors::Parsing => CustomError {
        status: 500,
        category: String::from("Parsing error"),
        message,
      },
      Errors::Filesystem => CustomError {
        status: 500,
        category: String::from("File system error"),
        message,
      },
      Errors::HttpClient => CustomError {
        status: 500,
        category: String::from("Http client error"),
        message,
      },
      Errors::Authentication => CustomError {
        status: 401,
        category: String::from("Authentication"),
        message,
      },
      Errors::Unauthorized => CustomError {
        status: 403,
        category: String::from("Unauthorized"),
        message,
      },
      Errors::AlreadyLogged => CustomError {
        status: 200,
        category: String::from("Authentication"),
        message,
      },
      Errors::UnprocessableEntity => CustomError {
        status: 422,
        category: String::from("Unprocessable Entity"),
        message,
      },
      Errors::Conflict => CustomError {
        status: 409,
        category: String::from("Conflict"),
        message,
      },
    }
  }
}
