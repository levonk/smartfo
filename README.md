# Smartfo

VCS-aware, safe, non-blocking replacement for `mv` and `rm` commands.

## Overview

Smartfo is a drop-in replacement for POSIX `mv` and `rm` that provides:

- **VCS-aware operations**: Automatically detects Git, Mercurial, SVN, Jujutsu and uses VCS-native commands
- **Trash instead of delete**: Files removed via `rm` are moved to a versioned trash directory
- **Async by default**: Background daemon returns the shell prompt immediately
- **Audit trail**: Every operation is logged with structured metadata
- **Git hooks**: Client-side and server-side hooks block raw deletions and renames
- **Agent mode**: Optimized for AI agents with token-efficient TOON output, session hooks, and installable skills

## Installation

### Nix (Recommended)

```bash
# Run directly from GitHub
nix run github:levonk/smartfo

# Install to profile
nix profile install github:levonk/smartfo

# After installation, set up symlinks
smartfo --install

# Development shell
nix develop
```

### From Source

```bash
# Install symlinks and hooks
smartfo --install

# Initialize config file
smartfo --init-config
```

## Usage

### Move Files

```bash
mv file1 file2           # Rename file1 to file2
mv file1 dir/            # Move file1 into dir/
mv --async large.bin /mnt/backup/  # Async move for large file
mv --force-outside-vcs tracked.txt /tmp/  # Allow moving tracked files outside repo
```

### Remove Files

```bash
rm file.txt             # Move file.txt to trash
rm -r directory/        # Recursively remove directory
rm --force file.txt     # Bypass prompts
rm --blocking file.txt  # Wait for operation to complete
```

### Agent Mode Usage Examples

Smartfo automatically detects agent sessions and defaults to agent-optimized output. Here are common agent workflows:

**List operations with minimal fields:**
```bash
smartfo list --fields=id,status,source
```

**Check queue status:**
```bash
smartfo status --fields=queue_size,daemon_status
```

**View operation details with full content:**
```bash
smartfo view 42 --full
```

**Get session context for orientation:**
```bash
smartfo --session-context
```

**Force human mode for debugging:**
```bash
smartfo list --human
```

### TUI Mode

Smartfo includes a Terminal User Interface (TUI) mode for interactive argument editing and configuration.

**Launch TUI for move operations:**
```bash
mv --tui file1 file2
mv --interactive-tui file1 file2
```

### Resource Limits

Smartfo supports resource limiting for daemon operations to prevent resource exhaustion.

**Set resource limits via CLI flags:**
```bash
mv --max-memory=4096 --max-cpu=80 file1 file2
rm --max-memory=2048 --max-cpu=50 file.txt
```

**Set resource limits via config file:**
```toml
[concurrency]
max_memory_mb = 4096  # 4GB memory limit
max_cpu_percent = 80  # 80% CPU usage limit
```

**Set resource limits via environment variables:**
```bash
export SMARTFO_CONCURRENCY_MAX_MEMORY_MB=4096
export SMARTFO_CONCURRENCY_MAX_CPU_PERCENT=80
```

**Resource Limit Guidelines:**
- **Memory limits**: Set based on available system memory. For systems with 8GB RAM, a 4GB limit (50%) is reasonable. For systems with 16GB+, 8GB (50%) or higher may be appropriate.
- **CPU limits**: Set based on system load. For single-user systems, 80-90% is reasonable. For shared systems, 50-70% prevents impact on other processes.
- **Unlimited (0)**: Use unlimited limits only when resource constraints are not a concern (e.g., dedicated systems with ample resources).
- **Monitoring**: Resource usage is logged when limits are enforced. Check logs for violations and adjust limits accordingly.
- **Graceful degradation**: When limits are exceeded, operations are rejected with clear error messages rather than causing system instability.

### Privacy Mode

Smartfo provides privacy mode to anonymize sensitive data in logs and output. This is useful when sharing audit logs or operation details with third parties.

**Enable privacy mode via CLI flag:**
```bash
mv --privacy file1 file2
rm --privacy file.txt
```

**Configure privacy mode in config file:**
```toml
[privacy]
mode = "privacy"  # normal, privacy, or strict
enabled_toggles = ["log_paths", "log_user_ids", "log_hostnames", "log_repo_info", "log_metadata", "log_session_context"]
ignore_patterns = ["/home/user/*", "/tmp/*"]
distinguish_unknown_anonymous = true
```

