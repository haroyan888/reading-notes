use std::fs;
use std::io::Write;
use std::path::Path;

use async_trait::async_trait;

use super::super::entitiy::book::BookInfo;
use super::super::repos::book::{BookRepository, BookSearcher, RepositoryError, SearchBooksResult};
use crate::repos_impl::{read, write};

#[derive(Clone)]
pub struct BookRepositoryForJson {
    path: String,
}

impl BookRepositoryForJson {
    pub fn new(path: &str) -> Result<Self, RepositoryError> {
        if !Path::new(path).is_file() {
            fs::File::create(path)
                .map_err(|e| RepositoryError::Unexpected(e.to_string()))?
                .write("[]".as_bytes())
                .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        }
        Ok(BookRepositoryForJson {
            path: path.to_string(),
        })
    }
}

#[async_trait]
impl BookRepository for BookRepositoryForJson {
    async fn find(&self, isbn_13: &str) -> Result<BookInfo, RepositoryError> {
        let data =
            read::<BookInfo>(&self.path).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        let option_book = data
            .iter()
            .find(|&book| book.isbn_13 == isbn_13)
            .ok_or_else(|| RepositoryError::NotFound(isbn_13.to_string()))?;
        Ok(option_book.clone())
    }

    async fn all(&self) -> Result<Vec<BookInfo>, RepositoryError> {
        Ok(read::<BookInfo>(&self.path).map_err(|e| RepositoryError::Unexpected(e.to_string()))?)
    }

    async fn create(&self, payload: BookInfo) -> Result<BookInfo, RepositoryError> {
        let mut data =
            read::<BookInfo>(&self.path).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        if data.iter().any(|book| book.isbn_13 == payload.isbn_13) {
            return Err(RepositoryError::Registered(payload.isbn_13.clone()));
        }
        data.push(payload.clone());
        write::<BookInfo>(&self.path, data)
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        let new_data =
            read::<BookInfo>(&self.path).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        let book_info = new_data
            .iter()
            .find(|&book| book.isbn_13 == payload.isbn_13)
            .ok_or_else(|| RepositoryError::Unexpected("registration failed".to_string()))?;
        Ok(book_info.clone())
    }

    async fn switch_complete(&self, isbn_13: &str) -> Result<BookInfo, RepositoryError> {
        let mut write_data =
            read::<BookInfo>(&self.path).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;

        let mut option_book = None;
        for book in &mut write_data {
            if book.isbn_13 == isbn_13 {
                book.switch_complete();
                option_book = Some(book.clone());
            }
        }
        let res_book = option_book.ok_or(RepositoryError::NotFound(isbn_13.to_string()))?;

        write::<BookInfo>(&self.path, write_data)
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;

        Ok(res_book)
    }

    async fn delete(&self, isbn_13: &str) -> Result<(), RepositoryError> {
        let data =
            read::<BookInfo>(&self.path).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        if !data.iter().any(|book| book.isbn_13 == isbn_13) {
            return Err(RepositoryError::NotFound(isbn_13.to_string()));
        }
        let removed_data = data
            .iter()
            .filter(|&book| book.isbn_13 != isbn_13)
            .cloned()
            .collect::<Vec<BookInfo>>();
        write(&self.path, removed_data).map_err(|e| RepositoryError::Unexpected(e.to_string()))?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct BookSearcherForGoogleAPI {}

impl BookSearcherForGoogleAPI {
    pub fn new() -> Self {
        BookSearcherForGoogleAPI {}
    }
}

#[async_trait]
impl BookSearcher for BookSearcherForGoogleAPI {
    async fn find(&self, isbn_13: &str) -> Result<BookInfo, RepositoryError> {
        const URL: &str = "https://www.googleapis.com/books/v1/volumes?q=isbn:";
        let res = reqwest::get(format!("{}{}", URL, &isbn_13))
            .await
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?
            .text()
            .await
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;

        let search_books_result = serde_json::from_str::<SearchBooksResult>(&res)
            .map_err(|e| RepositoryError::Unexpected(e.to_string()))?;

        if search_books_result.items.is_none() {
            return Err(RepositoryError::NotFound(isbn_13.to_string()));
        }

        // isbnで一意に検索しているためインデックス0で指定
        let book = search_books_result.items.unwrap()[0]
            .volume_info
            .to_book_info();

        // Googleがisbn不一致でも良しなに変換してくれるが、ここでははじく
        if isbn_13 != book.isbn_13 {
            return Err(RepositoryError::NotFound(isbn_13.to_string()));
        }

        Ok(book)
    }
}
