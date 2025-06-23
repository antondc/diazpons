use crate::application::{BookGetDataUseCase, IBookGetDataUseCase, ILanguageGetOneOrDefaultUseCase, LanguageGetOneOrDefaultUseCase};
use crate::domain::{Author, Book, BookWithAuthorSerieReviews};
use crate::infrastructure::http::adapters::BookHttpAdapter;
use crate::infrastructure::http::middlewares::CurrentPath;
use crate::infrastructure::http::types::HtmlTemplate;
use crate::infrastructure::persistence::{
  FileSystemAuthorRepository, FileSystemBookRepository, FileSystemLanguageRepository, FileSystemReviewRepository, FileSystemSerieRepository,
};
use crate::presentation::views::{BookTemplate, ServerErrorTemplate};
use rocket::get;
use rocket::response::{Responder, Response};
use rocket::{http::ContentType, Request};
use std::sync::Arc;

#[get("/books/<book_id>")]
pub async fn book_route_with_lang(book_id: String, current_path: CurrentPath) -> HtmlTemplate<BookTemplate, ServerErrorTemplate> {
  let language_repository = FileSystemLanguageRepository {};
  let book_repository = FileSystemBookRepository {};
  let author_repository = FileSystemAuthorRepository {};
  let serie_repository = FileSystemSerieRepository {};
  let review_repository = FileSystemReviewRepository {};

  let language_get_one_or_default_use_case = LanguageGetOneOrDefaultUseCase::new(Arc::new(language_repository));
  let book_use_case = BookGetDataUseCase::<LanguageGetOneOrDefaultUseCase>::new(
    Arc::new(book_repository),
    Arc::new(author_repository),
    Arc::new(serie_repository),
    Arc::new(review_repository),
    language_get_one_or_default_use_case,
  );
  let book_http_adapter = BookHttpAdapter::new(book_use_case);
  let book_data_result = book_http_adapter.execute(None, book_id, current_path.0).await;

  HtmlTemplate::new(book_data_result)
}
