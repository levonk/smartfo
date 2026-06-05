F# Smart File Operations (`smartfo`) — Requirements

A Rust CLI tool that transparently replaces `mv` and `rm` with VCS-aware, safe, non-blocking operations. One binary (`smartfo`) is installed via symlinks; it dispatches on `argv[0]`.

---

# **1. Entrypoints & Dispatch**

Install symlinks:
```
~/.local/bin/mv  -> ~/.local/libexec/smartfo
~/.local/bin/rm  -> ~/.local/libexec/smartfo
~/.local/bin/smv -> ~/.local/libexec/smartfo  (optional debug entry)
~/.local/bin/srm -> ~/.local/libexec/smartfo  (optional debug entry)
```

Dispatch logic inside `smartfo`:
- `argv[0]` ends with `mv` or `smv` → run **mv mode**
- `argv[0]` ends with `rm` or `srm` → run **rm mode**
- `argv[0]` is `smartfo` with `--install` → run **install mode**
- Each mode uses its own POSIX-compatible flag parser (no unified parser).

## **1.1 Install Mode (`smartfo --install`)**
One-shot setup command that configures the environment:

### **Symlink creation**
Creates `mv`, `rm`, `smv`, `srm` symlinks pointing to the `smartfo` binary.
- If `$XDG_BIN_HOME` is defined and is on PATH: symlinks go there (user install).
- If `$XDG_BIN_HOME` is not defined but `~/.local/bin` exists and is on PATH: symlinks go there (user install).
- If neither exists: create `~/.local/bin`, ensure it is on PATH (warn if not), and place symlinks there.
- If running as **root** and `/usr/local/bin` is on PATH: symlinks go there (system install).
- If target files already exist and are not symlinks to `smartfo`: refuse with error unless `--force` is passed.

### **Git hook installation** (only if invoked inside a Git repo)
The `smartfo` binary provides two hook subcommands:
- `smartfo-git-hook-client` — client-side hook (used as `pre-commit`)
- `smartfo-git-hook-server` — server-side hook (used as `pre-receive`)

- **Client-side hook (`pre-commit`)**: Verifies that staged deletions and renames have matching `smartfo` metadata in the audit log.
  - Blocks raw `rm` deletions (no metadata).
  - Blocks raw `mv` renames that bypass `smartfo` (detected when a file is removed and another added with similar content, but no `smartfo` move metadata exists).
- **Server-side hook (`pre-receive`)**: Scans incoming push for deleted files and renames, cross-referencing the repo-local audit log (`{REPO_ROOT}/.smartfo/audit/operations.jsonl`). Blocks pushes containing raw deletions or raw renames.

**Install flags:**
- `--hooks client` — install only client-side hook
- `--hooks server` — install only server-side hook
- `--hooks client,server` — install both (default)
- `--no-hooks` — skip hook installation entirely

If a hook file already exists and is not a symlink to `smartfo`: refuse with error unless `--force` is passed.

---

# **2. mv Mode — VCS-Aware Move**

## **2.1 VCS Detection**
Detect whether source and/or destination are inside a VCS working tree. Supported VCS: **Git, Mercurial (hg), SVN, Jujutsu (jj), and any future VCS exposing a comparable `mv` command or library API.**

- Discover repo root via VCS-specific commands or libraries (e.g., `git rev-parse --show-toplevel`, `hg root`, `svn info`, `jj root`).
- Detect whether the source path is tracked by that VCS.

## **2.2 Move Scenarios**
The tool must correctly handle all combinations of source and destination:

| Scenario | Behavior |
|----------|----------|
| **src tracked, dest in same repo** | Execute VCS-native move (e.g., `git mv`, `hg mv`). |
| **src tracked, dest outside repo** | Refuse by default; require `--force-outside-vcs`. If forced: VCS `rm` the source, then filesystem move to dest. With `--no-vcs-rm`: perform only the filesystem move (or copy), leaving the source tracked in VCS. Useful for temporary backups or working copies. |
| **src outside repo, dest inside repo** | Filesystem move to dest. |
| **src and dest both outside any repo** | Pure filesystem `rename`. |
| **src and dest both in repo, neither tracked** | Pure filesystem `rename`. |
| **src == dest** | No-op with exit code 0 (match POSIX `mv`). |
| **dest already exists** | Default: refuse (`-n` semantics). With `-f`: overwrite. With `-i`: prompt. With `--backup`: back up existing dest before overwrite. |

