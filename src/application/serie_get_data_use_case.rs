use crate::{
  application::ILanguageGetOneOrDefaultUseCase,
  domain::{Book, BookWithAuthorSerieReviews, IAuthorRepository, IBookRepository, ILanguageRepository, ISerieRepository},
  infrastructure::http::DataWithLanguage,
  types::{Errors, Result},
};
use async_trait::async_trait;
use itertools::Itertools;
use std::sync::Arc;

#[async_trait]
pub trait ISerieGetDataUseCase {
  fn new<K: ILanguageGetOneOrDefaultUseCase>(
    serie_repository: Arc<dyn ISerieRepository>,
    book_repository: Arc<dyn IBookRepository>,
    author_repository: Arc<dyn IAuthorRepository>,
    language_get_one_or_default_use_case: K,
  ) -> SerieGetDataUseCase<K>;
  async fn execute(&self, slug: Option<String>, serie_id: String) -> Result<DataWithLanguage<Vec<BookWithAuthorSerieReviews>>>;
}

pub struct SerieGetDataUseCase<K> {
  serie_repository: Arc<dyn ISerieRepository>,
  book_repository: Arc<dyn IBookRepository>,
  author_repository: Arc<dyn IAuthorRepository>,
  language_get_one_or_default_use_case: K,
}

#[async_trait]
impl<K: ILanguageGetOneOrDefaultUseCase> ISerieGetDataUseCase for SerieGetDataUseCase<K> {
  fn new<K2: ILanguageGetOneOrDefaultUseCase>(
    serie_repository: Arc<dyn ISerieRepository>,
    book_repository: Arc<dyn IBookRepository>,
    author_repository: Arc<dyn IAuthorRepository>,
    language_get_one_or_default_use_case: K2,
  ) -> SerieGetDataUseCase<K2> {
    SerieGetDataUseCase {
      serie_repository,
      book_repository,
      author_repository,
      language_get_one_or_default_use_case,
    }
  }

  async fn execute(&self, slug: Option<String>, serie_id: String) -> Result<DataWithLanguage<Vec<BookWithAuthorSerieReviews>>> {
    let (_, books) = self.book_repository.book_get_all().await.unwrap();
    let (_, authors) = self.author_repository.author_get_all().await.unwrap();
    let (_, series) = self.serie_repository.serie_get_all().await.unwrap();

    let books_authors_by_serie: Vec<BookWithAuthorSerieReviews> = books
      .iter()
      .cloned()
      .filter(|item| item.serie_id == serie_id) // Get books by collection
      .sorted_by_key(|item| item.year) // Sort by year
      .map(|item| BookWithAuthorSerieReviews {
        book: item.clone(),
        author: authors // Add author
          .iter()
          .find(|author| author.id == item.author_id)
          .expect("Author not found") // We know there will be an author for this book
          .clone(),
        serie: series.iter().find(|item| item.id == serie_id).expect("Serie not found").clone(),
        reviews: vec![],
      })
      .collect();

    // Get language from slug, or default if it doesnt exists
    // TODO: select series by language
    let language_or_default = self
      .language_get_one_or_default_use_case
      .execute(slug)
      .await
      .map_err(|_| Errors::new(Errors::NotFound, Some(String::from("Language not found"))))?;

    Ok(DataWithLanguage {
      language: language_or_default,
      data: books_authors_by_serie,
    })
  }
}
