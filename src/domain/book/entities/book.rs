use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Clone, Debug)]
pub struct Book {
  pub count: Option<i64>,
  pub id: String,
  pub title: String,
  pub subtitle: String,
  pub author_id: String,
  pub serie_id: String,
  pub year: i32,
  pub pages: i32,
  pub format: String,
  pub isbn: String,
  pub image_vertical: String,
  pub image_horizontal: String,
  pub description: String,
}
