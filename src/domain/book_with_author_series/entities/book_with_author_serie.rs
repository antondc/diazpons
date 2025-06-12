use crate::domain::Serie;
use serde::Deserialize;

use super::super::super::{Author, Book};

#[allow(dead_code)]
#[derive(Deserialize, Clone, Debug)]
pub struct BookWithAuthorSerie {
  pub book: Book,
  pub author: Author,
  pub serie: Serie,
}
