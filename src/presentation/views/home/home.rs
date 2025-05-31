use crate::domain::Book;
use askama::Template;
use axum::response::Html;

// Askama template struct
#[derive(Template)]
#[template(path = "views/home/home.html")]
struct HomeTemplate<'a> {
    title: String,
    books: &'a [Book],
}

pub async fn home_view(books: Vec<Book>) -> Html<String> {
    let template = HomeTemplate {
        title: String::from("Sci-Fi Books"),
        books: &books,
    };

    Html(template.render().unwrap())
}
