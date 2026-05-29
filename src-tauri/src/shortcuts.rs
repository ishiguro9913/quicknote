use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

/// デフォルトのグローバルホットキー。Phase 3 で設定変更できるようにする予定。
pub const DEFAULT_SHORTCUT: &str = "CmdOrCtrl+Shift+N";

/// グローバルショートカットを登録する。
/// 押されたらランチャーの表示/非表示をトグルする。
pub fn register(app: &AppHandle) -> Result<(), String> {
    let shortcut: Shortcut = DEFAULT_SHORTCUT
        .parse()
        .map_err(|e| format!("ショートカットの解析に失敗: {e:?}"))?;

    app.global_shortcut()
        .on_shortcut(shortcut, |app, _shortcut, event| {
            // キーが押された瞬間だけ反応（離した時は無視）
            if event.state() == ShortcutState::Pressed {
                toggle_launcher(app);
            }
        })
        .map_err(|e| format!("ショートカット登録に失敗: {e}"))?;

    Ok(())
}

/// ランチャーの表示/非表示を切り替える。
/// 表示するときは画面中央に出してフォーカスを当てる。
fn toggle_launcher(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("launcher") {
        let visible = win.is_visible().unwrap_or(false);
        if visible {
            let _ = win.hide();
        } else {
            let _ = win.center();
            let _ = win.show();
            let _ = win.set_focus();
        }
    }
}
