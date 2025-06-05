#[allow(dead_code)]
#[derive(serde::Deserialize, Clone)]
pub struct Author {
    pub id: String,
    pub name: String,
    pub image: String,
    pub biography: String,
    pub books: Vec<String>,
}
