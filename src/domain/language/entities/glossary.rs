use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all(deserialize = "snake_case", serialize = "camelCase"))]
pub struct Glossary {
  pub home: String,
  pub catalog: String,
  pub authors: String,
  pub about_us: String,
  pub press: String,
  pub series: String,
  pub not_found: String,
  pub server_error: String,
}
