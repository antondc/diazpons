use std::fs;

use askama::Template;
use axum::response::Html;

#[derive(Template, Debug)]
#[template(path = "views/server_error/server_error.html")]
pub struct ServerErrorTemplate {}

pub async fn server_error_view() -> ServerErrorTemplate {
  let template = ServerErrorTemplate {};

  template
}
