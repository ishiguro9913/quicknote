# QuickNote — 開発引き継ぎドキュメント

このドキュメントは、Claude (チャット) との要件整理セッションを経て確定した内容を、Claude Code が開発を引き継ぐためのものです。このファイルの内容を一次情報として開発を進めてください。

---

## 1. プロジェクト概要

**アプリ名**: QuickNote (仮)

**何を作るか**:
macOS デスクトップアプリ。Raycast ライクなランチャー UI で、グローバルホットキーを押すと小さな入力ウィンドウが出てきて、テキストを打って Enter で保存できる「クイックメモ」アプリ。

**ユースケース**:
- 思いついたことを瞬時に残したい
- 既存メモアプリを開く手間 (ウィンドウ切り替え、ノート選択) を省きたい

**配布範囲**: 自分用のみ (コード署名・公証は不要、`npm run tauri dev` または `tauri build` でローカル使用)

---

## 2. 機能要件

### コア機能

| # | 機能 | 詳細 |
|---|---|---|
| 1 | グローバルホットキーでランチャー起動 | OS フォーカスがどこにあっても発火。デフォルトホットキーは仮で `Cmd+Shift+N`、設定で変更可能 |
| 2 | テキスト入力 → Enter で保存 | 単一行 (またはシンプルな複数行) の TextField。Enter で SQLite に保存し、ランチャーを閉じる |
| 3 | Esc / フォーカス外れでランチャーを閉じる | 保存せずに閉じる |
| 4 | メモ一覧画面 | 別ウィンドウで保存済みメモを一覧表示。削除可能 |
| 5 | 一覧画面の開き方 (両対応) | (a) メニューバーアイコンから / (b) ランチャーからコマンド (例: `>list` と入力して Enter) で開く |
| 6 | メニューバー常駐 | Dock には出さず、メニューバーにアイコンを置いて常駐 |
| 7 | 設定画面 | ホットキー変更ができる |

### 非機能要件

- **速さ最優先**: ホットキー → ウィンドウ表示が体感即時 (目標 100ms 以内)
- **シンプル最優先**: 機能を増やしすぎない。プレーンテキストのみ (Markdown 非対応)
- **常駐前提**: アプリは起動しっぱなしでバックグラウンド待機

### スコープ外 (今は作らない)

- Markdown 対応 / リッチテキスト
- タグ・カテゴリ
- 検索 (一覧画面のシンプルなフィルタは Phase 3 で追加検討)
- クラウド同期
- 他デバイス連携
- 配布・自動アップデート

---

## 3. 技術スタック

| レイヤ | 採用技術 | 補足 |
|---|---|---|
| デスクトップフレームワーク | **Tauri 2.x** | 最新の v2 系。v1 の情報を参照しないこと |
| バックエンド言語 | **Rust** (stable) | rustup 経由でインストール |
| フロント言語 | **TypeScript** | |
| フロント UI | **React** + **Vite** | `create tauri-app` のテンプレートから |
| スタイリング | **Tailwind CSS** | 後から追加導入する |
| データ保存 | **SQLite** (`tauri-plugin-sql`) | ローカルファイルベース |
| グローバルホットキー | `tauri-plugin-global-shortcut` | 公式プラグイン |
| トレイ/メニューバー常駐 | Tauri 標準の `TrayIcon` API | 追加プラグイン不要 |
| ウィンドウ制御 | Tauri 標準 (`WebviewWindow`) | 透過・常に手前・フォーカス外れで非表示 |

### 技術選定の経緯 (なぜ Tauri か)

候補として Swift/SwiftUI, Tauri, Electron を比較した。

- ユーザーは Web エンジニア出身で **Rust・システムプログラミングへの学習意欲がある**
- 自分用のみで配布不要 → Swift の「ネイティブで配布が楽」という強みは効かない
- クロスプラットフォーム・将来性・Rust 学習リターンを総合して **Tauri に決定**

