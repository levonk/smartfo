use clap::Parser;
use std::path::PathBuf;

/// Arguments for mv mode (invoked via `mv` or `smv` symlink).
#[derive(Parser, Debug, Clone)]
#[command(
    name = "mv",
    about = "Move (rename) files with VCS awareness",
    version
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
}

/// Arguments for rm mode (invoked via `rm` or `srm` symlink).
#[derive(Parser, Debug, Clone)]
#[command(
    name = "rm",
    about = "Remove files with trash and VCS awareness",
    version
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

    /// File(s) or directories to remove
    #[arg(value_name = "FILE")]
    pub paths: Vec<PathBuf>,
}

/// Arguments for the main smartfo binary.
#[derive(Parser, Debug)]
#[command(
    name = "smartfo",
    about = "VCS-aware safe mv/rm replacement with trash and audit",
    version
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
}
