use std::fs;

use crate::domain::BookWithAuthor;
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "views/home/home.html")]
pub struct BooksTemplate {
  books_with_authors: Vec<BookWithAuthor>,
  twitter_svg: String,
  facebook_svg: String,
  instagram_svg: String,
}

pub async fn books_view(books_with_authors: Vec<BookWithAuthor>) -> BooksTemplate {
  let twitter_svg = fs::read_to_string("src/presentation/assets/svg/twitter-logo.svg").unwrap();
  let facebook_svg = fs::read_to_string("src/presentation/assets/svg/facebook-logo.svg").unwrap();
  let instagram_svg = fs::read_to_string("src/presentation/assets/svg/instagram-logo.svg").unwrap();

  let template = BooksTemplate {
    books_with_authors,
    twitter_svg,
    facebook_svg,
    instagram_svg,
  };

  template
}
