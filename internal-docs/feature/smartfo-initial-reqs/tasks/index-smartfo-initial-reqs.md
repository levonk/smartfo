---
prd_name: "smartfo-initial-reqs"
prd_file: "internal-docs/requirements/20260604-smartfo-initial-reqs/20260604-smartfo-initial-reqs.md"
created_at: "2026-06-05"
updated_at: "2026-06-05"
---

# Smartfo Initial Requirements — Task Index

## Parallel Development Sets

### Phase 01
- Story 01-001 | Project scaffolding and CLI framework | Branch: feature/current/smartfo-initial-reqs/story-01-001-project-scaffolding | Dependencies: None | Parallel-safe: true | Modules: main.rs, cli.rs, Cargo.toml
- Story 01-002 | Configuration system with env var expansion | Branch: feature/current/smartfo-initial-reqs/story-01-002-configuration-system | Dependencies: None | Parallel-safe: true | Modules: config.rs
- Story 01-003 | Logging and audit trail infrastructure | Branch: feature/current/smartfo-initial-reqs/story-01-003-logging-audit | Dependencies: None | Parallel-safe: true | Modules: logging.rs, audit.rs

### Phase 02
- Story 02-001 | VCS detection and tracked-file logic | Branch: feature/current/smartfo-initial-reqs/story-02-001-vcs-detection | Dependencies: 01-001 | Parallel-safe: true | Modules: vcs.rs
- Story 02-002 | Install mode with symlinks and Git hooks | Branch: feature/current/smartfo-initial-reqs/story-02-002-install-mode | Dependencies: 01-001 | Parallel-safe: true | Modules: cli.rs, main.rs

### Phase 03
- Story 03-001 | mv mode — POSIX-compatible VCS-aware move | Branch: feature/current/smartfo-initial-reqs/story-03-001-mv-mode | Dependencies: 01-001, 01-002, 02-001 | Parallel-safe: true | Modules: mv.rs
- Story 03-002 | rm mode — trash enqueueing and VCS-aware delete | Branch: feature/current/smartfo-initial-reqs/story-03-002-rm-mode | Dependencies: 01-001, 01-002, 02-001 | Parallel-safe: true | Modules: rm.rs
- Story 03-003 | Trash directory manager and index tracking | Branch: feature/current/smartfo-initial-reqs/story-03-003-trash-manager | Dependencies: 01-002, 01-003 | Parallel-safe: true | Modules: trash.rs

### Phase 04
- Story 04-001 | Durable job queue with SQLite WAL | Branch: feature/current/smartfo-initial-reqs/story-04-001-job-queue | Dependencies: 01-002, 01-003 | Parallel-safe: true | Modules: queue.rs
- Story 04-002 | Self-spawning daemon with Unix socket | Branch: feature/current/smartfo-initial-reqs/story-04-002-daemon | Dependencies: 01-001, 04-001 | Parallel-safe: true | Modules: daemon.rs
- Story 04-003 | Background worker — move/copy/fsync/retry | Branch: feature/current/smartfo-initial-reqs/story-04-003-worker | Dependencies: 03-003, 04-001, 04-002 | Parallel-safe: true | Modules: worker.rs

### Phase 05
- Story 05-001 | Git hooks — pre-commit and pre-receive | Branch: feature/current/smartfo-initial-reqs/story-05-001-git-hooks | Dependencies: 01-003, 02-002, 03-001, 03-002 | Parallel-safe: true | Modules: hooks/
- Story 05-002 | Disk space guard and auto-culling | Branch: feature/current/smartfo-initial-reqs/story-05-002-disk-space-guard | Dependencies: 03-003, 04-003 | Parallel-safe: true | Modules: trash.rs, worker.rs
- Story 05-003 | Concurrency and async mv triggers | Branch: feature/current/smartfo-initial-reqs/story-05-003-concurrency-async-mv | Dependencies: 03-001, 04-003 | Parallel-safe: true | Modules: mv.rs, worker.rs, daemon.rs

### Phase 06
- Story 06-001 | Integration tests | Branch: feature/current/smartfo-initial-reqs/story-06-001-integration-tests | Dependencies: 03-001, 03-002, 04-003, 05-001, 05-002, 05-003 | Parallel-safe: true | Modules: tests/integration/
- Story 06-002 | Property tests | Branch: feature/current/smartfo-initial-reqs/story-06-002-property-tests | Dependencies: 03-001, 03-002, 04-003, 05-001, 05-002 | Parallel-safe: true | Modules: tests/property/

