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
  pub designer: String,
  pub designer_url: String,
  pub isbn: String,
  pub pdf: String,
  pub image_vertical: String,
  pub image_horizontal: String,
  pub events: Vec<String>,
  pub description: String,
}
