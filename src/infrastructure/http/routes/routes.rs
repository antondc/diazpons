use crate::presentation::views::home_view;
use axum::response::Html;

pub async fn home() -> Html<String> {
    home_view().await
}

pub async fn book() -> &'static str {
    "Hello, book!"
}
