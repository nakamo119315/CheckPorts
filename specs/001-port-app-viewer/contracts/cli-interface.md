# CLI Interface Contract: ports

**Date**: 2026-01-02
**Feature**: 001-port-app-viewer

## コマンド概要

```text
ports [OPTIONS]
```

リッスン中のTCPポートと関連するプロセス情報を表示する。

## オプション

| オプション | 短縮形 | 説明 | デフォルト |
|-----------|--------|------|-----------|
| `--json` | `-j` | JSON形式で出力 | false |
| `--help` | `-h` | ヘルプを表示 | - |
| `--version` | `-V` | バージョンを表示 | - |

## 終了コード

| コード | 意味 |
|--------|------|
| 0 | 正常終了（ポートが0件の場合も含む） |
| 1 | 一般エラー（システムエラー等） |
| 2 | 引数エラー（不正なオプション） |

## 出力仕様

### 標準出力（テーブル形式）

**ヘッダー行**:
```text
PORT   PID     TYPE      UPTIME    COMMAND
```

**データ行**:
```text
{port:5}  {pid:6}  {type:8}  {uptime:8}  {command}
```

**フィールド幅**:
- PORT: 5文字（右寄せ）
- PID: 6文字（右寄せ）
- TYPE: 8文字（左寄せ）
- UPTIME: 8文字（右寄せ）
- COMMAND: 可変長（切り詰めなし）

**ポートなしの場合**:
```text
アクティブなポートはありません
```

**権限警告（該当する場合）**:
```text
注意: 一部のプロセス情報は権限不足のため取得できませんでした
```

### 標準出力（JSON形式）

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
  "timestamp": "2026-01-02T10:30:00Z",
  "warnings": []
}
```

**警告がある場合**:
```json
{
  "ports": [...],
  "warnings": [
    "一部のプロセス情報は権限不足のため取得できませんでした"
  ]
}
```

### 標準エラー出力

エラーメッセージは標準エラー出力に出力される。

**フォーマット**:
```text
error: {エラーメッセージ}
hint: {対処方法}
```

**例**:
```text
error: ポート情報の取得に失敗しました
hint: システムの権限設定を確認してください
```

## ヘルプ出力

```text
ports - リッスン中のTCPポートとアプリケーション情報を表示

Usage: ports [OPTIONS]

Options:
  -j, --json     JSON形式で出力
  -h, --help     ヘルプを表示
  -V, --version  バージョンを表示

Examples:
  ports          # テーブル形式で表示
  ports --json   # JSON形式で表示
```

## 使用例

### 基本的な使用

```bash
$ ports
PORT   PID     TYPE      UPTIME    COMMAND
3000   12345   Node.js   2h 15m    node server.js
5000   12346   Python    45m       python -m uvicorn app:app
8080   12347   .NET      1d 3h     dotnet run --project Api
```

### JSON出力

```bash
$ ports --json | jq '.ports[0].port'
3000
```

### ポートなし

```bash
$ ports
アクティブなポートはありません
```

### パイプとの組み合わせ

```bash
# 特定のポートをgrepで検索
$ ports | grep 3000

# JSON形式でjqを使用
$ ports --json | jq '.ports[] | select(.port == 3000)'
```
