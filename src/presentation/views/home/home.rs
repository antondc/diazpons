use std::fs;

use crate::domain::Book;
use askama::Template;
use axum::response::Html;

// Askama template struct
#[derive(Template)]
#[template(path = "views/home/home.html")]
struct HomeTemplate<'a> {
    title: String,
    books: &'a [Book],
    twitter_svg: String,
    facebook_svg: String,
    instagram_svg: String,
}

pub async fn home_view(books: Vec<Book>) -> Html<String> {
    let twitter_svg = fs::read_to_string("src/presentation/assets/svg/twitter-logo.svg").unwrap();
    let facebook_svg = fs::read_to_string("src/presentation/assets/svg/facebook-logo.svg").unwrap();
    let instagram_svg =
        fs::read_to_string("src/presentation/assets/svg/instagram-logo.svg").unwrap();

    let template = HomeTemplate {
        title: String::from("Sci-Fi Books"),
        books: &books,
        twitter_svg,
        facebook_svg,
        instagram_svg,
    };

    Html(template.render().unwrap())
}
