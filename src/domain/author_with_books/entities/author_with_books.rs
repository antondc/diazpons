use super::super::super::{Author, Book};

#[allow(dead_code)]
#[derive(serde::Deserialize, Clone)]
pub struct AuthorWithBooks {
  pub author: Author,
  pub books: Vec<Book>,
}
