use crate::application::IAuthorsGetDataUseCase;
use crate::presentation::types::ViewData;
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

  pub async fn execute(&self, slug: Option<String>, current_path: String) -> Result<AuthorsTemplate, ServerErrorTemplate> {
    if let Ok(authors_data) = self
      .authors_get_data_use_case
      .execute(slug.clone())
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      let view_data = ViewData {
        data: authors_data.data,
        language: authors_data.language.clone(),
        current_path,
        current_slug: slug.unwrap_or(authors_data.language.slug),
      };
      let rendered = authors_view(view_data).await;

      Ok(rendered)
    } else {
      Err(server_error_view().await)
    }
  }
}
