use std::{path::PathBuf, fs, borrow::Cow};
use std::path::Path;
use sha256::try_digest;

use crate::db::{DATABASE, DATABASE_DIR, DATABASE_FILE_DIR, DatabaseMapper};


#[tauri::command]
pub fn get_table_version() -> u32 {
    DatabaseMapper::get_table_version_code(&DATABASE.lock().unwrap())
}

#[tauri::command]
pub fn get_database_dir() -> Cow<'static, str> {
    DATABASE_DIR.to_string_lossy()
}

pub fn add_file_to_library<P: AsRef<Path>>(file: P) -> String {
    let sha256 = try_digest(file.as_ref()).unwrap();
    let target = DATABASE_FILE_DIR.join(&sha256);
    fs::copy(file, target).unwrap();
    sha256
}