**Privacy Mode Features:**
- **Ignore patterns**: Regex patterns to match paths that should be anonymized (e.g., `/home/user/*`)
- **Unknown vs Anonymous**: Distinguish between "unknown" (logged but not assigned) and "anonymous" (ignored entirely)
- **Privacy toggles**: Disable specific data collection categories (paths, user IDs, hostnames, repo info, metadata, session context)
- **Audit logging**: Paths and identifiers in audit logs are sanitized when privacy mode is enabled
- **Session hooks**: Session context output respects privacy settings

### Audit Log Sanitization

When privacy mode is enabled, audit log entries are automatically sanitized to protect sensitive information:

**Sanitized fields:**
- `source_path`: File paths are anonymized based on privacy patterns
- `dest_path`: Destination paths are anonymized for move operations
- `trash_path`: Trash paths are anonymized for delete operations
- `repo_root`: VCS repository paths are anonymized
- `reason`: User-provided reasons are sanitized for secrets

**Export with sanitization:**
```bash
# Export audit log with sanitization
smartfo export-audit --format json --sanitize

# Export without sanitization (default)
smartfo export-audit --format json
```

**Sanitization behavior:**
- Paths matching ignore patterns are replaced with "anonymous"
- In privacy mode, path components are replaced with "unknown"
- In strict mode, all paths are replaced with "anonymous"
- Toggles can disable specific data collection (e.g., `log_paths = false`)

**Example sanitized audit entry:**
```json
{
  "op": "delete",
  "source_path": "unknown/unknown/unknown",
  "trash_path": "unknown/unknown/unknown",
  "privacy_mode": true
}
```

**Privacy Modes:**
- **normal**: No privacy features enabled (default)
- **privacy**: Basic privacy with anonymization of matching patterns
- **strict**: Maximum privacy with all data collection disabled except essential operation metadata

**Launch TUI for remove operations:**
```bash
rm --tui file.txt
rm --interactive-tui file.txt
```

**Launch TUI for install:**
```bash
smartfo --install --tui
```

**Launch TUI for config editing:**
```bash
smartfo --init-config --tui
```

The TUI mode provides:
- Interactive argument editing before execution
- Visual navigation with arrow keys
- Enter to confirm, Esc to cancel
- Terminal resize support

### Smartfo Commands

```bash
smartfo list                    # List operations and queue status
smartfo list --all              # Show all items including completed
smartfo list --limit 10         # Limit number of items
smartfo status                  # Show daemon and queue status
smartfo status --detailed       # Show detailed status
smartfo health check            # Check daemon health status
smartfo health check --signal    # Use signal-based health check (SIGUSR1)
```

## Agent Integration

Smartfo provides two integration paths for AI agents:

### 1. Session Hooks

Session hooks provide ambient context injection for agent sessions (Claude Code, Codex).

```bash
# Install agent hooks
smartfo --install-agent-hooks

# Output session context in TOON format
smartfo --session-context
```

Session hooks inject:
- Current working directory
- Git repository root (if in a repo)
- Smartfo audit log path
- Recent operations count
- Queue size (if daemon is running)

**Hook Installation Paths:**
- Claude Code: `~/.claude/settings.json`
- Codex: `~/.codex/hooks.json`

### Session Hook Installation for Claude Code

For Claude Code integration, smartfo adds a session-start hook to inject context:

**Global installation:**
```bash
smartfo --install-agent-hooks
```

This adds to `~/.claude/settings.json`:
```json
{
  "sessionStart": {
    "hooks": [
      {
        "command": "smartfo",
        "args": ["--session-context"],
        "captureOutput": true
      }
    ]
  }
}
```

**Project-level installation:**
```bash
cd /path/to/project
smartfo --install-agent-hooks
```

This adds to `.claude/settings.json` in the project directory.

### Session Hook Installation for Codex

For Codex integration, smartfo adds hooks to the Codex hooks configuration:

**Global installation:**
```bash
smartfo --install-agent-hooks
```

This adds to `~/.codex/hooks.json`:
```json
{
  "sessionStart": [
    {
      "command": "smartfo",
      "args": ["--session-context"]
    }
  ]
}
```

**Project-level installation:**
```bash
cd /path/to/project
smartfo --install-agent-hooks
```

This adds to `.codex/hooks.json` in the project directory.

