use std::fs;

use crate::{
  domain::{AuthorWithBooks, Language},
  infrastructure::http::DataWithLanguage,
  presentation::types::ViewData,
};
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "views/authors/authors.html")]
pub struct AuthorsTemplate {
  authors_with_books: Vec<AuthorWithBooks>,
  current_path: String,
  current_slug: String,
  language: Language,
  twitter_svg: String,
  facebook_svg: String,
  instagram_svg: String,
}

pub async fn authors_view(view_data: ViewData<Vec<AuthorWithBooks>>) -> AuthorsTemplate {
  let twitter_svg = fs::read_to_string("src/presentation/assets/svg/twitter-logo.svg").unwrap();
  let facebook_svg = fs::read_to_string("src/presentation/assets/svg/facebook-logo.svg").unwrap();
  let instagram_svg = fs::read_to_string("src/presentation/assets/svg/instagram-logo.svg").unwrap();

  let template = AuthorsTemplate {
    authors_with_books: view_data.data,
    current_path: view_data.current_path,
    current_slug: view_data.current_slug,
    language: view_data.language,
    twitter_svg,
    facebook_svg,
    instagram_svg,
  };

  template
}