## **2.3 POSIX Compatibility**
- Accept standard flags: `-f`, `-n`, `-i`, `-v`, `-T`, `-t`, `--backup`, `--strip-trailing-slashes`, etc.
- `--plain` — disable all smart features for this invocation; behave exactly like standard POSIX `mv` (no VCS detection, no async, no safety guards).
- Preserve exit codes and stderr formatting so scripts do not break.
- Behave identically to GNU `mv` when no VCS is involved.

---

# **3. rm Mode — Trash Instead of Delete**

## **3.1 Trash Directory Mirroring**
Config example:
```toml
[trash]
root = "$XDG_DATA_HOME/smartfo/trash"
preserve_tree = true
backup_vcs_committed = false
backup_ignored_files = false
```

- When user runs `rm foo/bar/baz.txt`, compute a versioned destination:
  ```
  $TRASH_ROOT/<absolute-path-from-root>/foo/bar/baz.txt/<iso-timestamp>-<counter>
  ```
  Example: `~/.local/share/smartfo/trash/home/user/src/foo/bar/baz.txt/2026-06-04T09:15:30Z-001`
- Create parent directories as needed.
- Move the file (never unlink).
- **Same-file history**: Deleting the same source path multiple times preserves every version in timestamped subdirectories. A `.smartfo-index` JSONL file in the trash entry records the full history: original path, deletion timestamp, operation UUID, and reason (if provided via `--reason "refactor: replaced with new parser"`).
- **Default trash root**: `$XDG_DATA_HOME/smartfo/trash` (typically `~/.local/share/smartfo/trash`). This is user data, not cache — it must survive cache clears.
- **VCS-committed files**: If `backup_vcs_committed = false` (default), the behavior depends on whether the file has uncommitted changes:
  - **Clean** (no modifications since last commit): Do not move to trash. Perform a VCS-aware remove (e.g., `git rm`) since the committed version is recoverable from VCS history.
  - **Dirty** (modified since last commit): Move to trash **and** perform VCS-aware remove (`git rm`). The uncommitted changes are not in VCS history, so they must be preserved in trash.
- **Ignored files**: If `backup_ignored_files = false` (default), files matched by `.gitignore`/`.hgignore`/etc. are deleted directly without moving to trash. These are typically build artifacts, dependencies, or temp files that are reproducible and do not need backup.
- **Uncommitted, non-ignored files**: Behavior depends on `trash_mode`:
  - `trash_mode = "always"` (default): Always move to trash.
  - `trash_mode = "auto"`: Move to trash if space allows; if trash is full or disabled, fall back to direct delete with a warning.
  - `trash_mode = "never"`: Never use trash; perform VCS-aware delete (`git rm`) if tracked, or direct delete if untracked.

## **3.1.1 Disk Space Guard**
Before every trash move, check available disk space on the trash filesystem:
- If free space drops below `min_free_space_percent` (default: **20%**), auto-cull trash history starting with oldest entries until free space is above the threshold.
- If culling cannot free enough space, the default behavior is to **refuse the operation** with a clear error:
  ```
  smartfo: trash disk space critically low (8% free). Use --force-delete to bypass trash, or free space.
  ```
- **Override behavior** via `on_trash_full` config:
  - `on_trash_full = "refuse"` (default): Refuse the operation; user must free space or pass `--force-delete`.
  - `on_trash_full = "delete"`: Bypass trash and perform direct delete (VCS-aware if tracked) with a warning.
- Culling policy: remove oldest versions first; never remove the only remaining version of a file unless `allow_last_version_cull = true` in config.
- **CLI override**: `--force-delete` bypasses trash for this single invocation, regardless of `trash_mode` or disk space.

## **3.2 Asynchronous by Default**
**All rm operations are asynchronous by default.** The user must return to the CLI prompt immediately, even for large files or cross-device moves.

- Immediately enqueue the move job and print:
  ```
  smartfo: moving to trash in the background (use --blocking to wait)
  ```
