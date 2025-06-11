use crate::application::IBooksGetDataUseCase;
use crate::presentation::types::ViewData;
use crate::presentation::views::books_view;
use crate::presentation::views::server_error_view;
use crate::presentation::views::BooksTemplate;
use crate::presentation::views::ServerErrorTemplate;
use rocket::http::Status;
use rocket::response::status as rocketStatus;
use rocket::serde::json::Json;

pub struct BooksHttpAdapter<T> {
  books_get_data_use_case: T,
}

impl<T: IBooksGetDataUseCase> BooksHttpAdapter<T> {
  pub fn new(books_get_data_use_case: T) -> BooksHttpAdapter<T> {
    BooksHttpAdapter { books_get_data_use_case }
  }

  pub async fn execute(&self, slug: Option<String>, current_path: String) -> Result<BooksTemplate, ServerErrorTemplate> {
    if let Ok(books_data) = self
      .books_get_data_use_case
      .execute(slug.clone())
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      let view_data = ViewData {
        data: books_data.data,
        language: books_data.language.clone(),
        current_path,
        current_slug: slug.unwrap_or(books_data.language.slug),
      };
      let rendered = books_view(view_data).await;

      Ok(rendered)
    } else {
      Err(server_error_view().await)
    }
  }
}
