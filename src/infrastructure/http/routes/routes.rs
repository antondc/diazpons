use crate::domain::{Author, Book, BookWithAuthor};
use crate::presentation::views::home_view;
use axum::response::Html;
use serde_json;
use std::fs;

pub async fn home() -> Html<String> {
    let json_books = fs::read_to_string("src/infrastructure/persistence/json/books.json")
        .expect("Failed to read JSON");
    let json_authors = fs::read_to_string("src/infrastructure/persistence/json/authors.json")
        .expect("Failed to read JSON");
    let books: Vec<Book> = serde_json::from_str(&json_books).expect("Invalid JSON");
    let authors: Vec<Author> = serde_json::from_str(&json_authors).expect("Invalid JSON");
    let home_data: Vec<BookWithAuthor> = books
        .iter()
        .map(|item| BookWithAuthor {
            book: item.clone(),
            author: authors
                .iter()
                .find(|authors| authors.id == item.author_id)
                .expect("Author not found") // We know there will be an author for this book
                .clone(),
        })
        .collect();

    home_view(home_data).await
}
