//! Man page generation and display module
//!
//! This module provides functionality for generating and displaying Unix man pages
//! for smartfo, mv, and rm modes. It supports both embedded man page content
//! and reading from external roff source files.

use anyhow::{Context, Result};
use std::path::PathBuf;

/// Man page types
#[derive(Debug, Clone, Copy)]
pub enum ManPageType {
    /// Main smartfo binary
    Smartfo,
    /// mv mode (smartfo-mv)
    Mv,
    /// rm mode (smartfo-rm)
    Rm,
}

impl ManPageType {
    /// Get the man page filename
    pub fn filename(&self) -> &str {
        match self {
            ManPageType::Smartfo => "smartfo.1",
            ManPageType::Mv => "smartfo-mv.1",
            ManPageType::Rm => "smartfo-rm.1",
        }
    }

    /// Get the man page name for display
    pub fn name(&self) -> &str {
        match self {
            ManPageType::Smartfo => "smartfo",
            ManPageType::Mv => "smartfo-mv",
            ManPageType::Rm => "smartfo-rm",
        }
    }
}

/// Generate man page content for the specified type
pub fn generate_man_page(man_type: ManPageType) -> Result<String> {
    match man_type {
        ManPageType::Smartfo => generate_smartfo_man_page(),
        ManPageType::Mv => generate_mv_man_page(),
        ManPageType::Rm => generate_rm_man_page(),
    }
}