- A background worker performs the actual filesystem move.
- The `--blocking` flag forces the operation to block until completion when the user needs confirmation before proceeding.
- The `--sync` flag forces an `fsync` on the destination file and containing directory before the job is marked done (configurable default via `sync_on_complete` in config).

## **3.3 Background Worker (Daemon Model)**
To return the shell prompt immediately, the operation cannot run inside the CLI process. The CLI process enqueues the job and exits; a separate daemon performs the work.

- **Daemon lifecycle**: The daemon is **self-spawning** — no systemd, init.d, LaunchAgent, or other OS service manager is required. On the first async operation, the CLI binary double-forks to detach a background daemon process, then exits. The daemon writes its PID to a lockfile and listens on a Unix domain socket. Subsequent CLI invocations connect to the existing daemon via the socket; if the daemon has died, a new one is spawned automatically.
- **Job queue**: Durable store (SQLite WAL or append-only log). Each job: UUID, source path, dest path, status (queued/running/done/failed), retry count.
- **Worker performs**:
  - Atomic rename if same filesystem.
  - Chunked copy + fsync + unlink if cross-device.
- **Worker logs failures** and retries with exponential backoff.
- **Queue survives process restarts** and is idempotent.
- **Shutdown handling**: Graceful shutdown on `SIGTERM`; in-flight jobs are allowed to complete before exit.

## **3.4 Daemon Concurrency**
When multiple files are removed or moved in one invocation (e.g., `rm file1 file2 file3`):
- Each file becomes an independent job in the queue.
- The daemon adapts parallelism based on system conditions:
  - **Same filesystem, same directory**: Serialized to avoid lock contention.
  - **Same filesystem, different directories**: Parallel up to `cpu_cores` workers.
  - **Cross-device (different physical drives)**: Parallel up to `destination_drive_count` workers.
  - **Network-mounted destinations**: Limited to `network_concurrency` (default 2) to avoid saturating the link.
- All limits are capped by a global `max_concurrent_jobs` ceiling.
- If `--blocking` is used, the CLI waits until all enqueued jobs for that invocation reach `done` status.

## **3.5 POSIX Compatibility**
- Accept standard flags: `-f`, `-i`, `-I`, `-r`, `-R`, `-d`, `--preserve-root`, `--one-file-system`, etc.
- `--plain` — disable all smart features for this invocation; behave exactly like standard POSIX `rm` (no trash, no VCS awareness, direct deletion).
- Preserve exit codes and stderr formatting.

---

# **3.6 Operation Metadata & Audit Trail**
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

- **Storage**: Appended to `$XDG_DATA_HOME/smartfo/audit/operations.jsonl` (one JSON object per line). **All paths overridable** via `[paths]` config, environment variables (`SMARTFO_PATHS_AUDIT_LOG`), or CLI flags (`--audit-log-path`).
- **Server-side access**: For server-side hooks, the audit log must be accessible to the Git server. The server-side hook reads from `{REPO_ROOT}/.smartfo/audit/operations.jsonl` (a repo-local copy of the audit log, updated on push or via a post-receive sync).
- **Retention**: Audit log is pruned by age (configurable, default 90 days) independently of trash contents.
- **Git hook verification**: The `pre-commit` hook reads the audit log for the current repo and verifies every deletion and rename has a matching `smartfo` metadata entry.
  - Raw `rm` deletions (no metadata) cause the hook to fail with:
    ```
    smartfo hook: detected raw deletion of src/utils/old_parser.rs. Use 'srm' or 'rm' (smartfo) instead.
    ```
  - Raw `mv` renames (no metadata) cause the hook to fail with:
    ```
    smartfo hook: detected raw rename of src/utils/old_parser.rs -> src/utils/new_parser.rs. Use 'smv' or 'mv' (smartfo) instead.
    ```
- **Server-side hook (`pre-receive`)**: Rejects pushes containing raw deletions or raw renames by scanning the commit tree for removed files and renames, cross-referencing the repo-local audit log (`{REPO_ROOT}/.smartfo/audit/operations.jsonl`).
- **Reason flag**: `--reason "..."` allows the user to annotate the intent of an operation (e.g., `--reason "cleanup: remove dead code"`).

