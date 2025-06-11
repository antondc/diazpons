use crate::application::ISerieGetDataUseCase;
use crate::infrastructure::http::middlewares::QueryParams;
use crate::presentation::views::serie_view;
use crate::presentation::views::server_error_view;
use crate::presentation::views::SerieTemplate;
use crate::presentation::views::ServerErrorTemplate;
use rocket::http::Status;
use rocket::response::status as rocketStatus;
use rocket::serde::json::Json;

pub struct SerieHttpAdapter<T> {
  serie_use_case: T,
}

impl<T: ISerieGetDataUseCase> SerieHttpAdapter<T> {
  pub fn new(serie_use_case: T) -> SerieHttpAdapter<T> {
    SerieHttpAdapter { serie_use_case }
  }

  pub async fn execute(&self, query_params: QueryParams, serie_id: String) -> Result<SerieTemplate, ServerErrorTemplate> {
    if let Ok(serie_data) = self
      .serie_use_case
      .execute(query_params.lang.clone(), serie_id)
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      Ok(serie_view(serie_data).await)
    } else {
      Err(server_error_view().await)
    }
  }
}
