use crate::application::{BooksGetDataUseCase, IBooksGetDataUseCase, ILanguageGetOneOrDefaultUseCase, LanguageGetOneOrDefaultUseCase};
use crate::domain::{Author, Book, BookWithAuthor};
use crate::infrastructure::http::adapters::BooksHttpAdapter;
use crate::infrastructure::http::middlewares::CurrentPath;
use crate::infrastructure::http::types::HtmlTemplate;
use crate::infrastructure::persistence::{FileSystemAuthorRepository, FileSystemBookRepository, FileSystemLanguageRepository};
use crate::presentation::views::{BooksTemplate, ServerErrorTemplate};
use rocket::get;
use rocket::response::{Responder, Response};
use rocket::{http::ContentType, Request};
use std::sync::Arc;

#[get("/books")]
pub async fn books_route_with_lang(current_path: CurrentPath) -> HtmlTemplate<BooksTemplate, ServerErrorTemplate> {
  let language_repository = FileSystemLanguageRepository {};
  let book_repository = FileSystemBookRepository {};
  let author_repository = FileSystemAuthorRepository {};
  let language_get_one_or_default_use_case = LanguageGetOneOrDefaultUseCase::new(Arc::new(language_repository));
  let book_use_case =
    BooksGetDataUseCase::<LanguageGetOneOrDefaultUseCase>::new(Arc::new(book_repository), Arc::new(author_repository), language_get_one_or_default_use_case);
  let books_http_adapter = BooksHttpAdapter::new(book_use_case);
  let book_data_result = books_http_adapter.execute(None, current_path.0).await;

  HtmlTemplate::new(book_data_result)
}
