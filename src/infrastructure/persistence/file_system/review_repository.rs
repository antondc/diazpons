use async_trait::async_trait;
use std::fs;

use crate::{
  domain::{IReviewRepository, Review},
  shared::services::get_file_content,
  types::{Errors, Meta, Result},
};

#[derive(Copy, Clone)]
pub struct FileSystemReviewRepository;

#[async_trait]
impl IReviewRepository for FileSystemReviewRepository {
  async fn review_get_by_book_id(&self, book_id: &str) -> Result<(Meta, Vec<Review>)> {
    let json_reviews = get_file_content("data/reviews.json")
      .map_err(|error| Errors::new(Errors::Database, Some(error.to_string())))
      .unwrap();
    let reviews: Vec<Review> = serde_json::from_str(&json_reviews).expect("Invalid JSON");
    let book_reviews: Vec<Review> = reviews.iter().cloned().filter(|item| item.book_id == book_id).collect();
    let count = reviews.first().map(|item| item.count.unwrap_or(0));

    let meta = Meta {
      total_items: count,
      offset: None,
      size: None,
      sort: None,
    };

    Ok((meta, book_reviews))
  }
}
