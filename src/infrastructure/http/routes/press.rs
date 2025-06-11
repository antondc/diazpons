use crate::application::{ILanguageGetOneOrDefaultUseCase, IPressGetDataUseCase, LanguageGetOneOrDefaultUseCase, PressGetDataUseCase};
use crate::infrastructure::http::adapters::PressHttpAdapter;
use crate::infrastructure::http::middlewares::CurrentPath;
use crate::infrastructure::http::types::HtmlTemplate;
use crate::infrastructure::persistence::FileSystemLanguageRepository;
use crate::presentation::views::{PressTemplate, ServerErrorTemplate};
use rocket::get;
use rocket::response::{Responder, Response};
use rocket::{http::ContentType, Request};
use std::sync::Arc;

#[get("/<slug>/press", rank = 6)]
pub async fn press_route_with_lang(slug: String, current_path: CurrentPath) -> HtmlTemplate<PressTemplate, ServerErrorTemplate> {
  let language_repository = FileSystemLanguageRepository {};
  let language_get_one_or_default_use_case = LanguageGetOneOrDefaultUseCase::new(Arc::new(language_repository));
  let press_use_case = PressGetDataUseCase::<LanguageGetOneOrDefaultUseCase>::new(language_get_one_or_default_use_case);
  let press_http_adapter = PressHttpAdapter::new(press_use_case);
  let press_data_result = press_http_adapter.execute(Some(slug), current_path.0).await;

  HtmlTemplate::new(press_data_result)
}
