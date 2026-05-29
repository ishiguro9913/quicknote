mod commands;
mod db;
mod shortcuts;

use commands::notes::{create_note, delete_note, list_notes};
use commands::window::{hide_launcher, open_list};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // 起動時に DB を初期化し、State として管理する。
            // setup は同期関数なので、非同期の init を block_on で待つ。
            let handle = app.handle().clone();
            let db = tauri::async_runtime::block_on(db::init(&handle))
                .expect("DBの初期化に失敗しました");
            app.manage(db);

            // macOS: Dock に出さず、メニューバー常駐型アプリにする
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // グローバルショートカット (Cmd+Shift+N) を登録
            shortcuts::register(&handle).expect("ショートカット登録に失敗しました");

            // ランチャーはフォーカスを失ったら自動で隠す（Raycast 的な挙動）
            if let Some(launcher) = app.get_webview_window("launcher") {
                let l = launcher.clone();
                launcher.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        let _ = l.hide();
                    }
                });
            }

            // 一覧ウィンドウは「閉じる」操作で破棄せず、隠すだけにする。
            // 破棄してしまうと次に >list したとき表示できなくなるため、
            // CloseRequested を横取りして prevent_close + hide する。
            if let Some(list) = app.get_webview_window("list") {
                let l = list.clone();
                list.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = l.hide();
                    }
                });
            }

            Ok(())
        })
        // フロントから invoke で呼べるコマンドを登録する
        .invoke_handler(tauri::generate_handler![
            create_note,
            list_notes,
            delete_note,
            hide_launcher,
            open_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
