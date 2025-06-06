use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Links {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub self_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub next_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub previous_url: Option<String>,
}
