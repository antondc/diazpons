use crate::domain::language::{Glossary, ILanguageRepository, Language};
use crate::shared::services::get_file_content;
use crate::types::{Errors, Meta, Result};
use async_trait::async_trait;
use std::str;

#[derive(Copy, Clone)]
pub struct FileSystemLanguageRepository;

#[async_trait]
impl ILanguageRepository for FileSystemLanguageRepository {
  async fn language_get_one(&self, id_or_slug: &str) -> Result<Language> {
    let json_languages = get_file_content("data/languages.json").map_err(|error| Errors::new(Errors::Database, Some(error.to_string())))?;
    let languages: Vec<Language> = serde_json::from_str(&json_languages).expect("Invalid JSON");
    let language = languages
      .into_iter()
      .find(|item| item.slug == id_or_slug)
      .ok_or_else(|| Errors::new(Errors::NotFound, Some("Language not found".to_string())))?;

    Ok(language)
  }

  async fn language_get_all(&self) -> Result<(Meta, Vec<Language>)> {
    let json_languages = get_file_content("data/languages.json").map_err(|error| Errors::new(Errors::Parsing, Some(error.to_string())))?;
    let languages: Vec<Language> = serde_json::from_str(&json_languages).map_err(|error| Errors::new(Errors::Database, Some(error.to_string())))?;

    let items: Vec<Language> = languages.into_iter().map(Language::from).collect();
    let count = items.first().map(|item| item.count.unwrap_or(0));

    let meta = Meta {
      total_items: count,
      offset: None,
      size: None,
      sort: None,
    };

    Ok((meta, items))
  }
}
