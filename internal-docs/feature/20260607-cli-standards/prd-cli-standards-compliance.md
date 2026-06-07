---
# Product Requirements Document (PRD)

## Introduction / Overview
- **Feature name:** CLI Standards Compliance
- **Summary:** Bring smartfo into full compliance with the CLI Tool Standards ADR (adr-20260607001-cli-tool-standards v3.3.0), implementing all 35 standards including Developer UX standard compliance, standard arguments, configuration management with auto-initialization and versioning, install/uninstall functionality, input/output discipline with color control (--color=auto|always|never), logging modes with structured logging and format auto-detection, signal handling including config reload, TUI mode, dry-run, progress indicators, daemon processes (--daemon/--no-daemon flags with auto-spawning, explicit control, and job filtering) with platform fallback, error formatting, shell completion, man pages, pager integration, cross-platform compatibility, security, resource management, collection vs processing separation, health checks for containers, privacy mode with anonymous lists, and audit logging with retention.
- **Context:**
  - smartfo is a Rust CLI tool that transparently replaces `mv` and `rm` with VCS-aware, safe, non-blocking operations
  - Current implementation already has partial compliance with some standards (argv[0] dispatch, config precedence, daemon model, structured logging)
  - The CLI Tool Standards ADR defines 34 comprehensive standards for CLI programs across languages to ensure consistent UX, configuration handling, and operational posture
  - Full compliance will make smartfo a model CLI tool in the monorepo and improve user experience significantly
  - Related to existing smartfo requirements in `internal-docs/requirements/20260604-smartfo-initial-reqs/`

## Goals
- Implement all 35 standards from CLI Tool Standards ADR (adr-20260607001)
- Ensure the argv[0] dispatch model (mv/rm/smartfo) works seamlessly with new standards
- Enhance the existing daemon model without breaking async-by-default behavior
- Provide comprehensive documentation (shell completion, man pages) for all modes
- Support both interactive (TUI) and script-friendly (POSIX) usage patterns
- Implement config file versioning for future schema evolution

## User Stories
- As a **new user**, I want `--install` to automatically set up shell completion and default config so I can start using smartfo immediately
- As a **developer**, I want `--dry-run` to preview what smartfo will do before actually moving or deleting files
- As a **system administrator**, I want structured JSON logging for integration with log aggregation systems
- As a **power user**, I want a TUI mode to interactively configure and review complex operations
- As a **script author**, I want `--quiet` to suppress all non-essential output for clean automation
- As a **container operator**, I want a health check endpoint to verify smartfo daemon status in Kubernetes
- As a **security-conscious user**, I want privacy mode to prevent logging of sensitive file paths
- As a **user upgrading**, I want config files to auto-migrate to new formats without manual intervention
- As a **cross-platform user**, I want smartfo to handle path separators correctly on Windows, Linux, and macOS
- As a **long-running process user**, I want `SIGHUP` to reload config without restarting the daemon

## Functional Requirements

### Developer UX Standard (ADR #0)
- Conform to Developer UX standard including:
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
- Document and enforce precedence: CLI args > env vars > project config > user config (XDG) > system config > defaults
- Support project config at `{REPO_ROOT}/.config/smartfo/config.toml` (already implemented)
- Support user config at `$XDG_CONFIG_HOME/smartfo/config.toml` or `$HOME/.config/smartfo/config.toml` (already implemented)
- Add system config support at `/etc/smartfo/config.toml` (Linux) or equivalent platform-specific locations
- Add config variable to control noisy fallback behavior for daemon on unsupported platforms (e.g., `daemon_fallback_quiet = true`)

### Config File Initialization (ADR #3)
- On first run, detect if config file exists in expected location
- If missing, create default config file with all settings commented out
- Include default values and explanations for each option in comments
- Support `--init-config` flag to explicitly create/recreate default config

### Install/Uninstall Flag (ADR #4)
- Enhance existing `--install` flag to:
  - Generate shell completion scripts for bash, zsh, and fish
  - Initialize default config files in appropriate XDG locations
  - Set up any required environment variables
  - Install man pages to system man directory
- Add `--uninstall` counterpart for cleanup:
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
- Enhance existing self-spawning daemon model to support all ADR requirements simultaneously:
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
- Already partially implemented; complete coverage for all config options

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
- Already implemented in `src/audit.rs`; enhance with:
- Configurable retention period (default 90 days)
- Automatic cleanup of old audit entries
- Audit log rotation to prevent unbounded growth
- Support for export of audit logs
- Privacy mode integration to sanitize sensitive entries

