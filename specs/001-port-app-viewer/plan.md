# Implementation Plan: ポート別アプリケーション表示CLI

**Branch**: `001-port-app-viewer` | **Date**: 2026-01-02 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-port-app-viewer/spec.md`

## Summary

macOS向けCLIツール `ports` を Rust で実装する。現在リッスン中のTCPポートを検出し、各ポートを使用しているプロセスの情報（PID、コマンドライン、起動時間、アプリ種別）をわかりやすく表示する。開発者がプロセスを停止してよいかを判断するために必要な情報を提供する。

## Technical Context

**Language/Version**: Rust 1.75+
**Primary Dependencies**: clap (CLI引数解析), serde/serde_json (JSON出力), libproc (macOSプロセス情報)
**Storage**: N/A（状態保存なし）
**Testing**: cargo test (単体テスト + 統合テスト)
**Target Platform**: macOS (Darwin)
**Project Type**: single
**Performance Goals**: 10個以上のポートでも1秒以内に結果表示
**Constraints**: 一般ユーザー権限で動作、管理者権限不要
**Scale/Scope**: ローカル開発環境での使用（同時リッスンポート数: 通常10-50程度）

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Evidence |
|-----------|--------|----------|
| I. Explicit Over Implicit | ✅ Pass | 全設定値に明示的なデフォルト、マジックナンバーなし |
| II. Readable Code First | ✅ Pass | 完全な単語を使用、単一責任の関数設計 |
| III. CLI Interface Standards | ✅ Pass | `--help`, `--json`, POSIX終了コード、アクショナブルなエラーメッセージ |
| IV. Test Coverage | ✅ Pass | 単体テスト + CLI統合テスト計画 |
| V. Simplicity | ✅ Pass | 最小限の依存関係、YAGNI準拠 |

## Project Structure

### Documentation (this feature)

```text
specs/001-port-app-viewer/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (CLI interface contract)
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
src/
├── main.rs              # エントリーポイント、CLI引数解析
├── port_scanner.rs      # ポートスキャン機能
├── process_info.rs      # プロセス情報取得
├── app_detector.rs      # アプリケーション種別推測
├── output.rs            # 出力フォーマット（テーブル/JSON）
└── error.rs             # エラー型定義

tests/
├── integration/
│   └── cli_test.rs      # CLIコマンド統合テスト
└── unit/
    ├── port_scanner_test.rs
    ├── app_detector_test.rs
    └── output_test.rs
```

**Structure Decision**: シングルプロジェクト構成を採用。CLIツールとして単一のバイナリを生成し、モジュール分割で責務を明確化する。

## Complexity Tracking

> 違反なし - 全ての原則に準拠
