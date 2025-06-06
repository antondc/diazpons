#[allow(dead_code)]
#[derive(serde::Deserialize, Clone)]
pub struct Book {
  pub count: Option<i64>,
  pub id: String,
  pub title: String,
  pub subtitle: String,
  pub author_id: String,
  pub series_id: String,
  pub pages: i32,
  pub format: String,
  pub isbn: String,
  pub image_vertical: String,
  pub image_horizontal: String,
  pub description_short: String,
  pub description_long: String,
}
