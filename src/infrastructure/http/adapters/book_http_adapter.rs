use crate::application::IBookGetDataUseCase;
use crate::presentation::types::ViewData;
use crate::presentation::views::book_view;
use crate::presentation::views::server_error_view;
use crate::presentation::views::BookTemplate;
use crate::presentation::views::ServerErrorTemplate;
use rocket::http::Status;
use rocket::response::status as rocketStatus;
use rocket::serde::json::Json;

pub struct BookHttpAdapter<T> {
  book_use_case: T,
}

impl<T: IBookGetDataUseCase> BookHttpAdapter<T> {
  pub fn new(book_use_case: T) -> BookHttpAdapter<T> {
    BookHttpAdapter { book_use_case }
  }

  pub async fn execute(&self, slug: Option<String>, book_id: String, current_path: String) -> Result<BookTemplate, ServerErrorTemplate> {
    if let Ok(book_data) = self
      .book_use_case
      .execute(slug.clone(), book_id)
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      let view_data = ViewData {
        data: book_data.data,
        language: book_data.language.clone(),
        current_path,
        current_slug: slug.unwrap_or(book_data.language.slug),
      };
      let rendered = book_view(view_data).await;

      Ok(rendered)
    } else {
      Err(server_error_view().await)
    }
  }
}
