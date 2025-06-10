use crate::{
  application::ILanguageGetOneOrDefaultUseCase,
  domain::{ILanguageRepository, BookWithAuthor, IAuthorRepository, IBookRepository},
  types::{Errors, Result},
};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IHomeGetDataUseCase {
  fn new<K: ILanguageGetOneOrDefaultUseCase>(
    book_repository: Arc<dyn IBookRepository>,
    author_repository: Arc<dyn IAuthorRepository>,
    language_get_one_or_default_use_case: K,
  ) -> HomeGetDataUseCase<K>;
  async fn execute(&self, slug: Option<String>) -> Result<Vec<BookWithAuthor>>;
}

pub struct HomeGetDataUseCase<K> {
  book_repository: Arc<dyn IBookRepository>,
  author_repository: Arc<dyn IAuthorRepository>,
  language_get_one_or_default_use_case: K,
}

#[async_trait]
impl<K: ILanguageGetOneOrDefaultUseCase> IHomeGetDataUseCase for HomeGetDataUseCase<K> {
  fn new<K2: ILanguageGetOneOrDefaultUseCase>(
    book_repository: Arc<dyn IBookRepository>,
    author_repository: Arc<dyn IAuthorRepository>,
    language_get_one_or_default_use_case: K2,
  ) -> HomeGetDataUseCase<K2> {
    HomeGetDataUseCase {
      book_repository,
      author_repository,
      language_get_one_or_default_use_case,
    }
  }

  async fn execute(&self, slug: Option<String>) -> Result<Vec<BookWithAuthor>> {
    let (_, books) = self.book_repository.book_get_all().await.unwrap();
    let (_, authors) = self.author_repository.author_get_all().await.unwrap();

    // Get language from slug, or default if it doesnt exists
    // TODO: select books by language
    let _language_or_default = self
      .language_get_one_or_default_use_case
      .execute(slug)
      .await
      .map_err(|_| Errors::new(Errors::NotFound, Some(String::from("Language not found"))))?;

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
