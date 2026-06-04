Here’s the clean, technical, Rust‑centric answer you actually want — **no Python, no fluff, no alias hacks** — just the **requirements for a proper Rust tool** that:

- Transparently replaces `mv` with “`git mv` if tracked, else real mv”
- Transparently replaces `rm` with “move to a trash tree that mirrors the source path”
- Runs heavy operations asynchronously so the user never blocks
- Is configurable via `$HOME/<toolname>/config.toml`
- Is robust enough that you can force AI assistants to use it without re‑explaining anything

I’ll call the tool **`gitwise`** in this spec, but you can rename it.

---

# **Concise takeaway**
A solid Rust implementation needs:  
**(1)** a Git‑aware file‑operation engine,  
**(2)** a drop‑in `mv`/`rm` wrapper with POSIX‑compatible semantics,  
**(3)** async background workers for trash moves,  
**(4)** a TOML config defining behavior,  
**(5)** a daemon or job queue for non‑blocking deletes,  
**(6)** safety guarantees (atomic ops, fsync, journaling),  
**(7)** a CLI that exactly mirrors coreutils flags.

Below is the full engineering spec.

---

# **📦 1. Core functional requirements**

## **Git‑aware move engine**
- Detect whether the source path is inside a Git repo (`git rev-parse --show-toplevel` or libgit2).
- Detect whether the file is tracked (`git ls-files --error-unmatch` or libgit2 index lookup).
- If tracked **and** source/destination are in the same repo:
  - Execute `git mv` (via libgit2 or shell).
- Otherwise:
  - Execute a real filesystem rename (`std::fs::rename`).

## **POSIX‑compatible mv wrapper**
- Must accept all common flags: `-f`, `-n`, `-v`, `-T`, `-t`, `--backup`, etc.
- Must behave identically to GNU `mv` when Git is not involved.
- Must preserve exit codes and stderr formatting so scripts don’t break.

---

# **🗑️ 2. rm replacement with trash‑tree semantics**

## **Trash directory mirroring**
Config example:

```toml
[trash]
root = "/home/leo/.local/share/gitwise/trash"
preserve_tree = true
async = true
```

Behavior:
- When user runs `rm foo/bar/baz.txt`, the tool computes:

```
$TRASH_ROOT/<absolute-path-from-root>/foo/bar/baz.txt
```

- Creates parent directories as needed.
- Moves the file instead of deleting it.

## **Asynchronous background mover**
Large files must not block the user.

Requirements:
- Use a job queue (SQLite, sled, or simple append‑only log).
- Spawn a background worker (tokio task or separate daemon).
- `rm` returns immediately after enqueueing.
- Worker performs:
  - Atomic rename if same filesystem.
  - Chunked copy + fsync + unlink if cross‑device.
- Worker logs failures and retries.

---

# **⚙️ 3. Configuration system**

## **Config file loader**
- Path: `$HOME/gitwise/config.toml`
- Use `dirs` crate to resolve `$HOME`.
- Use `toml` or `toml_edit` crate.
- Config sections:

### `[git]`
- `prefer_git_mv = true`
- `fallback_to_mv = true`
- `use_libgit2 = true`

### `[trash]`
- `root = "/path"`
- `async = true`
- `preserve_tree = true`
- `max_concurrent_jobs = 4`

### `[logging]`
- `level = "info"`
- `log_file = "~/.local/share/gitwise/logs/current.log"`

---

# **🧵 4. Architecture**

## **CLI frontend**
- Subcommands:
  - `mv` (default)
  - `rm`
  - `daemon` (background worker)
  - `doctor` (diagnostics)
- Or: symlink `gitwise` → `mv`, `rm` so the tool auto‑detects mode by argv[0].

## **Core library crate**
- `git.rs` — Git detection + tracked‑file logic
- `mv.rs` — POSIX‑compatible move logic
- `trash.rs` — async trash mover
- `config.rs` — TOML loader + schema
- `daemon.rs` — job queue + worker
- `logging.rs` — structured logs

## **Daemon / job queue**
- Must survive restarts.
- Must be idempotent.
- Must handle partial moves safely.

---

# **🛡️ 5. Safety & correctness requirements**

## **Atomic operations**
- Use `renameat2` with `RENAME_EXCHANGE` when available.
- Fallback to temp‑file + fsync + rename.

## **Crash‑safe queue**
- Append‑only log or SQLite WAL mode.
- Each job has:
  - UUID
  - Source path
  - Destination path
  - Status: queued, running, done, failed
  - Retry count

## **Cross‑device moves**
- Detect via `statfs`.
- Use streaming copy + fsync + unlink.

---

# **🧰 6. Developer ergonomics**

## **Drop‑in replacement mode**
- Install via symlinks:
  ```
  ln -s /usr/local/bin/gitwise ~/.local/bin/mv
  ln -s /usr/local/bin/gitwise ~/.local/bin/rm
  ```
