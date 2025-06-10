mod home;
mod server_error;
pub use home::{home_view, HomeTemplate};
pub use server_error::{server_error_view, ServerErrorTemplate};
mod book;
pub use book::{book_view, BookTemplate};
mod books;
pub use books::{books_view, BooksTemplate};
