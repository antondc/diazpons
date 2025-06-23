use std::fs;

use crate::{
  domain::{BookWithAuthorSerieReviews, Language},
  presentation::types::ViewData,
};
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "views/book/book.html")]
pub struct BookTemplate {
  book_with_author: BookWithAuthorSerieReviews,
  language: Language,
  twitter_svg: String,
  facebook_svg: String,
  instagram_svg: String,
}

pub async fn book_view(view_data: ViewData<BookWithAuthorSerieReviews>) -> BookTemplate {
  let twitter_svg = fs::read_to_string("src/presentation/assets/svg/twitter-logo.svg").unwrap();
  let facebook_svg = fs::read_to_string("src/presentation/assets/svg/facebook-logo.svg").unwrap();
  let instagram_svg = fs::read_to_string("src/presentation/assets/svg/instagram-logo.svg").unwrap();

  let template = BookTemplate {
    book_with_author: view_data.data,
    language: view_data.language,
    twitter_svg,
    facebook_svg,
    instagram_svg,
  };

  template
}
