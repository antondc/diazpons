use crate::domain::Language;

#[derive(Debug)]
pub struct ViewData<T> {
  pub data: T,
  pub language: Language,
  #[allow(unused)]
  pub current_path: String,
  #[allow(unused)]
  pub current_slug: String,
}
