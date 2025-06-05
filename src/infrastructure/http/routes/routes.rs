use crate::application::{HomeGetDataUseCase, IHomeGetDataUseCase};
use crate::domain::{Author, Book, BookWithAuthor};
use crate::infrastructure::http::adapters::HomeHttpAdapter;
use crate::infrastructure::persistence::{FileSystemAuthorRepository, FileSystemBookRepository};
use crate::presentation::views::home_view;
use axum::response::Html;
use serde_json;
use std::fs;
use std::sync::Arc;

pub async fn home() -> Html<String> {
    let book_repository = FileSystemBookRepository {};
    let author_repository = FileSystemAuthorRepository {};
    let home_use_case =
        HomeGetDataUseCase::new(Arc::new(book_repository), Arc::new(author_repository));
    let home_http_adapter = HomeHttpAdapter::new(home_use_case);

    home_http_adapter.execute().await
}
