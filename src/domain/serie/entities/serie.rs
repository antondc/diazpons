use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Clone, Debug)]
pub struct Serie {
  pub count: Option<i64>,
  pub id: String,
  pub name: String,
}
