use crate::application::ILanguageGetOneOrDefaultUseCase;
use crate::domain::Language;
use crate::presentation::views::server_error_view;
use crate::presentation::views::ServerErrorTemplate;
use rocket::http::Status;
use rocket::response::status as rocketStatus;
use rocket::serde::json::Json;

pub struct HomeRedirectHttpAdapter<T> {
  language_get_one_or_default_use_case: T,
}

impl<T: ILanguageGetOneOrDefaultUseCase> HomeRedirectHttpAdapter<T> {
  pub fn new(language_get_one_or_default_use_case: T) -> HomeRedirectHttpAdapter<T> {
    HomeRedirectHttpAdapter {
      language_get_one_or_default_use_case,
    }
  }

  pub async fn execute(&self) -> Result<Language, ServerErrorTemplate> {
    if let Ok(default_language) = self
      .language_get_one_or_default_use_case
      .execute(None)
      .await
      .map_err(|error| rocketStatus::Custom(Status::new(error.status), Json(error)))
    {
      Ok(default_language)
    } else {
      Err(server_error_view().await)
    }
  }
}
