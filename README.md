# Smartfo

VCS-aware, safe, non-blocking replacement for `mv` and `rm` commands.

## Overview

Smartfo is a drop-in replacement for POSIX `mv` and `rm` that provides:

- **VCS-aware operations**: Automatically detects Git, Mercurial, SVN, Jujutsu and uses VCS-native commands
- **Trash instead of delete**: Files removed via `rm` are moved to a versioned trash directory
- **Async by default**: Background daemon returns the shell prompt immediately
- **Audit trail**: Every operation is logged with structured metadata
- **Git hooks**: Client-side and server-side hooks block raw deletions and renames

## Installation

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

### Smartfo Commands

```bash
smartfo list                    # List operations and queue status
smartfo list --all              # Show all items including completed
smartfo list --limit 10         # Limit number of items
smartfo status                  # Show daemon and queue status
smartfo status --detailed       # Show detailed status
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

## Output Formats

Smartfo supports multiple output formats for agent consumption:

- **TOON**: Token-efficient format (default in agent mode)
- **JSON**: Structured JSON output
- **Human**: Human-readable text output

```bash
smartfo list --toon              # Request TOON format
smartfo list --format=json       # Request JSON format
```

## Mode Selection

Smartfo automatically detects agent sessions and defaults to agent-optimized output.

```bash
smartfo list --agent             # Force agent mode (TOON output, minimal fields)
smartfo list --human             # Force human mode (friendly messages, full output)
```

## Field Selection

Reduce token consumption by selecting specific output fields:

```bash
smartfo list --fields=id,status,source
smartfo status --fields=operation,queue_size
```

## Content Truncation

Large text fields are automatically truncated to 1000 characters by default.

```bash
smartfo list --full              # Disable truncation and show full output
```

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

## License

[Specify your license here]
