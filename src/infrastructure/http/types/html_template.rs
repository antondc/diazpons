use askama::Template;
use rocket::response::{Responder, Response};
use rocket::Request;
use std::marker::PhantomData;

pub struct HtmlTemplate<T: Template, E: Template> {
  result: Result<T, E>,
  _marker: PhantomData<(T, E)>,
}

impl<T: Template, E: Template> HtmlTemplate<T, E> {
  pub fn new(result: Result<T, E>) -> Self {
    HtmlTemplate { result, _marker: PhantomData }
  }
}

impl<'r, T: Template + Responder<'r, 'static>, E: Template + Responder<'r, 'static>> Responder<'r, 'static> for HtmlTemplate<T, E> {
  fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
    match self.result {
      Ok(template) => template.respond_to(req),
      Err(error_template) => error_template.respond_to(req),
    }
  }
}
