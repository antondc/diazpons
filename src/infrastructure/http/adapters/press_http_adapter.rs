use crate::application::IPressGetDataUseCase;
use crate::presentation::types::ViewData;
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

  pub async fn execute(&self, slug: Option<String>, current_path: String) -> Result<PressTemplate, ServerErrorTemplate> {
    if let Ok(press_data) = self
      .press_use_case
      .execute(slug.clone())
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      let view_data = ViewData {
        data: press_data.data,
        language: press_data.language.clone(),
        current_path,
        current_slug: slug.unwrap_or(press_data.language.slug),
      };
      let rendered = press_view(view_data).await;

      Ok(rendered)
    } else {
      Err(server_error_view().await)
    }
  }
}
