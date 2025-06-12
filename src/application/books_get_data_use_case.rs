use crate::{
  application::ILanguageGetOneOrDefaultUseCase,
  domain::{BookWithAuthorSerie, IAuthorRepository, IBookRepository, ILanguageRepository, ISerieRepository},
  infrastructure::http::DataWithLanguage,
  types::{Errors, Result},
};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IBooksGetDataUseCase {
  fn new<K: ILanguageGetOneOrDefaultUseCase>(
    book_repository: Arc<dyn IBookRepository>,
    author_repository: Arc<dyn IAuthorRepository>,
    serie_repository: Arc<dyn ISerieRepository>,
    language_get_one_or_default_use_case: K,
  ) -> BooksGetDataUseCase<K>;
  async fn execute(&self, slug: Option<String>) -> Result<DataWithLanguage<Vec<BookWithAuthorSerie>>>;
}

pub struct BooksGetDataUseCase<K> {
  book_repository: Arc<dyn IBookRepository>,
  author_repository: Arc<dyn IAuthorRepository>,
  serie_repository: Arc<dyn ISerieRepository>,
  language_get_one_or_default_use_case: K,
}

#[async_trait]
impl<K: ILanguageGetOneOrDefaultUseCase> IBooksGetDataUseCase for BooksGetDataUseCase<K> {
  fn new<K2: ILanguageGetOneOrDefaultUseCase>(
    book_repository: Arc<dyn IBookRepository>,
    author_repository: Arc<dyn IAuthorRepository>,
    serie_repository: Arc<dyn ISerieRepository>,
    language_get_one_or_default_use_case: K2,
  ) -> BooksGetDataUseCase<K2> {
    BooksGetDataUseCase {
      book_repository,
      author_repository,
      serie_repository,
      language_get_one_or_default_use_case,
    }
  }

  async fn execute(&self, slug: Option<String>) -> Result<DataWithLanguage<Vec<BookWithAuthorSerie>>> {
    let (_, books) = self.book_repository.book_get_all().await.unwrap();
    let (_, authors) = self.author_repository.author_get_all().await.unwrap();
    let (_, series) = self.serie_repository.serie_get_all().await.unwrap();

    // Get language from slug, or default if it doesnt exists
    // TODO: select books by language
    let language_or_default = self
      .language_get_one_or_default_use_case
      .execute(slug)
      .await
      .map_err(|_| Errors::new(Errors::NotFound, Some(String::from("Language not found"))))?;

    let books_data: Vec<BookWithAuthorSerie> = books
      .iter()
      .map(|item| BookWithAuthorSerie {
        book: item.clone(),
        author: authors
          .iter()
          .find(|authors| authors.id == item.author_id)
          .expect("Author not found") // We know there will be an author for this book
          .clone(),
        serie: series.iter().find(|serie| serie.id == item.serie_id).expect("Serie not found").clone(),
      })
      .collect();

    Ok(DataWithLanguage {
      language: language_or_default,
      data: books_data,
    })
  }
}
