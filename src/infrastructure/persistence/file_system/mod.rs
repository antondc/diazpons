mod author_repository;
mod book_repository;
mod language_repository;
mod serie_repository;
pub use author_repository::FileSystemAuthorRepository;
pub use book_repository::FileSystemBookRepository;
pub use language_repository::FileSystemLanguageRepository;
pub use serie_repository::FileSystemSerieRepository;
