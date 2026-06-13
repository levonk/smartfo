# Smartfo Requirements Specification

**Version:** 1.0.0  
**Date:** 2026-06-09  
**Status:** Consolidated Specification

---

## Table of Contents

1. [Project Overview](#project-overview)
2. [Core Architecture](#core-architecture)
3. [Entrypoints & Dispatch](#entrypoints--dispatch)
4. [mv Mode — VCS-Aware Move](#mv-mode--vcs-aware-move)
5. [rm Mode — Trash Instead of Delete](#rm-mode--trash-instead-of-delete)
6. [CLI Standards Compliance](#cli-standards-compliance)
7. [Agent eXperience Interface (AXI)](#agent-experience-interface-axi)
8. [Configuration Management](#configuration-management)
9. [Safety & Correctness](#safety--correctness)
10. [Testing Requirements](#testing-requirements)
11. [Non-Functional Requirements](#non-functional-requirements)

---

## Project Overview

**smartfo** is a Rust CLI tool that transparently replaces `mv` and `rm` with VCS-aware, safe, non-blocking operations. One binary (`smartfo`) is installed via symlinks; it dispatches on `argv[0]`.

### Key Features

- **Drop-in replacement** for `mv` and `rm` via symlinks
- **VCS-aware operations**: Automatically detects Git, Mercurial, SVN, Jujutsu, and uses VCS-native move/remove when possible
- **Trash instead of delete**: Files removed via `rm` are moved to a versioned trash directory, never unlinked
- **Async by default**: Background daemon returns the shell prompt immediately; large moves and all deletes are non-blocking
- **Audit trail**: Every operation is logged with structured metadata (timestamp, UUID, VCS repo, reason)
- **Git hooks**: Client-side (`pre-commit`) and server-side (`pre-receive`) hooks block raw deletions and raw renames
- **POSIX compatibility**: Standard flags (`-f`, `-i`, `-n`, `-v`, `-r`, etc.) are fully supported; `--plain` bypasses all smart features
- **CLI standards compliance**: Full compliance with CLI Tool Standards ADR (35 standards)
- **Agent mode**: AXI-compliant agent interface with TOON output, session hooks, and installable skills

### Technology Stack

- **Language**: Rust 2021 Edition
- **Build System**: Cargo with devbox environment management
- **Architecture**: Single binary with `argv[0]` dispatch; self-spawning daemon for async operations
- **Test Framework**: Cargo test with property-based tests for safety guarantees

---

## Core Architecture

### Repository Structure

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
│   ├── audit.rs          # Metadata recording (operations.jsonl)
│   ├── axi.rs            # Agent mode: TOON output, session hooks
│   └── completion.rs     # Shell completion generation
├── tests/
│   ├── integration/      # Cross-device, crash-recovery, hook tests
│   └── property/         # No-data-loss, directory-tree, VCS consistency
├── internal-docs/
│   └── requirements/     # This specification
├── Cargo.toml
└── AGENTS.md
```

### argv[0] Dispatch

The binary inspects its invocation name to determine mode:

| Invocation | Mode |
|-----------|------|
| `mv` / `smv` | Move mode (`src/mv.rs`) |
| `rm` / `srm` | Remove mode (`src/rm.rs`) |
| `smartfo --install` | Install mode |
| `smartfo --uninstall` | Uninstall mode |

Each mode has its own POSIX-compatible flag parser. There is **no unified parser**.

---

## Entrypoints & Dispatch

### Install Mode (`smartfo --install`)

One-shot setup command that configures the environment.

#### Symlink Creation

Creates `mv`, `rm`, `smv`, `srm` symlinks pointing to the `smartfo` binary.

**Installation priority:**
1. If `$XDG_BIN_HOME` is defined and is on PATH: symlinks go there (user install)
2. If `$XDG_BIN_HOME` is not defined but `~/.local/bin` exists and is on PATH: symlinks go there (user install)
3. If neither exists: create `~/.local/bin`, ensure it is on PATH (warn if not), and place symlinks there
4. If running as **root** and `/usr/local/bin` is on PATH: symlinks go there (system install)
5. If target files already exist and are not symlinks to `smartfo`: refuse with error unless `--force` is passed

#### Shell Completion

- Generate shell completion scripts for bash, zsh, and fish
- Install completions to appropriate shell directories
- Use clap's completion generation features

#### Man Pages

- Generate traditional Unix man pages for documentation
- Provide man pages for smartfo, mv, and rm modes
- Make accessible via `man smartfo`, `man smartfo-mv`, `man smartfo-rm`
- Install man pages to system man directory

#### Config Initialization

- On first run, detect if config file exists in expected location
- If missing, create default config file with all settings commented out
- Include default values and explanations for each option in comments
- Support `--init-config` flag to explicitly create/recreate default config

#### Git Hook Installation

Only if invoked inside a Git repo. The `smartfo` binary provides two hook subcommands:

- `smartfo-git-hook-client` — client-side hook (used as `pre-commit`)
- `smartfo-git-hook-server` — server-side hook (used as `pre-receive`)

**Client-side hook (`pre-commit`)**: Verifies that staged deletions and renames have matching `smartfo` metadata in the audit log.
- Blocks raw `rm` deletions (no metadata)
- Blocks raw `mv` renames that bypass `smartfo`

**Server-side hook (`pre-receive`)**: Scans incoming push for deleted files and renames, cross-referencing the repo-local audit log (`{REPO_ROOT}/.smartfo/audit/operations.jsonl`).

**Install flags:**
- `--hooks client` — install only client-side hook
- `--hooks server` — install only server-side hook
- `--hooks client,server` — install both (default)
- `--no-hooks` — skip hook installation entirely

If a hook file already exists and is not a symlink to `smartfo`: refuse with error unless `--force` is passed.

#### Alias Warnings

When `--install` is called, smartfo checks for existing shell aliases that cover the `mv`, `rm`, `smv`, or `srm` commands.

**Alias Detection:**
- Detect shell aliases for: `mv`, `rm`, `smv`, `srm`
- Support detection in: bash, zsh, fish
- Check both current shell session and shell configuration files
- Shell config files:
  - bash: `~/.bashrc`, `~/.bash_profile`, `~/.profile`
  - zsh: `~/.zshrc`, `~/.zprofile`
  - fish: `~/.config/fish/config.fish`

**Warning Messages:**
- Display warning when aliases are detected
- Format: "Warning: to remove existing aliases, run `unalias <command>`"
- List all detected aliases with their removal commands
- Provide guidance on removing persistent aliases from shell config files
- Ignore aliases that point to smartfo itself (no warning needed)

**Integration:**
- Perform alias check after symlink creation
- Do not fail installation if aliases are found (only warn)
- Support `--force` flag to bypass alias warnings

### Uninstall Mode (`smartfo --uninstall`)

Cleanup counterpart to install mode.

**Uninstall actions:**
- Remove symlinks (mv, rm, smv, srm)
- Remove shell completion scripts
- Remove man pages
- Optionally remove config files with user confirmation
- Add `--force` flag to bypass confirmation prompts

---

## mv Mode — VCS-Aware Move

### VCS Detection

Detect whether source and/or destination are inside a VCS working tree. Supported VCS: **Git, Mercurial (hg), SVN, Jujutsu (jj), and any future VCS exposing a comparable `mv` command or library API.**

- Discover repo root via VCS-specific commands or libraries (e.g., `git rev-parse --show-toplevel`, `hg root`, `svn info`, `jj root`)
- Detect whether the source path is tracked by that VCS

### Move Scenarios

The tool must correctly handle all combinations of source and destination:

| Scenario | Behavior |
|----------|----------|
| **src tracked, dest in same repo** | Execute VCS-native move (e.g., `git mv`, `hg mv`) |
| **src tracked, dest outside repo** | Refuse by default; require `--force-outside-vcs`. If forced: VCS `rm` the source, then filesystem move to dest. With `--no-vcs-rm`: perform only the filesystem move (or copy), leaving the source tracked in VCS |
| **src outside repo, dest inside repo** | Filesystem move to dest |
| **src and dest both outside any repo** | Pure filesystem `rename` |
| **src and dest both in repo, neither tracked** | Pure filesystem `rename` |
| **src == dest** | No-op with exit code 0 (match POSIX `mv`) |
| **dest already exists** | Default: refuse (`-n` semantics). With `-f`: overwrite. With `-i`: prompt. With `--backup`: back up existing dest before overwrite |

### POSIX Compatibility

- Accept standard flags: `-f`, `-n`, `-i`, `-v`, `-T`, `-t`, `--backup`, `--strip-trailing-slashes`, etc.
- `--plain` — disable all smart features for this invocation; behave exactly like standard POSIX `mv` (no VCS detection, no async, no safety guards)
- Preserve exit codes and stderr formatting so scripts do not break
- Behave identically to GNU `mv` when no VCS is involved

### Async Behavior for mv Mode

#### When Async Kicks In

`mv` is **synchronous by default** because same-filesystem `rename()` is atomic and near-instant; making every move async would add daemon overhead with no benefit and could break scripts expecting the destination path to exist immediately. Async is triggered only when the operation is predictably expensive:

- Cross-device move detected via `statfs`
- Source file size exceeds configurable threshold (default: 100MB)
- Explicit `--async` flag passed

#### User Experience

When an async mv job is enqueued:
- Print to stderr: `smartfo: large file move queued in the background (use --blocking to wait)`
- Return exit code 0 immediately
- The background worker completes the move and logs the result

#### Synchronous Override

- `--blocking` flag forces the operation to wait until completion for both `mv` and `rm`
- Config override: `[behavior].default_blocking = true` (for users who prefer blocking by default)

---

## rm Mode — Trash Instead of Delete

### Trash Directory Mirroring

Config example:
```toml
[trash]
root = "$XDG_DATA_HOME/smartfo/trash"
preserve_tree = true
backup_vcs_committed = false
backup_ignored_files = false
```

**Trash path structure:**
```
$TRASH_ROOT/<absolute-path-from-root>/foo/bar/baz.txt/<iso-timestamp>-<counter>
```

Example: `~/.local/share/smartfo/trash/home/user/src/foo/bar/baz.txt/2026-06-04T09:15:30Z-001`

**Trash behavior:**
- When user runs `rm foo/bar/baz.txt`, compute a versioned destination
- Create parent directories as needed
- Move the file (never unlink)
- **Same-file history**: Deleting the same source path multiple times preserves every version in timestamped subdirectories. A `.smartfo-index` JSONL file in the trash entry records the full history: original path, deletion timestamp, operation UUID, and reason (if provided via `--reason "refactor: replaced with new parser"`)
- **Default trash root**: `$XDG_DATA_HOME/smartfo/trash` (typically `~/.local/share/smartfo/trash`). This is user data, not cache — it must survive cache clears

**VCS-committed files**: If `backup_vcs_committed = false` (default), the behavior depends on whether the file has uncommitted changes:
- **Clean** (no modifications since last commit): Do not move to trash. Perform a VCS-aware remove (e.g., `git rm`) since the committed version is recoverable from VCS history
- **Dirty** (modified since last commit): Move to trash **and** perform VCS-aware remove (`git rm`). The uncommitted changes are not in VCS history, so they must be preserved in trash

**Ignored files**: If `backup_ignored_files = false` (default), files matched by `.gitignore`/`.hgignore`/etc. are deleted directly without moving to trash. These are typically build artifacts, dependencies, or temp files that are reproducible and do not need backup.

**Uncommitted, non-ignored files**: Behavior depends on `trash_mode`:
- `trash_mode = "always"` (default): Always move to trash
- `trash_mode = "auto"`: Move to trash if space allows; if trash is full or disabled, fall back to direct delete with a warning
- `trash_mode = "never"`: Never use trash; perform VCS-aware delete (`git rm`) if tracked, or direct delete if untracked

### Disk Space Guard

Before every trash move, check available disk space on the trash filesystem:

- If free space drops below `min_free_space_percent` (default: **20%**), auto-cull trash history starting with oldest entries until free space is above the threshold
- If culling cannot free enough space, the default behavior is to **refuse the operation** with a clear error:
  ```
  smartfo: trash disk space critically low (8% free). Use --force-delete to bypass trash, or free space.
  ```
- **Override behavior** via `on_trash_full` config:
  - `on_trash_full = "refuse"` (default): Refuse the operation; user must free space or pass `--force-delete`
  - `on_trash_full = "delete"`: Bypass trash and perform direct delete (VCS-aware if tracked) with a warning
- Culling policy: remove oldest versions first; never remove the only remaining version of a file unless `allow_last_version_cull = true` in config
- **CLI override**: `--force-delete` bypasses trash for this single invocation, regardless of `trash_mode` or disk space

### Asynchronous by Default

**All rm operations are asynchronous by default.** The user must return to the CLI prompt immediately, even for large files or cross-device moves.

- Immediately enqueue the move job and print: `smartfo: moving to trash in the background (use --blocking to wait)`
- A background worker performs the actual filesystem move
- The `--blocking` flag forces the operation to block until completion when the user needs confirmation before proceeding
- The `--sync` flag forces an `fsync` on the destination file and containing directory before the job is marked done (configurable default via `sync_on_complete` in config)

### Background Worker (Daemon Model)

To return the shell prompt immediately, the operation cannot run inside the CLI process. The CLI process enqueues the job and exits; a separate daemon performs the work.

**Daemon lifecycle:**
- The daemon is **self-spawning** — no systemd, init.d, LaunchAgent, or other OS service manager is required
- On the first async operation, the CLI binary double-forks to detach a background daemon process, then exits
- The daemon writes its PID to a lockfile and listens on a Unix domain socket
- Subsequent CLI invocations connect to the existing daemon via the socket; if the daemon has died, a new one is spawned automatically
- **Job queue**: Durable store (SQLite WAL or append-only log). Each job: UUID, source path, dest path, status (queued/running/done/failed), retry count
- **Worker performs**:
  - Atomic rename if same filesystem
  - Chunked copy + fsync + unlink if cross-device
- **Worker logs failures** and retries with exponential backoff
- **Queue survives process restarts** and is idempotent
- **Shutdown handling**: Graceful shutdown on `SIGTERM`; in-flight jobs are allowed to complete before exit

### Daemon Concurrency

When multiple files are removed or moved in one invocation (e.g., `rm file1 file2 file3`):

- Each file becomes an independent job in the queue
- The daemon adapts parallelism based on system conditions:
  - **Same filesystem, same directory**: Serialized to avoid lock contention
  - **Same filesystem, different directories**: Parallel up to `cpu_cores` workers
  - **Cross-device (different physical drives)**: Parallel up to `destination_drive_count` workers
  - **Network-mounted destinations**: Limited to `network_concurrency` (default 2) to avoid saturating the link
- All limits are capped by a global `max_concurrent_jobs` ceiling
- If `--blocking` is used, the CLI waits until all enqueued jobs for that invocation reach `done` status

### POSIX Compatibility

- Accept standard flags: `-f`, `-i`, `-I`, `-r`, `-R`, `-d`, `--preserve-root`, `--one-file-system`, etc.
- `--plain` — disable all smart features for this invocation; behave exactly like standard POSIX `rm` (no trash, no VCS awareness, direct deletion)
- Preserve exit codes and stderr formatting

### Operation Metadata & Audit Trail

Every `mv` and `rm` operation records structured metadata:

```json
{
  "op": "delete",
  "source_path": "/home/user/src/utils/old_parser.rs",
  "trash_path": "/home/user/.local/share/smartfo/trash/.../old_parser.rs/2026-06-04T09:15:30Z-001",
  "reason": "refactor: replaced with new parser",
  "timestamp": "2026-06-04T09:15:30Z",
  "uuid": "a1b2c3d4-e5f6-...",
  "tool": "smartfo",
  "vcs": "git",
  "repo_root": "/home/user/src",
  "committed": true
}
```

**Storage:**
- Appended to `$XDG_DATA_HOME/smartfo/audit/operations.jsonl` (one JSON object per line)
- **All paths overridable** via `[paths]` config, environment variables (`SMARTFO_PATHS_AUDIT_LOG`), or CLI flags (`--audit-log-path`)
- **Server-side access**: For server-side hooks, the audit log must be accessible to the Git server. The server-side hook reads from `{REPO_ROOT}/.smartfo/audit/operations.jsonl` (a repo-local copy of the audit log, updated on push or via a post-receive sync)
- **Retention**: Audit log is pruned by age (configurable, default 90 days) independently of trash contents

**Git hook verification:**
- The `pre-commit` hook reads the audit log for the current repo and verifies every deletion and rename has a matching `smartfo` metadata entry
- Raw `rm` deletions (no metadata) cause the hook to fail with: `smartfo hook: detected raw deletion of src/utils/old_parser.rs. Use 'srm' or 'rm' (smartfo) instead.`
- Raw `mv` renames (no metadata) cause the hook to fail with: `smartfo hook: detected raw rename of src/utils/old_parser.rs -> src/utils/new_parser.rs. Use 'smv' or 'mv' (smartfo) instead.`
- **Server-side hook (`pre-receive`)**: Rejects pushes containing raw deletions or raw renames by scanning the commit tree for removed files and renames, cross-referencing the repo-local audit log

**Reason flag:** `--reason "..."` allows the user to annotate the intent of an operation (e.g., `--reason "cleanup: remove dead code"`)

---

## CLI Standards Compliance

Smartfo implements all 35 standards from the CLI Tool Standards ADR (adr-20260607001).

### Developer UX Standard (ADR #0)

Conform to Developer UX standard including:
- Support direnv for environment variable management
- Support devbox for development environment consistency
- Provide justfile for common development tasks
- Support nix for reproducible development environments
- Support nx for monorepo tooling (if applicable)
- Support containers for development and testing
- Follow unified logging standards

### Standard Arguments (ADR #1)

- Add `--help`/`-h` flag that displays comprehensive help for current mode (mv/rm/smartfo)
- Add `--version`/`-v` flag that displays version information
- Add `--usage` flag that displays brief usage summary
- Ensure all three flags work at root command and for each mode (mv, rm, install)

### Configuration Precedence (ADR #2)

Document and enforce precedence: CLI args > env vars > project config > user config (XDG) > system config > defaults

- Support project config at `{REPO_ROOT}/.config/smartfo/config.toml`
- Support user config at `$XDG_CONFIG_HOME/smartfo/config.toml` or `$HOME/.config/smartfo/config.toml`
- Add system config support at `/etc/smartfo/config.toml` (Linux) or equivalent platform-specific locations
- Add config variable to control noisy fallback behavior for daemon on unsupported platforms (e.g., `daemon_fallback_quiet = true`)

### Config File Initialization (ADR #3)

- On first run, detect if config file exists in expected location
- If missing, create default config file with all settings commented out
- Include default values and explanations for each option in comments
- Support `--init-config` flag to explicitly create/recreate default config

### Install/Uninstall Flag (ADR #4)

Enhanced `--install` flag (detailed in Entrypoints section):
- Generate shell completion scripts for bash, zsh, and fish
- Initialize default config files in appropriate XDG locations
- Set up any required environment variables
- Install man pages to system man directory

`--uninstall` counterpart for cleanup:
- Remove symlinks (mv, rm, smv, srm)
- Remove shell completion scripts
- Remove man pages
- Optionally remove config files with user confirmation
- Add `--force` flag to bypass confirmation prompts during uninstall

### Input & Globbing (ADR #5)

- Support recursive `**/*` globbing patterns for file arguments
- Support stdin input via `-` argument or piped input for operations that accept file lists
- Process files and stdin interchangeably where applicable
- Ensure globbing works correctly with VCS-aware operations

### Output Discipline (ADR #6)

- Ensure all results go to stdout
- Ensure all logs, progress indicators, and errors go to stderr
- Add `--json` output mode for machine-readable structured output
- Add `--color=auto|always|never` flag for color control (default: auto)
- Implement smart TTY detection in auto mode for color control
- Add `color` setting to config file with modes: auto|always|never
- Honor `NO_COLOR` environment variable (takes precedence over all other color settings)

### Logging Modes (ADR #7)

- Add `--verbose`/`-v` flag for increased logging verbosity
- Add `--quiet`/`-q` flag to suppress non-essential output including progress indicators
- Add `--debug` flag for detailed debug logging
- Ensure log levels are respected across all modules
- Integrate with existing structured logging in `src/logging.rs`

### Signals & Exit Codes (ADR #8)

- Implement graceful SIGINT handling with exit code 130
- Define standard exit codes:
  - 0: Success
  - 1: Generic error
  - 2: Usage error
  - 3: Network error
  - 4: Validation error
  - 5: File not found
  - 6: Permission denied
  - 7: VCS operation failed
  - 8: Daemon operation failed
- Ensure all error paths use appropriate exit codes

### TUI Mode (ADR #9)

- Implement TUI mode triggered by `--interactive` or `--tui` flag
- Allow users to view and modify all arguments before execution
- Support interactive configuration of smartfo settings
- Provide TUI for complex operations (install, config editing, batch operations)
- Use a Rust TUI library (e.g., ratatui, crossterm)
- Ensure TUI mode respects terminal size and handles resize events

### Dry-Run Mode (ADR #10)

- Add `--dry-run` flag to preview changes without executing them
- Show exactly what would be done for each operation
- Display VCS commands that would be executed
- Display file moves/deletes that would occur
- Display daemon operations that would be queued
- Ensure dry-run mode has no side effects (no file system changes, no VCS commands)

### Confirmation Prompts (ADR #11)

- Require confirmation for destructive operations (delete, overwrite)
- Add `--force` flag to bypass confirmation prompts
- Add `--interactive`/`-i` flag to enable prompts even for non-destructive operations
- Ensure prompts are clear about what will happen
- Support batch confirmation (yes to all, no to all)

### Progress Indicators (ADR #12)

- Show progress bars or spinners for long-running operations
- Display progress for async daemon operations when using `--no-daemon` mode (synchronous)
- Display progress for large file copies (cross-device moves)
- Respect `--quiet` flag (no progress indicators in quiet mode)
- Use a progress bar library (e.g., indicatif)

### Daemon Process Support (ADR #13)

Enhance existing self-spawning daemon model to support all ADR requirements simultaneously:

- Provide `--daemon` and `--no-daemon` flags as opposites
- Auto-spawn daemon on first async operation (current behavior)
- `--daemon` flag pre-launches daemon in background and waits for jobs
- `--no-daemon` flag forces synchronous in-process operation (disables auto-spawning)
- Return job ID immediately when daemon performs operation
- Include `--list-jobs` command to show background job status, with optional job ID list to filter results
- Include `--cancel-job <id>` command to cancel specific background jobs
- Provide instructions for monitoring job progress
- If daemon mode not supported on platform:
  - Fall back to synchronous processing with clear error message explaining limitation if `--no-daemon` is not used
  - Suggest alternatives if `--daemon` is used
  - Allow config variable to override this noisy behavior for unsupported platforms
- Maintain existing Unix socket and PID lockfile infrastructure

### Error Message Formatting (ADR #14)

- Ensure all error messages follow format: `ERROR: <description> - <suggestion>`
- Provide actionable suggestions for resolution
- Include file references in VSCode-compatible format: `file:///absolute/path/to/file:line:column`
- Include relevant context (e.g., which VCS operation failed, which file caused the error)

### File Reference Formatting (ADR #15)

- Ensure all file references with line numbers use VSCode-compatible format
- Support both `file:///absolute/path/to/file:line:column` and standard `file:line:column`
- Ensure modern terminals can auto-linkify these references

### URL Formatting (ADR #16)

- Ensure all URLs are in standard HTTP/HTTPS format with proper encoding
- Support copying URLs to browser
- Ensure smart terminal linking works for URLs

### Shell Completion (ADR #17)

- Generate shell completion scripts for bash, zsh, and fish
- Use clap's completion generation features
- Ensure completions match current command structure
- Include completions for all modes (mv, rm, smartfo)
- Include completions for config keys and values where applicable
- Install completions via `--install` flag

### Man Pages (ADR #18)

- Generate traditional Unix man pages for documentation
- Provide man pages for smartfo, mv, and rm modes
- Make accessible via `man smartfo`, `man smartfo-mv`, `man smartfo-rm`
- Add `--man` flag to display man page content
- Install man pages to system man directory via `--install`
- Use a man page generation library or manual roff source

### Pager Integration (ADR #19)

- Auto-pager for long output (help, config, job lists)
- Respect `PAGER` environment variable, default to `less`
- Add `--no-pager` flag to bypass paging
- Detect if output is interactive before enabling pager

### Subcommand Organization (ADR #20)

- Maintain hierarchical command structure
- Group related commands under logical subcommands
- Ensure consistency in command naming patterns
- Document command hierarchy in help output

### Configuration Validation (ADR #21)

- Validate config files on load
- Report clear, specific error messages with line numbers
- Provide suggestions for fixing config errors
- Validate config schema version
- Ensure invalid config doesn't crash the application

### Terminal Size Awareness (ADR #22)

- Detect terminal size on startup
- Format output based on terminal width
- Handle terminal resize events where possible
- Provide reasonable defaults for non-terminal output

### Environment Variable Naming (ADR #23)

- Ensure all environment variables use consistent `SMARTFO_` prefix
- Document all supported environment variables
- Ensure env var naming follows section_key pattern (e.g., `SMARTFO_BEHAVIOR_DEFAULT_BLOCKING`)
- Complete coverage for all config options

### Cross-Platform Path Handling (ADR #24)

- Ensure consistent path handling across Windows, Linux, and macOS
- Use platform-appropriate separators
- Handle both forward and backward slashes
- Use Rust's std::path for cross-platform path operations
- Test on all three platforms

### Credential/Secret Handling (ADR #25)

- Ensure no logging of secrets or sensitive data
- Provide secure storage options for any credentials (if needed in future)
- Add clear warnings about insecure config methods
- Sanitize audit logs to remove sensitive paths if privacy mode enabled

### Resource Limits (ADR #26)

- Add `--max-memory` flag for memory-intensive operations
- Add `--max-cpu` flag for CPU-intensive operations
- Document memory/CPU usage guidelines
- Implement resource limiting for daemon operations

### Testing (ADR #27)

- Add tests for help output for all modes
- Add tests for globbing patterns
- Add tests for stdin input handling
- Add tests for config precedence (CLI > env > project > user > system > defaults)
- Add tests for JSON vs human output modes
- Add tests for exit-code behavior for all error paths
- Add tests for standard arguments (--help, --version, --usage)
- Add tests for config file initialization
- Add tests for shell completion script generation
- Add tests for error handling and formatting
- Add tests for daemon mode where feasible
- Add tests for --list-jobs with optional job ID filtering
- Add tests for daemon platform fallback behavior and config variable override

### Collection vs Processing Separation (ADR #28)

- Separate daemon collection (background job processing) from CLI processing
- Allow data collection in one environment and processing in another
- Add export commands for collected job data
- Add analysis commands that operate on exported data without requiring daemon
- This aligns with existing daemon model; enhance with explicit export/analysis commands

### Config File Versioning (ADR #29)

- Add schema version field to config files for future evolution
- Validate config schema version on load
- Reject configs with unsupported schema versions with clear error message
- Document config schema version in upgrade notes

### Structured Logging with Format Auto-Detection (ADR #30)

- Use structured logging (JSON or structured text) with format auto-detection based on TTY
- Support language-native env filters (RUST_LOG)
- Resolve log level: env vars > CLI flags > config file > defaults
- Already partially implemented in `src/logging.rs`; enhance with auto-detection

### Signal-Based Config Reload (ADR #31)

- Support SIGHUP to reload config files without restart
- Validate new config before applying
- Log reload events to audit log
- Handle validation errors gracefully (keep old config active)
- Apply to both CLI and daemon processes

### Health Check for Containers (ADR #32)

- Provide health check mechanism for container orchestration
- Support HTTP endpoint or signal-based health check
- Validate operational state without side effects
- Work with Docker HEALTHCHECK and Kubernetes probes
- Return appropriate exit codes for health status

### Privacy Mode with Anonymous Lists (ADR #33)

- Support privacy mode with explicit ignore lists (identifiers to never log or process)
- Distinguish between "unknown" (logged but not assigned) and "anonymous" (ignored entirely)
- Add configurable privacy toggles to disable specific data collection
- Sanitize audit logs to remove sensitive paths when privacy mode enabled
- Add `--privacy` flag to enable privacy mode for single operation
- Add privacy settings to config file

### Audit Logging with Retention (ADR #34)

Already implemented in `src/audit.rs`; enhance with:
- Configurable retention period (default 90 days)
- Automatic cleanup of old audit entries
- Audit log rotation to prevent unbounded growth
- Support for export of audit logs
- Privacy mode integration to sanitize sensitive entries

---

## Agent eXperience Interface (AXI)

Smartfo implements AXI (Agent eXperience Interface) standards to enable autonomous AI agents to efficiently interact with smartfo via shell execution.

### Mode Selection (AXI Requirement #1)

**Default Agent Mode:**
- Agent mode is the default behavior when no explicit mode selection is provided
- Auto-detection: Use agent mode when TTY is not present OR when agent session is detected
- Agent session detection: Check for environment variables like `CLAUDE_SESSION`, `CODEX_SESSION`, or presence of agent-specific process parents

**Human Mode Triggers:**
- Explicit `--human` flag forces human mode
- Explicit `--interactive` or `--tui` flag forces human mode
- Auto-detection: Use human mode when TTY is present AND no agent session is detected
- Config file setting: `mode = "agent" | "human"` in config file
- Environment variable: `SMARTFO_MODE=agent|human` (highest precedence after CLI args)

**Mode Precedence Chain:**
- CLI flags (`--human`, `--interactive`) > Environment variable (`SMARTFO_MODE`) > Config file setting > Auto-detection

### Token-Efficient Output (TOON Format) (AXI Requirement #2)

**TOON Format Implementation:**
- Add `--toon` flag to output in TOON format (Token-Oriented Object Notation)
- TOON provides ~40% token savings over equivalent JSON
- Convert to TOON at output boundary — keep internal logic in JSON
- In agent mode, default to TOON format for stdout
- In human mode, continue using JSON or human-readable formats
- Support `--format=toon|json|human` flag for explicit format selection

**TOON Format Specification:**
- Follow TOON format specification: https://toonformat.dev/reference/spec.html
- Use compact, agent-readable syntax
- Example: `operations[2]{id,type,status}: "42",move,completed "43",delete,pending`
- Implement TOON encoder/decoder for Rust

### Minimal Default Schemas (AXI Requirement #3)

**Default Output Schema Design:**
- Default list schemas: 3-4 fields (id, type, status, source), not 10+
- Default limits: high enough for common cases (e.g., 100 operations if most repos have <100)
- Long-form content (file paths, VCS messages) belongs in detail views, not lists
- Offer `--fields` flag to let agents request additional fields explicitly
- Example: `smartfo list --fields id,type,status,source,destination`

**Schema Implementation:**
- Define default output schemas for each command (list, status, install, etc.)
- Implement field selection logic
- Support comma-separated field names in `--fields` flag
- Validate field names against available fields
- Apply schema to both TOON and JSON output formats

### Content Truncation (AXI Requirement #4)

**Truncation Strategy:**
- Truncate large text fields by default (500-1500 chars)
- Never omit large fields entirely — always include a truncated preview
- Show total size so the agent knows how much it's missing
- Suggest escape hatch (`--full`) only when content is actually truncated
- Choose truncation limit that covers most use cases (configurable, default 1000 chars)

**Truncation Implementation:**
- Add `--full` flag to disable truncation and show complete content
- Implement truncation logic for all large text fields (file paths, VCS messages, error details)
- Include truncation metadata in output: `... (truncated, 8432 chars total)`
- Add help suggestions: `help[1]: Run smartfo view 42 --full to see complete details`
- Apply to both agent and human modes

### Pre-computed Aggregates (AXI Requirement #5)

**Aggregate Counts:**
- Include total count in list output, not just page size
- Format: `count: 5 of 23 total`
- Agents need "how many are there?" and will paginate if answer isn't definitive
- Compute counts efficiently at query time

**Derived Status Fields:**
- Include lightweight summary inline when next step commonly involves checking related state
- Only include derived fields backend can provide cheaply
- Example: `operations: 3/3 completed`, `queue: 7 pending`
- Provide summary, not full data
- Apply to detail views and list views where relevant

### Definitive Empty States (AXI Requirement #6)

**Empty State Formatting:**
- When answer is "nothing", say so explicitly
- State the zero with context
- Make clear command succeeded — absence of results is the answer
- Example: `operations: 0 pending operations found in queue`

**Empty State Implementation:**
- Detect empty result sets across all commands (list, status, queue)
- Format empty states consistently
- Include context (filter criteria, scope)
- Ensure exit code 0 for successful empty queries

### Structured Errors & Exit Codes (AXI Requirement #7)

**Idempotent Mutations:**
- Don't error when desired state already exists
- If agent removes a file already removed, acknowledge and move on with exit code 0
- Reserve non-zero exit codes for situations where agent's intent cannot be satisfied
- Example: `file: /path/to/file already removed (no-op) # exit 0`

**Structured Errors on Stdout:**
- Errors go to stdout in same structured format as normal output
- Include what went wrong and actionable suggestion
- Never let raw dependency output (API errors, stack traces) leak through
- Example: `error: --source is required help: smartfo mv --source <path> --destination <path>`
- Validate required flags before calling any dependency
- Translate errors — extract actionable meaning, discard noise
- Never leak dependency names — suggestions reference smartfo commands, not underlying tools

**No Interactive Prompts:**
- Every operation must be completable with flags alone
- If required value is missing, fail immediately with clear error — don't prompt
- Suppress prompts from wrapped tools in agent mode
- Human mode can retain prompts (unless `--force` is used)

**Output Channels:**
- stdout: structured output (data, errors, suggestions)
- stderr: debug logging, progress indicators, diagnostics (agents don't read this)
- Exit codes: 0 = success (including no-ops), 1 = error, 2 = usage error
- Never mix progress messages into stdout

### Ambient Context via Session Integrations (AXI Requirement #8)

**Session Hook Infrastructure:**
- Add `--session-context` command that outputs compact state in TOON format
- Output should be token-budget-aware (ruthlessly minimized)
- Include just enough for agent to orient and act; deep data belongs in explicit invocations
- Directory-scoped: show only state relevant to current working directory
- Example output: `operations[2]{id,type,status}: 42,move,completed 43,delete,pending help[1]: Run smartfo view <id> for details`

**Setup Command for Hooks:**
- Add `--install-agent-hooks` command to register session hooks
- Check existing hooks and update executable path if changed
- Idempotent: repeated installs with same path are silent no-ops
- Portable commands: use PATH-verified binary name, fall back to absolute path
- Explicit opt-in: only register from user-invoked setup command, not ordinary CLI commands
- Support hook installation for:
  - Claude Code: `~/.claude/settings.json` or project `.claude/settings.json`
  - Codex: `~/.codex/hooks.json` or project `.codex/hooks.json`
  - Future: OpenCode plugin system

**Lifecycle Capture:**
- Use session-end hooks to capture what happened (transcripts, files touched, VCS commands)
- Future session-start context gets richer over time
- Implement session-end hook registration in setup command
- Store session metadata in local cache for future context enrichment

### Installable Agent Skill (AXI Requirement #9)

**Agent Skill Support:**
- Generate `SKILL.md` from same content as no-args home view
- Add `--check-skill` build step to CI that fails if committed skill is stale
- Strip live state from skill (static only, no dynamic data like active operations)
- Rewrite command examples to non-interactive form (e.g., `smartfo mv --source <path> --destination <path>`)
- Include trigger-shaped frontmatter: `name` and `description` as trigger
- Document both paths (hook and skill) in README
- Make clear user only needs one (hook or skill)

**Skill Generation:**
- Add `--generate-skill` command to output SKILL.md content
- Template-based generation from CLI help and examples
- Include in CI/CD pipeline for automatic skill updates
- Support skill installation via agentskills.io

### Content First (AXI Requirement #10)

**No-Args Behavior:**
- Running CLI with no arguments shows most relevant live content, not usage manual
- When agent sees actual state, it can act immediately
- When it sees help text, it has to make a second call
- Example: `$ smartfo` outputs `operations[2]{id,type,status}: 42,move,completed 43,delete,pending help[2]: Run smartfo view <id> for details Run smartfo mv --source <path> --destination <path> to queue operation`

**Content-First Implementation:**
- Redesign no-args invocation to show state summary
- Move detailed help to `--help` flag (unchanged)
- Apply to both agent and human modes
- Show different content based on current directory/context

### Contextual Disclosure (AXI Requirement #11)

**Next Steps Suggestions:**
- Include few next steps that follow logically from current output
- Agent discovers CLI surface area organically by using it
- Relevant: after viewing operation → suggest executing; after empty list → suggest queuing operation; after list → suggest status
- Actionable: every suggestion is complete command carrying forward disambiguating flags
- Concise: 2-4 suggestions maximum, ranked by relevance
- Structured: use `help[]` array in TOON output for machine parsing

**Contextual Disclosure Implementation:**
- Add suggestion engine for each command
- Generate contextual help based on current state and output
- Format as structured `help[]` array in TOON
- Include in all command outputs
- Make suggestions smart (context-aware, not generic)

---

## Configuration Management

### Config File Location

Path: `$HOME/smartfo/config.toml` or `$XDG_CONFIG_HOME/smartfo/config.toml`

### Environment Variable Expansion

All string values in the config file support POSIX-style environment variable expansion (`$VAR` and `${VAR}`). The following variables are resolved when present:
- `$XDG_DATA_HOME` — defaults to `~/.local/share` if unset
- `$XDG_CACHE_HOME` — defaults to `~/.cache` if unset
- `$XDG_CONFIG_HOME` — defaults to `~/.config` if unset
- `$HOME` — user's home directory

### Precedence Hierarchy

All configuration values follow this override order (highest wins):

1. **CLI flags** (e.g., `--blocking`, `--sync`, `--backup-vcs-committed`)
2. **Environment variables** (e.g., `SMARTFO_DEFAULT_BLOCKING=true`, `SMARTFO_SYNC_ON_COMPLETE=true`)
3. **Project config** (`{REPO_ROOT}/.config/smartfo/config.toml`)
4. **User config file** (`$HOME/smartfo/config.toml`)
5. **System config** (`/etc/smartfo/config.toml` or platform-specific equivalent)
6. **Built-in defaults**

Naming convention for environment variables: `SMARTFO_<SECTION>_<KEY>` in UPPER_SNAKE_CASE.

### Config Schema

```toml
[vcs]
prefer_vcs_mv = true              # Use VCS-native move when possible
fallback_to_fs = true             # Fallback to filesystem rename if VCS move fails
vcs_list = ["git", "hg", "svn", "jj"]  # VCS systems to detect and support

[trash]
root = "~/.local/share/smartfo/trash"
preserve_tree = true
trash_mode = "always"             # "always" | "auto" | "never"
backup_vcs_committed = false      # If true, committed files still moved to trash
backup_ignored_files = false      # If true, ignored files backed up to trash
min_free_space_percent = 20       # Auto-cull when free space drops below this
allow_last_version_cull = false   # Allow culling last version when space critical
on_trash_full = "refuse"          # "refuse" | "delete"
audit_retention_days = 90         # How long to keep operation metadata

[concurrency]
max_concurrent_jobs = 8           # Global ceiling on parallel workers
network_concurrency = 2           # Limit for network-mounted destinations
auto_detect_drives = true          # Detect destination physical drives

[behavior]
smart_mv = true                   # If false, mv behaves like POSIX mv
smart_rm = true                   # If false, rm behaves like POSIX rm
mv_async_threshold_mb = 100       # Size threshold for async mv
default_blocking = false          # If true, both mv and rm block by default
sync_on_complete = false          # If true, fsync after every operation
mode = "agent"                    # "agent" | "human" - default mode selection
daemon_fallback_quiet = false     # Suppress noisy fallback on unsupported platforms

[logging]
level = "info"
log_file = "~/.local/share/smartfo/logs/current.log"

[paths]
trash_root = "~/.local/share/smartfo/trash"
audit_log = "~/.local/share/smartfo/audit/operations.jsonl"
cache_dir = "~/.cache/smartfo"
config_dir = "~/.config/smartfo"

[privacy]
enabled = false                   # Enable privacy mode
ignore_patterns = []              # Paths/patterns to never log

[output]
color = "auto"                    # "auto" | "always" | "never"
format = "auto"                   # "auto" | "toon" | "json" | "human"

[config]
version = 1                       # Config schema version for migration
```

### Config File Versioning

- Add schema version field to config files for future evolution
- Validate config schema version on load
- Reject configs with unsupported schema versions with clear error message
- Document config schema version in upgrade notes
- Support automatic migration between schema versions

---

## Safety & Correctness

### Atomic Operations

- Use `renameat2` with `RENAME_EXCHANGE` when available
- Fallback: temp-file + fsync + rename

### Crash-Safe Queue

- SQLite WAL or append-only log
- Each job: UUID, source, destination, status, retry count
- Queue survives process restarts and is idempotent

### Cross-Device Moves

- Detect via `statfs`
- Streaming copy + fsync + unlink

### Dest Already Exists

- Default refuse (`-n`)
- `-f` overwrite
- `-i` interactive prompt
- `--backup` back up existing file with suffix

### Disk Space Guard

- Auto-cull oldest trash entries when free space drops below threshold
- Refuse operation if culling is insufficient
- Configurable via `min_free_space_percent` and `on_trash_full`

---

## Testing Requirements

### Integration Tests

- Git / hg / svn / jj repos with tracked and untracked files
- All six move scenarios (in→in, in→out, out→in, out→out, same, untracked-in-repo)
- Cross-device mount tests
- Large file async mv tests (verify prompt return, queue completion)
- rm async tests (verify prompt return, trash arrival)
- Crash-recovery tests (restart mid-move, verify resume or cleanup)
- Dest-already-exists tests for all overwrite modes
- Alias detection tests for bash, zsh, and fish
- Shell completion script generation tests
- Config precedence tests (CLI > env > project > user > system > defaults)
- Daemon platform fallback behavior tests
- Health check endpoint tests
- Privacy mode sanitization tests

### Property Tests

- No data loss under any path
- Directory trees preserved in trash
- VCS state consistent after move
- Same-file deletion history preserved across multiple deletes
- Disk space guard correctly culls oldest entries first
- Audit log contains valid metadata for every operation
- Git hooks correctly detect and block raw deletions and raw renames
- `--install` correctly creates symlinks and hooks without overwriting existing files
- `--install --hooks client` installs only client-side hook; `--install --hooks server` installs only server-side
- `--force-delete` bypasses trash regardless of `trash_mode` or disk space
- `trash_mode = "never"` performs direct delete without trash
- `trash_mode = "auto"` falls back to direct delete when trash is full
- Server-side hook correctly reads from `{REPO_ROOT}/.smartfo/audit/operations.jsonl`
- Agent mode auto-detection works correctly
- TOON format achieves ~40% token savings over JSON
- Session hooks install correctly and provide context
- Content truncation preserves metadata
- Empty states are formatted correctly
- Idempotent operations return exit code 0

### CLI Standards Tests

- Help output for all modes
- Globbing patterns
- Stdin input handling
- JSON vs human output modes
- Exit-code behavior for all error paths
- Standard arguments (--help, --version, --usage)
- Config file initialization
- Shell completion script generation
- Error handling and formatting
- Daemon mode where feasible
- --list-jobs with optional job ID filtering
- Signal handling (SIGINT, SIGHUP)
- Color control (--color=auto|always|never)
- Logging modes (--verbose, --quiet, --debug)
- Dry-run mode
- Confirmation prompts
- Progress indicators
- TUI mode
- Pager integration
- Terminal size awareness
- Cross-platform path handling
- Resource limits

---

## Non-Functional Requirements

### Performance

- All new features (TUI, completion, man pages) must not impact startup time significantly (<100ms additional overhead)
- TOON encoding/decoding must add minimal overhead (<10ms)
- TOON output must achieve ~40% token savings over JSON

### Cross-Platform

- Must work on Linux, macOS, and Windows
- Ensure consistent path handling across platforms
- Test on all three platforms

### Security

- No secrets in logs
- Secure config file handling
- Proper file permissions
- Sanitize audit logs in privacy mode
- No logging of sensitive data

### Reliability

- Daemon must survive restarts
- Config validation must be robust
- Crash-safe queue with idempotent operations
- Graceful shutdown on signals

### Usability

- TUI must be intuitive
- Error messages must be actionable
- Help must be comprehensive
- Agent mode works seamlessly with auto-detection
- Human mode retains all existing functionality

### Maintainability

- Code must follow existing patterns
- Tests must cover new functionality
- Code must follow Rust best practices
- All new features must be documented

### Documentation

- All new features must be documented in man pages and help output
- Clear documentation on mode selection, TOON format, and session integration
- Agent skill generation works correctly
- Install alias warnings documented

---

## Build, Lint, and Test

```bash
# Build
devbox run cargo build

# Run tests
devbox run cargo test

# Run with all features
devbox run cargo test --all-features

# Release build
devbox run cargo build --release

# Lint
devbox run cargo clippy -- -D warnings

# Format
devbox run cargo fmt
```

---

## Developer Ergonomics

### Drop-in Replacement

- Symlink `mv` and `rm` to `smartfo` so AI assistants, scripts, and build tools use it automatically
- Optional explicit `smv`/`srm` entrypoints for debugging

### Machine-Friendly Output

- `--json` for structured operation logs
- `--dry-run` to preview operations without executing
- `--reason "..."` to annotate intent in the audit log
- `--toon` for token-efficient agent output
- `--session-context` for ambient agent context

### Explicit Debug Entrypoints

- `smv` and `srm` are optional symlink names for debugging smartfo behavior without overriding system `mv`/`rm`

---

## Proposed Future Enhancements

The following features were identified as gaps in the requirements and are proposed for future implementation:

### Priority 1 (Critical Features)

#### Trash Restoration/Undelete
- `smartfo restore <path>` - Restore file from trash to original location
- `smartfo restore --id <uuid>` - Restore by operation UUID
- `smartfo restore --to <dest>` - Restore to different location
- Conflict resolution when destination already exists

#### Trash Browsing & Inspection
- `smartfo trash list` - List trash entries with metadata
- `smartfo trash list --pattern <glob>` - Filter by path pattern
- `smartfo trash list --age <duration>` - Filter by deletion age
- `smartfo trash view <id>` - View detailed trash entry info
- `smartfo trash info` - Show trash statistics (size, count, disk usage)

#### Manual Trash Management
- `smartfo trash empty` - Empty all trash
- `smartfo trash empty --older-than <duration>` - Empty old entries
- `smartfo trash empty --force` - Bypass confirmation
- `smartfo trash prune` - Run manual culling based on disk space

#### Audit Log Querying & Analysis
- `smartfo audit list` - List recent operations
- `smartfo audit list --filter <criteria>` - Filter by type, path, date
- `smartfo audit view <uuid>` - View detailed operation metadata
- `smartfo audit stats` - Show operation statistics
- `smartfo audit export --format json|csv` - Export audit log

#### Operation History & Undo
- `smartfo history` - Show recent operations
- `smartfo undo <uuid>` - Reverse an operation (move back from trash, reverse mv)
- `smartfo undo --last` - Undo most recent operation
- `smartfo redo <uuid>` - Re-apply a previously undone operation

### Priority 2 (Important Enhancements)

#### VCS Branch Awareness
- Detect current branch and include in audit metadata
- Branch-specific trash organization (optional)
- Warn when moving files across branches

#### Performance Monitoring
- `smartfo status` - Show daemon status, queue size, active jobs
- `smartfo stats` - Show performance metrics (operations/sec, avg duration)
- `smartfo queue list` - Show pending/running jobs
- `smartfo queue cancel <id>` - Cancel specific job

#### Network File System Support
- Detect NFS mounts and apply appropriate concurrency limits
- Handle NFS-specific error conditions (stale file handles, permission issues)
- Configurable NFS-specific timeouts and retry behavior

#### Batch Operations
- `smartfo batch` - Interactive batch mode for multiple operations
- `smartfo batch --file <manifest>` - Execute operations from manifest file
- Batch confirmation with summary before execution

#### Scheduled Cleanup
- `smartfo schedule cleanup` - Set up periodic trash cleanup
- Integration with cron/systemd timers
- Configurable cleanup schedules (daily, weekly)

### Priority 3 (Nice-to-Have Features)

#### Trash Deduplication
- If same file deleted multiple times, store only one copy with metadata

#### Compression
- Optional compression for large files in trash to save space

#### Trash Sync
- Sync trash across multiple machines (with conflict resolution)

#### System Trash Integration
- Option to use system trash (macOS Trash, GNOME Files trash) instead of custom trash

#### Snapshot/Backup Integration
- Integration with backup systems (restic, borg, Time Machine) for critical deletions

---

## Open Questions

- Should TUI mode be the default when no arguments provided, or require explicit flag?
- What should be the default retention period for audit logs? (Current: 90 days)
- Should health check use HTTP endpoint or signal-based approach?
- Should privacy mode be enabled by default for sensitive paths?
- What is the priority order for implementing the 35 CLI standards? (Can be phased)
- Should TOON format be the default in agent mode, or require explicit `--toon` flag?
- What specific session metadata should be captured for lifecycle enrichment?
- Should skill generation be automated in CI or manual?
- How should truncation limits be configured (global, per-field, per-command)?
- Should proposed future enhancements be prioritized over completing CLI standards compliance?

---

## Dependencies

- CLI Tool Standards ADR (adr-20260607001 v4.0.0) - must align with latest version
- AXI specification: https://github.com/kunchenguid/axi/blob/main/.agents/skills/axi/SKILL.md
- TOON format specification: https://toonformat.dev/reference/spec.html
- Existing smartfo requirements and implementation
- Rust ecosystem crates (ratatui, indicatif, pager, etc.)
- Container orchestration platforms (Docker, Kubernetes) for health check testing
- Shell environments (bash, zsh, fish) for completion testing
- Cross-platform test environments (Linux, macOS, Windows)

---

## Timeline / Milestones

### Phase 1 (Week 1-2): Core CLI Standards
- Developer UX standard, standard arguments, config initialization, install/uninstall enhancement, input/globbing, output discipline

### Phase 2 (Week 3-4): Logging & Signals
- Logging modes, signals/exit codes, dry-run, confirmation prompts, progress indicators

### Phase 3 (Week 5-6): Daemon & Error Handling
- Daemon enhancements, error formatting, file/URL formatting, shell completion, man pages

### Phase 4 (Week 7-8): UX Enhancements
- Pager integration, subcommand organization, config validation, terminal size awareness, env var naming

### Phase 5 (Week 9-10): Cross-Platform & Security
- Cross-platform paths, credential handling, resource limits, testing infrastructure

### Phase 6 (Week 11-12): Config & Logging
- Collection/processing separation, config versioning, structured logging auto-detection, SIGHUP reload

### Phase 7 (Week 13-14): Advanced Features
- Health checks, privacy mode, audit logging enhancements, TUI mode

### Phase 8 (Week 15-16): Agent Mode
- Mode selection, TOON format, minimal schemas, content truncation, pre-computed aggregates, empty states, structured errors, session hooks, agent skill support, content-first no-args, contextual disclosure

### Phase 9 (Week 17-18): Integration & Release
- Integration testing, documentation, cross-platform testing, release preparation

---

*Consolidated from incremental requirements:*
- 20260604-smartfo-initial-reqs.md
- prd-cli-standards-compliance.md
- prd-20260608-cli-axi.md
- 20260609-install-warn-aliases.md
- 20260610-requirements-gaps.md
- 2026066611-feature-gaps.md