### Install Agent Hooks Command

The `--install-agent-hooks` command registers session hooks for agent integration.

```bash
smartfo --install-agent-hooks
```

**Hook installation features:**
- Checks existing hooks and updates executable path if changed
- Idempotent: repeated installs with same path are silent no-ops
- Portable commands: uses PATH-verified binary name
- Explicit opt-in: only registers from user-invoked setup command
- Supports both global and project-level hook configuration

**Hook Configuration:**
- Claude Code: Adds to `~/.claude/settings.json` or project `.claude/settings.json`
- Codex: Adds to `~/.codex/hooks.json` or project `.codex/hooks.json`
- Future: OpenCode plugin system support

### Session Context Command

The `--session-context` command outputs compact state in TOON format for agent orientation.

```bash
smartfo --session-context
```

**Example output:**
```
cwd: /home/user/project
repo_root: /home/user/project
audit_log: /home/user/.smartfo/audit/operations.jsonl
operations: 2 recent
queue: 3 pending
help[1]: Run smartfo list for operations
help[2]: Run smartfo status for queue details
```

**Session context features:**
- Token-budget-aware output (ruthlessly minimized)
- Directory-scoped (only state relevant to current working directory)
- Includes just enough for agent to orient and act
- Deep data belongs in explicit invocations

### 2. Agent Skill

The agent skill provides a comprehensive guide for AI agents to use smartfo effectively.

```bash
# Generate agent skill
smartfo generate-skill

# Generate to file
smartfo generate-skill --output SKILL.md

# Check if skill is stale
smartfo check-skill
```

**Skill Features:**
- Trigger-shaped frontmatter for automatic activation
- Non-interactive command examples
- Static content (no dynamic data)
- Token-efficient TOON format support
- Field selection guidance
- Mode selection documentation

**CI Integration:**
Add to your CI pipeline to ensure skill stays current:
```bash
smartfo check-skill || (smartfo generate-skill --output SKILL.md && git add SKILL.md)
```

### Agent Skill Generation and Installation

The agent skill is a standalone SKILL.md file that agents can load to understand smartfo capabilities.

**Generate the skill:**
```bash
smartfo --generate-skill --output SKILL.md
```

**Skill file structure:**
```markdown
---
name: smartfo
description: VCS-aware, safe, non-blocking replacement for mv and rm commands
---

# Smartfo Agent Skill

[Comprehensive guide for agents...]
```

**Installation options:**

1. **Manual installation**: Copy SKILL.md to your agent's skills directory
2. **agentskills.io**: Upload to the agent skills marketplace (future)
3. **Project-level**: Add to `.claude/skills/` or `.codex/skills/` in your project

**Skill maintenance:**
- Regenerate after CLI changes: `smartfo --generate-skill --output SKILL.md`
- Check staleness in CI: `smartfo --check-skill`
- Commit SKILL.md to your repository for version control

### Generate Skill Command

The `--generate-skill` command outputs a SKILL.md file with comprehensive agent guidance.

```bash
smartfo --generate-skill
smartfo --generate-skill --output SKILL.md
```

**Skill generation features:**
- Template-based generation from CLI help and examples
- Strips live state (static only, no dynamic data like active operations)
- Rewrites command examples to non-interactive form
- Includes trigger-shaped frontmatter for automatic activation
- Documents both integration paths (hooks and skills)

### Check Skill Command

The `--check-skill` command validates that the committed skill file is current.

```bash
smartfo --check-skill
```

**Check behavior:**
- Exits with code 0 if skill is current
- Exits with code 1 if skill is stale and needs regeneration
- Used in CI pipelines to ensure documentation stays synchronized

## Terminal Size Awareness

Smartfo automatically detects terminal size and formats output accordingly for optimal readability.

### Terminal Size Detection

Smartfo detects terminal dimensions on startup using:
- **ioctl TIOCGWINSZ** (Unix/Linux/macOS): Direct terminal size query
- **Environment variables**: Falls back to `COLUMNS` and `LINES` if ioctl fails
- **Default values**: Uses 80x24 if running in a non-terminal environment

### Output Formatting

**Help text**: Usage messages are wrapped to fit terminal width
```bash
mv --usage  # Wrapped to terminal width
rm --usage  # Wrapped to terminal width
```

**Human format output**: JSON output is wrapped based on terminal width when in human mode
```bash
smartfo list --format=human  # Wrapped to terminal width
```

