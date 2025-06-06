use super::super::middlewares::QueryParams;
use crate::application::{HomeGetDataUseCase, IHomeGetDataUseCase, ILanguageGetOneOrDefaultUseCase, LanguageGetOneOrDefaultUseCase};
use crate::domain::{Author, Book, BookWithAuthor};
use crate::infrastructure::http::adapters::HomeHttpAdapter;
use crate::infrastructure::http::types::HtmlTemplate;
use crate::infrastructure::persistence::{FileSystemAuthorRepository, FileSystemBookRepository, FileSystemLanguageRepository};
use crate::presentation::views::{HomeTemplate, ServerErrorTemplate};
use rocket::get;
use rocket::response::{Responder, Response};
use rocket::{http::ContentType, Request};
use std::sync::Arc;

#[get("/")]
pub async fn home_route(query_params: QueryParams) -> HtmlTemplate<HomeTemplate, ServerErrorTemplate> {
  let language_repository = FileSystemLanguageRepository {};
  let book_repository = FileSystemBookRepository {};
  let author_repository = FileSystemAuthorRepository {};
  let language_get_one_or_default_use_case = LanguageGetOneOrDefaultUseCase::new(Arc::new(language_repository));
  let home_use_case =
    HomeGetDataUseCase::<LanguageGetOneOrDefaultUseCase>::new(Arc::new(book_repository), Arc::new(author_repository), language_get_one_or_default_use_case);
  let home_http_adapter = HomeHttpAdapter::new(home_use_case);
  let home_data_result = home_http_adapter.execute(query_params).await;

  HtmlTemplate::new(home_data_result)
}
