use super::super::middlewares::QueryParams;
use crate::application::{AuthorsGetDataUseCase, IAuthorsGetDataUseCase, ILanguageGetOneOrDefaultUseCase, LanguageGetOneOrDefaultUseCase};
use crate::domain::{Author, Book, BookWithAuthor};
use crate::infrastructure::http::adapters::AuthorsHttpAdapter;
use crate::infrastructure::http::types::HtmlTemplate;
use crate::infrastructure::persistence::{FileSystemAuthorRepository, FileSystemBookRepository, FileSystemLanguageRepository};
use crate::presentation::views::{AuthorsTemplate, ServerErrorTemplate};
use rocket::get;
use rocket::response::{Responder, Response};
use rocket::{http::ContentType, Request};
use std::sync::Arc;

#[get("/authors")]
pub async fn authors_route(query_params: QueryParams) -> HtmlTemplate<AuthorsTemplate, ServerErrorTemplate> {
  let language_repository = FileSystemLanguageRepository {};
  let authors_repository = FileSystemBookRepository {};
  let author_repository = FileSystemAuthorRepository {};
  let language_get_one_or_default_use_case = LanguageGetOneOrDefaultUseCase::new(Arc::new(language_repository));
  let authors_use_case =
    AuthorsGetDataUseCase::<LanguageGetOneOrDefaultUseCase>::new(Arc::new(authors_repository), Arc::new(author_repository), language_get_one_or_default_use_case);
  let authors_http_adapter = AuthorsHttpAdapter::new(authors_use_case);
  let authors_data_result = authors_http_adapter.execute(query_params).await;

  HtmlTemplate::new(authors_data_result)
}
