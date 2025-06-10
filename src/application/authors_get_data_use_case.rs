use crate::{
  application::ILanguageGetOneOrDefaultUseCase,
  domain::{AuthorWithBooks, IAuthorRepository, IBookRepository, ILanguageRepository},
  types::{Errors, Result},
};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IAuthorsGetDataUseCase {
  fn new<K: ILanguageGetOneOrDefaultUseCase>(
    book_repository: Arc<dyn IBookRepository>,
    author_repository: Arc<dyn IAuthorRepository>,
    language_get_one_or_default_use_case: K,
  ) -> AuthorsGetDataUseCase<K>;
  async fn execute(&self, slug: Option<String>) -> Result<Vec<AuthorWithBooks>>;
}

pub struct AuthorsGetDataUseCase<K> {
  book_repository: Arc<dyn IBookRepository>,
  author_repository: Arc<dyn IAuthorRepository>,
  language_get_one_or_default_use_case: K,
}

#[async_trait]
impl<K: ILanguageGetOneOrDefaultUseCase> IAuthorsGetDataUseCase for AuthorsGetDataUseCase<K> {
  fn new<K2: ILanguageGetOneOrDefaultUseCase>(
    book_repository: Arc<dyn IBookRepository>,
    author_repository: Arc<dyn IAuthorRepository>,
    language_get_one_or_default_use_case: K2,
  ) -> AuthorsGetDataUseCase<K2> {
    AuthorsGetDataUseCase {
      book_repository,
      author_repository,
      language_get_one_or_default_use_case,
    }
  }

  async fn execute(&self, slug: Option<String>) -> Result<Vec<AuthorWithBooks>> {
    let (_, _books) = self.book_repository.book_get_all().await.unwrap();
    let (_, _authors) = self.author_repository.author_get_all().await.unwrap();

    // Get language from slug, or default if it doesnt exists
    // TODO: select books by language
    let _language_or_default = self
      .language_get_one_or_default_use_case
      .execute(slug)
      .await
      .map_err(|_| Errors::new(Errors::NotFound, Some(String::from("Language not found"))))?;

    let books_data: Vec<AuthorWithBooks> = vec![]; // TODO: fill this with actual data

    Ok(books_data)
  }
}
