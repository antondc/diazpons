use std::fs;

use crate::domain::BookWithAuthor;
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "views/home/home.html")]
pub struct HomeTemplate {
  books_with_authors: Vec<BookWithAuthor>,
  twitter_svg: String,
  facebook_svg: String,
  instagram_svg: String,
}

pub async fn home_view(books_with_authors: Vec<BookWithAuthor>) -> HomeTemplate {
  let twitter_svg = fs::read_to_string("src/presentation/assets/svg/twitter-logo.svg").unwrap();
  let facebook_svg = fs::read_to_string("src/presentation/assets/svg/facebook-logo.svg").unwrap();
  let instagram_svg = fs::read_to_string("src/presentation/assets/svg/instagram-logo.svg").unwrap();

  let template = HomeTemplate {
    books_with_authors,
    twitter_svg,
    facebook_svg,
    instagram_svg,
  };

  template
}
