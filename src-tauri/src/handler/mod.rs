pub mod book;
pub mod reading_note;

use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

pub async fn get_state<T: ?Sized + Send + Sync + 'static>(
    app_handle: &tauri::AppHandle,
) -> Result<tauri::State<Arc<Mutex<T>>>, String> {
    app_handle
        .try_state::<Arc<Mutex<T>>>()
        .ok_or("データ取得に失敗しました".to_string())
}