## Non-Functional Requirements
- **Performance**: All new features (TUI, completion, man pages) must not impact startup time significantly (<100ms additional overhead)
- **Cross-Platform**: Must work on Linux, macOS, and Windows
- **Security**: No secrets in logs; secure config file handling; proper file permissions
- **Reliability**: Daemon must survive restarts; config validation must be robust
- **Usability**: TUI must be intuitive; error messages must be actionable; help must be comprehensive
- **Maintainability**: Code must follow existing patterns; tests must cover new functionality
- **Documentation**: All new features must be documented in man pages and help output

## Technical Considerations
- **Developer UX Tools**: Support direnv, devbox, justfile, nix, containers for development environment
- **TUI Library**: Use ratatui (formerly tui-rs) for terminal UI implementation
- **Shell Completion**: Use clap's built-in completion generation
- **Man Pages**: Use pandoc or manual roff source; consider using help2man if applicable
- **Progress Indicators**: Use indicatif crate for progress bars
- **Config Versioning**: Add version field to config schema in `src/config.rs`
- **Signal Handling**: Use nix crate for signal handling (already a dependency)
- **Health Check**: Implement simple HTTP server using actix-web or hyper, or signal-based check
- **Pager Integration**: Use pager crate or call external pager directly
- **Cross-Platform Paths**: Leverage std::path::Path and std::path::PathBuf
- **Structured Logging**: Enhance existing tracing subscriber in `src/logging.rs`
- **Testing**: Add new test modules for each standard; use tempfile for filesystem tests
- **Dependencies**: May need to add new crates (ratatui, indicatif, pager, etc.)
- **Daemon Model**: Enhance existing `src/daemon.rs` without breaking async-by-default behavior
- **Config Schema**: Add version field to TOML schema in `src/config.rs` for future evolution; add daemon_fallback_quiet config option

## Success Metrics
- All 35 standards from CLI Tool Standards ADR implemented and tested
- Developer UX standard compliance (direnv, devbox, justfile, nix, containers)
- Shell completion works for bash, zsh, and fish
- Man pages installed and accessible via man command
- TUI mode functional for all major operations
- Config versioning implemented and validated
- Health check returns correct status in container environments
- Privacy mode successfully sanitizes audit logs
- All tests pass (including new tests for each standard)
- Startup time increase <100ms with all new features enabled
- Code coverage maintained or improved for new modules

## Open Questions
- Should TUI mode be the default when no arguments provided, or require explicit flag?
- What should be the default retention period for audit logs? (Current: 90 days)
- Should health check use HTTP endpoint or signal-based approach?
- Should privacy mode be enabled by default for sensitive paths?
- What is the priority order for implementing the 34 standards? (Can be phased)

## Dependencies
- CLI Tool Standards ADR (adr-20260607001 v3.3.0) - must align with latest version
- Existing smartfo requirements and implementation
- Rust ecosystem crates (ratatui, indicatif, pager, etc.)
- Container orchestration platforms (Docker, Kubernetes) for health check testing
- Shell environments (bash, zsh, fish) for completion testing
- Cross-platform test environments (Linux, macOS, Windows)

## Timeline / Milestones
- **Phase 1** (Week 1-2): Developer UX standard, standard arguments, config initialization, install/uninstall enhancement, input/globbing, output discipline
- **Phase 2** (Week 3-4): Logging modes, signals/exit codes, dry-run, confirmation prompts, progress indicators
- **Phase 3** (Week 5-6): Daemon enhancements, error formatting, file/URL formatting, shell completion, man pages
- **Phase 4** (Week 7-8): Pager integration, subcommand organization, config validation, terminal size awareness, env var naming
- **Phase 5** (Week 9-10): Cross-platform paths, credential handling, resource limits, testing infrastructure
- **Phase 6** (Week 11-12): Collection/processing separation, config versioning, structured logging auto-detection, SIGHUP reload
- **Phase 7** (Week 13-14): Health checks, privacy mode, audit logging enhancements, TUI mode
- **Phase 8** (Week 15-16): Integration testing, documentation, cross-platform testing, release preparation

---
*Generated from PRD template*