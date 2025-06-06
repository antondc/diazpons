use super::super::constants::SESSION_TOKEN_NAME;
use crate::domain::user::entities::User;
use crate::shared::services::TokenService;
use rocket::request::{FromRequest, Outcome, Request};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Session(pub Option<User>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
  type Error = ();

  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let cookies = req.cookies();
    let session_token = cookies.get(SESSION_TOKEN_NAME).map_or("", |item| item.value());

    match TokenService::decode_token::<User>(session_token) {
      Ok(user) => Outcome::Success(Session(Some(user))),
      Err(_) => Outcome::Success(Session(None)),
    }
  }
}
