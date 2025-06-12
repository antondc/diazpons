use super::super::super::{Author, Book};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Clone, Debug)]
pub struct AuthorWithBooks {
  pub author: Author,
  pub books: Vec<Book>,
}
