mod commands;
mod db;

use commands::notes::{create_note, delete_note, list_notes};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // 起動時に DB を初期化し、State として管理する。
            // setup は同期関数なので、非同期の init を block_on で待つ。
            let handle = app.handle().clone();
            let db = tauri::async_runtime::block_on(db::init(&handle))
                .expect("DBの初期化に失敗しました");
            app.manage(db);
            Ok(())
        })
        // フロントから invoke で呼べるコマンドを登録する
        .invoke_handler(tauri::generate_handler![
            create_note,
            list_notes,
            delete_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
