use std::fs;

use crate::domain::BookWithAuthor;
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "views/serie/serie.html")]
pub struct SerieTemplate {
  twitter_svg: String,
  facebook_svg: String,
  instagram_svg: String,
}

pub async fn serie_view(_serie_data: ()) -> SerieTemplate {
  let twitter_svg = fs::read_to_string("src/presentation/assets/svg/twitter-logo.svg").unwrap();
  let facebook_svg = fs::read_to_string("src/presentation/assets/svg/facebook-logo.svg").unwrap();
  let instagram_svg = fs::read_to_string("src/presentation/assets/svg/instagram-logo.svg").unwrap();

  let template = SerieTemplate {
    twitter_svg,
    facebook_svg,
    instagram_svg,
  };

  template
}
