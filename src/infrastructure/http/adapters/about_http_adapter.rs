use crate::application::IAboutGetDataUseCase;
use crate::presentation::types::ViewData;
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

  pub async fn execute(&self, slug: Option<String>, current_path: String) -> Result<AboutTemplate, ServerErrorTemplate> {
    if let Ok(about_data) = self
      .about_use_case
      .execute(slug.clone())
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      let view_data = ViewData {
        data: about_data.data,
        language: about_data.language.clone(),
        current_path,
        current_slug: slug.unwrap_or(about_data.language.slug),
      };
      let rendered = about_view(view_data).await;

      Ok(rendered)
    } else {
      Err(server_error_view().await)
    }
  }
}
