use async_trait::async_trait;
use std::fs;

use crate::{
  domain::{Book, IBookRepository},
  shared::services::get_file_content,
  types::{Errors, Meta, Result},
};

#[derive(Copy, Clone)]
pub struct FileSystemBookRepository;

#[async_trait]
impl IBookRepository for FileSystemBookRepository {
  async fn book_get_all(&self) -> Result<(Meta, Vec<Book>)> {
    let json_books = get_file_content("data/books.json")
      .map_err(|error| Errors::new(Errors::Database, Some(error.to_string())))
      .unwrap();
    let books: Vec<Book> = serde_json::from_str(&json_books).expect("Invalid JSON");

    let count = books.first().map(|item| item.count.unwrap_or(0));

    let meta = Meta {
      total_items: count,
      offset: None,
      size: None,
      sort: None,
    };

    Ok((meta, books))
  }
}
