#[allow(dead_code)]
#[derive(serde::Deserialize, Clone)]
pub struct Serie {
  pub count: Option<i64>,
  pub id: String,
  pub name: String,
}