**Fallback behavior**: When running in non-terminal environments (pipes, redirects, CI), smartfo uses default dimensions (80x24) to ensure output remains readable.

### Resize Handling

Terminal resize events (SIGWINCH) are logged when detected. The terminal size cache provides infrastructure for future TUI mode enhancements with interactive resize handling.

### Configuration

Terminal size detection is automatic and requires no configuration. The detection happens at startup and the size is used for all subsequent output formatting.

### TOON Format (Token-Oriented Object Notation)

TOON is a compact, agent-readable format that provides ~40% token savings over equivalent JSON. It is the default format in agent mode.

**Example TOON output:**
```
operations[2]{id,type,status}: "42",move,completed "43",delete,pending
count: 2 of 5 total
help[1]: Run smartfo view <id> for details
```

**TOON features:**
- Compact syntax with minimal punctuation
- Array notation with counts: `operations[2]`
- Object notation with field names: `{id,type,status}`
- Efficient for agent parsing
- Follows [TOON format specification](https://toonformat.dev/reference/spec.html)

### JSON Format

Structured JSON output for programmatic consumption.

```bash
smartfo list --format=json
```

### Human Format

Human-readable text output with friendly messages and full details.

```bash
smartfo list --format=human
smartfo list --human
```

### Format Selection

```bash
smartfo list --toon              # Request TOON format
smartfo list --format=json       # Request JSON format
smartfo list --format=human      # Request human format
```

## Mode Selection

Smartfo automatically detects agent sessions and defaults to agent-optimized output.

### Mode Detection

Smartfo uses the following precedence chain to determine output mode:

1. **CLI flags** (highest precedence)
   - `--agent` or `--toon`: Force agent mode with TOON output
   - `--human` or `--interactive`: Force human mode with friendly output

2. **Environment variable**
   - `SMARTFO_MODE=agent`: Force agent mode
   - `SMARTFO_MODE=human`: Force human mode

3. **Config file**
   - `mode = "agent"` or `mode = "human"` in `$HOME/smartfo/config.toml`

4. **Auto-detection** (default)
   - Agent mode: When TTY is not present OR agent session is detected
   - Human mode: When TTY is present AND no agent session is detected

### Agent Session Detection

Smartfo detects agent sessions by checking for:
- Environment variables: `CLAUDE_SESSION`, `CODEX_SESSION`, `OPENCODE_SESSION`
- Agent-specific process parents

### Examples

```bash
# Force agent mode
smartfo list --agent
smartfo list --toon

# Force human mode
smartfo list --human
smartfo list --interactive

# Use environment variable
SMARTFO_MODE=agent smartfo list
SMARTFO_MODE=human smartfo list
```

## Field Selection

Reduce token consumption by selecting specific output fields. Default schemas include 3-4 fields (id, type, status, source) to provide just enough information for agents to decide next steps.

### Default Fields

- **list command**: id, type, status, source
- **status command**: operation, queue_size, daemon_status
- **view command**: All fields by default

### Field Selection

```bash
smartfo list --fields=id,status,source
smartfo list --fields=id,type,status,source,destination
smartfo status --fields=operation,queue_size
```

### Available Fields

Common fields across commands:
- `id`: Operation identifier
- `type`: Operation type (move, delete)
- `status`: Operation status (pending, completed, failed)
- `source`: Source path
- `destination`: Destination path (for moves)
- `timestamp`: Operation timestamp
- `reason`: Operation reason (if provided)

## Content Truncation

Large text fields (file paths, VCS messages, error details) are automatically truncated to 1000 characters by default to reduce token consumption for agents.

### Truncation Behavior

- Truncated fields show: `... (truncated, 8432 chars total)`
- Total size is always displayed so agents know how much content is missing
- Truncation limit is configurable via config file
- Default limit: 1000 characters

### Escape Hatch

Use `--full` flag to disable truncation and show complete content:

```bash
smartfo list --full              # Disable truncation for all fields
smartfo view 42 --full           # Show full details for operation 42
```

### Truncation Metadata

When content is truncated, the output includes:
- Truncation indicator: `...`
- Total character count: `(truncated, 8432 chars total)`
- Help suggestion: `help[1]: Run smartfo view 42 --full to see complete details`

## Configuration

Config file: `$HOME/smartfo/config.toml` (or `$XDG_CONFIG_HOME/smartfo/config.toml`)

Override precedence (highest wins):
1. CLI flags
2. Environment variables (`SMARTFO_<SECTION>_<KEY>`)
3. User config file
4. Built-in defaults

## Environment Variables

| Variable | Purpose |
|----------|---------|
| `SMARTFO_BEHAVIOR_DEFAULT_BLOCKING` | Force blocking mode globally |
| `SMARTFO_TRASH_ROOT` | Override trash directory |
| `SMARTFO_PATHS_AUDIT_LOG` | Override audit log path |
| `SMARTFO_CONCURRENCY_MAX_CONCURRENT_JOBS` | Global parallel job ceiling |
| `SMARTFO_MODE` | Force output mode (agent/human) |
| `SMARTFO_TOKEN_BUDGET` | Token budget for TOON output |

## Development

```bash
# Build
devbox run cargo build

# Run tests
devbox run cargo test

# Lint
devbox run cargo clippy -- -D warnings

# Format
devbox run cargo fmt
```

## Migration Guide

### For Existing Users

Smartfo's agent mode is backward compatible with existing workflows. No changes are required for human usage.

**What's new:**
- Agent mode auto-detection for AI agents
- TOON format for token-efficient output
- Session hooks for ambient context injection
- Installable agent skills

**Optional enhancements:**

1. **Install session hooks** (recommended for agent users):
   ```bash
   smartfo --install-agent-hooks
   ```

2. **Generate agent skill** (for agent integration):
   ```bash
   smartfo --generate-skill --output SKILL.md
   ```

3. **Force human mode** (if auto-detection is incorrect):
   ```bash
   smartfo list --human
   # Or set environment variable
   export SMARTFO_MODE=human
   ```

**No breaking changes:**
- All existing commands work unchanged
- Human mode is the default when TTY is present
- Agent mode only activates when detected or explicitly requested

## Breaking Changes and Backward Compatibility

### Version Compatibility

Smartfo maintains full backward compatibility with existing human workflows. All breaking changes are opt-in for agent mode.

### No Breaking Changes

- **Existing commands**: All `mv` and `rm` commands work exactly as before
- **Default behavior**: Human mode remains the default for TTY sessions
- **Output format**: Human-readable output is unchanged in human mode
- **Configuration**: Existing config files continue to work without modification

### Agent Mode Changes (Opt-In)

The following changes only affect agent mode or when explicitly requested:

1. **Default output format**: In agent mode, TOON format is now the default (was JSON or human)
2. **Default field selection**: Agent mode now uses minimal schemas by default (3-4 fields instead of all fields)
3. **Content truncation**: Large fields are truncated by default in agent mode

**To maintain previous behavior in agent mode:**
```bash
# Use human mode explicitly
smartfo list --human

# Request full fields
smartfo list --fields=id,type,status,source,destination,timestamp

# Disable truncation
smartfo list --full
```

### Environment Variable Changes

New environment variable added:
- `SMARTFO_MODE`: Force output mode (agent/human)
- `SMARTFO_TOKEN_BUDGET`: Token budget for TOON output

These are optional and do not affect existing behavior unless set.

### Hook Configuration Changes

Hook configuration files are modified by `--install-agent-hooks`:
- `~/.claude/settings.json` (Claude Code)
- `~/.codex/hooks.json` (Codex)

**Idempotent installation**: Repeated installs with the same path are silent no-ops.

### Skill File Changes

The SKILL.md format has been updated with:
- Trigger-shaped frontmatter for automatic activation
- Non-interactive command examples
- Static content (no dynamic data)

**Regeneration required**: After CLI changes, regenerate with:
```bash
smartfo --generate-skill --output SKILL.md
```

## Troubleshooting

### Agent Mode Issues

**Problem: Agent mode not activating**

**Solution**: Verify agent session detection:
```bash
# Check if agent session is detected
echo $CLAUDE_SESSION $CODEX_SESSION $OPENCODE_SESSION

# Force agent mode explicitly
smartfo list --agent
```

**Problem: TOON format not working**

**Solution**: Verify format selection:
```bash
# Force TOON format
smartfo list --toon

# Check mode
smartfo list --agent
```

**Problem: Session hooks not injecting context**

**Solution**: Verify hook installation:
```bash
# Reinstall hooks
smartfo --install-agent-hooks

# Test session context manually
smartfo --session-context
```

### Hook Installation Issues

**Problem: Hooks not found by agent**

**Solution**: Verify hook configuration paths:
- Claude Code: Check `~/.claude/settings.json` or project `.claude/settings.json`
- Codex: Check `~/.codex/hooks.json` or project `.codex/hooks.json`

**Problem: Hook path incorrect after binary move**

**Solution**: Reinstall hooks to update path:
```bash
smartfo --install-agent-hooks
```

### Skill Issues

**Problem: Skill is stale**

**Solution**: Regenerate skill:
```bash
smartfo --generate-skill --output SKILL.md
```

**Problem: CI check fails**

**Solution**: Add to CI pipeline:
```bash
smartfo check-skill || (smartfo generate-skill --output SKILL.md && git add SKILL.md)
```

### General Issues

**Problem: Commands not found**

**Solution**: Verify smartfo is in PATH:
```bash
which smartfo
# Or use absolute path
/path/to/smartfo list
```

**Problem: Permission denied**

**Solution**: Check file permissions and ownership for trash directory and audit log.

**Problem: Daemon not responding**

**Solution**: Check daemon status:
```bash
smartfo status
# If daemon is not running, it will auto-spawn on next async operation
```

## Development

This project uses several Developer UX tools to provide a consistent development environment.

### Development Environment Setup

#### Using direnv (Recommended)

The project includes a `.envrc` file for automatic environment setup with direnv:

```bash
# Install direnv if not already installed
brew install direnv  # macOS
# or
sudo apt install direnv  # Linux

# Hook direnv into your shell
echo 'eval "$(direnv hook bash)"' >> ~/.bashrc
# or for zsh
echo 'eval "$(direnv hook zsh)"' >> ~/.zshrc

# Allow the project's .envrc
cd /path/to/smartfo
direnv allow
```

The `.envrc` file automatically:
- Ensures Nix is available
- Installs and configures devbox
- Sets up smartfo-specific environment variables
- Adds project paths to PATH

#### Using devbox

The project uses devbox for reproducible development environments:

```bash
# Start devbox shell
devbox shell

# Or use direnv (recommended) - it automatically loads devbox
cd /path/to/smartfo
direnv allow
```

The `devbox.json` configuration includes:
- Rust toolchain (rustc, cargo, rust-analyzer)
- Development tools (clippy, rustfmt, cargo-watch)
- Testing tools (cargo-tarpaulin, cargo-audit)
- SQLite for local development
- Just command runner

#### Using justfile

The project provides a justfile with common development tasks:

```bash
# List all available commands
just --list

# Build the project
just build

# Run tests
just test

# Run linting
just lint

# Type checking
just typecheck

# Clean build artifacts
just clean

# Development mode
just dev

# Release build
just release

# Install locally
just install

# Health check
just doctor
```

### Environment Variables

The development environment sets these environment variables automatically:

- `SMARTFO_PROJECT_ROOT`: Project root directory
- `SMARTFO_CONFIG_DIR`: Configuration directory (default: `~/.config/smartfo`)
- `SMARTFO_DATA_DIR`: Data directory (default: `~/.local/share/smartfo`)
- `SMARTFO_CACHE_DIR`: Cache directory (default: `~/.cache/smartfo`)
- `SMARTFO_TRASH_ROOT`: Trash directory (default: `~/.local/share/smartfo/trash`)
- `SMARTFO_AUDIT_LOG`: Audit log path (default: `~/.local/share/smartfo/audit/operations.jsonl`)
- `RUST_LOG`: Log level (default: `info`)
- `RUST_BACKTRACE`: Rust backtrace (default: `1`)
- `DATABASE_URL`: SQLite database path (default: `sqlite:./smartfo.db`)

You can override these in your shell or in the `.envrc` file.

### Container Development

For container-based development, the project includes Docker and docker-compose configurations:

```bash
# Build the container
docker-compose build

# Run commands in the container
docker-compose run cli smartfo --help

# For interactive development
docker-compose run cli bash
```

### Nix Support

The project includes Nix flake support for reproducible builds:

```bash
# Enter Nix development shell
nix develop

# Build with Nix
nix build

# Run with Nix
nix run . -- --help

# Install to Nix profile
nix profile install .

# Install with hooks setup
just nix-install-with-hooks
```

## License

[Specify your license here]
