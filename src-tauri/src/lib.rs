mod entitiy;
mod handler;
mod repos;
mod repos_impl;

use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

use repos_impl::{
    book::{BookRepositoryForJson, BookSearcherForGoogleAPI},
    reading_note::ReadingNoteRepositoryForJson,
};

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = app.path().app_local_data_dir()?;
    let book_repos_path = base_dir.join("book-repos.json");
    app.manage(Arc::new(Mutex::new(BookRepositoryForJson::new(
        book_repos_path.to_str().unwrap(),
    )?)));
    app.manage(Arc::new(Mutex::new(BookSearcherForGoogleAPI::new())));
    let reading_note_repos_path = base_dir.join("reading_note-repos.json");
    app.manage(Arc::new(Mutex::new(ReadingNoteRepositoryForJson::new(
        reading_note_repos_path.to_str().unwrap(),
    )?)));
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            handler::book::all_book,
            handler::book::find_book,
            handler::book::create_book,
            handler::book::switch_complete_book,
            handler::book::delete_book,
            handler::reading_note::get_reading_notes,
            handler::reading_note::create_reading_note,
            handler::reading_note::delete_reading_note,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
