-- notes テーブル: 1メモ = 1行
CREATE TABLE notes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  content TEXT NOT NULL,
  created_at INTEGER NOT NULL  -- Unix timestamp (ms)
);

-- 一覧は新しい順で出すので created_at の降順インデックスを張る
CREATE INDEX idx_notes_created_at ON notes(created_at DESC);
