# Tasks: ポート別アプリケーション表示CLI

**Input**: Design documents from `/specs/001-port-app-viewer/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Constitution IV (Test Coverage) に基づき、単体テスト・統合テストを含む

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- Rust project with `cargo` build system

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Rustプロジェクトの初期化と基本構造の作成

- [x] T001 Create Rust project with `cargo init` and configure Cargo.toml with dependencies (clap, serde, serde_json, listeners, libproc, thiserror, chrono)
- [x] T002 [P] Create project directory structure: src/, tests/integration/, tests/unit/
- [x] T003 [P] Configure rustfmt.toml and clippy settings for code formatting

**Checkpoint**: Rust project compiles with `cargo build`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全ユーザーストーリーが依存するコアインフラの実装

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Define error types with thiserror in src/error.rs (PermissionDenied, ProcessNotFound, SystemError)
- [x] T005 [P] Define Protocol enum (Tcp) in src/models.rs
- [x] T006 [P] Define AppType enum with all variants (NodeJs, Python, DotNet, Java, Go, Ruby, Php, Rust, Nginx, Apache, Unknown) in src/models.rs
- [x] T007 [P] Define ProcessInfo struct with all fields (pid, name, command, started_at, elapsed, user) in src/models.rs
- [x] T008 Define PortEntry struct with fields (port, protocol, process, app_type) in src/models.rs
- [x] T009 Implement CLI argument parsing with clap derive in src/main.rs (--json, --help, --version)

**Checkpoint**: Foundation ready - `cargo build` succeeds, `ports --help` displays usage

---

## Phase 3: User Story 1 - ポート使用状況の一覧表示 (Priority: P1) 🎯 MVP

**Goal**: リッスン中のTCPポートとプロセス情報の基本一覧を表示

**Independent Test**: `ports` を実行し、リッスン中のポート一覧が表示されることを確認

### Tests for User Story 1

- [x] T010 [P] [US1] Unit test for port scanning in tests/unit/port_scanner_test.rs
- [x] T011 [P] [US1] Integration test for CLI basic output in tests/integration/cli_test.rs

### Implementation for User Story 1

- [x] T012 [US1] Implement port scanning with listeners crate in src/port_scanner.rs (get all listening TCP ports with PID and process name)
- [x] T013 [US1] Implement table output formatter in src/output.rs (PORT, PID, COMMAND columns)
- [x] T014 [US1] Implement empty state message "アクティブなポートはありません" in src/output.rs
- [x] T015 [US1] Implement sorting by port number (ascending) in src/port_scanner.rs
- [x] T016 [US1] Wire port scanner and output to main.rs, handle basic execution flow
- [x] T017 [US1] Implement POSIX exit codes (0=success, 1=error, 2=argument error) in src/main.rs

**Checkpoint**: `ports` displays listening ports in table format, sorted by port number

---

## Phase 4: User Story 2 - アプリケーション種別の識別表示 (Priority: P2)

**Goal**: コマンドラインからアプリケーション種別を推測して表示

**Independent Test**: Node.js, Python等のアプリを起動し、種別が正しく識別されることを確認

### Tests for User Story 2

- [x] T018 [P] [US2] Unit test for app detection patterns in tests/unit/app_detector_test.rs

### Implementation for User Story 2

- [x] T019 [US2] Implement app type detection logic with pattern matching in src/app_detector.rs
- [x] T020 [US2] Add TYPE column to table output in src/output.rs
- [x] T021 [US2] Integrate app detector with port scanner results in src/main.rs

**Checkpoint**: `ports` displays app type (Node.js, Python, etc.) for each port

---

## Phase 5: User Story 3 - プロセス停止判断のための詳細情報表示 (Priority: P3)

**Goal**: PID、コマンドライン、起動時間など停止判断に必要な詳細情報を表示

**Independent Test**: 起動中のプロセスについてPID、完全なコマンド、経過時間が表示されることを確認

### Tests for User Story 3

- [x] T022 [P] [US3] Unit test for process info retrieval in tests/unit/process_info_test.rs
- [x] T023 [P] [US3] Unit test for elapsed time formatting in tests/unit/output_test.rs

### Implementation for User Story 3

- [x] T024 [US3] Implement detailed process info retrieval with libproc in src/process_info.rs (command line, start time, user)
- [x] T025 [US3] Implement elapsed time calculation and human-readable formatting (e.g., "2h 15m", "1d 3h") in src/process_info.rs
- [x] T026 [US3] Add UPTIME column to table output in src/output.rs
- [x] T027 [US3] Handle permission denied gracefully with partial info display in src/process_info.rs
- [x] T028 [US3] Add warning message for permission issues in src/output.rs

**Checkpoint**: `ports` displays complete process info including uptime and handles permission errors gracefully

---

## Phase 6: JSON出力機能 (Cross-cutting)

**Goal**: --json オプションでJSON形式出力を実現

**Independent Test**: `ports --json | jq .` でJSONが正しくパースされることを確認

- [x] T029 Implement JSON output structure with serde Serialize derives in src/models.rs
- [x] T030 Implement JSON formatter with timestamp and warnings in src/output.rs
- [x] T031 Wire --json flag to output selection in src/main.rs
- [x] T032 [P] Integration test for JSON output in tests/integration/cli_test.rs

**Checkpoint**: `ports --json` outputs valid JSON with all port and process information

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: 品質向上とドキュメント整備

- [x] T033 [P] Add actionable error messages with hints in src/error.rs
- [x] T034 [P] Add inline documentation comments to all public functions
- [x] T035 Run clippy and fix all warnings
- [x] T036 Run cargo fmt to ensure consistent formatting
- [x] T037 Validate against quickstart.md scenarios
- [x] T038 Update README.md with installation and usage instructions

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-5)**: All depend on Foundational phase completion
  - US1 (P1) → US2 (P2) → US3 (P3) in priority order
  - US2 depends on US1's output module
  - US3 depends on US1's basic infrastructure
- **JSON Output (Phase 6)**: Depends on US1 at minimum, benefits from US2/US3
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - Core MVP
- **User Story 2 (P2)**: Can start after US1 table output is complete
- **User Story 3 (P3)**: Can start after US1 basic infrastructure is complete

### Within Each User Story

- Tests written first (TDD per Constitution IV)
- Models/types before logic
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- T002, T003 can run in parallel (different files)
- T005, T006, T007 can run in parallel (independent type definitions)
- T010, T011 can run in parallel (different test files)
- T018, T022, T023 can run in parallel (different test files)

---

## Parallel Example: Phase 2 Foundation

```bash
# These tasks can run in parallel:
Task: T005 "Define Protocol enum in src/models.rs"
Task: T006 "Define AppType enum in src/models.rs"
Task: T007 "Define ProcessInfo struct in src/models.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: `ports` displays basic port list
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → `cargo build` works
2. Add User Story 1 → Basic port listing works (MVP!)
3. Add User Story 2 → App type detection works
4. Add User Story 3 → Detailed process info works
5. Add JSON output → Scripting/automation ready
6. Polish → Production ready

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
