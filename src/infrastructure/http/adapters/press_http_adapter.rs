use crate::application::IPressGetDataUseCase;
use crate::infrastructure::http::middlewares::QueryParams;
use crate::presentation::views::press_view;
use crate::presentation::views::server_error_view;
use crate::presentation::views::PressTemplate;
use crate::presentation::views::ServerErrorTemplate;
use rocket::http::Status;
use rocket::response::status as rocketStatus;
use rocket::serde::json::Json;

pub struct PressHttpAdapter<T> {
  press_use_case: T,
}

impl<T: IPressGetDataUseCase> PressHttpAdapter<T> {
  pub fn new(press_use_case: T) -> PressHttpAdapter<T> {
    PressHttpAdapter { press_use_case }
  }

  pub async fn execute(&self, query_params: QueryParams) -> Result<PressTemplate, ServerErrorTemplate> {
    if let Ok(press_data) = self
      .press_use_case
      .execute(query_params.lang.clone())
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      Ok(press_view(press_data).await)
    } else {
      Err(server_error_view().await)
    }
  }
}
