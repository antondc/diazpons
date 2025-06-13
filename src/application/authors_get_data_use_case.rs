use crate::{
  application::ILanguageGetOneOrDefaultUseCase,
  domain::{Author, IAuthorRepository, IBookRepository, ILanguageRepository},
  infrastructure::http::DataWithLanguage,
  types::{Errors, Result},
};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IAuthorsGetDataUseCase {
  fn new<K: ILanguageGetOneOrDefaultUseCase>(
    author_repository: Arc<dyn IAuthorRepository>,
    language_get_one_or_default_use_case: K,
  ) -> AuthorsGetDataUseCase<K>;
  async fn execute(&self, slug: Option<String>) -> Result<DataWithLanguage<Vec<Author>>>;
}

pub struct AuthorsGetDataUseCase<K> {
  author_repository: Arc<dyn IAuthorRepository>,
  language_get_one_or_default_use_case: K,
}

#[async_trait]
impl<K: ILanguageGetOneOrDefaultUseCase> IAuthorsGetDataUseCase for AuthorsGetDataUseCase<K> {
  fn new<K2: ILanguageGetOneOrDefaultUseCase>(
    author_repository: Arc<dyn IAuthorRepository>,
    language_get_one_or_default_use_case: K2,
  ) -> AuthorsGetDataUseCase<K2> {
    AuthorsGetDataUseCase {
      author_repository,
      language_get_one_or_default_use_case,
    }
  }

  async fn execute(&self, slug: Option<String>) -> Result<DataWithLanguage<Vec<Author>>> {
    let (_, authors) = self.author_repository.author_get_all().await.unwrap();
    let language_or_default = self
      .language_get_one_or_default_use_case
      .execute(slug)
      .await
      .map_err(|_| Errors::new(Errors::NotFound, Some(String::from("Language not found"))))?;

    let data: DataWithLanguage<Vec<Author>> = DataWithLanguage {
      language: language_or_default,
      data: authors,
    };

    Ok(data)
  }
}
