use axum::response::Html;
use serde_json::Value;
use std::fs;
use tera::{Context, Tera};

pub async fn header() -> Html<String> {
    let mut tera = Tera::default();
    tera.add_template_file("src/presentation/components/header/header.tera", Some("home"))
        .expect("Failed to load template");

    let data_str = fs::read_to_string("src/infrastructure/http/routes/data.json")
        .expect("Failed to read JSON");
    let json: Value = serde_json::from_str(&data_str).expect("Invalid JSON");

    let css =
        fs::read_to_string("src/presentation/components/header/header.css").expect("Failed to read CSS");

    let js = fs::read_to_string("src/presentation/components/header/header.js").expect("Failed to read JS");

    let mut context = Context::new();
    context.insert("data", &json);
    context.insert("css", &css);
    context.insert("js", &js);

    let html = tera.render("home", &context).expect("Failed to render");

    Html(html)
}
