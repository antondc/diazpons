use std::fs;

use crate::{
  domain::{BookWithAuthorSerie, Language},
  infrastructure::http::DataWithLanguage,
  presentation::types::ViewData,
};
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "views/about/about.html")]
pub struct AboutTemplate {
  language: Language,
  twitter_svg: String,
  facebook_svg: String,
  instagram_svg: String,
}

pub async fn about_view(view_data: ViewData<()>) -> AboutTemplate {
  let twitter_svg = fs::read_to_string("src/presentation/assets/svg/twitter-logo.svg").unwrap();
  let facebook_svg = fs::read_to_string("src/presentation/assets/svg/facebook-logo.svg").unwrap();
  let instagram_svg = fs::read_to_string("src/presentation/assets/svg/instagram-logo.svg").unwrap();

  let template = AboutTemplate {
    language: view_data.language,
    twitter_svg,
    facebook_svg,
    instagram_svg,
  };

  template
}
