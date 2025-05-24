

use super::super::Glossary;
use crate::types::DateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all(deserialize = "snake_case", serialize = "camelCase"))]
pub struct Language {
  pub count: Option<i64>,
  pub id: String,
  pub slug: String,
  pub name: String,
  pub is_default: bool,
  pub glossary: Glossary,
  pub updated_at: Option<DateTime>,
}
