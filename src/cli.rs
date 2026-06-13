use clap::Parser;
use std::path::PathBuf;
use anyhow::Context;
use crate::globbing::{expand_globs, is_stdin_piped, read_stdin_paths, process_input_args};

/// Arguments for mv mode (invoked via `mv` or `smv` symlink).
#[derive(Parser, Debug, Clone)]
#[command(
    name = "mv",
    about = "Move (rename) files with VCS awareness",
    long_about = "Move (rename) files with VCS awareness and trash support.

This is a drop-in replacement for POSIX mv that:
- Uses VCS-native moves (git mv, hg mv, etc.) when possible
- Supports async operations for large files
- Provides comprehensive audit logging
- Supports all standard POSIX flags (-f, -i, -n, -v, etc.)
- Includes contextual suggestions for next steps in agent mode
- Supports glob patterns (*.txt, **/*.rs) for batch operations
- Supports stdin input via `-` argument or piped input

Examples:
  mv file1 file2           # Rename file1 to file2
  mv file1 dir/            # Move file1 into dir/
  mv -i file1 file2        # Prompt before overwrite
  mv --async large.bin /mnt/backup/  # Async move for large file
  mv *.txt /backup/        # Move all .txt files using glob pattern
  mv **/*.rs src/          # Move all .rs files recursively
  echo -e 'file1.txt\\nfile2.txt' | mv - /dest/  # Read paths from stdin",
    disable_version_flag = true
)]
pub struct MvArgs {
    /// Do not prompt before overwriting
    #[arg(short = 'f', long)]
    pub force: bool,

    /// Prompt before overwrite
    #[arg(short = 'i', long)]
    pub interactive: bool,

    /// Do not overwrite an existing file
    #[arg(short = 'n', long)]
    pub no_clobber: bool,

    /// Explain what is being done
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Print version information
    #[arg(short = 'V', long = "version")]
    pub version: bool,

    /// Treat DEST as a normal file
    #[arg(short = 'T', long = "no-target-directory")]
    pub no_target_directory: bool,

    /// Move all SOURCE arguments into DIRECTORY
    #[arg(short = 't', long = "target-directory", value_name = "DIRECTORY")]
    pub target_directory: Option<PathBuf>,

    /// Make a backup of each existing destination file
    #[arg(long)]
    pub backup: bool,

    /// Remove any trailing slashes from each SOURCE argument
    #[arg(long)]
    pub strip_trailing_slashes: bool,

    /// Disable all smart features; behave exactly like POSIX mv
    #[arg(long)]
    pub plain: bool,

    /// Allow moving tracked files outside repo
    #[arg(long)]
    pub force_outside_vcs: bool,

    /// Force async move even for small/same-fs files
    #[arg(long = "async")]
    pub async_mode: bool,

    /// Wait for operation to complete
    #[arg(long)]
    pub blocking: bool,

    /// Fsync destination file and directory after operation
    #[arg(long)]
    pub sync: bool,

    /// Annotate intent in the audit log
    #[arg(long, value_name = "REASON")]
    pub reason: Option<String>,

    /// Output operation metadata as JSON
    #[arg(long)]
    pub json: bool,

    /// Color output: auto, always, never
    #[arg(long, value_name = "WHEN")]
    pub color: Option<String>,

    /// Preview operations without executing
    #[arg(long)]
    pub dry_run: bool,

    /// Show brief usage message
    #[arg(long = "usage")]
    pub usage: bool,

    /// Force human mode (interactive output, friendly messages)
    #[arg(long, help = "Force human mode with interactive output and friendly messages")]
    pub human: bool,

    /// Force agent mode (structured output)
    #[arg(long, help = "Force agent mode with structured output optimized for AI consumption")]
    pub agent: bool,

    /// Output in TOON format (token-efficient for agents)
    #[arg(long, help = "Output in TOON format (token-efficient for agents)")]
    pub toon: bool,

    /// Output format: toon, json, or human
    #[arg(long, value_name = "FORMAT", help = "Output format: toon, json, or human")]
    pub format: Option<String>,

    /// Select specific output fields (comma-separated)
    #[arg(long, value_name = "FIELDS", help = "Select specific output fields (comma-separated)")]
    pub fields: Option<String>,

