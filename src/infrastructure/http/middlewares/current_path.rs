use rocket::http::uri::Origin;
use rocket::request::{FromRequest, Outcome, Request};

pub struct CurrentPath(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CurrentPath {
  type Error = ();

  async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let segments: Vec<_> = request.uri().path().segments().collect();

    let cleaned_segments = match segments.first() {
      Some(first) if first.len() == 2 => &segments[1..],
      _ => &segments[..],
    };

    let path = format!("/{}", cleaned_segments.join("/"));
    Outcome::Success(CurrentPath(path))
  }
}
