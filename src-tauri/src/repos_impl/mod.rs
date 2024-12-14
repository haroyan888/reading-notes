pub mod book;
pub mod reading_note;

use serde::de::DeserializeOwned;
use std::fmt::{Debug, Display};
use std::fs;

pub fn read<T: Debug + DeserializeOwned>(path: &str) -> Result<Vec<T>, String> {
    let file = fs::read(path).map_err(|e| e.to_string())?;
    let data = serde_json::from_slice::<Vec<T>>(&file).map_err(|e| e.to_string())?;
    Ok(data)
}

pub fn write<T: Debug + DeserializeOwned + Display>(
    path: &str,
    data: Vec<T>,
) -> Result<Vec<T>, String> {
    let writable_data_list = data
        .iter()
        .enumerate()
        .map(|item| {
            if item.0 == 0 {
                item.1.to_string()
            } else {
                format!(", {}", item.1)
            }
        })
        .collect::<Vec<String>>();
    let mut write_data = String::new();
    for writable_data in writable_data_list {
        write_data.push_str(&writable_data);
    }
    fs::write(path, format!("[{}]", write_data)).map_err(|e| e.to_string())?;
    read::<T>(path)
}
