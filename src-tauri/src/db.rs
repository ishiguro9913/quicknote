use std::fs;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use tauri::{AppHandle, Manager};

/// アプリ全体で共有する DB 接続プール。
/// Tauri の `State` として管理し、各コマンドから参照する。
pub struct Db {
    pub pool: SqlitePool,
}

/// DB を初期化する。
/// OS ごとのアプリ専用データフォルダに `quicknote.db` を作り、
/// 接続してマイグレーション（テーブル作成）を適用する。
///
/// 戻り値が `Result<Db, String>` なのは「成功なら Db、失敗ならエラー文字列」を表す Rust の慣習。
/// `?` 演算子は「エラーならその場で早期 return する」糖衣構文。
pub async fn init(app: &AppHandle) -> Result<Db, String> {
    // macOS なら ~/Library/Application Support/<identifier>/ などOS標準の保存先
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("app_data_dir の取得に失敗: {e}"))?;

    // フォルダが無ければ作る（初回起動時）
    fs::create_dir_all(&dir).map_err(|e| format!("データフォルダ作成に失敗: {e}"))?;

    let db_path = dir.join("quicknote.db");

    // create_if_missing(true): DBファイルが無ければ自動作成
    let options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(options)
        .await
        .map_err(|e| format!("DB接続に失敗: {e}"))?;

    // migrations/ 配下のSQLを順に適用する。
    // 適用済みのものは自動でスキップされる（履歴は _sqlx_migrations テーブルで管理）。
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| format!("マイグレーションに失敗: {e}"))?;

    Ok(Db { pool })
}
