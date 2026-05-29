import React from "react";
import ReactDOM from "react-dom/client";
import { getCurrentWindow } from "@tauri-apps/api/window";
import Launcher from "./routes/Launcher";
import NotesList from "./routes/NotesList";
import "./App.css";

// このウィンドウの label を見て表示する画面を決める（launcher / list）。
// 各ウィンドウは同じ index.html を読み込み、label で出し分ける。
const label = getCurrentWindow().label;
document.documentElement.dataset.window = label;

const Screen = label === "launcher" ? Launcher : NotesList;

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Screen />
  </React.StrictMode>,
);
