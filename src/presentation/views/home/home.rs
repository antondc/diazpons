use crate::presentation::layouts::layout_five_columns;
use axum::response::Html;
use serde_json::Value;
use std::fs;
use tera::{Context, Tera};

pub async fn home_view(data: Value) -> Html<String> {
    let mut tera = Tera::default();

    // Load both templates
    tera.add_template_file("src/presentation/views/home/home.tera", Some("home"))
        .expect("Failed to load home template");

    // Render the inner view (home)
    let mut inner_ctx = Context::new();
    inner_ctx.insert("data", &data);
    let inner_html = tera
        .render("home", &inner_ctx)
        .expect("Failed to render home");

    let wrapped_html = layout_five_columns(tera, inner_html).await;

    // Load outer HTML shell
    let shell = fs::read_to_string("src/presentation/index.html").expect("Failed to read shell");
    let final_html = shell.replace("<!--CONTENT-->", &wrapped_html);

    Html(final_html)
}
