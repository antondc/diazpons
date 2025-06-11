use crate::domain::Language;

#[derive(Debug)]
pub struct ViewData<T> {
  pub data: T,
  pub language: Language,
  pub current_path: String,
  pub current_slug: String,
}
