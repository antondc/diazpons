use crate::{
  application::ILanguageGetOneOrDefaultUseCase,
  domain::{AuthorWithBooks, IAuthorRepository, IBookRepository, ILanguageRepository},
  infrastructure::http::DataWithLanguage,
  types::{Errors, Result},
};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IAuthorGetDataUseCase {
  fn new<K: ILanguageGetOneOrDefaultUseCase>(
    book_repository: Arc<dyn IBookRepository>,
    author_repository: Arc<dyn IAuthorRepository>,
    language_get_one_or_default_use_case: K,
  ) -> AuthorGetDataUseCase<K>;
  async fn execute(&self, slug: Option<String>, author_id: String) -> Result<DataWithLanguage<AuthorWithBooks>>;
}

pub struct AuthorGetDataUseCase<K> {
  book_repository: Arc<dyn IBookRepository>,
  author_repository: Arc<dyn IAuthorRepository>,
  language_get_one_or_default_use_case: K,
}

#[async_trait]
impl<K: ILanguageGetOneOrDefaultUseCase> IAuthorGetDataUseCase for AuthorGetDataUseCase<K> {
  fn new<K2: ILanguageGetOneOrDefaultUseCase>(
    book_repository: Arc<dyn IBookRepository>,
    author_repository: Arc<dyn IAuthorRepository>,
    language_get_one_or_default_use_case: K2,
  ) -> AuthorGetDataUseCase<K2> {
    AuthorGetDataUseCase {
      book_repository,
      author_repository,
      language_get_one_or_default_use_case,
    }
  }

  async fn execute(&self, slug: Option<String>, author_id: String) -> Result<DataWithLanguage<AuthorWithBooks>> {
    let (_, books) = self.book_repository.book_get_all().await.unwrap();
    let (_, authors) = self.author_repository.author_get_all().await.unwrap();

    let author_with_books: AuthorWithBooks = authors
      .iter()
      .map(|item| AuthorWithBooks {
        author: item.clone(),
        books: books.iter().cloned().filter(|book| book.author_id == author_id).collect(),
      })
      .find(|item| item.author.id == author_id)
      .unwrap();

    let language_or_default = self
      .language_get_one_or_default_use_case
      .execute(slug)
      .await
      .map_err(|_| Errors::new(Errors::NotFound, Some(String::from("Language not found"))))?;

    let author_data: DataWithLanguage<AuthorWithBooks> = DataWithLanguage {
      language: language_or_default,
      data: author_with_books,
    };

    Ok(author_data)
  }
}
