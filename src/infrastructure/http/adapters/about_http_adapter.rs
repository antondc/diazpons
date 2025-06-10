use crate::application::IAboutGetDataUseCase;
use crate::infrastructure::http::middlewares::QueryParams;
use crate::presentation::views::about_view;
use crate::presentation::views::server_error_view;
use crate::presentation::views::AboutTemplate;
use crate::presentation::views::ServerErrorTemplate;
use rocket::http::Status;
use rocket::response::status as rocketStatus;
use rocket::serde::json::Json;

pub struct AboutHttpAdapter<T> {
  about_use_case: T,
}

impl<T: IAboutGetDataUseCase> AboutHttpAdapter<T> {
  pub fn new(about_use_case: T) -> AboutHttpAdapter<T> {
    AboutHttpAdapter { about_use_case }
  }

  pub async fn execute(&self, query_params: QueryParams) -> Result<AboutTemplate, ServerErrorTemplate> {
    if let Ok(about_data) = self
      .about_use_case
      .execute(query_params.lang.clone())
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      Ok(about_view(about_data).await)
    } else {
      Err(server_error_view().await)
    }
  }
}
