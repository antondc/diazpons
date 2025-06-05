use super::super::Book;
use async_trait::async_trait;

#[allow(dead_code)]
#[async_trait]
pub trait IBookRepository: Send + Sync {
    async fn book_get_all(&self) -> Result<Vec<Book>, String>;
}
