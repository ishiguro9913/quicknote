use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tauri::State;

use crate::db::Db;

/// メモ1件。
/// - `Serialize`/`Deserialize`: フロント(JS)との JSON 変換を自動生成
/// - `FromRow`: SQLの結果行から Note への変換を自動生成（sqlx 用）
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Note {
    pub id: i64,
    pub content: String,
    pub created_at: i64,
}

/// 現在時刻を Unix ミリ秒で返す
fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// メモを1件保存して、保存した行を返す。
///
/// 引数の `db: State<'_, Db>` は lib.rs で `manage` した DB 接続を受け取る仕組み。
/// `'_` はライフタイム注釈で、async コマンドでは付ける必要がある（今は「おまじない」程度の理解でOK）。
#[tauri::command]
pub async fn create_note(content: String, db: State<'_, Db>) -> Result<Note, String> {
    let created_at = now_ms();
    // RETURNING で INSERT した行をそのまま受け取る（SQLite 3.35+）
    let note = sqlx::query_as::<_, Note>(
        "INSERT INTO notes (content, created_at) VALUES (?, ?) \
         RETURNING id, content, created_at",
    )
    .bind(content)
    .bind(created_at)
    .fetch_one(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(note)
}

/// 全メモを新しい順で返す。
#[tauri::command]
pub async fn list_notes(db: State<'_, Db>) -> Result<Vec<Note>, String> {
    let notes = sqlx::query_as::<_, Note>(
        "SELECT id, content, created_at FROM notes ORDER BY created_at DESC",
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(notes)
}

/// 指定IDのメモを削除する。
#[tauri::command]
pub async fn delete_note(id: i64, db: State<'_, Db>) -> Result<(), String> {
    sqlx::query("DELETE FROM notes WHERE id = ?")
        .bind(id)
        .execute(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
