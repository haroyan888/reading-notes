use crate::entitiy::reading_note::ReadingNote;
use crate::handler::get_state;
use crate::repos::reading_note::ReadingNoteRepository;
use crate::repos_impl::reading_note::ReadingNoteRepositoryForJson;

#[tauri::command]
pub async fn get_reading_notes(
    app_handle: tauri::AppHandle,
    isbn_13: String,
) -> Result<Vec<ReadingNote>, String> {
    let state = get_state::<ReadingNoteRepositoryForJson>(&app_handle).await?;
    let reading_note_repos = state.lock().await;

    reading_note_repos
        .all(&isbn_13)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_reading_note(
    app_handle: tauri::AppHandle,
    isbn_13: String,
    text: String,
) -> Result<ReadingNote, String> {
    let state = get_state::<ReadingNoteRepositoryForJson>(&app_handle).await?;
    let reading_note_repos = state.lock().await;

    reading_note_repos
        .create(&text, &isbn_13)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_reading_note(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let state = get_state::<ReadingNoteRepositoryForJson>(&app_handle).await?;
    let reading_note_repos = state.lock().await;

    reading_note_repos
        .delete(&id)
        .await
        .map_err(|e| e.to_string())
}