    /// Disable content truncation (show full output)
    #[arg(long, help = "Disable content truncation and show full output")]
    pub full: bool,

    /// Decrease logging verbosity (suppress non-essential output)
    #[arg(short = 'q', long, help = "Decrease logging verbosity (suppress non-essential output)")]
    pub quiet: bool,

    /// Enable debug logging
    #[arg(long, help = "Enable debug logging")]
    pub debug: bool,

    /// Pre-launch daemon in background and wait for jobs
    #[arg(long, help = "Pre-launch daemon in background and wait for jobs")]
    pub daemon: bool,

    /// Force synchronous in-process operation (disable auto-spawning)
    #[arg(long, help = "Force synchronous in-process operation (disable auto-spawning)")]
    pub no_daemon: bool,

    /// Disable pager for long output
    #[arg(long, help = "Disable pager for long output")]
    pub no_pager: bool,

    /// Launch TUI mode for interactive argument editing
    #[arg(long, help = "Launch TUI mode for interactive argument editing")]
    pub interactive_tui: bool,

    /// Launch TUI mode (alias for --interactive-tui)
    #[arg(long, help = "Launch TUI mode (alias for --interactive-tui)")]
    pub tui: bool,

    /// Maximum memory limit in MB (0 = unlimited)
    #[arg(long, value_name = "MB", help = "Maximum memory limit in MB (0 = unlimited)")]
    pub max_memory: Option<u64>,

    /// Maximum CPU usage as percentage (0 = unlimited)
    #[arg(long, value_name = "PERCENT", help = "Maximum CPU usage as percentage (0 = unlimited)")]
    pub max_cpu: Option<u8>,

    /// Enable privacy mode for this operation
    #[arg(long, help = "Enable privacy mode for this operation (anonymize sensitive data)")]
    pub privacy: bool,

    /// Source file(s) to move (supports glob patterns like *.txt, **/*.rs)
    /// Use `-` to read paths from stdin
    #[arg(value_name = "SOURCE")]
    pub sources: Vec<PathBuf>,
}

impl MvArgs {
    /// Returns the destination path and source paths.
    /// When `-t DIRECTORY` is used, all positional args are sources and dest is the directory.
    /// Otherwise, the last positional arg is the destination and the rest are sources.
    /// Expands glob patterns and handles stdin input.
    pub fn resolve_paths(&self) -> anyhow::Result<(Vec<PathBuf>, Option<PathBuf>)> {
        // Check for stdin flag (`-` argument)
        let stdin_flag = self.sources.iter().any(|p| p.to_string_lossy() == "-");

        // Process input arguments (glob expansion and stdin handling)
        let processed_sources = if stdin_flag {
            // Remove the `-` argument and read from stdin
            let sources_without_stdin: Vec<PathBuf> = self.sources
                .iter()
                .filter(|p| p.to_string_lossy() != "-")
                .cloned()
                .collect();

            if is_stdin_piped() || !sources_without_stdin.is_empty() {
                process_input_args(&sources_without_stdin, true)
                    .context("Failed to process input")?
            } else {
                // Only `-` flag, read from stdin
                read_stdin_paths()
                    .context("Failed to read from stdin")?
            }
        } else if is_stdin_piped() {
            // stdin is piped but no explicit `-` flag
            read_stdin_paths()
                .context("Failed to read from stdin")?
        } else {
            // Expand glob patterns in file arguments
            expand_globs(&self.sources)
                .context("Failed to expand glob patterns")?
        };

        if let Some(ref dir) = self.target_directory {
            return Ok((processed_sources, Some(dir.clone())));
        }

        if processed_sources.len() >= 2 {
            let dest = processed_sources.last().cloned();
            let sources = processed_sources[..processed_sources.len() - 1].to_vec();
            return Ok((sources, dest));
        }

        Ok((processed_sources, None))
    }

