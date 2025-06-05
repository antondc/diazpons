use crate::domain::{BookWithAuthor, IAuthorRepository, IBookRepository};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IHomeGetDataUseCase {
    fn new(
        book_repository: Arc<dyn IBookRepository>,
        author_repository: Arc<dyn IAuthorRepository>,
    ) -> HomeGetDataUseCase;
    async fn execute(&self) -> Result<Vec<BookWithAuthor>, String>;
}

pub struct HomeGetDataUseCase {
    book_repository: Arc<dyn IBookRepository>,
    author_repository: Arc<dyn IAuthorRepository>,
}

#[async_trait]
impl IHomeGetDataUseCase for HomeGetDataUseCase {
    fn new(
        book_repository: Arc<dyn IBookRepository>,
        author_repository: Arc<dyn IAuthorRepository>,
    ) -> HomeGetDataUseCase {
        HomeGetDataUseCase {
            book_repository,
            author_repository,
        }
    }

    async fn execute(&self) -> Result<Vec<BookWithAuthor>, String> {
        let books = self.book_repository.book_get_all().await.unwrap();
        let authors = self.author_repository.author_get_all().await.unwrap();

        let home_data: Vec<BookWithAuthor> = books
            .iter()
            .map(|item| BookWithAuthor {
                book: item.clone(),
                author: authors
                    .iter()
                    .find(|authors| authors.id == item.author_id)
                    .expect("Author not found") // We know there will be an author for this book
                    .clone(),
            })
            .collect();

        Ok(home_data)
    }
}
