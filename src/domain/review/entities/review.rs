use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Clone, Debug)]
pub struct Review {
  pub count: Option<i64>,
  pub book_id: String,
  pub author: String,
  pub title: String,
  pub media: String,
  pub date: String,
  pub url: String,
}
