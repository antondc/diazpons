use crate::application::IBookGetDataUseCase;
use crate::infrastructure::http::middlewares::QueryParams;
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

  pub async fn execute(&self, query_params: QueryParams, book_id: String) -> Result<BookTemplate, ServerErrorTemplate> {
    if let Ok(book_data) = self
      .book_use_case
      .execute(query_params.lang.clone(), book_id)
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      Ok(book_view(book_data).await)
    } else {
      Err(server_error_view().await)
    }
  }
}
