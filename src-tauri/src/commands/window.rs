use tauri::{AppHandle, Manager, Window};

/// 呼び出し元ウィンドウ（ランチャー）を隠す。
/// 引数 `window: Window` は「このコマンドを呼んだウィンドウ」を Tauri が自動で渡してくれる。
#[tauri::command]
pub fn hide_launcher(window: Window) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

/// 一覧ウィンドウ（label = "list"）を表示して前面に出す。
#[tauri::command]
pub fn open_list(app: AppHandle) -> Result<(), String> {
    if let Some(list) = app.get_webview_window("list") {
        list.show().map_err(|e| e.to_string())?;
        list.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}
