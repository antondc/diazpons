use crate::domain::Book;
use crate::presentation::views::home_view;
use axum::response::Html;
use serde_json;
use std::fs;

pub async fn home() -> Html<String> {
    let json_str = fs::read_to_string("src/infrastructure/persistence/json/data.json")
        .expect("Failed to read JSON");
    let books: Vec<Book> = serde_json::from_str(&json_str).expect("Invalid JSON");

    home_view(books).await
}