/// Generate man page for smartfo (main binary)
fn generate_smartfo_man_page() -> Result<String> {
    let version = env!("CARGO_PKG_VERSION");
    let date = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let man_content = format!(
        r#".TH SMARTFO 1 "{}" "smartfo {}" "User Commands"
.SH NAME
smartfo \- VCS-aware safe mv/rm replacement with trash and audit
.SH SYNOPSIS
.B smartfo
[\fIOPTIONS\fR]
.br
.B smartfo
[\fIOPTIONS\fR] \fICOMMAND\fR [\fIARGS\fR]...
.SH DESCRIPTION
Smartfo is a drop-in replacement for POSIX mv and rm that provides:
.IP
VCS-aware file operations (Git, Mercurial, SVN, Jujutsu)
.IP
Trash instead of permanent deletion
.IP
Async background processing
.IP
Comprehensive audit logging
.IP
Git hooks to prevent data loss
.SH OPTIONS
.TP
\fB\-\-install\fR
Install symlinks and Git hooks
.TP
\fB\-\-uninstall\fR
Uninstall smartfo (remove symlinks, completions, man pages)
.TP
\fB\-\-force-uninstall\fR
Bypass confirmation prompts during uninstall
.TP
\fB\-\-init-config\fR
Initialize or recreate default config file with all settings commented out. Use \fB\-\-force\fR to overwrite an existing config file.
.TP
\fB\-\-hooks=TYPE\fR
Hook types to install: client, server, or client,server (default: both)
.TP
\fB\-\-no-hooks\fR
Skip hook installation
.TP
\fB\-\-force-hooks\fR
Overwrite existing hook files when installing
.TP
\fB\-\-force\fR
Overwrite existing files when installing
.TP
\fB\-\-man\fR
Display man page content
.TP
\fB\-\-version\fR, \fB\-V\fR
Print version information
.TP
\fB\-\-usage\fR
Show brief usage message
.TP
\fB\-\-help\fR, \fB\-h\fR
Show help message
.TP
\fB\-\-json\fR
Output operation metadata as JSON
.TP
\fB\-\-toon\fR
Output in TOON format (token-efficient for agents)
.TP
\fB\-\-format=FORMAT\fR
Output format: toon, json, or human
.TP
\fB\-\-fields=FIELDS\fR
Select specific output fields (comma-separated)
.TP
\fB\-\-full\fR
Disable content truncation (show full output)
.TP
\fB\-\-session-context\fR
Output session context in TOON format for agent consumption
.TP
\fB\-\-install-agent-hooks\fR
Install agent hooks for Claude Code or Codex
.TP
\fB\-\-color=WHEN\fR
Color output: auto, always, never
.TP
\fB\-q\fR, \fB\-\-quiet\fR
Decrease logging verbosity (suppress non-essential output)
.TP
\fB\-\-debug\fR
Enable debug logging
.TP
\fB\-\-daemon\fR
Pre-launch daemon in background and wait for jobs
.TP
\fB\-\-no-daemon\fR
Force synchronous in-process operation
.TP
\fB\-\-max-memory=MB\fR
Maximum memory limit in MB for daemon operations (0 = unlimited)
.TP
\fB\-\-max-cpu=PERCENT\fR
Maximum CPU usage as percentage for daemon operations (0 = unlimited)
.TP
\fB\-\-privacy\fR
Enable privacy mode for this operation (anonymize sensitive data in logs and output)
.TP
\fB\-\-interactive-tui\fR, \fB\-\-tui\fR
Launch TUI mode for interactive configuration
.TP
\fB\-\-no-pager\fR
Disable pager for long output
.TP
\fB\-\-generate-completion[=SHELL]\fR
Generate shell completion script for bash, zsh, or fish
.SH COMMANDS
.TP
\fBgit-hook-client\fR
Run client-side pre-commit hook
.TP
\fBgit-hook-server\fR
Run server-side pre-receive hook
.TP
\fBlist\fR
List operations and queue status
.TP
\fBstatus\fR
Show daemon and queue status
.TP
\fBsession-context\fR
Output session context in TOON format
.TP
\fBinstall-agent-hooks\fR
Install agent hooks for Claude Code or Codex
.TP
\fBgenerate-skill\fR
Generate agent skill from CLI metadata
.TP
\fBcheck-skill\fR
Check if generated skill is stale
.TP
\fBlist-jobs\fR
List background jobs
.TP
\fBcancel-job\fR
Cancel a specific background job
.SH EXAMPLES
.PP
Install smartfo:
.RS
.nf
smartfo --install
.fi
.RE
.PP
Uninstall smartfo:
.RS
.nf
smartfo --uninstall
.fi
.RE
.PP
Show status:
.RS
.nf
smartfo status
.fi
.RE
.PP
List operations:
.RS
.nf
smartfo list --all
.fi
.RE
.SH ENVIRONMENT
.TP
\fBSMARTFO_CONFIG_HOME\fR
Override config directory
.TP
\fBSMARTFO_TRASH_ROOT\fR
Override trash directory
.TP
\fBSMARTFO_PATHS_AUDIT_LOG\fR
Override audit log path
.TP
\fBSMARTFO_CONCURRENCY_MAX_CONCURRENT_JOBS\fR
Global parallel job ceiling
.TP
\fBXDG_DATA_HOME\fR
Base directory for data files (default: ~/.local/share)
.TP
\fBXDG_CACHE_HOME\fR
Base directory for cache files (default: ~/.cache)
.TP
\fBXDG_CONFIG_HOME\fR
Base directory for config files (default: ~/.config)
.TP
\fBXDG_BIN_HOME\fR
Preferred symlink target for --install
.SH FILES
.TP
\fI~/.config/smartfo/config.toml\fR
User configuration file
.TP
\fI$XDG_DATA_HOME/smartfo/trash/\fR
Trash directory
.TP
\fI$XDG_DATA_HOME/smartfo/audit/operations.jsonl\fR
Audit log
.TP
\fI$XDG_DATA_HOME/smartfo/queue.db\fR
Job queue database
.SH EXIT STATUS
.TP
0
Success
.TP
1
General error
.TP
2
Invalid command-line arguments
.TP
3
File not found
.TP
4
Permission denied
.TP
5
VCS operation failed
.TP
6
Configuration error
.TP
7
Daemon error
.SH SEE ALSO
.BR mv (1),
.BR rm (1),
.BR git (1),
.BR hg (1),
.BR svn (1),
.BR jj (1)
.SH AUTHOR
smartfo development team
"#,
        date, version
    );

    Ok(man_content)
}

