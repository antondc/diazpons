use crate::types::{Meta, Result};

use super::super::Review;
use async_trait::async_trait;

#[async_trait]
pub trait IReviewRepository: Send + Sync {
  async fn review_get_by_book_id(&self, book_id: &str) -> Result<(Meta, Vec<Review>)>;
}
