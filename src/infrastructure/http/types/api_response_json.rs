use super::{DataVariants, Links};
use crate::types::Meta;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ApiResponseJson<T> {
  pub status: u8,
  pub message: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub links: Option<Links>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub meta: Option<Meta>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<DataVariants<T>>,
}
