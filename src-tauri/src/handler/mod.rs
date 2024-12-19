pub mod book;
pub mod reading_note;

use tauri::Manager;

use crate::AppState;

pub fn get_state(app_handle: &tauri::AppHandle) -> Result<tauri::State<AppState>, String> {
    app_handle
        .try_state::<AppState>()
        .ok_or("データ取得に失敗しました".to_string())
}
