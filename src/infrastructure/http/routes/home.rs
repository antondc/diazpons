use crate::application::{HomeGetDataUseCase, IHomeGetDataUseCase, ILanguageGetOneOrDefaultUseCase, LanguageGetOneOrDefaultUseCase};
use crate::domain::{Author, Book, BookWithAuthorSerieReviews};
use crate::infrastructure::http::adapters::HomeHttpAdapter;
use crate::infrastructure::http::middlewares::{CurrentPath, QueryParams};
use crate::infrastructure::http::types::HtmlTemplate;
use crate::infrastructure::persistence::{FileSystemAuthorRepository, FileSystemBookRepository, FileSystemLanguageRepository, FileSystemSerieRepository};
use crate::presentation::views::{HomeTemplate, ServerErrorTemplate};
use rocket::http::uri::Query;
use rocket::response::Redirect;
use rocket::response::{Responder, Response};
use rocket::{get, uri};
use rocket::{http::ContentType, Request};
use std::path::PathBuf;
use std::sync::Arc;

#[get("/")]
pub async fn home_route_with_lang(current_path: CurrentPath) -> HtmlTemplate<HomeTemplate, ServerErrorTemplate> {
  let language_repository = FileSystemLanguageRepository {};
  let book_repository = FileSystemBookRepository {};
  let author_repository = FileSystemAuthorRepository {};
  let serie_repository = FileSystemSerieRepository {};
  let language_get_one_or_default_use_case = LanguageGetOneOrDefaultUseCase::new(Arc::new(language_repository));
  let home_use_case = HomeGetDataUseCase::<LanguageGetOneOrDefaultUseCase>::new(
    Arc::new(book_repository),
    Arc::new(author_repository),
    Arc::new(serie_repository),
    language_get_one_or_default_use_case,
  );
  let home_http_adapter = HomeHttpAdapter::new(home_use_case);
  let home_data_result = home_http_adapter.execute(None, current_path.0).await;

  HtmlTemplate::new(home_data_result)
}
