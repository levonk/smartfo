# Agent Documentation: smartfo

## Quick Reference

- **Project Type**: Rust CLI application
- **Language**: Rust 2021 Edition
- **Build System**: Cargo with devbox environment management
- **Architecture**: Single binary with `argv[0]` dispatch; self-spawning daemon for async operations
- **Test Framework**: Cargo test with property-based tests for safety guarantees

---

## Project Overview

**smartfo** is a Rust CLI tool that transparently replaces `mv` and `rm` with VCS-aware, safe, non-blocking operations. One binary (`smartfo`) is installed via symlinks; it dispatches on `argv[0]`.

### What smartfo Does

- **Drop-in replacement** for `mv` and `rm` via symlinks
- **VCS-aware operations**: Automatically detects Git, Mercurial, SVN, Jujutsu, and uses VCS-native move/remove when possible
- **Trash instead of delete**: Files removed via `rm` are moved to a versioned trash directory, never unlinked
- **Async by default**: Background daemon returns the shell prompt immediately; large moves and all deletes are non-blocking
- **Audit trail**: Every operation is logged with structured metadata (timestamp, UUID, VCS repo, reason)
- **Git hooks**: Client-side (`pre-commit`) and server-side (`pre-receive`) hooks block raw deletions and raw renames
- **POSIX compatibility**: Standard flags (`-f`, `-i`, `-n`, `-v`, `-r`, etc.) are fully supported; `--plain` bypasses all smart features

---

## Repository Structure

```
smartfo/
├── src/
│   ├── main.rs           # Entry point, argv[0] dispatch
│   ├── cli.rs            # POSIX-compatible flag parsing per mode
│   ├── config.rs         # TOML loader + environment variable expansion
│   ├── vcs.rs            # VCS detection (git, hg, svn, jj) + tracked-file logic
│   ├── mv.rs             # Move logic + scenario routing
│   ├── rm.rs             # Trash enqueueing + flag handling
│   ├── trash.rs          # Trash mover / versioned directory management
│   ├── daemon.rs         # Self-spawning background daemon + Unix socket
│   ├── queue.rs          # Durable job queue (SQLite WAL)
│   ├── worker.rs         # Background worker: move/copy/fsync/retry
│   ├── logging.rs        # Structured logging + --json output
│   └── audit.rs          # Metadata recording (operations.jsonl)
├── tests/
│   ├── integration/      # Cross-device, crash-recovery, hook tests
│   └── property/         # No-data-loss, directory-tree, VCS consistency
├── internal-docs/
│   └── requirements/     # Initial requirements spec
├── Cargo.toml
└── AGENTS.md             # This file
```

---

## Core Concepts

### argv[0] Dispatch

The binary inspects its invocation name to determine mode:

| Invocation | Mode |
|-----------|------|
| `mv` / `smv` | Move mode (`src/mv.rs`) |
| `rm` / `srm` | Remove mode (`src/rm.rs`) |
| `smartfo --install` | Install symlinks and hooks |

Each mode has its own POSIX-compatible flag parser. There is **no unified parser**.

### Move Scenarios

`mv` handles six scenarios based on VCS tracking and repo boundaries:

| Scenario | Behavior |
|----------|----------|
| Tracked source → same repo | VCS-native move (`git mv`) |
| Tracked source → outside repo | Refuse by default; `--force-outside-vcs` required |
| Outside repo → inside repo | Filesystem move |
| Both outside any repo | Pure filesystem `rename` |
| Neither tracked in repo | Pure filesystem `rename` |
| src == dest | No-op, exit 0 |

### Trash Behavior

All `rm` operations are **asynchronous by default**. The CLI enqueues a job and exits; a daemon performs the filesystem move.

Trash path structure:
```
$TRASH_ROOT/<absolute-path-from-root>/foo/bar/baz.txt/<iso-timestamp>-<counter>
```

- **VCS-committed files**: If clean (no uncommitted changes), VCS-aware remove is used instead of trash.
- **Dirty files**: Moved to trash + VCS-aware remove.
- **Ignored files**: Deleted directly (configurable).

### Daemon Model

- **Self-spawning**: No systemd or LaunchAgent required.
- **Double-fork** on first async operation to detach a background process.
- **PID lockfile** + Unix domain socket for CLI-to-daemon communication.
- **Graceful shutdown** on `SIGTERM`; in-flight jobs complete before exit.
- **Queue store**: SQLite WAL or append-only log; survives restarts.

---

## Build, Lint, and Test

