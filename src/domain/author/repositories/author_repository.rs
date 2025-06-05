use super::super::Author;
use async_trait::async_trait;

#[allow(dead_code)]
#[async_trait]
pub trait IAuthorRepository: Send + Sync {
    async fn author_get_all(&self) -> Result<Vec<Author>, String>;
}
