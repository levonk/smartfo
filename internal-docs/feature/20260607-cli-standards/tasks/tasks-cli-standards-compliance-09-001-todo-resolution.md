---
story_id: "09-001"
story_title: "TODO Resolution and Feature Completion"
story_name: "todo-resolution"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 9
parallel_id: 1
branch: "feature/current/cli-standards-compliance/story-09-001-todo-resolution"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-001", "01-002", "01-003", "02-001", "02-002", "03-001", "03-002", "04-001", "04-002", "04-003", "05-001", "05-002"]
parallel_safe: false
modules: ["src/main.rs", "src/rm.rs", "src/daemon.rs", "src/hooks.rs", "src/trash.rs", "src/vcs.rs"]
priority: "MUST"
risk_level: "high"
tags: ["bugfix", "completion", "core"]
due: "2026-06-15"
created_at: "2026-06-09"
updated_at: "2026-06-09"
---

## Summary

Resolve all TODO comments in the codebase that indicate incomplete implementation of core features. Many stories were marked as "done" but contain TODOs suggesting incomplete implementation, similar to the hook installation gap. This story systematically addresses 11 TODOs across 5 modules, prioritizing critical bugs and missing core functionality.

## Sub-Tasks

### Critical Priority (Core Functionality Bugs)

- [ ] Fix VCS info extraction bug in rm.rs (line 318) - VCS info always returns None despite calling detect_vcs()
- [ ] Implement VCS-aware move logic in main.rs (line 223) - Story 03-001 marked done but move not VCS-aware
- [ ] Implement full smart rm logic in main.rs (line 295) - Story 03-002 marked done but rm not fully smart
- [ ] Implement full daemon event loop in daemon.rs (line 356) - Story 04-002 marked done but daemon only handles single connection
- [ ] Implement job queue enqueue in rm.rs (line 515) - Story 04-001 marked done but async operations don't use queue

### High Priority (Data Loss Risks)

- [ ] Implement blocking trash operation in rm.rs (line 490) - Story 04-003 marked done but blocking mode doesn't wait for worker
- [ ] Implement in-flight job completion in daemon.rs (line 365) - Story 04-003 marked done but shutdown doesn't wait for jobs

### Medium Priority (Feature Completeness)

- [ ] Replace sample data in list command main.rs (line 330) - CLI AXI feature shows fake data instead of real queue/audit data
- [ ] Replace sample aggregates in status command main.rs (line 418) - CLI AXI feature shows fake aggregates instead of real daemon/queue data
- [ ] Implement auto-cleanup trigger in trash.rs (line 503) - Story 05-002 marked done but auto-cleanup not implemented

### Low Priority (Enhancements)

- [ ] Implement daemon queue size check in hooks.rs (line 56) - Session context doesn't show queue size

## Relevant Files

- `src/main.rs` — VCS-aware move logic, smart rm logic, sample data replacement
- `src/rm.rs` — VCS info extraction bug, job queue enqueue, blocking trash operation
- `src/daemon.rs` — Full daemon event loop, in-flight job completion
- `src/hooks.rs` — Daemon queue size check
- `src/trash.rs` — Auto-cleanup trigger
- `src/vcs.rs` — VCS detection logic (may need updates for rm.rs integration)

## Acceptance Criteria

### Critical Priority

- [ ] VCS info extraction in rm.rs correctly returns VCS type and repo root
- [ ] Move operations are VCS-aware (use git mv when appropriate)
- [ ] RM operations have full smart behavior (VCS-aware, trash, async)
- [ ] Daemon processes jobs from queue in event loop
- [ ] Async operations enqueue to job queue correctly

### High Priority

- [ ] Blocking trash mode waits for worker completion
- [ ] Daemon shutdown waits for in-flight jobs to complete

### Medium Priority

- [ ] List command shows real queue/audit data instead of sample data
- [ ] Status command shows real daemon/queue aggregates instead of sample data
- [ ] Trash auto-cleanup triggers when disk space is low

### Low Priority

- [ ] Session context shows accurate queue size

### General

- [ ] All TODO comments are removed or updated with implementation status
- [ ] All previously marked "done" stories are actually complete
- [ ] Unit tests pass for all fixed functionality
- [ ] Integration tests pass for all fixed functionality
- [ ] Code review confirms no TODOs remain in core paths

## Test Plan

- Unit: `cargo test` (focus on rm, daemon, vcs modules)
- Integration: `cargo test --test integration` (test VCS-aware operations, daemon job processing)
- Manual: Test VCS-aware move in git repository
- Manual: Test smart rm with VCS detection
- Manual: Test daemon job queue processing
- Manual: Test blocking vs async trash operations
- Manual: Test daemon graceful shutdown with in-flight jobs
- Manual: Verify list command shows real data
- Manual: Verify status command shows real aggregates
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log VCS detection results for move/rm operations
- Log daemon job queue processing
- Log daemon shutdown with job completion status
- Log trash auto-cleanup triggers

## Compliance

- Ensures all core features work as documented in AGENTS.md
- Ensures audit logging captures complete VCS information
- Ensures no data loss during daemon shutdown

## Risks & Mitigations

- Risk: VCS detection may be complex across different VCS types — Mitigation: Start with Git, add others incrementally
- Risk: Daemon event loop may have race conditions — Mitigation: Comprehensive testing with concurrent operations
- Risk: Blocking mode may hang if worker never completes — Mitigation: Add timeout and error handling
- Risk: Removing sample data may break CLI AXI features — Mitigation: Ensure real data sources are available before removal

## Dependencies & Sequencing

- Depends on: All previous stories (01-001 through 05-002) since this completes their incomplete implementations
- Unblocks: True release readiness (cannot release with core features incomplete)

## Definition of Done

- All 11 TODOs are addressed (implemented or properly documented as deferred)
- All core functionality works as documented
- All tests pass
- No TODOs remain in critical code paths
- Stories marked as "done" are actually complete

## Commit Conventions

- Use conventional commits with module scoping, e.g., `fix(rm): resolve VCS info extraction bug`, `feat(daemon): implement full event loop`

## Changelog

- 2026-06-09: initialized story file to address 11 TODOs found in codebase

## Notes

- Many stories were marked as "done" but contain TODOs, indicating a planning/documentation gap similar to the hook installation issue
- This story represents a systematic cleanup to ensure all "done" stories are actually complete
- Some TODOs reference future stories that may not exist - these should be either implemented or properly documented as deferred
- Consider updating story completion criteria to require TODO removal before marking as "done"