use crate::entitiy::book::BookInfo;
use crate::handler::get_state;

#[tauri::command]
pub async fn all_book(app_handle: tauri::AppHandle) -> Result<Vec<BookInfo>, String> {
    let state = get_state(&app_handle)?;
    let book_repos = state.book_repos.lock().await;
    book_repos.all().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn find_book(app_handle: tauri::AppHandle, isbn_13: String) -> Result<BookInfo, String> {
    let state = get_state(&app_handle)?;
    let book_repos = state.book_repos.lock().await;
    book_repos.find(&isbn_13).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_book(
    app_handle: tauri::AppHandle,
    isbn_13: String,
) -> Result<BookInfo, String> {
    let state = get_state(&app_handle)?;
    let book_repos = state.book_repos.lock().await;
    let book_searcher = state.book_searcher.lock().await;

    let book = book_searcher
        .find(&isbn_13)
        .await
        .map_err(|e| e.to_string())?;

    book_repos.create(book).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn switch_complete_book(
    app_handle: tauri::AppHandle,
    isbn_13: String,
) -> Result<BookInfo, String> {
    let state = get_state(&app_handle)?;
    let book_repos = state.book_repos.lock().await;

    book_repos
        .switch_complete(&isbn_13)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_book(app_handle: tauri::AppHandle, isbn_13: String) -> Result<(), String> {
    let state = get_state(&app_handle)?;
    let book_repos = state.book_repos.lock().await;
    let reading_note_repos = state.reading_note_repos.lock().await;

    reading_note_repos
        .delete_all(&isbn_13)
        .await
        .map_err(|e| e.to_string())?;

    book_repos.delete(&isbn_13).await.map_err(|e| e.to_string())
}
