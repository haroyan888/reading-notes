use async_trait::async_trait;
use thiserror::Error;

use super::super::entitiy::book::BookInfo;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Unexpected Error: [{0}]")]
    Unexpected(String),
    #[error("NotFound, isbn is {0}")]
    NotFound(String),
    #[error("Registered, isbn is {0}")]
    Registered(String),
}

#[async_trait]
pub trait BookRepository: Send + Sync + 'static {
    async fn find(&self, isbn_13: &str) -> Result<BookInfo, RepositoryError>;
    async fn all(&self) -> Result<Vec<BookInfo>, RepositoryError>;
    async fn create(&self, payload: BookInfo) -> Result<BookInfo, RepositoryError>;
    async fn switch_complete(&self, isbn_13: &str) -> Result<BookInfo, RepositoryError>;
    async fn delete(&self, isbn_13: &str) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait BookSearcher: Send + Sync + 'static {
    async fn find(&self, isbn_13: &str) -> Result<BookInfo, RepositoryError>;
}