    /// Validate flag combinations and required arguments
    pub fn validate(&self) -> Result<(), String> {
        // Check for conflicting overwrite flags
        if self.force && self.no_clobber {
            return Err("Cannot specify both --force and --no-clobber".to_string());
        }

        if self.interactive && self.force {
            return Err("Cannot specify both --interactive and --force".to_string());
        }

        // Check for conflicting async/blocking flags
        if self.async_mode && self.blocking {
            return Err("Cannot specify both --async and --blocking".to_string());
        }

        // Validate sources are provided (before glob expansion)
        if self.sources.is_empty() && !is_stdin_piped() {
            return Err("Missing source file(s)".to_string());
        }

        // Validate destination or target directory is provided
        // Note: We can't fully validate this until after glob expansion
        // because the number of sources might change
        if self.target_directory.is_none() && self.sources.len() < 2 && !is_stdin_piped() {
            return Err("Missing destination file operand".to_string());
        }

        Ok(())
    }

    /// Get effective interactive flag (suppressed in agent mode)
    pub fn effective_interactive(&self, agent_mode: bool) -> bool {
        if agent_mode {
            false // Suppress interactive prompts in agent mode
        } else {
            self.interactive
        }
    }

    /// Get effective force flag (force in agent mode to avoid prompts)
    pub fn effective_force(&self, agent_mode: bool) -> bool {
        if agent_mode {
            true // Force mode in agent mode to avoid prompts
        } else {
            self.force
        }
    }
}

/// Arguments for rm mode (invoked via `rm` or `srm` symlink).
#[derive(Parser, Debug, Clone)]
#[command(
    name = "rm",
    about = "Remove files with trash and VCS awareness",
    long_about = "Remove files with trash and VCS awareness.

This is a drop-in replacement for POSIX rm that:
- Moves files to trash instead of permanent deletion
- Uses VCS-aware removal for tracked files
- Supports async operations
- Provides comprehensive audit logging
- Supports all standard POSIX flags (-f, -i, -r, -v, etc.)
- Includes contextual suggestions for next steps in agent mode
- Supports glob patterns (*.txt, **/*.rs) for batch operations
- Supports stdin input via `-` argument or piped input

Examples:
  rm file.txt             # Move file.txt to trash
  rm -rf dir/             # Recursively remove directory
  rm --force-delete file  # Bypass trash and delete directly
  rm --async large.bin     # Async deletion for large file
  rm *.log                # Remove all .log files using glob pattern
  rm **/*.tmp             # Remove all .tmp files recursively
  echo -e 'file1.txt\\nfile2.txt' | rm -  # Read paths from stdin",
    disable_version_flag = true
)]
pub struct RmArgs {
    /// Ignore non-existent files, never prompt
    #[arg(short = 'f', long)]
    pub force: bool,

    /// Prompt before every removal
    #[arg(short = 'i')]
    pub interactive: bool,

    /// Prompt once before removing more than three files, or when removing recursively
    #[arg(short = 'I')]
    pub interactive_once: bool,

    /// Remove directories and their contents recursively
    #[arg(short = 'r', long = "recursive")]
    pub recursive: bool,

    /// Remove empty directories
    #[arg(short = 'd', long = "dir")]
    pub dir: bool,

    /// Print version information
    #[arg(short = 'V', long = "version")]
    pub version: bool,

    /// Do not remove '/' (default)
    #[arg(long)]
    pub preserve_root: bool,

    /// Skip directories on different file systems
    #[arg(long)]
    pub one_filesystem: bool,

    /// Disable all smart features; behave exactly like POSIX rm
    #[arg(long)]
    pub plain: bool,

    /// Bypass trash and delete directly
    #[arg(long)]
    pub force_delete: bool,

    /// Wait for operation to complete
    #[arg(long)]
    pub blocking: bool,

    /// Fsync after operation
    #[arg(long)]
    pub sync: bool,

    /// Annotate intent in the audit log
    #[arg(long, value_name = "REASON")]
    pub reason: Option<String>,

    /// Output operation metadata as JSON
    #[arg(long)]
    pub json: bool,

    /// Color output: auto, always, never
    #[arg(long, value_name = "WHEN")]
    pub color: Option<String>,

    /// Preview operations without executing
    #[arg(long)]
    pub dry_run: bool,

    /// Show brief usage message
    #[arg(long = "usage")]
    pub usage: bool,

    /// Force human mode (interactive output, friendly messages)
    #[arg(long, help = "Force human mode with interactive output and friendly messages")]
    pub human: bool,

    /// Force agent mode (structured output)
    #[arg(long, help = "Force agent mode with structured output optimized for AI consumption")]
    pub agent: bool,

