use crate::application::{ILanguageGetOneOrDefaultUseCase, ISerieGetDataUseCase, LanguageGetOneOrDefaultUseCase, SerieGetDataUseCase};
use crate::infrastructure::http::adapters::SerieHttpAdapter;
use crate::infrastructure::http::middlewares::CurrentPath;
use crate::infrastructure::http::types::HtmlTemplate;
use crate::infrastructure::persistence::{FileSystemAuthorRepository, FileSystemBookRepository, FileSystemLanguageRepository, FileSystemSerieRepository};
use crate::presentation::views::{SerieTemplate, ServerErrorTemplate};
use rocket::get;
use rocket::response::{Responder, Response};
use rocket::{http::ContentType, Request};
use std::sync::Arc;

#[get("/serie/<serie_id>")]
pub async fn serie_route_with_lang(serie_id: String, current_path: CurrentPath) -> HtmlTemplate<SerieTemplate, ServerErrorTemplate> {
  let language_repository = FileSystemLanguageRepository {};
  let serie_repository = FileSystemSerieRepository {};
  let book_repository = FileSystemBookRepository {};
  let author_repository = FileSystemAuthorRepository {};
  let language_get_one_or_default_use_case = LanguageGetOneOrDefaultUseCase::new(Arc::new(language_repository));
  let serie_use_case = SerieGetDataUseCase::<LanguageGetOneOrDefaultUseCase>::new(
    Arc::new(serie_repository),
    Arc::new(book_repository),
    Arc::new(author_repository),
    language_get_one_or_default_use_case,
  );
  let serie_http_adapter = SerieHttpAdapter::new(serie_use_case);
  let serie_data_result = serie_http_adapter.execute(None, serie_id, current_path.0).await;

  HtmlTemplate::new(serie_data_result)
}
