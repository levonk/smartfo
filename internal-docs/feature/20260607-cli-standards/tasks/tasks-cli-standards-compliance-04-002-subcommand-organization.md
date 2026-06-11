---
story_id: "04-002"
story_title: "Subcommand Organization"
story_name: "subcommand-organization"
prd_name: "cli-standards-compliance"
prd_file: "internal-docs/feature/20260607-cli-standards/prd-cli-standards-compliance.md"
phase: 4
parallel_id: 2
branch: "feature/current/cli-standards-compliance/story-04-002-subcommand-organization"
status: "todo"
assignee: ""
reviewer: ""
dependencies: ["01-001"]
parallel_safe: true
modules: ["cli.rs"]
priority: "SHOULD"
risk_level: "low"
tags: ["feat", "cli"]
due: "2026-08-02"
created_at: "2026-06-07"
updated_at: "2026-06-07"
---

## Summary

Implement subcommand organization as specified in ADR #20. Maintain hierarchical command structure, group related commands under logical subcommands, ensure consistency in command naming patterns, and document command hierarchy in help output. This ensures the argv[0] dispatch model (mv/rm/smartfo) works seamlessly with any new subcommands.

## Current Command Hierarchy

### argv[0] Dispatch Model (Primary Entry Points)
```
smartfo (binary)
├── mv (symlink) → MvArgs
├── smv (symlink) → MvArgs (debug variant)
├── rm (symlink) → RmArgs
├── srm (symlink) → RmArgs (debug variant)
└── smartfo (direct) → SmartfoArgs + SmartfoCommand subcommands
```

### SmartfoCommand Subcommands (Current Flat Structure)
```
SmartfoCommand (enum)
├── git-hook-client
├── git-hook-server
├── list
├── status
├── session-context
├── install-agent-hooks
├── generate-skill
├── check-skill
├── list-jobs
└── cancel-job
```

### SmartfoArgs Top-Level Flags
- `--install`, `--uninstall`, `--init-config` (installation/management)
- `--hooks`, `--no-hooks`, `--force-hooks` (hook configuration)
- `--version`, `--usage`, `--man` (information)
- `--human`, `--agent`, `--toon`, `--format`, `--fields` (output format)
- `--json`, `--color`, `--quiet`, `--debug` (logging)
- `--daemon`, `--no-daemon` (daemon control)
- `--no-pager` (output control)
- `--generate-completion` (completions)
- `--session-context`, `--install-agent-hooks` (agent integration)

## Proposed Subcommand Grouping

### Logical Groupings Identified

1. **Git Hooks Group** → `git` subcommand
   - `git hook-client` (was `git-hook-client`)
   - `git hook-server` (was `git-hook-server`)

2. **Job Management Group** → `job` subcommand
   - `job list` (was `list-jobs`)
   - `job cancel <id>` (was `cancel-job`)

3. **Agent Integration Group** → `agent` subcommand
   - `agent session-context` (was `session-context`)
   - `agent install-hooks` (was `install-agent-hooks`)
   - `agent generate-skill` (was `generate-skill`)
   - `agent check-skill` (was `check-skill`)

4. **Information/Query Group** → `info` subcommand
   - `info list` (was `list`)
   - `info status` (was `status`)

### Proposed New Hierarchy
```
SmartfoCommand (enum)
├── git
│   ├── hook-client
│   └── hook-server
├── job
│   ├── list
│   └── cancel <id>
├── agent
│   ├── session-context
│   ├── install-hooks
│   ├── generate-skill
│   └── check-skill
└── info
    ├── list
    └── status
```

### Benefits of This Grouping
- **Logical organization**: Related commands are grouped together
- **Discoverability**: Users can explore subcommands with `smartfo <group> --help`
- **Namespace clarity**: Avoids command name collisions (e.g., `list` vs `list-jobs`)
- **Future extensibility**: Easy to add new commands to each group
- **Maintains argv[0] dispatch**: mv/rm/smartfo entry points unchanged

## Sub-Tasks

- [x] Audit current command structure for consistency
- [x] Document existing command hierarchy
- [x] Identify opportunities for logical subcommand grouping
- [x] Group related commands under logical subcommands (e.g., config commands, job commands)
- [x] Ensure command naming patterns are consistent
- [x] Document command hierarchy in help output
- [x] Add subcommand help for each logical group
- [x] Ensure argv[0] dispatch (mv/rm/smartfo) remains the primary entry point
- [x] Add smartfo subcommands for operations not covered by mv/rm modes
- [x] Ensure subcommand help is accessible via --help
- [x] Ensure subcommand help shows hierarchy
- [x] Add unit tests for subcommand structure
- [x] Add unit tests for command naming consistency
- [x] Add integration tests for subcommand help output
- [x] Verify argv[0] dispatch still works correctly

## Relevant Files

- `src/cli.rs` — Organize subcommands and command hierarchy
- `src/main.rs` — Ensure dispatch logic works with subcommands
- `tests/subcommand_tests.rs` — Add tests for subcommand organization

## Acceptance Criteria

- [x] Command structure is hierarchical and logical
- [x] Related commands are grouped under logical subcommands
- [x] Command naming patterns are consistent
- [x] Command hierarchy is documented in help output
- [x] argv[0] dispatch (mv/rm/smartfo) works seamlessly with subcommands
- [x] Subcommand help is accessible via --help
- [x] Subcommand help shows command hierarchy
- [x] All tests pass

## Test Plan

- Unit: `cargo test subcommand_tests::command_hierarchy`
- Unit: `cargo test subcommand_tests::naming_consistency`
- Integration: `cargo test subcommand_tests::subcommand_help`
- Integration: `cargo test subcommand_tests::argv0_dispatch`
- Lint: `cargo clippy -- -D warnings`
- Types: `cargo check`

## Observability

- Log subcommand dispatch events (debug level)

## Compliance

- Follows ADR #20: Subcommand Organization

## Risks & Mitigations

- Risk: Subcommand reorganization may break existing scripts — Mitigation: Maintain backward compatibility for argv[0] dispatch
- Risk: Too many subcommands may confuse users — Mitigation: Keep subcommand hierarchy shallow and intuitive

## Dependencies

- 01-001 (Standard Arguments Implementation) — ensures command structure is stable

## Notes

- Keep argv[0] dispatch as the primary model (mv/rm/smartfo)
- Consider adding smartfo subcommands for: config management, job management, health check
- Ensure subcommand names are short and memorable
