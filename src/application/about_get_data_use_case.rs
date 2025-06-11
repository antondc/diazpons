use crate::{
  application::ILanguageGetOneOrDefaultUseCase,
  domain::ILanguageRepository,
  infrastructure::http::DataWithLanguage,
  types::{Errors, Result},
};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IAboutGetDataUseCase {
  fn new<K: ILanguageGetOneOrDefaultUseCase>(language_get_one_or_default_use_case: K) -> AboutGetDataUseCase<K>;
  async fn execute(&self, slug: Option<String>) -> Result<DataWithLanguage<()>>;
}

pub struct AboutGetDataUseCase<K> {
  language_get_one_or_default_use_case: K,
}

#[async_trait]
impl<K: ILanguageGetOneOrDefaultUseCase> IAboutGetDataUseCase for AboutGetDataUseCase<K> {
  fn new<K2: ILanguageGetOneOrDefaultUseCase>(language_get_one_or_default_use_case: K2) -> AboutGetDataUseCase<K2> {
    AboutGetDataUseCase {
      language_get_one_or_default_use_case,
    }
  }

  async fn execute(&self, slug: Option<String>) -> Result<DataWithLanguage<()>> {
    // Get language from slug, or default if it doesnt exists
    // TODO: select books by language
    let language_or_default = self
      .language_get_one_or_default_use_case
      .execute(slug)
      .await
      .map_err(|_| Errors::new(Errors::NotFound, Some(String::from("Language not found"))))?;

    Ok(DataWithLanguage {
      language: language_or_default,
      data: (),
    })
  }
}