```bash
# Build
cargo build

# Run tests
cargo test

# Run with all features
cargo test --all-features

# Release build
cargo build --release

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt
```

---

## Configuration

Config file: `$HOME/smartfo/config.toml` (or `$XDG_CONFIG_HOME/smartfo/config.toml`)

Key sections:
- `[vcs]` — VCS preference, fallback, supported systems
- `[trash]` — trash root, mode, disk-space guard, retention
- `[concurrency]` — max jobs, network limits, drive detection
- `[behavior]` — smart mode toggles, async thresholds, blocking default
- `[logging]` — level, log file path
- `[paths]` — trash root, audit log, cache dir, config dir override

All string values support POSIX-style environment variable expansion (`$VAR`, `${VAR}`).

Override precedence (highest wins):
1. CLI flags
2. Environment variables (`SMARTFO_<SECTION>_<KEY>`)
3. User config file
4. Built-in defaults

---

## Environment Variables

| Variable | Purpose |
|----------|---------|
| `SMARTFO_BEHAVIOR_DEFAULT_BLOCKING` | Force blocking mode globally |
| `SMARTFO_TRASH_ROOT` | Override trash directory |
| `SMARTFO_PATHS_AUDIT_LOG` | Override audit log path |
| `SMARTFO_CONCURRENCY_MAX_CONCURRENT_JOBS` | Global parallel job ceiling |
| `XDG_DATA_HOME` | Trash default root base |
| `XDG_CACHE_HOME` | Cache directory base |
| `XDG_CONFIG_HOME` | Config directory base |
| `XDG_BIN_HOME` | Preferred symlink target for `--install` |

---

## Common Pitfalls

### Do NOT forget argv[0] is the dispatch mechanism

The binary relies on `argv[0]` to select mode. Running `./smartfo mv` will not work — the binary expects to be invoked as `mv` (via symlink) or with `--install`.

### Daemon lifecycle is not manual

Do not attempt to start or stop the daemon via scripts. It is self-spawning; the CLI spawns it on first async operation and reconnects via socket thereafter. Killing the daemon is safe — it will respawn automatically on the next async call.

### Git hooks require repo-local audit log

The `pre-commit` hook reads from the repo-local audit log at `{REPO_ROOT}/.smartfo/audit/operations.jsonl`. Ensure the audit log is committed or synced if server-side hooks are used. The server-side `pre-receive` hook cannot read the user's global audit log; it must read from the repo-local copy.

### POSIX compatibility is strict

The `--plain` flag must bypass **all** smart features: no VCS detection, no trash, no async, no daemon. This is the escape hatch for scripts that depend on exact POSIX behavior. Ensure any new feature is gated behind the smart path and does not leak into `--plain`.

### Cross-device moves must stream + fsync

When `statfs` detects different filesystems, the worker must perform a chunked copy + `fsync` + `unlink`. A naive `rename` will fail with `EXDEV`.

---

## Testing Requirements

### Integration Tests
- Git / hg / svn / jj repos with tracked and untracked files
- All six move scenarios (in→in, in→out, out→in, out→out, same, untracked-in-repo)
- Cross-device mount tests
- Large file async `mv` tests (verify prompt return, queue completion)
- `rm` async tests (verify prompt return, trash arrival)
- Crash-recovery tests (restart mid-move, verify resume or cleanup)
- Dest-already-exists tests for all overwrite modes

### Property Tests
- No data loss under any path
- Directory trees preserved in trash
- VCS state consistent after move
- Same-file deletion history preserved across multiple deletes
- Disk space guard correctly culls oldest entries first
- Audit log contains valid metadata for every operation
- Git hooks correctly detect and block raw deletions and raw renames
- `--install` correctly creates symlinks and hooks without overwriting existing files

---

## Developer Ergonomics

### Explicit debug entrypoints
- `smv` and `srm` are optional symlink names for debugging smartfo behavior without overriding system `mv`/`rm`.

### Machine-friendly output
- `--json` for structured operation logs
- `--dry-run` to preview operations without executing
- `--reason "..."` to annotate intent in the audit log

---

## Safety & Correctness

- **Atomic operations**: Use `renameat2` with `RENAME_EXCHANGE` when available; fallback to temp-file + fsync + rename.
- **Crash-safe queue**: SQLite WAL or append-only log; each job has UUID, status, retry count.
- **Disk space guard**: Auto-cull oldest trash entries when free space drops below threshold; refuse operation if culling is insufficient.