    /// Output in TOON format (token-efficient for agents)
    #[arg(long, help = "Output in TOON format (token-efficient for agents)")]
    pub toon: bool,

    /// Output format: toon, json, or human
    #[arg(long, value_name = "FORMAT", help = "Output format: toon, json, or human")]
    pub format: Option<String>,

    /// Select specific output fields (comma-separated)
    #[arg(long, value_name = "FIELDS", help = "Select specific output fields (comma-separated)")]
    pub fields: Option<String>,

    /// Disable content truncation (show full output)
    #[arg(long, help = "Disable content truncation and show full output")]
    pub full: bool,

    /// Decrease logging verbosity (suppress non-essential output)
    #[arg(short = 'q', long, help = "Decrease logging verbosity (suppress non-essential output)")]
    pub quiet: bool,

    /// Enable debug logging
    #[arg(long, help = "Enable debug logging")]
    pub debug: bool,

    /// Pre-launch daemon in background and wait for jobs
    #[arg(long, help = "Pre-launch daemon in background and wait for jobs")]
    pub daemon: bool,

    /// Force synchronous in-process operation (disable auto-spawning)
    #[arg(long, help = "Force synchronous in-process operation (disable auto-spawning)")]
    pub no_daemon: bool,

    /// Disable pager for long output
    #[arg(long, help = "Disable pager for long output")]
    pub no_pager: bool,

    /// Display man page content
    #[arg(long, help = "Display man page content")]
    pub man: bool,

    /// Launch TUI mode for interactive argument editing
    #[arg(long, help = "Launch TUI mode for interactive argument editing")]
    pub interactive_tui: bool,

    /// Launch TUI mode (alias for --interactive-tui)
    #[arg(long, help = "Launch TUI mode (alias for --interactive-tui)")]
    pub tui: bool,

    /// Maximum memory limit in MB (0 = unlimited)
    #[arg(long, value_name = "MB", help = "Maximum memory limit in MB (0 = unlimited)")]
    pub max_memory: Option<u64>,

    /// Maximum CPU usage as percentage (0 = unlimited)
    #[arg(long, value_name = "PERCENT", help = "Maximum CPU usage as percentage (0 = unlimited)")]
    pub max_cpu: Option<u8>,

    /// Enable privacy mode for this operation
    #[arg(long, help = "Enable privacy mode for this operation (anonymize sensitive data)")]
    pub privacy: bool,

    /// File(s) or directories to remove (supports glob patterns like *.txt, **/*.rs)
    /// Use `-` to read paths from stdin
    #[arg(value_name = "FILE")]
    pub paths: Vec<PathBuf>,
}

impl RmArgs {
    /// Process paths with glob expansion and stdin handling
    pub fn resolve_paths(&self) -> anyhow::Result<Vec<PathBuf>> {
        // Check for stdin flag (`-` argument)
        let stdin_flag = self.paths.iter().any(|p| p.to_string_lossy() == "-");

        // Process input arguments (glob expansion and stdin handling)
        let processed_paths = if stdin_flag {
            // Remove the `-` argument and read from stdin
            let paths_without_stdin: Vec<PathBuf> = self.paths
                .iter()
                .filter(|p| p.to_string_lossy() != "-")
                .cloned()
                .collect();

            if is_stdin_piped() || !paths_without_stdin.is_empty() {
                process_input_args(&paths_without_stdin, true)
                    .context("Failed to process input")?
            } else {
                // Only `-` flag, read from stdin
                read_stdin_paths()
                    .context("Failed to read from stdin")?
            }
        } else if is_stdin_piped() {
            // stdin is piped but no explicit `-` flag
            read_stdin_paths()
                .context("Failed to read from stdin")?
        } else {
            // Expand glob patterns in file arguments
            expand_globs(&self.paths)
                .context("Failed to expand glob patterns")?
        };

        Ok(processed_paths)
    }

