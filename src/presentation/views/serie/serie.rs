use std::fs;

use crate::{
  domain::{BookWithAuthorSerie, Language},
  presentation::types::ViewData,
};
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "views/serie/serie.html")]
pub struct SerieTemplate {
  serie_name: String,
  books_with_authors_serie: Vec<BookWithAuthorSerie>,
  language: Language,
  twitter_svg: String,
  facebook_svg: String,
  instagram_svg: String,
}

pub async fn serie_view(view_data: ViewData<Vec<BookWithAuthorSerie>>) -> SerieTemplate {
  let twitter_svg = fs::read_to_string("src/presentation/assets/svg/twitter-logo.svg").unwrap();
  let facebook_svg = fs::read_to_string("src/presentation/assets/svg/facebook-logo.svg").unwrap();
  let instagram_svg = fs::read_to_string("src/presentation/assets/svg/instagram-logo.svg").unwrap();
  let series_name = view_data.data.get(0).map(|item| item.serie.name.clone()).unwrap_or_else(|| String::from(""));

  let template = SerieTemplate {
    serie_name: series_name,
    books_with_authors_serie: view_data.data,
    language: view_data.language,
    twitter_svg,
    facebook_svg,
    instagram_svg,
  };

  template
}
