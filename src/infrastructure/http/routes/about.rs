use crate::application::{AboutGetDataUseCase, IAboutGetDataUseCase, ILanguageGetOneOrDefaultUseCase, LanguageGetOneOrDefaultUseCase};
use crate::infrastructure::http::adapters::AboutHttpAdapter;
use crate::infrastructure::http::middlewares::CurrentPath;
use crate::infrastructure::http::types::HtmlTemplate;
use crate::infrastructure::persistence::FileSystemLanguageRepository;
use crate::presentation::views::{AboutTemplate, ServerErrorTemplate};
use rocket::get;
use rocket::response::{Responder, Response};
use rocket::Request;
use std::sync::Arc;

#[get("/about")]
pub async fn about_route_with_lang(current_path: CurrentPath) -> HtmlTemplate<AboutTemplate, ServerErrorTemplate> {
  let language_repository = FileSystemLanguageRepository {};
  let language_get_one_or_default_use_case = LanguageGetOneOrDefaultUseCase::new(Arc::new(language_repository));
  let about_use_case = AboutGetDataUseCase::<LanguageGetOneOrDefaultUseCase>::new(language_get_one_or_default_use_case);
  let about_http_adapter = AboutHttpAdapter::new(about_use_case);
  let about_data_result = about_http_adapter.execute(None, current_path.0).await;

  HtmlTemplate::new(about_data_result)
}
