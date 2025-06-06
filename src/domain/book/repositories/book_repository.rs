use crate::types::{Meta, Result};

use super::super::Book;
use async_trait::async_trait;

#[async_trait]
pub trait IBookRepository: Send + Sync {
  async fn book_get_all(&self) -> Result<(Meta, Vec<Book>)>;
}
