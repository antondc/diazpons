use crate::types::{Meta, Result};

use super::super::Serie;
use async_trait::async_trait;

#[allow(dead_code)]
#[async_trait]
pub trait ISerieRepository: Send + Sync {
  async fn serie_get_all(&self) -> Result<(Meta, Vec<Serie>)>;
}
