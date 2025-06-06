use super::Session;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all(deserialize = "snake_case", serialize = "camelCase"))]
pub struct Value<T> {
  pub value_type: String,
  pub id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub session: Option<Session>,
  pub attributes: T,
}
