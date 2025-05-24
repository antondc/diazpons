use super::super::Book;
use crate::domain::user::entities::Role;
use crate::types::{Meta, Result};
use async_trait::async_trait;

#[async_trait]
pub trait IBookRepository: Send + Sync {
    async fn book_get_all(&self, slug: &str, roles: Vec<Role>) -> Result<(Meta, Vec<Book>)>;
}
