use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BookInfo {
    pub isbn_13: String,
    pub title: String,
    pub authors: Vec<String>,
    pub publisher: String,
    pub published_date: String,
    pub description: String,
    pub image_url: String,
    pub is_complete: bool,
}

impl BookInfo {
    pub fn switch_complete(&mut self) {
        self.is_complete = !self.is_complete;
    }
}

impl std::fmt::Display for BookInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\"isbn_13\":\"{}\",\"title\":\"{}\",\"authors\":{:?},\"publisher\":\"{}\",\"published_date\":\"{}\",\"description\":\"{}\",\"image_url\":\"{}\",\"is_complete\":{}}}",
            self.isbn_13,
            self.title,
            self.authors,
            self.publisher,
            self.published_date,
            self.description,
            self.image_url,
            self.is_complete,
        )
    }
}
