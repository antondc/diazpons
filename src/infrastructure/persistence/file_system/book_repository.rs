use async_trait::async_trait;
use std::fs;

use crate::domain::{Book, IBookRepository};

#[derive(Copy, Clone)]
pub struct FileSystemBookRepository;

#[async_trait]
impl IBookRepository for FileSystemBookRepository {
    async fn book_get_all(&self) -> Result<Vec<Book>, String> {
        let json_books = fs::read_to_string("data/books.json")
            .expect("Failed to read JSON");
        let books: Vec<Book> = serde_json::from_str(&json_books).expect("Invalid JSON");

        Ok(books)
    }
}
