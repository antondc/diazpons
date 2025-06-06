use async_trait::async_trait;
use std::fs;

use crate::{
  domain::{Author, IAuthorRepository},
  shared::services::get_file_content,
  types::{Meta, Result},
};

#[derive(Copy, Clone)]
pub struct FileSystemAuthorRepository;

#[async_trait]
impl IAuthorRepository for FileSystemAuthorRepository {
  async fn author_get_all(&self) -> Result<(Meta, Vec<Author>)> {
    let json_authors = get_file_content("data/authors.json").expect("Failed to read JSON");
    let authors: Vec<Author> = serde_json::from_str(&json_authors).expect("Invalid JSON");

    let count = authors.first().map(|item| item.count.unwrap_or(0));

    let meta = Meta {
      total_items: count,
      offset: None,
      size: None,
      sort: None,
    };

    Ok((meta, authors))
  }
}
