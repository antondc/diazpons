mod author;
mod book;
mod book_with_author;
mod serie;
pub use author::{Author, IAuthorRepository};
pub use book::{Book, IBookRepository};
pub use book_with_author::BookWithAuthor;
pub use serie::{ISerieRepository, Serie};
