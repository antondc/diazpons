use tera::{Context, Tera};

pub async fn layout_five_columns(mut tera: Tera, inner_html: String) -> String {
    tera.add_template_file(
        "src/presentation/layouts/layout_five_columns/layout_five_columns.tera",
        Some("layout_five_columns"),
    )
    .expect("Failed to load layout_five_columns template");

    // Wrap with layout_five_columns
    let mut container_ctx = Context::new();
    container_ctx.insert("children", &inner_html);
    let wrapped_html = tera
        .render("layout_five_columns", &container_ctx)
        .expect("Failed to render layout_five_columns");

    wrapped_html
}
