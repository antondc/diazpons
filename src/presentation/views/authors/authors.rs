use std::fs;

use crate::domain::AuthorWithBooks;
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "views/authors/authors.html")]
pub struct AuthorsTemplate {
  authors_with_books: Vec<AuthorWithBooks>,
  twitter_svg: String,
  facebook_svg: String,
  instagram_svg: String,
}

pub async fn authors_view(authors_with_books: Vec<AuthorWithBooks>) -> AuthorsTemplate {
  let twitter_svg = fs::read_to_string("src/presentation/assets/svg/twitter-logo.svg").unwrap();
  let facebook_svg = fs::read_to_string("src/presentation/assets/svg/facebook-logo.svg").unwrap();
  let instagram_svg = fs::read_to_string("src/presentation/assets/svg/instagram-logo.svg").unwrap();

  let template = AuthorsTemplate {
    authors_with_books,
    twitter_svg,
    facebook_svg,
    instagram_svg,
  };

  template
}
