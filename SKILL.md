---
name: smartfo
description: VCS-aware, safe, non-blocking replacement for mv and rm commands
version: 0.1.0
triggers:
  - move files
  - remove files
  - delete files
  - smartfo
  - safe delete
  - vcs move
---

# Smartfo Agent Skill

Smartfo is a VCS-aware, safe, non-blocking replacement for `mv` and `rm` commands. It provides automatic trash management, async operations, and comprehensive audit logging.

## When to Use

Use this skill when you need to:
- Move or rename files with VCS awareness
- Safely remove files (trash instead of delete)
- Perform async operations for large files
- Maintain audit trails for file operations

## Available Commands

### mv - Move Files

Move (rename) files with VCS awareness and async support

**Usage:**
- `mv file1 file2`
- `mv file1 dir/`
- `mv --async large.bin /mnt/backup/`
- `mv --force-outside-vcs tracked.txt /tmp/`

**Flags:**
- `-f`, `--force`: Do not prompt before overwriting
- `-i`, `--interactive`: Prompt before overwrite
- `-n`, `--no-clobber`: Do not overwrite an existing file
- `-v`, `--verbose`: Explain what is being done
- `--async`: Force async move even for small/same-fs files
- `--blocking`: Wait for operation to complete
- `--force-outside-vcs`: Allow moving tracked files outside repo
- `--plain`: Disable all smart features; behave exactly like POSIX mv

### rm - Remove Files

Remove files by moving to trash (async by default)

**Usage:**
- `rm file.txt`
- `rm -r directory/`
- `rm --force file.txt`
- `rm --blocking file.txt`

**Flags:**
- `-f`, `--force`: Ignore nonexistent files and arguments, never prompt
- `-i`, `--interactive`: Prompt before every removal
- `-r`, `--recursive`: Remove directories and their contents recursively
- `-v`, `--verbose`: Explain what is being done
- `--blocking`: Wait for operation to complete (sync mode)
- `--plain`: Disable all smart features; behave exactly like POSIX rm

### list - List Operations

List queued and completed operations

**Usage:**
- `smartfo list`
- `smartfo list --all`
- `smartfo list --limit 10`
- `smartfo list --fields=id,status,source`

### status - Show Status

Show daemon and queue status

**Usage:**
- `smartfo status`
- `smartfo status --detailed`
- `smartfo status --fields=operation,queue_size`

## Output Formats

Smartfo supports multiple output formats for agent consumption:

- **TOON**: Token-efficient format (default in agent mode)
- **JSON**: Structured JSON output
- **Human**: Human-readable text output

Use `--toon` or `--format=toon` to request TOON format.

## Mode Selection

Smartfo automatically detects agent sessions and defaults to agent-optimized output. You can explicitly control mode with:

- `--agent`: Force agent mode (TOON output, minimal fields)
- `--human`: Force human mode (friendly messages, full output)

## Field Selection

Reduce token consumption by selecting specific output fields:

```bash
smartfo list --fields=id,status,source
smartfo status --fields=operation,queue_size
```

## Content Truncation

Large text fields are automatically truncated to 1000 characters by default. Use `--full` to disable truncation and show complete content.

## Session Context

Smartfo provides session context for agent awareness:
- Current working directory
- Git repository root (if in a repo)
- Recent operations count
- Queue size (if daemon is running)

## Installation

Smartfo is installed via symlinks. Use `smartfo --install` to set up:
- `mv` symlink for move operations
- `rm` symlink for remove operations
- Git hooks for VCS integration

## Notes

- All `rm` operations are async by default (files moved to trash)
- VCS-aware operations use native commands (git mv, hg mv, etc.)
- Audit logs are maintained in `$HOME/smartfo/audit/operations.jsonl`
- Use `--plain` to disable all smart features for POSIX compatibility

