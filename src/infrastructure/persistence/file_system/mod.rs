mod author_repository;
mod book_repository;
mod serie_repository;
pub use author_repository::FileSystemAuthorRepository;
pub use book_repository::FileSystemBookRepository;
pub use serie_repository::FileSystemSerieRepository;
