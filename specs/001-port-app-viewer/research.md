# Research: ポート別アプリケーション表示CLI

**Date**: 2026-01-02
**Feature**: 001-port-app-viewer

## 調査サマリー

macOSでリッスン中のTCPポートとプロセス情報を取得するためのRustクレートを調査し、最適なアプローチを決定した。

---

## Decision 1: ポート情報取得ライブラリ

**Decision**: `listeners` クレートを使用

**Rationale**:
- 目的に完全に合致：リッスン中のソケットとプロセスを関連付ける専用ライブラリ
- 必要な情報（PID、プロセス名、ソケット情報、プロトコル）を一度に取得可能
- クロスプラットフォーム対応だが、macOSで最適化されている
- 構造化データを返すため、出力フォーマットが容易

**Alternatives Considered**:

| ライブラリ | 評価 | 不採用理由 |
|-----------|------|-----------|
| `netstat2` | △ | プロセス情報を含まない。別途libprocと組み合わせが必要 |
| `sysinfo` | × | ネットワークソケット情報を提供しない |
| `lsof` subprocess | △ | プロセス起動オーバーヘッド、テキストパース必要 |

---

## Decision 2: 詳細プロセス情報取得

**Decision**: `libproc` クレートで補完

**Rationale**:
- macOSネイティブのlibprocライブラリのRustバインディング
- コマンドライン引数、起動時刻など詳細情報を取得可能
- lsofが内部で使用しているのと同じAPI（より高速）
- サブプロセス起動なしでネイティブに動作

**Alternatives Considered**:

| 方法 | 評価 | 不採用理由 |
|------|------|-----------|
| `darwin-libproc` | ○ | より安全なラッパーだが、libprocで十分 |
| `ps` subprocess | × | パフォーマンス劣化、テキストパース必要 |

---

## Decision 3: アーキテクチャ

**Decision**: `listeners` + `libproc` の2層構成

```text
┌─────────────────────────────────────────────────────┐
│                   ports CLI                          │
├─────────────────────────────────────────────────────┤
│  listeners crate                                     │
│  └─ リッスン中ポート + PID + プロセス名              │
├─────────────────────────────────────────────────────┤
│  libproc crate (補完)                               │
│  └─ コマンドライン、起動時刻、詳細情報               │
├─────────────────────────────────────────────────────┤
│  Graceful Fallback                                  │
│  └─ 権限不足時は取得可能な情報のみ表示              │
└─────────────────────────────────────────────────────┘
```

**Rationale**:
- `listeners`で基本情報を確実に取得
- `libproc`で詳細情報を補完（失敗時は基本情報のみ表示）
- パフォーマンス要件（10ポート/1秒以内）を余裕で達成可能

---

## Decision 4: CLI引数解析

**Decision**: `clap` クレート（derive マクロ使用）

**Rationale**:
- Rust CLIアプリのデファクトスタンダード
- `--help`自動生成でConstitution III準拠
- 型安全な引数解析
- derive マクロで宣言的に定義可能

---

## Decision 5: JSON出力

**Decision**: `serde` + `serde_json`

**Rationale**:
- Rustのシリアライズデファクトスタンダード
- derive マクロで構造体から自動生成
- 整形出力（pretty print）対応

---

## Decision 6: アプリケーション種別推測

**Decision**: コマンドライン文字列のパターンマッチング

**Approach**:
```text
コマンドライン文字列を以下のパターンで照合:
- "node" → Node.js
- "python" → Python
- "dotnet" / ".dll" → .NET
- "java" / ".jar" → Java
- "go" / "gin" / "echo" → Go
- "ruby" / "rails" → Ruby
- "php" → PHP
- "nginx" → Nginx
- "httpd" / "apache" → Apache
```

**Rationale**:
- 80%以上の識別精度要件を満たす（一般的な開発環境）
- 複雑な推測ロジックは不要（YAGNI原則）
- 新しいパターンは容易に追加可能

---

## Decision 7: エラーハンドリング

**Decision**: カスタムエラー型 + `thiserror` クレート

**Rationale**:
- 明示的なエラー型でConstitution I準拠
- アクショナブルなエラーメッセージ生成が容易
- `?`演算子との相性が良い

---

## 依存クレート一覧

| クレート | バージョン | 用途 |
|----------|-----------|------|
| `clap` | 4.x | CLI引数解析 |
| `listeners` | latest | ポート・プロセス情報取得 |
| `libproc` | latest | 詳細プロセス情報 |
| `serde` | 1.x | シリアライズ |
| `serde_json` | 1.x | JSON出力 |
| `thiserror` | 1.x | エラー型定義 |
| `chrono` | 0.4.x | 時刻計算（起動時間） |

---

## パフォーマンス見積もり

| 操作 | 予測時間 |
|------|---------|
| listeners によるポート取得 | ~10ms |
| libproc によるプロセス詳細取得（10プロセス） | ~50ms |
| 出力フォーマット | ~5ms |
| **合計** | **~65ms** |

**結論**: 1秒以内の要件を大幅に余裕を持って達成可能

---

## ビルド環境の注意点

libproc クレートはbindgenを使用するため、以下の環境変数が必要な場合がある:

```bash
CLANG_PATH="/usr/bin/clang" cargo build
```

これはHomebrewでインストールしたLLVMとXcodeのclangが競合する場合に必要。

---

## References

- [listeners - crates.io](https://crates.io/crates/listeners)
- [libproc - crates.io](https://crates.io/crates/libproc)
- [clap - crates.io](https://crates.io/crates/clap)
- [Apple Developer Forums: Using libproc](https://developer.apple.com/forums/thread/728731)
