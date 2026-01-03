# ports

リッスン中のTCPポートとアプリケーション情報を表示するmacOS向けCLIツール。

## 概要

`ports` は、開発者がローカル環境でどのポートがどのアプリケーションに使用されているかを確認し、プロセスを停止してよいかを判断するための情報を提供します。

## 機能

- リッスン中のTCPポートを一覧表示
- 各ポートを使用しているプロセスの情報（PID、コマンド、起動時間）を表示
- アプリケーション種別を自動検出（Node.js、Python、.NET、Java等）
- テーブル形式とJSON形式の出力に対応

## インストール

### ソースからビルド

```bash
# リポジトリをクローン
git clone https://github.com/your-repo/checkPortCli.git
cd checkPortCli

# ビルド
cargo build --release

# インストール（オプション）
cargo install --path .
```

### 前提条件

- macOS 10.15 (Catalina) 以降
- Rust 1.75 以降

## 使い方

### 基本的な使用

```bash
$ ports
 PORT     PID  TYPE          UPTIME  COMMAND
 3000   12345  Node.js       2h 15m  node server.js
 5000   12346  Python          45m   python -m uvicorn app:app
 8080   12347  .NET          1d 3h   dotnet run --project Api
```

### JSON形式で出力

```bash
$ ports --json
{
  "ports": [...],
  "total_count": 3,
  "timestamp": "2026-01-02T10:30:00Z"
}
```

### ヘルプを表示

```bash
$ ports --help
```

## オプション

| オプション | 短縮形 | 説明 |
|-----------|--------|------|
| `--json` | `-j` | JSON形式で出力 |
| `--help` | `-h` | ヘルプを表示 |
| `--version` | `-V` | バージョンを表示 |

## 終了コード

| コード | 意味 |
|--------|------|
| 0 | 正常終了 |
| 1 | 一般エラー |
| 2 | 引数エラー |

## ライセンス

MIT
