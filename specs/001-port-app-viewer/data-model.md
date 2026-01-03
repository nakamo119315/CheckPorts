# Data Model: ポート別アプリケーション表示CLI

**Date**: 2026-01-02
**Feature**: 001-port-app-viewer

## エンティティ定義

### PortEntry

リッスン中のポートとそれに関連するプロセス情報を表す主要エンティティ。

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `port` | u16 | ✓ | ポート番号 (1-65535) |
| `protocol` | Protocol | ✓ | プロトコル（現在はTCPのみ） |
| `process` | ProcessInfo | ✓ | プロセス情報 |
| `app_type` | Option<AppType> | | 推測されたアプリケーション種別 |

### ProcessInfo

ポートを使用しているプロセスの情報。

| フィールド | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `pid` | u32 | ✓ | プロセスID |
| `name` | String | ✓ | プロセス名 |
| `command` | Option<String> | | 完全なコマンドライン |
| `started_at` | Option<DateTime> | | 起動時刻 |
| `elapsed` | Option<Duration> | | 起動からの経過時間 |
| `user` | Option<String> | | 実行ユーザー |

### AppType（列挙型）

推測されたアプリケーション種別。

| バリアント | 識別パターン | 表示名 |
|-----------|-------------|--------|
| `NodeJs` | `node`, `npm`, `yarn` | Node.js |
| `Python` | `python`, `python3`, `uvicorn`, `gunicorn` | Python |
| `DotNet` | `dotnet`, `.dll` | .NET |
| `Java` | `java`, `.jar` | Java |
| `Go` | `go run`, `gin`, `echo` | Go |
| `Ruby` | `ruby`, `rails`, `puma` | Ruby |
| `Php` | `php`, `artisan` | PHP |
| `Rust` | `cargo run`, target path | Rust |
| `Nginx` | `nginx` | Nginx |
| `Apache` | `httpd`, `apache` | Apache |
| `Unknown` | (default) | Unknown |

### Protocol（列挙型）

ネットワークプロトコル。

| バリアント | 説明 |
|-----------|------|
| `Tcp` | TCP接続 |

## 関係図

```text
┌─────────────────────────────────────────────────────┐
│                    PortEntry                         │
├─────────────────────────────────────────────────────┤
│  port: u16                                          │
│  protocol: Protocol                                  │
│  process: ProcessInfo ─────────┐                    │
│  app_type: Option<AppType>     │                    │
└────────────────────────────────┼────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────┐
│                   ProcessInfo                        │
├─────────────────────────────────────────────────────┤
│  pid: u32                                           │
│  name: String                                        │
│  command: Option<String>                            │
│  started_at: Option<DateTime>                       │
│  elapsed: Option<Duration>                          │
│  user: Option<String>                               │
└─────────────────────────────────────────────────────┘
```

## 出力形式

### テーブル形式（デフォルト）

```text
PORT   PID     TYPE      UPTIME    COMMAND
3000   12345   Node.js   2h 15m    node server.js
5000   12346   Python    45m       python app.py
8080   12347   .NET      1d 3h     dotnet MyApi.dll
```

### JSON形式（--json オプション）

```json
{
  "ports": [
    {
      "port": 3000,
      "protocol": "TCP",
      "process": {
        "pid": 12345,
        "name": "node",
        "command": "node server.js",
        "elapsed_seconds": 8100,
        "elapsed_human": "2h 15m",
        "user": "developer"
      },
      "app_type": "Node.js"
    }
  ],
  "total_count": 1,
  "timestamp": "2026-01-02T10:30:00Z"
}
```

## バリデーションルール

| ルール | 対象 | 条件 |
|--------|------|------|
| ポート範囲 | `port` | 1 <= port <= 65535 |
| PID正数 | `pid` | pid > 0 |
| 名前非空 | `name` | name.len() > 0 |

## 状態遷移

このアプリケーションは状態を持たない（ステートレス）。
各実行時にシステムから現在のスナップショットを取得して表示する。

## エラー状態

| エラー | 原因 | 対処 |
|--------|------|------|
| `PermissionDenied` | 他ユーザーのプロセス情報へのアクセス拒否 | 取得可能な情報のみ表示 + 警告 |
| `ProcessNotFound` | プロセスが終了直後 | 「情報取得不可」と表示 |
| `SystemError` | OS API呼び出し失敗 | エラーメッセージ表示 + 終了コード1 |
