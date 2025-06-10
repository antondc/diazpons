use super::super::middlewares::QueryParams;
use crate::application::{AboutGetDataUseCase, IAboutGetDataUseCase, ILanguageGetOneOrDefaultUseCase, LanguageGetOneOrDefaultUseCase};
use crate::infrastructure::http::adapters::AboutHttpAdapter;
use crate::infrastructure::http::types::HtmlTemplate;
use crate::infrastructure::persistence::FileSystemLanguageRepository;
use crate::presentation::views::{AboutTemplate, ServerErrorTemplate};
use rocket::get;
use rocket::response::{Responder, Response};
use rocket::{http::ContentType, Request};
use std::sync::Arc;

#[get("/about")]
pub async fn about_route(query_params: QueryParams) -> HtmlTemplate<AboutTemplate, ServerErrorTemplate> {
  let language_repository = FileSystemLanguageRepository {};
  let language_get_one_or_default_use_case = LanguageGetOneOrDefaultUseCase::new(Arc::new(language_repository));
  let about_use_case = AboutGetDataUseCase::<LanguageGetOneOrDefaultUseCase>::new(language_get_one_or_default_use_case);
  let about_http_adapter = AboutHttpAdapter::new(about_use_case);
  let about_data_result = about_http_adapter.execute(query_params).await;

  HtmlTemplate::new(about_data_result)
}
