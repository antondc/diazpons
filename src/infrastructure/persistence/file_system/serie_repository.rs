use async_trait::async_trait;
use std::fs;

use crate::{
  domain::{ISerieRepository, Serie},
  types::{Meta, Result},
};

#[derive(Copy, Clone)]
pub struct FileSystemSerieRepository;

#[async_trait]
impl ISerieRepository for FileSystemSerieRepository {
  async fn serie_get_all(&self) -> Result<(Meta, Vec<Serie>)> {
    let json_series = fs::read_to_string("data/series.json").expect("Failed to read JSON");
    let series: Vec<Serie> = serde_json::from_str(&json_series).expect("Invalid JSON");

    let count = series.first().map(|item| item.count.unwrap_or(0));

    let meta = Meta {
      total_items: count,
      offset: None,
      size: None,
      sort: None,
    };

    Ok((meta, series))
  }
}
