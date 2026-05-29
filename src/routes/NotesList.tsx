import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { deleteNote, listNotes, type Note } from "../lib/notes";
import "../App.css";

export default function NotesList() {
  const [notes, setNotes] = useState<Note[]>([]);

  async function refresh() {
    setNotes(await listNotes());
  }

  useEffect(() => {
    refresh();
    // ウィンドウが前面に来るたびに最新化（ランチャーで追加した分を反映）
    const win = getCurrentWindow();
    const unlistenP = win.listen("tauri://focus", () => refresh());
    return () => {
      unlistenP.then((un) => un());
    };
  }, []);

  async function handleDelete(id: number) {
    await deleteNote(id);
    await refresh();
  }

  return (
    <main className="container">
      <h1>メモ一覧</h1>
      <ul style={{ textAlign: "left", padding: 0, listStyle: "none" }}>
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
        {notes.length === 0 && (
          <li style={{ opacity: 0.6 }}>まだメモがありません</li>
        )}
      </ul>
    </main>
  );
}
