use serde_json::Value;
use std::fs;

use crate::presentation::views::home_view;
use axum::response::Html;

pub async fn home() -> Html<String> {
    let data_str = fs::read_to_string("src/infrastructure/persistence/json/data.json")
        .expect("Failed to read JSON");
    let json: Value = serde_json::from_str(&data_str).expect("Invalid JSON");

    home_view(json).await
}
