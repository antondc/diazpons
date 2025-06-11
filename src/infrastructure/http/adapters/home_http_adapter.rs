use crate::application::IHomeGetDataUseCase;
use crate::presentation::types::ViewData;
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

  pub async fn execute(&self, slug: Option<String>, current_path: String) -> Result<HomeTemplate, ServerErrorTemplate> {
    if let Ok(home_data) = self
      .home_use_case
      .execute(slug.clone())
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      let view_data = ViewData {
        data: home_data.data,
        language: home_data.language.clone(),
        current_path,
        current_slug: slug.unwrap_or(home_data.language.slug),
      };
      let rendered = home_view(view_data).await;

      Ok(rendered)
    } else {
      Err(server_error_view().await)
    }
  }
}