    /// Validate flag combinations and required arguments
    pub fn validate(&self) -> Result<(), String> {
        // Check for conflicting interactive flags
        if self.interactive && self.force {
            return Err("Cannot specify both -i and --force".to_string());
        }

        if self.interactive_once && self.force {
            return Err("Cannot specify both -I and --force".to_string());
        }

        if self.interactive && self.interactive_once {
            return Err("Cannot specify both -i and -I".to_string());
        }

        // Check for conflicting async/blocking flags
        if self.blocking && self.force_delete {
            // These can actually be combined, but we should document the behavior
            // For now, allow it
        }

        // Validate paths are provided (before glob expansion)
        if self.paths.is_empty() && !is_stdin_piped() {
            return Err("Missing file operand(s)".to_string());
        }

        // Validate recursive flag usage
        if self.recursive && self.dir {
            return Err("Cannot specify both -r and -d".to_string());
        }

        Ok(())
    }

    /// Get effective interactive flag (suppressed in agent mode)
    pub fn effective_interactive(&self, agent_mode: bool) -> bool {
        if agent_mode {
            false // Suppress interactive prompts in agent mode
        } else {
            self.interactive
        }
    }

    /// Get effective interactive_once flag (suppressed in agent mode)
    pub fn effective_interactive_once(&self, agent_mode: bool) -> bool {
        if agent_mode {
            false // Suppress interactive prompts in agent mode
        } else {
            self.interactive_once
        }
    }

    /// Get effective force flag (force in agent mode to avoid prompts)
    pub fn effective_force(&self, agent_mode: bool) -> bool {
        if agent_mode {
            true // Force mode in agent mode to avoid prompts
        } else {
            self.force
        }
    }
}

/// Arguments for the main smartfo binary.
#[derive(Parser, Debug)]
#[command(
    name = "smartfo",
    about = "VCS-aware safe mv/rm replacement with trash and audit",
    long_about = "VCS-aware safe mv/rm replacement with trash and audit.

Smartfo is a drop-in replacement for POSIX mv and rm that provides:
- VCS-aware file operations (Git, Mercurial, SVN, Jujutsu)
- Trash instead of permanent deletion
- Async background processing
- Comprehensive audit logging
- Git hooks to prevent data loss

Installation:
  smartfo --install         # Install symlinks and hooks
  smartfo --init-config     # Create default config file

Usage:
  mv file1 file2            # Use via mv symlink
  rm file.txt               # Use via rm symlink
  smartfo --install         # Install or update
  smartfo                   # Show content-first state summary (context-aware)
  smartfo --help            # Show detailed usage information

No-Args Behavior:
  When invoked without arguments, smartfo shows a content-first state summary
  instead of a usage manual. The output is context-aware and includes:
  - Current directory and git repository status
  - Operations queue summary (if in git repository)
  - Daemon status (if running)
  - Contextual help suggestions

  Use --help flag for detailed usage information.",
    disable_version_flag = true
)]
pub struct SmartfoArgs {
    /// Install symlinks and Git hooks
    #[arg(long)]
    pub install: bool,

    /// Hook types to install: client, server, or client,server (default: both)
    #[arg(long, value_name = "TYPE")]
    pub hooks: Option<String>,

    /// Skip hook installation
    #[arg(long)]
    pub no_hooks: bool,

    /// Overwrite existing hook files when installing
    #[arg(long = "force-hooks")]
    pub force_hooks: bool,

    /// Overwrite existing files when installing
    #[arg(long)]
    pub force: bool,

    /// Initialize or recreate default config file with all settings commented out
    #[arg(long = "init-config", help = "Initialize or recreate default config file with all settings commented out. Use --force to overwrite an existing config file.")]
    pub init_config: bool,

    /// Validate configuration file without loading
    #[arg(long = "validate-config")]
    pub validate_config: bool,

    /// Uninstall smartfo (remove symlinks, completions, man pages)
    #[arg(long)]
    pub uninstall: bool,

    /// Bypass confirmation prompts during uninstall
    #[arg(long)]
    pub force_uninstall: bool,

    /// Print version information
    #[arg(short = 'V', long = "version")]
    pub version: bool,

    /// Show brief usage message
    #[arg(long = "usage")]
    pub usage: bool,

    /// Force human mode (interactive output, friendly messages)
    #[arg(long, help = "Force human mode with interactive output and friendly messages")]
    pub human: bool,

    /// Force agent mode (structured output)
    #[arg(long, help = "Force agent mode with structured output optimized for AI consumption")]
    pub agent: bool,

