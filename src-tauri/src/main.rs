#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use cmd::library::{get_default_sticker_dir, StickerDBState};
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

pub mod cfg;
pub mod cmd;
pub mod library;
pub mod search;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Init sticker databasestate not managed for field state on command You must call `.manage()` before using this command
            let dir = cfg::get_config_value(app.handle().clone(), "db.sticker-dir")
                .map_err(|e| e.to_string())?
                .map(|json| json.as_str().map(|s| s.to_owned()))
                .flatten()
                .or_else(|| get_default_sticker_dir(app.handle().clone()).ok())
                .expect("fail to get sticker directory");
            log::info!("Load sticker from {}", &dir);
            let state = StickerDBState::new(PathBuf::from(dir)).map_err(|e| e.to_string())?;
            app.manage(state);
            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            cmd::library::get_default_sticker_dir,
            cmd::library::create_pic_sticker,
            cmd::library::create_text_sticker,
            cmd::library::has_sticker_file,
            cmd::library::search_package,
            cmd::library::search_sticker,
            cmd::library::count_search_sticker_page,
            cmd::library::search_tag_ns,
            cmd::library::search_tag_value,
            cmd::library::is_path_blacklist,
            cmd::library::blacklist_path,
            cmd::library::get_sticker_by_id,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
