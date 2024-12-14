use async_trait::async_trait;
use thiserror::Error;

use crate::entitiy::reading_note::ReadingNote;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Unexpected Error: [{0}]")]
    Unexpected(String),
    #[error("NotFound, id is {0}")]
    NotFound(String),
}

#[async_trait]
pub trait ReadingNoteRepository: Send + Sync + 'static {
    async fn all(&self, isbn_13: &str) -> Result<Vec<ReadingNote>, RepositoryError>;
    async fn create(&self, text: &str, isbn_13: &str) -> Result<ReadingNote, RepositoryError>;
    async fn delete(&self, id: &str) -> Result<(), RepositoryError>;
}
