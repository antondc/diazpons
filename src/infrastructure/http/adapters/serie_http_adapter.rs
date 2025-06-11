use crate::application::ISerieGetDataUseCase;
use crate::infrastructure::http::routes::serie;
use crate::presentation::types::ViewData;
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

  pub async fn execute(&self, slug: Option<String>, serie_id: String, current_path: String) -> Result<SerieTemplate, ServerErrorTemplate> {
    if let Ok(serie_data) = self
      .serie_use_case
      .execute(slug.clone(), serie_id)
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      let view_data = ViewData {
        data: serie_data.data,
        language: serie_data.language.clone(),
        current_path,
        current_slug: slug.unwrap_or(serie_data.language.slug),
      };
      let rendered = serie_view(view_data).await;

      Ok(rendered)
    } else {
      Err(server_error_view().await)
    }
  }
}
