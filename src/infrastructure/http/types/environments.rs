use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Eq, PartialEq)]
pub enum Environments {
  Development,
  Staging,
  Production,
}

impl FromStr for Environments {
  type Err = ();

  fn from_str(input: &str) -> Result<Environments, Self::Err> {
    match input {
      "development" => Ok(Environments::Development),
      "staging" => Ok(Environments::Staging),
      "production" => Ok(Environments::Production),
      _ => Ok(Environments::Production),
    }
  }
}

impl Display for Environments {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match self {
      Environments::Development => write!(f, "development"),
      Environments::Staging => write!(f, "staging"),
      Environments::Production => write!(f, "production"),
    }
  }
}
