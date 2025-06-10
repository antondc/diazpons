use crate::application::IBooksGetDataUseCase;
use crate::infrastructure::http::middlewares::QueryParams;
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

  pub async fn execute(&self, query_params: QueryParams) -> Result<BooksTemplate, ServerErrorTemplate> {
    if let Ok(books_data) = self
      .books_get_data_use_case
      .execute(query_params.lang.clone())
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      Ok(books_view(books_data).await)
    } else {
      Err(server_error_view().await)
    }
  }
}
