use crate::application::{AuthorGetDataUseCase, IAuthorGetDataUseCase, ILanguageGetOneOrDefaultUseCase, LanguageGetOneOrDefaultUseCase};
use crate::domain::{Author, Book, BookWithAuthorSerieReviews};
use crate::infrastructure::http::adapters::AuthorHttpAdapter;
use crate::infrastructure::http::middlewares::CurrentPath;
use crate::infrastructure::http::types::HtmlTemplate;
use crate::infrastructure::persistence::{FileSystemAuthorRepository, FileSystemBookRepository, FileSystemLanguageRepository};
use crate::presentation::views::{AuthorTemplate, ServerErrorTemplate};
use rocket::get;
use rocket::response::{Responder, Response};
use rocket::{http::ContentType, Request};
use std::sync::Arc;

#[get("/authors/<author_id>")]
pub async fn author_route_with_lang(author_id: String, current_path: CurrentPath) -> HtmlTemplate<AuthorTemplate, ServerErrorTemplate> {
  let language_repository = FileSystemLanguageRepository {};
  let book_repository = FileSystemBookRepository {};
  let author_repository = FileSystemAuthorRepository {};
  let language_get_one_or_default_use_case = LanguageGetOneOrDefaultUseCase::new(Arc::new(language_repository));
  let author_use_case =
    AuthorGetDataUseCase::<LanguageGetOneOrDefaultUseCase>::new(Arc::new(book_repository), Arc::new(author_repository), language_get_one_or_default_use_case);
  let author_http_adapter = AuthorHttpAdapter::new(author_use_case);
  let author_data_result = author_http_adapter.execute(None, current_path.0, author_id).await;

  HtmlTemplate::new(author_data_result)
}
