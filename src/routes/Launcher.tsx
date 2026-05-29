import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { createNote } from "../lib/notes";
import "./Launcher.css";

export default function Launcher() {
  const [text, setText] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    // 初回 + ホットキーで再表示されるたびに入力欄へフォーカスする
    inputRef.current?.focus();
    const win = getCurrentWindow();
    const unlistenP = win.listen("tauri://focus", () => {
      inputRef.current?.focus();
    });
    return () => {
      unlistenP.then((un) => un());
    };
  }, []);

  // ランチャーを閉じる（保存せず or 保存後）
  async function close() {
    setText("");
    await invoke("hide_launcher");
  }

  async function onKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Escape") {
      e.preventDefault();
      await close();
    }
  }

  async function onSubmit(e: React.FormEvent) {
    e.preventDefault();
    const value = text.trim();
    if (!value) return;

    // 特殊コマンド: ">list" で一覧ウィンドウを開く
    if (value === ">list") {
      await invoke("open_list");
      await close();
      return;
    }

    await createNote(value);
    await close();
  }

  return (
    <form className="launcher" onSubmit={onSubmit}>
      <input
        ref={inputRef}
        className="launcher-input"
        value={text}
        onChange={(e) => setText(e.currentTarget.value)}
        onKeyDown={onKeyDown}
        placeholder="メモを入力して Enter（>list で一覧）"
        autoFocus
      />
    </form>
  );
}