- Tool inspects `argv[0]` to decide behavior.

## **Machine‑friendly output**
- `--json` flag for structured logs.
- `--dry-run` for debugging.

---

# **🧪 7. Testing requirements**

## **Integration tests**
- Git repo with tracked/untracked files.
- Cross‑device mount tests.
- Large file async tests.
- Crash‑recovery tests.

## **Property tests**
- Ensure no data loss.
- Ensure directory trees preserved.

---

# **🧭 8. Optional advanced features**

## **Versioned trash**
- Keep multiple versions of deleted files.
- Use timestamped directories.

## **Auto‑prune**
- Configurable retention policy.

## **FUSE mount for trash browsing**
- Expose trash as a virtual filesystem.

---

---

# **Bottom line**
- **`smv`** and **`srm`** are excellent names. Short, memorable, POSIX‑ish, and intuitive.  
- You *can* implement them as **one binary with two personalities** (argv[0] dispatch).  
- But: **mv and rm have different flag sets**, and normalizing them into one parser *will* create edge‑case ambiguity unless you’re careful.

So the best architecture is:

> **One binary → two symlinks → two distinct CLI modes → shared internal library.**

This gives you the simplicity of one codebase without the UX hazards of a merged CLI.

---

# **1. Are the names `smv` and `srm` good?**
Yes — they’re ideal.

### Strengths
- **Short** (3 letters, like coreutils)
- **Semantic** (“smart mv”, “smart rm”)
- **Tab‑completion friendly**
- **No namespace collisions** with common tools
- **Easy to force AI assistants to use** (clear, unambiguous names)

### No meaningful downsides
- They don’t collide with existing Linux tools.
- They don’t violate POSIX naming conventions.
- They’re easy to explain in documentation.

---

# **2. Should they be separate tools or the same tool?**

## **Best practice: One binary, two entrypoints**
This is how `busybox`, `ripgrep` (`rg`), `fd`, and many others do it.

### Structure:
```
/usr/local/bin/smv -> /usr/local/libexec/smartfs
/usr/local/bin/srm -> /usr/local/libexec/smartfs
```

The binary inspects `argv[0]`:

- If invoked as `smv` → run the mv logic  
- If invoked as `srm` → run the rm logic  

### Why this is the best approach
- **Shared config** (`$HOME/smartfs/config.toml`)
- **Shared async job queue**
- **Shared logging**
- **Shared Git detection**
- **Shared trash engine**

But:
- **Separate flag parsers**  
- **Separate help text**  
- **Separate semantics**  

This avoids the CLI ambiguity problem.

---

# **3. Why not merge them into one CLI with subcommands?**

You *could* do:

```
smartfs mv …
smartfs rm …
```

But this has drawbacks:

- AI assistants will default to `mv` and `rm`, not `smartfs mv`.
- You lose the drop‑in replacement semantics.
- You break scripts that expect `mv`/`rm`‑like behavior.
- Users will forget the subcommand.

So: **don’t do this** unless you want a non‑POSIX workflow.

---

# **4. Do you need to worry about CLI argument conflicts?**
Yes — and this is the main reason to keep the modes separate.

### `mv` flags:
- `-f`, `-n`, `-v`, `-T`, `-t`, `--backup`, `--strip-trailing-slashes`

### `rm` flags:
- `-f`, `-i`, `-I`, `-r`, `-R`, `-d`, `--preserve-root`, `--one-file-system`

### Conflicts:
- `-r` means nothing for `mv` but is critical for `rm`
- `-T` exists for `mv` but not for `rm`
- `--preserve-root` is rm‑only
- `--backup` is mv‑only

If you merge them into one parser, you get:

- Ambiguous flags  
- Confusing error messages  
- Hard‑to‑document behavior  
- AI assistants generating invalid combinations  

**Separate modes eliminate all of this.**

---

# **5. Recommended architecture**

## **Binary name:** `smartfs` (or `smv-core`, or whatever you prefer)

## **Entry points:**
- `/usr/local/bin/smv` → symlink → `smartfs`
- `/usr/local/bin/srm` → symlink → `smartfs`

## **Dispatch logic:**
```rust
let invoked_as = std::env::args().next().unwrap();
match invoked_as.as_str() {
    "smv" => run_smv(),
    "srm" => run_srm(),
    _ => show_help(),
}
```

## **Internal crates:**
- `libsmartfs`  
  - `git.rs`  
  - `mv.rs`  
  - `rm.rs`  
  - `trash.rs`  
  - `config.rs`  
  - `daemon.rs`  
  - `queue.rs`  

## **Config path:**
```
$HOME/smartfs/config.toml
```

---

# **6. Final recommendation**
- **Use `smv` and `srm` as separate entrypoints.**
- **Implement them as one binary with argv dispatch.**
- **Keep their flag parsers separate.**
- **Share all internal logic.**

