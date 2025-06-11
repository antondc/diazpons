use super::super::middlewares::QueryParams;
use crate::application::{PressGetDataUseCase, IPressGetDataUseCase, ILanguageGetOneOrDefaultUseCase, LanguageGetOneOrDefaultUseCase};
use crate::infrastructure::http::adapters::PressHttpAdapter;
use crate::infrastructure::http::types::HtmlTemplate;
use crate::infrastructure::persistence::FileSystemLanguageRepository;
use crate::presentation::views::{PressTemplate, ServerErrorTemplate};
use rocket::get;
use rocket::response::{Responder, Response};
use rocket::{http::ContentType, Request};
use std::sync::Arc;

#[get("/press")]
pub async fn press_route(query_params: QueryParams) -> HtmlTemplate<PressTemplate, ServerErrorTemplate> {
  let language_repository = FileSystemLanguageRepository {};
  let language_get_one_or_default_use_case = LanguageGetOneOrDefaultUseCase::new(Arc::new(language_repository));
  let press_use_case = PressGetDataUseCase::<LanguageGetOneOrDefaultUseCase>::new(language_get_one_or_default_use_case);
  let press_http_adapter = PressHttpAdapter::new(press_use_case);
  let press_data_result = press_http_adapter.execute(query_params).await;

  HtmlTemplate::new(press_data_result)
}
