use crate::entitiy::book::BookInfo;
use crate::handler::get_state;
use crate::repos::book::{BookRepository, BookSearcher};
use crate::repos::reading_note::ReadingNoteRepository;
use crate::repos_impl::book::{BookRepositoryForJson, BookSearcherForGoogleAPI};
use crate::repos_impl::reading_note::ReadingNoteRepositoryForJson;

#[tauri::command]
pub async fn all_book(app_handle: tauri::AppHandle) -> Result<Vec<BookInfo>, String> {
    let state = get_state::<BookRepositoryForJson>(&app_handle).await?;
    let book_repos = state.lock().await;
    book_repos.all().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn find_book(app_handle: tauri::AppHandle, isbn_13: String) -> Result<BookInfo, String> {
    let state = get_state::<BookRepositoryForJson>(&app_handle).await?;
    let book_repos = state.lock().await;
    book_repos.find(&isbn_13).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_book(
    app_handle: tauri::AppHandle,
    isbn_13: String,
) -> Result<BookInfo, String> {
    let state_repos = get_state::<BookRepositoryForJson>(&app_handle).await?;
    let book_repos = state_repos.lock().await;
    let state_searcher = get_state::<BookSearcherForGoogleAPI>(&app_handle).await?;
    let book_searcher = state_searcher.lock().await;

    let book = book_searcher
        .find(&isbn_13)
        .await
        .map_err(|e| e.to_string())?;

    book_repos.create(book).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_book(app_handle: tauri::AppHandle, isbn_13: String) -> Result<(), String> {
    let state = get_state::<BookRepositoryForJson>(&app_handle).await?;
    let book_repos = state.lock().await;
    let state = get_state::<ReadingNoteRepositoryForJson>(&app_handle).await?;
    let reading_note_repos = state.lock().await;

    reading_note_repos
        .delete_all(&isbn_13)
        .await
        .map_err(|e| e.to_string())?;

    book_repos.delete(&isbn_13).await.map_err(|e| e.to_string())
}
