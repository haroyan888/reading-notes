use async_trait::async_trait;
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::entitiy::reading_note::ReadingNote;
use crate::repos::reading_note::{ReadingNoteRepository, RepositoryError};
use crate::repos_impl::{read, write};

pub struct ReadingNoteRepositoryForJson {
    path: String,
}

impl ReadingNoteRepositoryForJson {
    pub fn new(path: &str) -> Result<Self, RepositoryError> {
        if !Path::new(path).is_file() {
            fs::File::create(path)
                .map_err(|e| RepositoryError::Unexpected(e.to_string()))?
                .write("[]".as_bytes())
                .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        }
        Ok(ReadingNoteRepositoryForJson {
            path: path.to_string(),
        })
    }
}

#[async_trait]
impl ReadingNoteRepository for ReadingNoteRepositoryForJson {
    async fn all(&self, isbn_13: &str) -> Result<Vec<ReadingNote>, RepositoryError> {
        let data = read::<ReadingNote>(&self.path)
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        Ok(data
            .iter()
            .filter(|&reading_note| reading_note.isbn_13 == isbn_13)
            .cloned()
            .collect::<Vec<ReadingNote>>())
    }

    async fn create(&self, text: &str, isbn_13: &str) -> Result<ReadingNote, RepositoryError> {
        let mut data = read::<ReadingNote>(&self.path)
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        let reading_note = ReadingNote::new(isbn_13, text);
        data.push(reading_note.clone());
        write::<ReadingNote>(&self.path, data)
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        Ok(reading_note)
    }

    async fn delete(&self, id: &str) -> Result<(), RepositoryError> {
        let data = read::<ReadingNote>(&self.path)
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;

        if !data.iter().any(|reading_note| reading_note.id == id) {
            return Err(RepositoryError::NotFound(id.to_string()));
        }

        let deleted_data = data
            .iter()
            .filter(|&reading_note| reading_note.id != id)
            .cloned()
            .collect::<Vec<ReadingNote>>();
        write::<ReadingNote>(&self.path, deleted_data)
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        Ok(())
    }
}
