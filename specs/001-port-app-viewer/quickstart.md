# Quickstart: ports CLI

**Date**: 2026-01-02
**Feature**: 001-port-app-viewer

## 概要

`ports` は、macOSでリッスン中のTCPポートとそれを使用しているアプリケーションの情報を表示するCLIツールです。

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
- Xcode Command Line Tools

## 基本的な使い方

### ポート一覧を表示

```bash
$ ports
PORT   PID     TYPE      UPTIME    COMMAND
3000   12345   Node.js   2h 15m    node server.js
5000   12346   Python    45m       python -m uvicorn app:app
8080   12347   .NET      1d 3h     dotnet run --project Api
```

### JSON形式で出力

```bash
$ ports --json
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
        "elapsed_human": "2h 15m"
      },
      "app_type": "Node.js"
    }
  ],
  "total_count": 1
}
```

### ヘルプを表示

```bash
$ ports --help
```

## ユースケース

### 1. 開発サーバーの確認

複数のプロジェクトを同時に開発しているとき、どのポートでどのアプリが動いているかを確認:

```bash
$ ports
PORT   PID     TYPE      UPTIME    COMMAND
3000   1234    Node.js   30m       npm run dev
3001   1235    Node.js   25m       next dev
5000   1236    Python    1h        flask run
8080   1237    Java      2h        mvn spring-boot:run
```

### 2. ポートを解放したいとき

特定のポートを使用しているプロセスを特定し、停止:

```bash
# ポート3000を使っているプロセスを確認
$ ports | grep 3000
3000   1234    Node.js   30m       npm run dev

# PIDを使って停止
$ kill 1234
```

### 3. スクリプトでの利用

JSON出力を使って自動化:

```bash
# 特定のポートが使用中かチェック
if ports --json | jq -e '.ports[] | select(.port == 3000)' > /dev/null; then
  echo "Port 3000 is in use"
fi

# 全ポートのPIDを取得
ports --json | jq -r '.ports[].process.pid'
```

## トラブルシューティング

### 一部のプロセス情報が表示されない

権限の問題で他のユーザーが起動したプロセスの情報が取得できない場合があります。

```bash
# sudoで実行すると全プロセスの情報を取得可能
$ sudo ports
```

### ビルドエラー（libproc関連）

複数のLLVMがインストールされている場合:

```bash
CLANG_PATH="/usr/bin/clang" cargo build
```

## 次のステップ

- [CLI Interface Contract](./contracts/cli-interface.md) - 詳細な出力仕様
- [Data Model](./data-model.md) - データ構造の詳細
