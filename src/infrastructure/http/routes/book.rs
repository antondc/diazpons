use super::super::middlewares::QueryParams;
use crate::application::{BookGetDataUseCase, IBookGetDataUseCase, ILanguageGetOneOrDefaultUseCase, LanguageGetOneOrDefaultUseCase};
use crate::domain::{Author, Book, BookWithAuthor};
use crate::infrastructure::http::adapters::BookHttpAdapter;
use crate::infrastructure::http::types::HtmlTemplate;
use crate::infrastructure::persistence::{FileSystemAuthorRepository, FileSystemBookRepository, FileSystemLanguageRepository};
use crate::presentation::views::{BookTemplate, ServerErrorTemplate};
use rocket::get;
use rocket::response::{Responder, Response};
use rocket::{http::ContentType, Request};
use std::sync::Arc;

#[get("/books/<book_id>")]
pub async fn book_route(query_params: QueryParams, book_id: String) -> HtmlTemplate<BookTemplate, ServerErrorTemplate> {
  let language_repository = FileSystemLanguageRepository {};
  let book_repository = FileSystemBookRepository {};
  let author_repository = FileSystemAuthorRepository {};
  let language_get_one_or_default_use_case = LanguageGetOneOrDefaultUseCase::new(Arc::new(language_repository));
  let book_use_case =
    BookGetDataUseCase::<LanguageGetOneOrDefaultUseCase>::new(Arc::new(book_repository), Arc::new(author_repository), language_get_one_or_default_use_case);
  let book_http_adapter = BookHttpAdapter::new(book_use_case);
  let book_data_result = book_http_adapter.execute(query_params, book_id).await;

  HtmlTemplate::new(book_data_result)
}
