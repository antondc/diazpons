use super::super::Language;
use crate::types::{Meta, Result};
use async_trait::async_trait;

#[async_trait]
pub trait ILanguageRepository: Send + Sync {
  #[allow(dead_code)]
  async fn language_get_one(&self, id_or_slug: &str) -> Result<Language>;
  async fn language_get_all(&self) -> Result<(Meta, Vec<Language>)>;
}
