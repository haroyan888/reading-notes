use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReadingNote {
    pub id: String,
    pub isbn_13: String,
    pub text: String,
}

impl ReadingNote {
    pub fn new(isbn_13: &str, text: &str) -> Self {
        ReadingNote {
            id: uuid::Uuid::new_v4().to_string(),
            isbn_13: isbn_13.to_string(),
            text: text.to_string(),
        }
    }
}

impl std::fmt::Display for ReadingNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\"id\":\"{}\",\"isbn_13\":\"{}\",\"text\":\"{}\"}}",
            self.id, self.isbn_13, self.text,
        )
    }
}
