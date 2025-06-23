use crate::domain::{Review, Serie};
use serde::Deserialize;

use super::super::super::{Author, Book};

#[allow(dead_code)]
#[derive(Deserialize, Clone, Debug)]
pub struct BookWithAuthorSerieReviews {
  pub book: Book,
  pub author: Author,
  pub serie: Serie,
  pub reviews: Vec<Review>,
}
