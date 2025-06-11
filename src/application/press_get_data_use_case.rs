use crate::{
  application::ILanguageGetOneOrDefaultUseCase,
  domain::ILanguageRepository,
  types::{Errors, Result},
};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IPressGetDataUseCase {
  fn new<K: ILanguageGetOneOrDefaultUseCase>(language_get_one_or_default_use_case: K) -> PressGetDataUseCase<K>;
  async fn execute(&self, slug: Option<String>) -> Result<()>;
}

pub struct PressGetDataUseCase<K> {
  language_get_one_or_default_use_case: K,
}

#[async_trait]
impl<K: ILanguageGetOneOrDefaultUseCase> IPressGetDataUseCase for PressGetDataUseCase<K> {
  fn new<K2: ILanguageGetOneOrDefaultUseCase>(language_get_one_or_default_use_case: K2) -> PressGetDataUseCase<K2> {
    PressGetDataUseCase {
      language_get_one_or_default_use_case,
    }
  }

  async fn execute(&self, slug: Option<String>) -> Result<()> {
    // Get language from slug, or default if it doesnt exists
    // TODO: select books by language
    let _language_or_default = self
      .language_get_one_or_default_use_case
      .execute(slug)
      .await
      .map_err(|_| Errors::new(Errors::NotFound, Some(String::from("Language not found"))))?;

    Ok(())
  }
}
