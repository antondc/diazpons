use crate::domain::Language;

#[derive(Debug)]
pub struct DataWithLanguage<T> {
  pub language: Language,
  pub data: T,
}