---

# **4. Async Behavior for mv Mode**

## **4.1 When Async Kicks In**
`mv` is **synchronous by default** because same-filesystem `rename()` is atomic and near-instant; making every move async would add daemon overhead with no benefit and could break scripts expecting the destination path to exist immediately. Async is triggered only when the operation is predictably expensive:

- Cross-device move detected via `statfs`.
- Source file size exceeds configurable threshold (default: 100MB).
- Explicit `--async` flag passed.

## **4.2 User Experience**
When an async mv job is enqueued:
- Print to stderr:
  ```
  smartfo: large file move queued in the background (use --blocking to wait)
  ```
- Return exit code 0 immediately.
- The background worker completes the move and logs the result.

## **4.3 Synchronous Override**
- `--blocking` flag forces the operation to wait until completion for both `mv` and `rm`.
- Config override: `[behavior].default_blocking = true` (for users who prefer blocking by default).

---

# **5. Configuration**

Path: `$HOME/smartfo/config.toml`

## **5.1 Environment Variable Expansion**
All string values in the config file support POSIX-style environment variable expansion (`$VAR` and `${VAR}`). The following variables are resolved when present:
- `$XDG_DATA_HOME` — defaults to `~/.local/share` if unset
- `$XDG_CACHE_HOME` — defaults to `~/.cache` if unset
- `$XDG_CONFIG_HOME` — defaults to `~/.config` if unset
- `$HOME` — user's home directory

## **5.2 Precedence Hierarchy**
All configuration values follow this override order (highest wins):

1. **CLI flags** (e.g., `--blocking`, `--sync`, `--backup-vcs-committed`)
2. **Environment variables** (e.g., `SMARTFO_DEFAULT_BLOCKING=true`, `SMARTFO_SYNC_ON_COMPLETE=true`)
4. **User config file** (`$HOME/smartfo/config.toml`)
5. **Built-in defaults**

Naming convention for environment variables: `SMARTFO_<SECTION>_<KEY>` in UPPER_SNAKE_CASE.
Examples:
- `SMARTFO_BEHAVIOR_DEFAULT_BLOCKING=true`
- `SMARTFO_TRASH_ROOT=/mnt/bigdisk/trash`
- `SMARTFO_CONCURRENCY_MAX_CONCURRENT_JOBS=16`

### `[vcs]`
- `prefer_vcs_mv = true` — use VCS-native move when possible. . default true
- `fallback_to_fs = true` — fallback to filesystem rename if VCS move fails. default true
- `vcs_list = ["git", "hg", "svn", "jj"]` — VCS systems to detect and support. default all

### `[trash]`
- `root = "~/.local/share/smartfo/trash"` — trash root; default follows `$XDG_DATA_HOME`
- `preserve_tree = true`
- `trash_mode = "always"` — `"always"` = always move to trash; `"auto"` = trash if space allows, else warn and delete; `"never"` = never use trash
- `backup_vcs_committed = false` — if true, committed files are still moved to trash; if false, VCS-aware delete is used instead
- `backup_ignored_files = false` — if true, files matched by `.gitignore`/`.hgignore`/etc. are backed up to trash; if false, they are deleted directly without trash
- `min_free_space_percent = 20` — auto-cull oldest trash entries when free space drops below this percentage
- `allow_last_version_cull = false` — if true, allow culling the last remaining version of a file when space is critical
- `on_trash_full = "refuse"` — `"refuse"` = block operation when trash is full; `"delete"` = bypass trash and delete directly
- `audit_retention_days = 90` — how long to keep operation metadata in the audit log

### `[concurrency]`
- `max_concurrent_jobs = 8` — global ceiling on parallel workers
- `network_concurrency = 2` — limit for network-mounted destinations
- `auto_detect_drives = true` — if true, detect destination physical drives and limit parallelism accordingly

