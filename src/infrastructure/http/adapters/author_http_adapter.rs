use crate::application::IAuthorGetDataUseCase;
use crate::presentation::types::ViewData;
use crate::presentation::views::author_view;
use crate::presentation::views::server_error_view;
use crate::presentation::views::AuthorTemplate;
use crate::presentation::views::ServerErrorTemplate;
use rocket::http::Status;
use rocket::response::status as rocketStatus;
use rocket::serde::json::Json;

pub struct AuthorHttpAdapter<T> {
  author_get_data_use_case: T,
}

impl<T: IAuthorGetDataUseCase> AuthorHttpAdapter<T> {
  pub fn new(author_get_data_use_case: T) -> AuthorHttpAdapter<T> {
    AuthorHttpAdapter { author_get_data_use_case }
  }

  pub async fn execute(&self, slug: Option<String>, current_path: String, author_id: String) -> Result<AuthorTemplate, ServerErrorTemplate> {
    if let Ok(author_data) = self
      .author_get_data_use_case
      .execute(slug.clone(), author_id)
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      let view_data = ViewData {
        data: author_data.data,
        language: author_data.language.clone(),
        current_path,
        current_slug: slug.unwrap_or(author_data.language.slug),
      };
      let rendered = author_view(view_data).await;

      Ok(rendered)
    } else {
      Err(server_error_view().await)
    }
  }
}
