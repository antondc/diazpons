use crate::{
  domain::{ILanguageRepository, Language},
  types::{Errors, Result},
};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait ILanguageGetOneOrDefaultUseCase: Send + Sync {
  fn new(language_repository: Arc<dyn ILanguageRepository>) -> LanguageGetOneOrDefaultUseCase;
  async fn execute(&self, language_slug: Option<String>) -> Result<Language>;
}

pub struct LanguageGetOneOrDefaultUseCase {
  language_repository: Arc<dyn ILanguageRepository>,
}

#[async_trait]
impl ILanguageGetOneOrDefaultUseCase for LanguageGetOneOrDefaultUseCase {
  fn new(language_repository: Arc<dyn ILanguageRepository>) -> LanguageGetOneOrDefaultUseCase {
    LanguageGetOneOrDefaultUseCase { language_repository }
  }

  async fn execute(&self, language_slug: Option<String>) -> Result<Language> {
    // 1.
    let (_, languages) = self
      .language_repository
      .language_get_all()
      .await
      .map_err(|_| Errors::new(Errors::NotFound, Some(String::from("Languages not found"))))?;

    // 2.
    let default_language = languages
      .clone()
      .into_iter()
      .find(|language| language.is_default)
      .ok_or_else(|| Errors::new(Errors::NotFound, Some(String::from("Default language not found"))))?;

    // 3.
    let language_to_set = match language_slug {
      None => default_language, // 4.
      Some(slug) => {
        let language_to_set = languages.into_iter().find(|language| language.slug == slug);

        match language_to_set {
          Some(language) => language, // 5.
          None => default_language,   // 6.
        }
      }
    };

    Ok(language_to_set)
  }
}

/*
  Logic to retrieve a language from a slug, or a default one if it doesnt exists:
    1. get all languages
    2. find default language in them
    3. compare input language slug with existing languages
    4. if language slug is None use the default language
    5. if language slug exists use it
    6. on any other case use the default language
*/
