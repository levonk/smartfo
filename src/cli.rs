use clap::Parser;
use std::path::PathBuf;

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

Examples:
  mv file1 file2           # Rename file1 to file2
  mv file1 dir/            # Move file1 into dir/
  mv -i file1 file2        # Prompt before overwrite
  mv --async large.bin /mnt/backup/  # Async move for large file",
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

    /// Source file(s) to move
    #[arg(value_name = "SOURCE")]
    pub sources: Vec<PathBuf>,
}

impl MvArgs {
    /// Returns the destination path and source paths.
    /// When `-t DIRECTORY` is used, all positional args are sources and dest is the directory.
    /// Otherwise, the last positional arg is the destination and the rest are sources.
    pub fn resolve_paths(&self) -> (Vec<PathBuf>, Option<PathBuf>) {
        if let Some(ref dir) = self.target_directory {
            return (self.sources.clone(), Some(dir.clone()));
        }
        if self.sources.len() >= 2 {
            let dest = self.sources.last().cloned();
            let sources = self.sources[..self.sources.len() - 1].to_vec();
            return (sources, dest);
        }
        (self.sources.clone(), None)
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
        
        // Validate sources are provided
        if self.sources.is_empty() {
            return Err("Missing source file(s)".to_string());
        }
        
        // Validate destination or target directory is provided
        if self.target_directory.is_none() && self.sources.len() < 2 {
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

Examples:
  rm file.txt             # Move file.txt to trash
  rm -rf dir/             # Recursively remove directory
  rm --force-delete file  # Bypass trash and delete directly
  rm --async large.bin     # Async deletion for large file",
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

    /// File(s) or directories to remove
    #[arg(value_name = "FILE")]
    pub paths: Vec<PathBuf>,
}

impl RmArgs {
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
        
        // Validate paths are provided
        if self.paths.is_empty() {
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

    /// Overwrite existing files when installing
    #[arg(long)]
    pub force: bool,

    /// Initialize or recreate default config file
    #[arg(long = "init-config")]
    pub init_config: bool,

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

    /// Subcommands
    #[command(subcommand)]
    pub command: Option<SmartfoCommand>,
}

/// Subcommands for the smartfo binary.
#[derive(Parser, Debug)]
pub enum SmartfoCommand {
    /// Client-side pre-commit hook
    #[command(name = "git-hook-client")]
    GitHookClient,
    /// Server-side pre-receive hook
    #[command(name = "git-hook-server")]
    GitHookServer,
    /// List operations and queue status
    #[command(name = "list", about = "List operations and queue status with aggregate counts. Returns empty state with context when no results found.")]
    List {
        /// Show all items including completed
        #[arg(long)]
        all: bool,
        /// Limit number of items to show
        #[arg(long, value_name = "N")]
        limit: Option<usize>,
    },
    /// Show daemon and queue status
    #[command(name = "status", about = "Show daemon and queue status with aggregate information. Returns empty state with context when no status data available.")]
    Status {
        /// Show detailed status
        #[arg(long)]
        detailed: bool,
    },
    /// Output session context in TOON format for agent consumption
    #[command(name = "session-context", about = "Output session context in TOON format for agent consumption. Includes current directory, git repo info, audit log path, and recent operations count.")]
    SessionContext,
    /// Install agent hooks for Claude Code or Codex
    #[command(name = "install-agent-hooks", about = "Install agent hooks for Claude Code or Codex. Registers session-start and session-end hooks for ambient context injection.")]
    InstallAgentHooks,
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
