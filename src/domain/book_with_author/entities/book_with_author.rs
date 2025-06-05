use super::super::super::{Author, Book};

#[allow(dead_code)]
#[derive(serde::Deserialize, Clone)]
pub struct BookWithAuthor {
    pub book: Book,
    pub author: Author,
}
