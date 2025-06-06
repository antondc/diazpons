use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all(deserialize = "snake_case", serialize = "camelCase"))]
pub struct Meta {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub total_items: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub offset: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub size: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sort: Option<String>,
}