Swift は「Apple エコシステムへの深入り」が主目的なら最適だが、今回のユーザーの興味方向 (Rust) とは異なるため見送り。Electron はランチャー系の速度要件と合わず却下。

---

## 4. ディレクトリ構成 (目標形)

```
quicknote/
├── src/                          # フロント (React + TS)
│   ├── App.tsx
│   ├── main.tsx
│   ├── routes/
│   │   ├── Launcher.tsx          # ランチャー画面
│   │   ├── NotesList.tsx         # 一覧画面
│   │   └── Settings.tsx          # ホットキー設定
│   ├── lib/
│   │   ├── notes.ts              # Rust コマンド呼び出しラッパ (invoke)
│   │   └── shortcuts.ts          # ショートカット設定の保存・読み込み
│   └── styles.css
│
├── src-tauri/                    # バックエンド (Rust)
│   ├── src/
│   │   ├── main.rs               # エントリ
│   │   ├── lib.rs                # Tauri 初期化、コマンド登録
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   └── notes.rs          # create_note, list_notes, delete_note
│   │   ├── db.rs                 # SQLite 接続・マイグレーション
│   │   ├── shortcuts.rs          # グローバルショートカット登録
│   │   └── tray.rs               # トレイアイコン・メニュー
│   ├── Cargo.toml
│   ├── tauri.conf.json           # ウィンドウ設定・プラグイン設定
│   └── migrations/
│       └── 001_create_notes.sql
│
├── package.json
├── vite.config.ts
└── tsconfig.json
```

ポイント: Tauri は「フロントから `invoke('command_name', args)` で Rust 側の関数を呼ぶ」設計。REST/GraphQL を挟む必要はない。

---

## 5. データモデル

```sql
-- src-tauri/migrations/001_create_notes.sql
CREATE TABLE notes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  content TEXT NOT NULL,
  created_at INTEGER NOT NULL  -- Unix timestamp (ms)
);

CREATE INDEX idx_notes_created_at ON notes(created_at DESC);
```

将来の拡張余地 (今は作らない): `updated_at`, `tags`, `archived` など。

---

## 6. 開発フェーズ (Claude Code はこの順序で進めること)

### Phase 0: 環境構築 — **ユーザーが手動で完了済みの想定**

すでに以下が揃っている前提。もし足りなければ案内すること。

- Xcode Command Line Tools (`xcode-select --install`)
- Homebrew
- Node.js (LTS, v20+)
- Rust (rustup 経由、stable)
- `npm create tauri-app@latest` で React + TypeScript テンプレートのプロジェクトを生成済み
- `npm run tauri dev` でサンプルウィンドウが起動することを確認済み

確認コマンド:
```bash
node --version    # v20+ or v22+
rustc --version
cargo --version
```

### Phase 1: データ層 (Rust + SQLite)

目標: 「フォームに打ち込んで保存ボタン → 一覧に出る」をまず動かす。ランチャー UI はまだ作らない。

1. `tauri-plugin-sql` を依存追加 (`src-tauri/Cargo.toml` と `package.json` の両方)
2. `tauri.conf.json` の plugins に `sql` を登録、capability も追加
3. `src-tauri/migrations/001_create_notes.sql` を作成
4. `src-tauri/src/db.rs` で SQLite 接続とマイグレーション実行
5. Rust コマンド (`#[tauri::command]`) を実装:
   - `create_note(content: String) -> Result<Note, String>`
   - `list_notes() -> Result<Vec<Note>, String>`
   - `delete_note(id: i64) -> Result<(), String>`
6. `lib.rs` でコマンドを `invoke_handler` に登録
7. フロントの `src/lib/notes.ts` で型付き invoke ラッパを書く
8. 仮の UI (App.tsx に TextField と一覧) で保存・表示を動作確認

**Rust 学習のポイント**: ここで `Result`, `Option`, 構造体 `derive(Serialize, Deserialize)`, async 関数を体験する。