/// Generate man page for mv mode
fn generate_mv_man_page() -> Result<String> {
    let version = env!("CARGO_PKG_VERSION");
    let date = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let man_content = format!(
        r#".TH SMARTFO-MV 1 "{}" "smartfo {}" "User Commands"
.SH NAME
smartfo-mv \- Move (rename) files with VCS awareness
.SH SYNOPSIS
.B mv
[\fIOPTIONS\fR]... \fISOURCE\fR... \fIDEST\fR
.br
.B smv
[\fIOPTIONS\fR]... \fISOURCE\fR... \fIDEST\fR
.SH DESCRIPTION
Move (rename) SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.
.PP
This is a drop-in replacement for POSIX mv that:
.IP
Uses VCS-native moves (git mv, hg mv, etc.) when possible
.IP
Supports async operations for large files
.IP
Provides comprehensive audit logging
.IP
Supports all standard POSIX flags
.IP
Supports glob patterns (*.txt, **/*.rs) for batch operations
.IP
Supports stdin input via - argument or piped input
.SH OPTIONS
.TP
\fB\-f\fR, \fB\-\-force\fR
Do not prompt before overwriting
.TP
\fB\-i\fR, \fB\-\-interactive\fR
Prompt before overwrite
.TP
\fB\-n\fR, \fB\-\-no-clobber\fR
Do not overwrite an existing file
.TP
\fB\-v\fR, \fB\-\-verbose\fR
Explain what is being done
.TP
\fB\-T\fR, \fB\-\-no-target-directory\fR
Treat DEST as a normal file
.TP
\fB\-t\fR, \fB\-\-target-directory=DIRECTORY\fR
Move all SOURCE arguments into DIRECTORY
.TP
\fB\-\-backup\fR
Make a backup of each existing destination file
.TP
\fB\-\-strip-trailing-slashes\fR
Remove trailing slashes from SOURCE arguments
.TP
\fB\-\-plain\fR
Disable all smart features; behave exactly like POSIX mv
.TP
\fB\-\-force-outside-vcs\fR
Allow moving tracked files outside repo
.TP
\fB\-\-async\fR
Force async move even for small/same-fs files
.TP
\fB\-\-blocking\fR
Wait for operation to complete
.TP
\fB\-\-sync\fR
Fsync destination file and directory after operation
.TP
\fB\-\-reason=REASON\fR
Annotate intent in the audit log
.TP
\fB\-\-json\fR
Output operation metadata as JSON
.TP
\fB\-\-toon\fR
Output in TOON format (token-efficient for agents)
.TP
\fB\-\-format=FORMAT\fR
Output format: toon, json, or human
.TP
\fB\-\-fields=FIELDS\fR
Select specific output fields (comma-separated)
.TP
\fB\-\-dry-run\fR
Preview operations without executing
.TP
\fB\-\-usage\fR
Show brief usage message
.TP
\fB\-\-color=WHEN\fR
Color output: auto, always, never
.TP
\fB\-q\fR, \fB\-\-quiet\fR
Decrease logging verbosity
.TP
\fB\-\-debug\fR
Enable debug logging
.TP
\fB\-\-interactive-tui\fR, \fB\-\-tui\fR
Launch TUI mode for interactive argument editing
.TP
\fB\-\-daemon\fR
Pre-launch daemon in background and wait for jobs
.TP
\fB\-\-no-daemon\fR
Force synchronous in-process operation
.TP
\fB\-\-max-memory=MB\fR
Maximum memory limit in MB for daemon operations (0 = unlimited)
.TP
\fB\-\-max-cpu=PERCENT\fR
Maximum CPU usage as percentage for daemon operations (0 = unlimited)
.TP
\fB\-\-privacy\fR
Enable privacy mode for this operation (anonymize sensitive data in logs and output)
.TP
\fB\-V\fR, \fB\-\-version\fR
Print version information
.TP
\fB\-h\fR, \fB\-\-help\fR
Show help message
.SH EXAMPLES
.PP
Rename file:
.RS
.nf
mv file1 file2
.fi
.RE
.PP
Move file into directory:
.RS
.nf
mv file1 dir/
.fi
.RE
.PP
Prompt before overwrite:
.RS
.nf
mv -i file1 file2
.fi
.RE
.PP
Async move for large file:
.RS
.nf
mv --async large.bin /mnt/backup/
.fi
.RE
.PP
Move all .txt files using glob:
.RS
.nf
mv *.txt /backup/
.fi
.RE
.PP
Move all .rs files recursively:
.RS
.nf
mv **/*.rs src/
.fi
.RE
.PP
Read paths from stdin:
.RS
.nf
echo -e 'file1.txt\\nfile2.txt' | mv - /dest/
.fi
.RE
.SH ENVIRONMENT
.TP
\fBSMARTFO_CONFIG_HOME\fR
Override config directory
.TP
\fBSMARTFO_TRASH_ROOT\fR
Override trash directory
.TP
\fBXDG_DATA_HOME\fR
Base directory for data files
.TP
\fBXDG_CACHE_HOME\fR
Base directory for cache files
.TP
\fBXDG_CONFIG_HOME\fR
Base directory for config files
.SH EXIT STATUS
.TP
0
Success
.TP
1
General error
.TP
2
Invalid command-line arguments
.TP
3
Source file not found
.TP
4
Permission denied
.TP
5
VCS operation failed
.SH SEE ALSO
.BR smartfo (1),
.BR smartfo-rm (1),
.BR git-mv (1),
.BR hg-mv (1)
.SH AUTHOR
smartfo development team
"#,
        date, version
    );

    Ok(man_content)
}

/// Generate man page for rm mode
fn generate_rm_man_page() -> Result<String> {
    let version = env!("CARGO_PKG_VERSION");
    let date = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let man_content = format!(
        r#".TH SMARTFO-RM 1 "{}" "smartfo {}" "User Commands"
.SH NAME
smartfo-rm \- Remove files with trash and VCS awareness
.SH SYNOPSIS
.B rm
[\fIOPTIONS\fR]... \fIFILE\fR...
.br
.B srm
[\fIOPTIONS\fR]... \fIFILE\fR...
.SH DESCRIPTION
Remove (unlink) the FILE(s).
.PP
This is a drop-in replacement for POSIX rm that:
.IP
Moves files to trash instead of permanent deletion
.IP
Uses VCS-aware removal for tracked files
.IP
Supports async operations
.IP
Provides comprehensive audit logging
.IP
Supports all standard POSIX flags
.IP
Supports glob patterns (*.txt, **/*.rs) for batch operations
.IP
Supports stdin input via - argument or piped input
.SH OPTIONS
.TP
\fB\-f\fR, \fB\-\-force\fR
Ignore non-existent files, never prompt
.TP
\fB\-i\fR
Prompt before every removal
.TP
\fB\-I\fR
Prompt once before removing more than three files
.TP
\fB\-r\fR, \fB\-R\fR, \fB\-\-recursive\fR
Remove directories and their contents recursively
.TP
\fB\-d\fR, \fB\-\-dir\fR
Remove empty directories
.TP
\fB\-\-preserve-root\fR
Do not remove '/' (default)
.TP
\fB\-\-one-filesystem\fR
Skip directories on different file systems
.TP
\fB\-\-plain\fR
Disable all smart features; behave exactly like POSIX rm
.TP
\fB\-\-force-delete\fR
Bypass trash and delete directly
.TP
\fB\-\-blocking\fR
Wait for operation to complete
.TP
\fB\-\-sync\fR
Fsync after operation
.TP
\fB\-\-reason=REASON\fR
Annotate intent in the audit log
.TP
\fB\-\-json\fR
Output operation metadata as JSON
.TP
\fB\-\-toon\fR
Output in TOON format (token-efficient for agents)
.TP
\fB\-\-format=FORMAT\fR
Output format: toon, json, or human
.TP
\fB\-\-fields=FIELDS\fR
Select specific output fields (comma-separated)
.TP
\fB\-\-dry-run\fR
Preview operations without executing
.TP
\fB\-\-usage\fR
Show brief usage message
.TP
\fB\-\-color=WHEN\fR
Color output: auto, always, never
.TP
\fB\-q\fR, \fB\-\-quiet\fR
Decrease logging verbosity
.TP
\fB\-\-debug\fR
Enable debug logging
.TP
\fB\-\-interactive-tui\fR, \fB\-\-tui\fR
Launch TUI mode for interactive argument editing
.TP
\fB\-\-daemon\fR
Pre-launch daemon in background and wait for jobs
.TP
\fB\-\-no-daemon\fR
Force synchronous in-process operation
.TP
\fB\-\-max-memory=MB\fR
Maximum memory limit in MB for daemon operations (0 = unlimited)
.TP
\fB\-\-max-cpu=PERCENT\fR
Maximum CPU usage as percentage for daemon operations (0 = unlimited)
.TP
\fB\-\-privacy\fR
Enable privacy mode for this operation (anonymize sensitive data in logs and output)
.TP
\fB\-V\fR, \fB\-\-version\fR
Print version information
.TP
\fB\-h\fR, \fB\-\-help\fR
Show help message
.SH EXAMPLES
.PP
Move file to trash:
.RS
.nf
rm file.txt
.fi
.RE
.PP
Recursively remove directory:
.RS
.nf
rm -rf dir/
.fi
.RE
.PP
Bypass trash and delete directly:
.RS
.nf
rm --force-delete file
.fi
.RE
.PP
Async deletion for large file:
.RS
.nf
rm --async large.bin
.fi
.RE
.PP
Remove all .log files using glob:
.RS
.nf
rm *.log
.fi
.RE
.PP
Remove all .tmp files recursively:
.RS
.nf
rm **/*.tmp
.fi
.RE
.PP
Read paths from stdin:
.RS
.nf
echo -e 'file1.txt\\nfile2.txt' | rm -
.fi
.RE
.SH ENVIRONMENT
.TP
\fBSMARTFO_CONFIG_HOME\fR
Override config directory
.TP
\fBSMARTFO_TRASH_ROOT\fR
Override trash directory
.TP
\fBXDG_DATA_HOME\fR
Base directory for data files
.TP
\fBXDG_CACHE_HOME\fR
Base directory for cache files
.TP
\fBXDG_CONFIG_HOME\fR
Base directory for config files
.SH EXIT STATUS
.TP
0
Success
.TP
1
General error
.TP
2
Invalid command-line arguments
.TP
3
File not found
.TP
4
Permission denied
.TP
5
VCS operation failed
.SH SEE ALSO
.BR smartfo (1),
.BR smartfo-mv (1),
.BR git-rm (1),
.BR hg-rm (1)
.SH AUTHOR
smartfo development team
"#,
        date, version
    );

    Ok(man_content)
}

/// Get the man directory for installation
pub fn get_man_dir() -> Result<PathBuf> {
    Ok(PathBuf::from("/usr/local/share/man"))
}

/// Install man pages to the system man directory
pub fn install_man_pages(man_dir: &PathBuf) -> Result<()> {
    use std::fs;
    use tracing::{info, debug};

    info!("Installing man pages to {}", man_dir.display());

    // Create man directory structure
    let man1_dir = man_dir.join("man1");
    fs::create_dir_all(&man1_dir)
        .context("Failed to create man1 directory")?;

    // Install smartfo man page
    let smartfo_man = generate_man_page(ManPageType::Smartfo)?;
    let smartfo_path = man1_dir.join(ManPageType::Smartfo.filename());
    fs::write(&smartfo_path, smartfo_man)
        .context("Failed to write smartfo man page")?;
    debug!("Installed smartfo man page to {}", smartfo_path.display());

    // Install mv man page
    let mv_man = generate_man_page(ManPageType::Mv)?;
    let mv_path = man1_dir.join(ManPageType::Mv.filename());
    fs::write(&mv_path, mv_man)
        .context("Failed to write mv man page")?;
    debug!("Installed mv man page to {}", mv_path.display());

    // Install rm man page
    let rm_man = generate_man_page(ManPageType::Rm)?;
    let rm_path = man1_dir.join(ManPageType::Rm.filename());
    fs::write(&rm_path, rm_man)
        .context("Failed to write rm man page")?;
    debug!("Installed rm man page to {}", rm_path.display());

    info!("Man pages installed successfully");
    Ok(())
}

/// Remove man pages from the system man directory
pub fn remove_man_pages(man_dir: &PathBuf) -> Result<()> {
    use std::fs;
    use tracing::{info, debug};

    info!("Removing man pages");

    let man1_dir = man_dir.join("man1");

    for man_type in [ManPageType::Smartfo, ManPageType::Mv, ManPageType::Rm] {
        let man_path = man1_dir.join(man_type.filename());
        if man_path.exists() {
            fs::remove_file(&man_path)
                .with_context(|| format!("Failed to remove man page {}", man_path.display()))?;
            debug!("Removed man page from {}", man_path.display());
        }
    }

    info!("Man pages removed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_man_page_generation() {
        let smartfo_man = generate_man_page(ManPageType::Smartfo).unwrap();
        assert!(!smartfo_man.is_empty());
        assert!(smartfo_man.contains("smartfo"));
        assert!(smartfo_man.contains("VCS-aware"));
        assert!(smartfo_man.contains("SYNOPSIS"));
        assert!(smartfo_man.contains("OPTIONS"));
        assert!(smartfo_man.contains("EXAMPLES"));
        assert!(smartfo_man.contains("ENVIRONMENT"));
        assert!(smartfo_man.contains("EXIT STATUS"));
    }

    #[test]
    fn test_mv_man_page_generation() {
        let mv_man = generate_man_page(ManPageType::Mv).unwrap();
        assert!(!mv_man.is_empty());
        assert!(mv_man.contains("smartfo-mv"));
        assert!(mv_man.contains("Move (rename)"));
        assert!(mv_man.contains("force-outside-vcs"));
        assert!(mv_man.contains("async"));
    }

    #[test]
    fn test_rm_man_page_generation() {
        let rm_man = generate_man_page(ManPageType::Rm).unwrap();
        assert!(!rm_man.is_empty());
        assert!(rm_man.contains("smartfo-rm"));
        assert!(rm_man.contains("Remove files"));
        assert!(rm_man.contains("force-delete"));
        assert!(rm_man.contains("recursive"));
    }

    #[test]
    fn test_man_page_type_filename() {
        assert_eq!(ManPageType::Smartfo.filename(), "smartfo.1");
        assert_eq!(ManPageType::Mv.filename(), "smartfo-mv.1");
        assert_eq!(ManPageType::Rm.filename(), "smartfo-rm.1");
    }

    #[test]
    fn test_man_page_type_name() {
        assert_eq!(ManPageType::Smartfo.name(), "smartfo");
        assert_eq!(ManPageType::Mv.name(), "smartfo-mv");
        assert_eq!(ManPageType::Rm.name(), "smartfo-rm");
    }
}
