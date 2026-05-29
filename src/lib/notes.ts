import { invoke } from "@tauri-apps/api/core";

/** メモ1件。Rust 側の Note 構造体に対応する。 */
export type Note = {
  id: number;
  content: string;
  /** Unix timestamp (ミリ秒) */
  created_at: number;
};

/** メモを保存し、保存された Note を返す。 */
export const createNote = (content: string) =>
  invoke<Note>("create_note", { content });

/** 全メモを新しい順で取得する。 */
export const listNotes = () => invoke<Note[]>("list_notes");

/** 指定IDのメモを削除する。 */
export const deleteNote = (id: number) => invoke<void>("delete_note", { id });