### Phase 2: ランチャー UX (Tauri の核)

目標: グローバルホットキーで本物のランチャーが出てくる体験を完成させる。

1. **ランチャー専用ウィンドウを定義** (`tauri.conf.json` で別ウィンドウとして宣言)
   - サイズ: 幅 600px × 高さ 60px くらい (小さく)
   - `decorations: false` (タイトルバーなし)
   - `transparent: true`
   - `always_on_top: true`
   - `skip_taskbar: true`
   - `resizable: false`
   - `visible: false` (起動時は非表示)
2. **メインウィンドウは Dock から消す**: `tauri.conf.json` の `macOS` セクションで `activation_policy: "accessory"` 相当の設定、または起動時に一覧ウィンドウも非表示にする
3. **グローバルホットキー登録** (`tauri-plugin-global-shortcut`)
   - デフォルト `CmdOrCtrl+Shift+N`
   - 発火時の挙動: ランチャーウィンドウを show + フォーカス + 中央配置
4. **ランチャーの閉じる条件**:
   - Esc キー (フロント側で keydown を拾う)
   - フォーカス外れ (`on_window_event` で `Focused(false)` を拾う)
   - Enter で保存完了後
5. **`src/routes/Launcher.tsx`** を作成:
   - 単一の TextField のみ
   - Enter キーで `create_note(content)` を呼んで閉じる
   - 特殊コマンド `>list` を入力したら一覧ウィンドウを開いてランチャーを閉じる
6. **ランチャー URL の振り分け**: Tauri の複数ウィンドウは URL で振り分ける。ランチャーは `/launcher`、一覧は `/list` などのルーティング (React Router or 単純な URL ハッシュ判定)

**つまずきやすいポイント**:
- macOS でフォーカス外れの判定がうまく動かない場合、`NSPanel` 相当の挙動が必要になることがある (`tauri-plugin-positioner` や `tauri-nspanel` が候補)
- 透過ウィンドウは CSS の背景色を `transparent` にする必要がある
- ホットキー押下時にウィンドウが既に表示中ならトグルで閉じる挙動も入れる

### Phase 3: 仕上げ

1. **メニューバー常駐** (`TrayIcon`)
   - アイコンクリックでメニュー表示: 「一覧を開く」「設定」「終了」
   - 16x16 / 32x32 のテンプレートアイコン (黒の単色 PNG) を `src-tauri/icons/` に置く
2. **一覧画面** (`src/routes/NotesList.tsx`)
   - SQLite から全件取得して表示
   - 各メモに削除ボタン
   - メモ本文クリックでクリップボードにコピー (おまけ)
3. **設定画面** (`src/routes/Settings.tsx`)
   - ホットキー変更 UI: キー入力を受け付けて表示
   - 保存先: `tauri-plugin-store` でアプリ設定ファイルに永続化
   - 保存時に Rust 側で既存ショートカットを unregister して新規 register
4. **起動時の自動復帰**: アプリ起動時に保存済みホットキーを読み込んで register
5. **ログイン項目への追加** (任意): `tauri-plugin-autostart` で OS 起動時に自動起動

---

## 7. 必要なプラグイン一覧

`Cargo.toml` と `package.json` の両方に同じバージョン系の依存を入れること。

| プラグイン | 用途 | 必須度 |
|---|---|---|
| `tauri-plugin-sql` (sqlite feature) | データ永続化 | 必須 |
| `tauri-plugin-global-shortcut` | グローバルホットキー | 必須 |
| `tauri-plugin-store` | 設定 (ホットキー) の永続化 | Phase 3 で必須 |
| `tauri-plugin-autostart` | OS 起動時の自動起動 | 任意 |
| `tauri-plugin-positioner` | ウィンドウ位置調整 (画面中央など) | 推奨 |

**重要**: Tauri 2 は capability (`src-tauri/capabilities/default.json`) でプラグインの権限を明示する必要がある。プラグイン追加時は必ず capability にも追記すること。

