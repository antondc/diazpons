use crate::application::IHomeGetDataUseCase;
use crate::infrastructure::http::middlewares::QueryParams;
use crate::presentation::views::home_view;
use crate::presentation::views::server_error_view;
use crate::presentation::views::HomeTemplate;
use crate::presentation::views::ServerErrorTemplate;
use rocket::http::Status;
use rocket::response::status as rocketStatus;
use rocket::serde::json::Json;

pub struct HomeHttpAdapter<T> {
  home_use_case: T,
}

impl<T: IHomeGetDataUseCase> HomeHttpAdapter<T> {
  pub fn new(home_use_case: T) -> HomeHttpAdapter<T> {
    HomeHttpAdapter { home_use_case }
  }

  pub async fn execute(&self, query_params: QueryParams) -> Result<HomeTemplate, ServerErrorTemplate> {
    if let Ok(home_data) = self
      .home_use_case
      .execute(query_params.lang.clone())
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      Ok(home_view(home_data).await)
    } else {
      Err(server_error_view().await)
    }
  }
}
