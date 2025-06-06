use super::CustomError;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, CustomError>;