---

## 8. tauri.conf.json の重要設定 (抜粋イメージ)

```json
{
  "app": {
    "windows": [
      {
        "label": "launcher",
        "title": "QuickNote Launcher",
        "width": 600,
        "height": 60,
        "resizable": false,
        "decorations": false,
        "transparent": true,
        "alwaysOnTop": true,
        "skipTaskbar": true,
        "visible": false,
        "center": true,
        "url": "index.html#/launcher"
      },
      {
        "label": "list",
        "title": "QuickNote",
        "width": 700,
        "height": 500,
        "visible": false,
        "url": "index.html#/list"
      }
    ],
    "macOSPrivateApi": true
  },
  "bundle": {
    "active": true,
    "category": "Productivity",
    "icon": ["icons/icon.icns"]
  }
}
```

実際のキー名・構造は Tauri 2 の最新スキーマに合わせること (公式: https://v2.tauri.app)。

---

## 9. ユーザーについて (重要)

- **Web エンジニア出身** — React / TypeScript / Vite / npm は熟知している
- **Rust は初学者** — 所有権・ライフタイムは未学習。コード内で重要な Rust 概念に触れたら、コメントで一言補足してくれると学習になる
- **macOS ネイティブ開発も未経験** — Xcode/Swift の知識前提のコードや用語は説明が必要
- **学習も兼ねている** — 「動けば OK」より「なぜそうするのか」を簡潔に教える方が嬉しい。ただし冗長な解説は不要

---

## 10. 開発の進め方の依頼

1. まず Phase 1 から着手すること
2. 各 Phase の頭で「これから何を作るか」を 2〜3 行で説明してから手を動かす
3. ファイルを作る/編集する際は、なぜそのファイルか・なぜその構造かを簡潔に補足する
4. Rust 特有の概念 (`Result`, `?` 演算子, ライフタイム等) が初登場するときだけ短く解説
5. 詰まったら推測せず、エラーメッセージとともにユーザーに確認すること
6. 各 Phase 完了時に「動作確認手順」を提示してユーザーが手元で確認できるようにする

---

## 11. 既知の注意点・落とし穴

- **Tauri v1 と v2 で API が大きく違う**。ネットの記事は v1 のものが多い。**必ず v2 系の公式ドキュメント (https://v2.tauri.app) を参照すること**
- **Cargo の初回ビルドは 5〜10 分かかる**。ハングではない
- **macOS のグローバルホットキーはアクセシビリティ権限が要らないことが多い**が、もし要求されたら「システム設定 > プライバシーとセキュリティ > アクセシビリティ」での許可をユーザーに案内する
- **`npm run tauri dev` のホットリロード**はフロント (React) には効くが、Rust 側の変更は再ビルドが走るので数秒〜数十秒待つ必要がある
- **複数ウィンドウ間の通信** が必要な場合 (例: ランチャーで保存したら一覧画面を更新) は Tauri の event システム (`emit` / `listen`) を使う

---

## 12. 完成の定義 (Definition of Done)

以下が全部動けば完成:

- [ ] `Cmd+Shift+N` (または設定したキー) でランチャーが瞬時に出てくる
- [ ] ランチャーに文字を打って Enter で保存され、ランチャーが閉じる
- [ ] Esc でランチャーが閉じる (保存しない)
- [ ] フォーカス外れでランチャーが閉じる
- [ ] メニューバーアイコンから一覧画面を開ける
- [ ] ランチャーから `>list` で一覧画面を開ける
- [ ] 一覧画面で保存済みメモを確認・削除できる
- [ ] 設定画面でホットキーを変更でき、再起動なしで反映される
- [ ] アプリが Dock に出ず、メニューバーに常駐する
- [ ] アプリを終了するまで常駐し続け、ホットキーがいつでも効く

---

以上。Phase 1 から始めてください。