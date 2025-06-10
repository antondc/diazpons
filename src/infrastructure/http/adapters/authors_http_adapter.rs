use crate::application::IAuthorsGetDataUseCase;
use crate::infrastructure::http::middlewares::QueryParams;
use crate::presentation::views::authors_view;
use crate::presentation::views::server_error_view;
use crate::presentation::views::AuthorsTemplate;
use crate::presentation::views::ServerErrorTemplate;
use rocket::http::Status;
use rocket::response::status as rocketStatus;
use rocket::serde::json::Json;

pub struct AuthorsHttpAdapter<T> {
  authors_get_data_use_case: T,
}

impl<T: IAuthorsGetDataUseCase> AuthorsHttpAdapter<T> {
  pub fn new(authors_get_data_use_case: T) -> AuthorsHttpAdapter<T> {
    AuthorsHttpAdapter { authors_get_data_use_case }
  }

  pub async fn execute(&self, query_params: QueryParams) -> Result<AuthorsTemplate, ServerErrorTemplate> {
    if let Ok(authors_data) = self
      .authors_get_data_use_case
      .execute(query_params.lang.clone())
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      Ok(authors_view(authors_data).await)
    } else {
      Err(server_error_view().await)
    }
  }
}