    /// Output in TOON format (token-efficient for agents)
    #[arg(long, help = "Output in TOON format (token-efficient for agents)")]
    pub toon: bool,

    /// Output format: toon, json, or human
    #[arg(long, value_name = "FORMAT", help = "Output format: toon, json, or human")]
    pub format: Option<String>,

    /// Select specific output fields (comma-separated)
    #[arg(long, value_name = "FIELDS", help = "Select specific output fields (comma-separated)")]
    pub fields: Option<String>,

    /// Disable content truncation (show full output)
    #[arg(long, help = "Disable content truncation and show full output")]
    pub full: bool,

    /// Output session context in TOON format
    #[arg(long = "session-context", help = "Output session context in TOON format for agent consumption")]
    pub session_context: bool,

    /// Install agent hooks for Claude Code or Codex
    #[arg(long = "install-agent-hooks", help = "Install agent hooks for Claude Code or Codex")]
    pub install_agent_hooks: bool,

    /// Output operation metadata as JSON
    #[arg(long, help = "Output operation metadata as JSON")]
    pub json: bool,

    /// Color output: auto, always, never
    #[arg(long, value_name = "WHEN", help = "Color output: auto, always, never")]
    pub color: Option<String>,

    /// Decrease logging verbosity (suppress non-essential output)
    #[arg(short = 'q', long, help = "Decrease logging verbosity (suppress non-essential output)")]
    pub quiet: bool,

    /// Enable debug logging
    #[arg(long, help = "Enable debug logging")]
    pub debug: bool,

    /// Pre-launch daemon in background and wait for jobs
    #[arg(long, help = "Pre-launch daemon in background and wait for jobs")]
    pub daemon: bool,

    /// Force synchronous in-process operation (disable auto-spawning)
    #[arg(long, help = "Force synchronous in-process operation (disable auto-spawning)")]
    pub no_daemon: bool,

    /// Disable pager for long output
    #[arg(long, help = "Disable pager for long output")]
    pub no_pager: bool,

    /// Generate shell completion script for the specified shell (auto-detects if not specified)
    #[arg(long, value_name = "SHELL", num_args = 0..=1, default_missing_value = "auto", help = "Generate shell completion script for bash, zsh, or fish (auto-detects if not specified)")]
    pub generate_completion: Option<String>,

    /// Display man page content
    #[arg(long, help = "Display man page content")]
    pub man: bool,

    /// Launch TUI mode for interactive configuration
    #[arg(long, help = "Launch TUI mode for interactive configuration")]
    pub interactive_tui: bool,

    /// Launch TUI mode (alias for --interactive-tui)
    #[arg(long, help = "Launch TUI mode (alias for --interactive-tui)")]
    pub tui: bool,

    /// Subcommands
    #[command(subcommand)]
    pub command: Option<SmartfoCommand>,
}

/// Subcommands for the smartfo binary.
#[derive(Parser, Debug)]
pub enum SmartfoCommand {
    /// Git hook commands
    #[command(subcommand)]
    Git(GitCommand),
    /// Job management commands
    #[command(subcommand)]
    Job(JobCommand),
    /// Agent integration commands
    #[command(subcommand)]
    Agent(AgentCommand),
    /// Information and query commands
    #[command(subcommand)]
    Info(InfoCommand),
    /// Health check commands
    #[command(subcommand)]
    Health(HealthCommand),
}

/// Git hook subcommands
#[derive(Parser, Debug)]
pub enum GitCommand {
    /// Client-side pre-commit hook
    #[command(name = "hook-client", about = "Run client-side pre-commit hook to block raw deletions and renames")]
    HookClient,
    /// Server-side pre-receive hook
    #[command(name = "hook-server", about = "Run server-side pre-receive hook to block raw deletions and renames")]
    HookServer,
}