## Dashboard

| Story ID | Title | Phase | Status | Assignee | Parallel-safe | Dependencies | Dependants | Modules | Branch |
|---|---|---:|---|---|---|---|---|---|---|
| 01-001 | Project scaffolding and CLI framework | 01 | [x] Done | | true | — | 02-001, 02-002, 03-001, 03-002 | main.rs, cli.rs, Cargo.toml | feature/current/smartfo-initial-reqs/story-01-001-project-scaffolding |
| 01-002 | Configuration system with env var expansion | 01 | [x] Done | | true | — | 02-001, 03-001, 03-002, 03-003, 04-001 | config.rs | feature/current/smartfo-initial-reqs/story-01-002-configuration-system |
| 01-003 | Logging and audit trail infrastructure | 01 | [x] Done | | true | — | 03-003, 04-001, 05-001, 06-001, 06-002 | logging.rs, audit.rs | feature/current/smartfo-initial-reqs/story-01-003-logging-audit |
| 02-001 | VCS detection and tracked-file logic | 02 | [x] Done | | true | 01-001 | 03-001, 03-002 | vcs.rs | feature/current/smartfo-initial-reqs/story-02-001-vcs-detection |
| 02-002 | Install mode with symlinks and Git hooks | 02 | [x] Done | | true | 01-001 | 05-001 | cli.rs, main.rs | feature/current/smartfo-initial-reqs/story-02-002-install-mode |
| 03-001 | mv mode — POSIX-compatible VCS-aware move | 03 | [x] Done | | true | 01-001, 01-002, 02-001 | 04-003, 05-001, 05-003, 06-001, 06-002 | mv.rs | feature/current/smartfo-initial-reqs/story-03-001-mv-mode |
| 03-002 | rm mode — trash enqueueing and VCS-aware delete | 03 | [~] In-Progress | | true | 01-001, 01-002, 02-001 | 04-003, 05-001, 06-001, 06-002 | rm.rs | feature/current/smartfo-initial-reqs/story-03-002-rm-mode |
| 03-003 | Trash directory manager and index tracking | 03 | [ ] Todo | | true | 01-002, 01-003 | 04-003, 05-002 | trash.rs | feature/current/smartfo-initial-reqs/story-03-003-trash-manager |
| 04-001 | Durable job queue with SQLite WAL | 04 | [ ] Todo | | true | 01-002, 01-003 | 04-002, 04-003 | queue.rs | feature/current/smartfo-initial-reqs/story-04-001-job-queue |
| 04-002 | Self-spawning daemon with Unix socket | 04 | [ ] Todo | | true | 01-001, 04-001 | 04-003, 05-003 | daemon.rs | feature/current/smartfo-initial-reqs/story-04-002-daemon |
| 04-003 | Background worker — move/copy/fsync/retry | 04 | [ ] Todo | | true | 03-003, 04-001, 04-002 | 05-002, 05-003, 06-001, 06-002 | worker.rs | feature/current/smartfo-initial-reqs/story-04-003-worker |
| 05-001 | Git hooks — pre-commit and pre-receive | 05 | [ ] Todo | | true | 01-003, 02-002, 03-001, 03-002 | 06-001, 06-002 | hooks/ | feature/current/smartfo-initial-reqs/story-05-001-git-hooks |
| 05-002 | Disk space guard and auto-culling | 05 | [ ] Todo | | true | 03-003, 04-003 | 06-001, 06-002 | trash.rs, worker.rs | feature/current/smartfo-initial-reqs/story-05-002-disk-space-guard |
| 05-003 | Concurrency and async mv triggers | 05 | [ ] Todo | | true | 03-001, 04-003 | 06-001, 06-002 | mv.rs, worker.rs, daemon.rs | feature/current/smartfo-initial-reqs/story-05-003-concurrency-async-mv |
| 06-001 | Integration tests | 06 | [ ] Todo | | true | 03-001, 03-002, 04-003, 05-001, 05-002, 05-003 | — | tests/integration/ | feature/current/smartfo-initial-reqs/story-06-001-integration-tests |
| 06-002 | Property tests | 06 | [ ] Todo | | true | 03-001, 03-002, 04-003, 05-001, 05-002 | — | tests/property/ | feature/current/smartfo-initial-reqs/story-06-002-property-tests |
