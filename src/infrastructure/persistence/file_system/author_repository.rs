use async_trait::async_trait;
use std::fs;

use crate::domain::{Author, IAuthorRepository};

#[derive(Copy, Clone)]
pub struct FileSystemAuthorRepository;

#[async_trait]
impl IAuthorRepository for FileSystemAuthorRepository {
    async fn author_get_all(&self) -> Result<Vec<Author>, String> {
        let json_authors = fs::read_to_string("data/authors.json").expect("Failed to read JSON");
        let authors: Vec<Author> = serde_json::from_str(&json_authors).expect("Invalid JSON");

        Ok(authors)
    }
}