/// Job management subcommands
#[derive(Parser, Debug)]
pub enum JobCommand {
    /// List background jobs with optional filtering
    #[command(name = "list", about = "List background jobs with optional job ID filtering")]
    List {
        /// Optional job IDs to filter (comma-separated)
        #[arg(long, value_name = "IDS")]
        ids: Option<String>,
        /// Decrease logging verbosity (suppress non-essential output)
        #[arg(short = 'q', long, help = "Decrease logging verbosity (suppress non-essential output)")]
        quiet: bool,
        /// Enable debug logging
        #[arg(long, help = "Enable debug logging")]
        debug: bool,
    },
    /// Cancel a specific background job
    #[command(name = "cancel", about = "Cancel a specific background job by ID")]
    Cancel {
        /// Job ID to cancel
        #[arg(value_name = "ID")]
        job_id: String,
        /// Decrease logging verbosity (suppress non-essential output)
        #[arg(short = 'q', long, help = "Decrease logging verbosity (suppress non-essential output)")]
        quiet: bool,
        /// Enable debug logging
        #[arg(long, help = "Enable debug logging")]
        debug: bool,
    },
}

/// Agent integration subcommands
#[derive(Parser, Debug)]
pub enum AgentCommand {
    /// Output session context in TOON format for agent consumption
    #[command(name = "session-context", about = "Output session context in TOON format for agent consumption. Includes current directory, git repo info, audit log path, and recent operations count.")]
    SessionContext,
    /// Install agent hooks for Claude Code or Codex
    #[command(name = "install-hooks", about = "Install agent hooks for Claude Code or Codex. Registers session-start and session-end hooks for ambient context injection.")]
    InstallHooks,
    /// Generate agent skill from CLI metadata
    #[command(name = "generate-skill", about = "Generate agent skill (SKILL.md) from CLI metadata. Outputs static skill content with trigger-shaped frontmatter and non-interactive examples.")]
    GenerateSkill {
        /// Output file path (default: stdout)
        #[arg(long, value_name = "PATH")]
        output: Option<std::path::PathBuf>,
    },
    /// Check if generated skill is stale
    #[command(name = "check-skill", about = "Check if generated skill is stale compared to current version. Exits with error if skill needs regeneration.")]
    CheckSkill {
        /// Skill file path to check (default: SKILL.md)
        #[arg(long, value_name = "PATH")]
        skill_file: Option<std::path::PathBuf>,
    },
}

/// Information and query subcommands
#[derive(Parser, Debug)]
pub enum InfoCommand {
    /// List operations and queue status
    #[command(name = "list", about = "List operations and queue status with aggregate counts. Returns empty state with context when no results found. Includes contextual suggestions for next steps in TOON output.")]
    List {
        /// Show all items including completed
        #[arg(long)]
        all: bool,
        /// Limit number of items to show
        #[arg(long, value_name = "N")]
        limit: Option<usize>,
        /// Decrease logging verbosity (suppress non-essential output)
        #[arg(short = 'q', long, help = "Decrease logging verbosity (suppress non-essential output)")]
        quiet: bool,
        /// Enable debug logging
        #[arg(long, help = "Enable debug logging")]
        debug: bool,
    },
    /// Show daemon and queue status
    #[command(name = "status", about = "Show daemon and queue status with aggregate information. Returns empty state with context when no status data available. Includes contextual suggestions for next steps in TOON output.")]
    Status {
        /// Show detailed status
        #[arg(long)]
        detailed: bool,
        /// Decrease logging verbosity (suppress non-essential output)
        #[arg(short = 'q', long, help = "Decrease logging verbosity (suppress non-essential output)")]
        quiet: bool,
        /// Enable debug logging
        #[arg(long, help = "Enable debug logging")]
        debug: bool,
    },
}

/// Health check subcommands
#[derive(Parser, Debug)]
pub enum HealthCommand {
    /// Check daemon health status
    #[command(name = "check", about = "Check daemon health status. Exits with 0 if healthy, 1 if unhealthy. Supports both HTTP endpoint and signal-based checks.")]
    Check {
        /// Use HTTP endpoint for health check
        #[arg(long, help = "Use HTTP endpoint for health check (default: true)")]
        http: bool,
        /// Use signal-based health check (SIGUSR1)
        #[arg(long, help = "Use signal-based health check via SIGUSR1")]
        signal: bool,
        /// Decrease logging verbosity (suppress non-essential output)
        #[arg(short = 'q', long, help = "Decrease logging verbosity (suppress non-essential output)")]
        quiet: bool,
        /// Enable debug logging
        #[arg(long, help = "Enable debug logging")]
        debug: bool,
    },
}
