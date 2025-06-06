use crate::types::CustomError;
use rocket::response::status as rocketStatus;
use rocket::serde::json::Json;

pub type HttpError = rocketStatus::Custom<Json<CustomError>>;
