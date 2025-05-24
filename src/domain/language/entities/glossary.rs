use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all(deserialize = "snake_case", serialize = "camelCase"))]
pub struct Glossary {
    pub book: String,
    pub not_found: String,
    pub server_error: String,
    pub created_at: String,
    pub updated_at: String,
}