### `[behavior]`
- `smart_mv = true` — if false, `mv` mode behaves exactly like standard POSIX `mv` (no VCS detection, no async, no safety features). The `--plain` CLI flag overrides this to `false` for a single invocation.
- `smart_rm = true` — if false, `rm` mode behaves exactly like standard POSIX `rm` (no trash, no VCS awareness, direct deletion). The `--plain` CLI flag overrides this to `false` for a single invocation.
- `mv_async_threshold_mb = 100` — size threshold for async mv
- `default_blocking = false` — if true, both mv and rm block by default; async requires `--async`
- `sync_on_complete = false` — if true, fsync destination file and directory after every operation (equivalent to always passing `--sync`)

### `[logging]`
- `level = "info"`
- `log_file = "~/.local/share/smartfo/logs/current.log"`

### `[paths]`
- `trash_root = "~/.local/share/smartfo/trash"` — defaults to `$XDG_DATA_HOME/smartfo/trash`
- `audit_log = "~/.local/share/smartfo/audit/operations.jsonl"` — defaults to `$XDG_DATA_HOME/smartfo/audit/operations.jsonl`
- `cache_dir = "~/.cache/smartfo"` — defaults to `$XDG_CACHE_HOME/smartfo`
- `config_dir = "~/.config/smartfo"` — defaults to `$XDG_CONFIG_HOME/smartfo`

---

# **6. Architecture**

## **Internal crates/modules**
- `vcs.rs` — VCS detection (git, hg, svn, jj, extensible) + tracked-file logic
- `mv.rs` — POSIX-compatible move logic + scenario routing
- `rm.rs` — trash enqueueing + flag handling
- `trash.rs` — async trash mover / background worker
- `config.rs` — TOML loader + schema
- `queue.rs` — durable job queue (SQLite WAL)
- `logging.rs` — structured logs

## **Daemon / job queue**
- Embedded in the binary; starts on first async operation if not already running.
- Survives restarts via persistent queue store.
- Handles partial moves safely (resume or cleanup on restart).

---

# **7. Safety & Correctness**

## **Atomic operations**
- Use `renameat2` with `RENAME_EXCHANGE` when available.
- Fallback: temp-file + fsync + rename.

## **Crash-safe queue**
- SQLite WAL or append-only log.
- Each job: UUID, source, destination, status, retry count.

## **Cross-device moves**
- Detect via `statfs`.
- Streaming copy + fsync + unlink.

## **Dest already exists**
- Default refuse (`-n`).
- `-f` overwrite.
- `-i` interactive prompt.
- `--backup` back up existing file with suffix.

---

# **8. Developer Ergonomics**

## **Drop-in replacement**
- Symlink `mv` and `rm` to `smartfo` so AI assistants, scripts, and build tools use it automatically.
- Optional explicit `smv`/`srm` entrypoints for debugging.

## **Machine-friendly output**
- `--json` for structured logs.
- `--dry-run` for debugging.

---

# **9. Testing Requirements**

## **Integration tests**
- Git / hg / svn / jj repos with tracked and untracked files.
- All six move scenarios (in→in, in→out, out→in, out→out, same, untracked-in-repo).
- Cross-device mount tests.
- Large file async mv tests (verify prompt return, queue completion).
- rm async tests (verify prompt return, trash arrival).
- Crash-recovery tests (restart mid-move, verify resume or cleanup).
- Dest-already-exists tests for all overwrite modes.

## **Property tests**
- No data loss.
- Directory trees preserved in trash.
- VCS state consistent after move.
- Same-file deletion history preserved across multiple deletes.
- Disk space guard correctly culls oldest entries first.
- Audit log contains valid metadata for every operation.
- Git hooks correctly detect and block raw deletions and raw renames.
- `--install` correctly creates symlinks and hooks without overwriting existing files.
- `--install --hooks client` installs only client-side hook; `--install --hooks server` installs only server-side.
- `--force-delete` bypasses trash regardless of `trash_mode` or disk space.
- `trash_mode = "never"` performs direct delete without trash.
- `trash_mode = "auto"` falls back to direct delete when trash is full.
- Server-side hook correctly reads from `{REPO_ROOT}/.smartfo/audit/operations.jsonl`.

---

# **10. Advanced Features**

## **Versioned trash**
- Timestamped subdirectories to keep multiple versions of deleted files.

## **Auto-prune**
- Configurable retention policy (age or size-based).

## **Future: FUSE mount for trash browsing**
- Expose trash as a virtual filesystem.

---
