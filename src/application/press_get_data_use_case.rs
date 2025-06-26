use crate::{
  application::ILanguageGetOneOrDefaultUseCase,
  domain::{ILanguageRepository, IReviewRepository, Review},
  infrastructure::http::DataWithLanguage,
  types::{Errors, Result},
};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IPressGetDataUseCase {
  fn new<K: ILanguageGetOneOrDefaultUseCase>(review_repository: Arc<dyn IReviewRepository>, language_get_one_or_default_use_case: K) -> PressGetDataUseCase<K>;
  async fn execute(&self, slug: Option<String>) -> Result<DataWithLanguage<Vec<Review>>>;
}

pub struct PressGetDataUseCase<K> {
  review_repository: Arc<dyn IReviewRepository>,
  language_get_one_or_default_use_case: K,
}

#[async_trait]
impl<K: ILanguageGetOneOrDefaultUseCase> IPressGetDataUseCase for PressGetDataUseCase<K> {
  fn new<K2: ILanguageGetOneOrDefaultUseCase>(
    review_repository: Arc<dyn IReviewRepository>,
    language_get_one_or_default_use_case: K2,
  ) -> PressGetDataUseCase<K2> {
    PressGetDataUseCase {
      review_repository,
      language_get_one_or_default_use_case,
    }
  }

  async fn execute(&self, slug: Option<String>) -> Result<DataWithLanguage<Vec<Review>>> {
    let (_, reviews) = self.review_repository.review_get_all().await.unwrap();

    let language_or_default = self
      .language_get_one_or_default_use_case
      .execute(slug)
      .await
      .map_err(|_| Errors::new(Errors::NotFound, Some(String::from("Language not found"))))?;

    Ok(DataWithLanguage {
      language: language_or_default,
      data: reviews,
    })
  }
}
