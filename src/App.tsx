import { useEffect, useState } from "react";
import { createNote, deleteNote, listNotes, type Note } from "./lib/notes";
import "./App.css";

function App() {
  const [content, setContent] = useState("");
  const [notes, setNotes] = useState<Note[]>([]);

  // 一覧を再取得して state を更新する
  async function refresh() {
    setNotes(await listNotes());
  }

  // 初回マウント時に一覧を読み込む
  useEffect(() => {
    refresh();
  }, []);

  async function handleSave(e: React.FormEvent) {
    e.preventDefault();
    const text = content.trim();
    if (!text) return;
    await createNote(text);
    setContent("");
    await refresh();
  }

  async function handleDelete(id: number) {
    await deleteNote(id);
    await refresh();
  }

  return (
    <main className="container">
      <h1>QuickNote — Phase 1 動作確認</h1>

      <form onSubmit={handleSave} style={{ display: "flex", gap: 8 }}>
        <input
          value={content}
          onChange={(e) => setContent(e.currentTarget.value)}
          placeholder="メモを入力して Enter / 保存"
          style={{ flex: 1 }}
          autoFocus
        />
        <button type="submit">保存</button>
      </form>

      <ul style={{ textAlign: "left", marginTop: 16, padding: 0, listStyle: "none" }}>
        {notes.map((n) => (
          <li
            key={n.id}
            style={{
              display: "flex",
              alignItems: "center",
              gap: 8,
              marginBottom: 8,
            }}
          >
            <span style={{ flex: 1 }}>{n.content}</span>
            <small style={{ opacity: 0.6 }}>
              {new Date(n.created_at).toLocaleString()}
            </small>
            <button onClick={() => handleDelete(n.id)}>削除</button>
          </li>
        ))}
        {notes.length === 0 && <li style={{ opacity: 0.6 }}>まだメモがありません</li>}
      </ul>
    </main>
  );
}

export default App;