This gives you:

- Clean UX  
- Zero ambiguity  
- Maximum compatibility  
- Minimum maintenance burden  
- AI‑friendly command names  

---
**Short version:**  
`smartfo` is a clean, unique, collision‑free name for the *binary*, and yes — the correct installation pattern is:

> **`mv` → smartfo**  
> **`rm` → smartfo**

NOT:

> `mv` → `smv` → `smartfo`

That extra hop creates more problems than it solves.

Below is the deeper reasoning, structured around the real engineering constraints you’re navigating.

---

# **Is `smartfo` a good name?**
Yes — and here’s why it works well:

### **Strengths**
- **Unique** — no collisions with existing Linux tools, crates, or packages  
- **Semantic** — “smart file ops” is obvious to anyone reading it  
- **Short enough** — 7 characters is fine for a binary that users rarely type directly  
- **Expandable** — you can later add `smartfo daemon`, `smartfo doctor`, etc.  
- **Neutral** — doesn’t imply Git‑only or trash‑only behavior  

### **No meaningful concerns**
- Not similar to `smartctl` (disk utility)  
- Not similar to `smartmontools`  
- Not similar to `srm` (secure delete)  
- Not similar to `smf` (Solaris service mgmt)  

**Conclusion:**  
`smartfo` is a safe, future‑proof name for the underlying binary.

---

# **Should the entrypoints be `mv` and `rm` instead of `smv` and `srm`?**
Yes — if your goal is:

- AI assistants doing the right thing automatically  
- Shell scripts behaving correctly  
- Makefiles, Justfiles, Devbox hooks, and random tooling “just working”  
- Zero cognitive overhead for humans  

Then the entrypoints **must** be:

```
mv -> smartfo
rm -> smartfo
```

This is the only way to achieve *true drop‑in replacement semantics*.

---

# **Why NOT use smv → smartfo and srm → smartfo?**

### **1. Extra alias layers break predictability**
AI assistants often generate:

```
mv foo bar
rm foo
```

They do **not** generate:

```
smv foo bar
srm foo
```

If you rely on `smv`/`srm`, you’re back to explaining your environment to the AI — which is exactly what you’re trying to eliminate.

### **2. Aliases don’t propagate into non-interactive shells**
- Cron  
- Systemd units  
- Justfile recipes  
- Makefile recipes  
- Devbox shell hooks  
- Git hooks  
- Python subprocess calls  
- Node child_process  
- Rust `Command::new("mv")`  

All of these bypass aliases unless you explicitly force them.

### **3. Double indirection complicates debugging**
`mv → smv → smartfo` means:

- Two layers of PATH resolution  
- Two layers of argv rewriting  
- Two layers of error handling  
- Two layers of potential misconfiguration  

### **4. Users expect mv/rm semantics**
If you want to override behavior, override the names people and tools already use.

---

# **Correct architecture**
This is the cleanest, most robust design:

```
~/.local/bin/mv  -> ~/.local/libexec/smartfo
~/.local/bin/rm  -> ~/.local/libexec/smartfo
~/.local/bin/smv -> ~/.local/libexec/smartfo   (optional)
~/.local/bin/srm -> ~/.local/libexec/smartfo   (optional)
```

### **Dispatch logic inside smartfo**
```
if argv[0] ends with "mv": run mv mode
if argv[0] ends with "rm": run rm mode
if argv[0] ends with "smv": run mv mode (debug)
if argv[0] ends with "srm": run rm mode (debug)
```

### **Why this is ideal**
- One binary to maintain  
- Two (or four) entrypoints  
- Zero CLI flag conflicts  
- Perfect drop‑in behavior  
- Optional explicit commands for debugging  

---

# **Do you need to worry about CLI argument conflicts?**
Only if you try to unify the parsers.

Since you’re using `argv[0]` dispatch:

- `mv` mode uses the **mv flag parser**  
- `rm` mode uses the **rm flag parser**  

This avoids all ambiguity.

If you tried to unify them, you’d get:

- `-r` means “recursive delete” for rm  
- `-r` means nothing for mv  
- `-T` exists for mv but not rm  
- `--preserve-root` exists for rm but not mv  
- `--backup` exists for mv but not rm  

So yes — **keep the parsers separate**.

---

# **Final recommendation**
- **Binary name:** `smartfo`  
- **Primary entrypoints:** `mv`, `rm`  
- **Optional entrypoints:** `smv`, `srm`  
- **Architecture:** one binary, argv‑based dispatch  
- **Parsers:** separate mv and rm parsers  
- **Config:** `$HOME/smartfo/config.toml`  

This gives you:

- Maximum compatibility  
- Minimum cognitive load  
- AI‑friendly behavior  
- Clean internal design  
- Zero ambiguity  

---
