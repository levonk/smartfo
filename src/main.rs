use clap::Parser;
use anyhow::{Result, Context};
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter, prelude::*};
use std::path::PathBuf;

mod cli;
mod config;
mod vcs;
mod mv;
mod rm;
mod audit;
mod trash;
mod queue;
mod daemon;
mod worker;
mod hooks;
use cli::{MvArgs, RmArgs, SmartfoArgs, SmartfoCommand};

/// Resolve the symlink target directory based on XDG conventions and permissions.
/// Priority: $XDG_BIN_HOME > ~/.local/bin (create if missing) > /usr/local/bin (if root)
fn resolve_symlink_target_dir() -> Result<PathBuf> {
    // Check XDG_BIN_HOME first
    if let Ok(xdg_bin) = std::env::var("XDG_BIN_HOME") {
        let path = PathBuf::from(xdg_bin);
        if path.exists() || path.parent().map_or(false, |p| p.exists()) {
            return Ok(path);
        }
    }

    // Try ~/.local/bin (create if it doesn't exist)
    let home = std::env::var("HOME")
        .map_err(|_| anyhow::anyhow!("HOME environment variable not set"))?;
    let local_bin = PathBuf::from(home).join(".local/bin");

    if !local_bin.exists() {
        if let Some(parent) = local_bin.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::create_dir(&local_bin)?;
        info!("Created directory: {}", local_bin.display());
    }

    Ok(local_bin)

    // Note: /usr/local/bin for root is handled by the caller checking permissions
}

/// Determine the invocation mode from argv[0].
fn detect_mode() -> String {
    std::env::args()
        .next()
        .and_then(|s| {
            std::path::Path::new(&s)
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "smartfo".to_string())
}

fn init_logging(json: bool, verbose: u8, quiet: bool) -> Result<()> {
    let log_level = match verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };

    let filter_str = if quiet {
        "error"
    } else {
        log_level
    };

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(filter_str));

    let subscriber = tracing_subscriber::registry().with(env_filter);

    if json {
        let json_layer = fmt::layer()
            .with_writer(std::io::stderr)
            .json()
            .with_target(true)
            .with_level(true);
        subscriber.with(json_layer).init();
    } else {
        let console_layer = fmt::layer()
            .with_writer(std::io::stderr)
            .with_ansi(std::env::var("NO_COLOR").is_err())
            .with_target(true)
            .with_level(true);
        subscriber.with(console_layer).init();
    }

    Ok(())
}

fn run_mv(args: MvArgs) -> Result<()> {
    // Handle --version flag
    if args.version {
        info!("--version flag triggered for mv mode");
        println!("smartfo mv {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Handle --usage flag
    if args.usage {
        info!("--usage flag triggered for mv mode");
        println!("Usage: mv [OPTION]... SOURCE... DEST");
        println!("Move (rename) SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.");
        println!();
        println!("Options:");
        println!("  -f, --force              Do not prompt before overwriting");
        println!("  -i, --interactive        Prompt before overwrite");
        println!("  -n, --no-clobber         Do not overwrite an existing file");
        println!("  -v, --verbose            Explain what is being done");
        println!("  -T, --no-target-directory  Treat DEST as a normal file");
        println!("  -t, --target-directory=DIRECTORY  Move all SOURCE arguments into DIRECTORY");
        println!("      --backup             Make a backup of each existing destination file");
        println!("      --strip-trailing-slashes  Remove trailing slashes from SOURCE arguments");
        println!("      --plain              Disable all smart features; behave exactly like POSIX mv");
        println!("      --force-outside-vcs  Allow moving tracked files outside repo");
        println!("      --async              Force async move even for small/same-fs files");
        println!("      --blocking           Wait for operation to complete");
        println!("      --sync               Fsync destination file and directory after operation");
        println!("      --reason=REASON      Annotate intent in the audit log");
        println!("      --json               Output operation metadata as JSON");
        println!("      --dry-run            Preview operations without executing");
        println!("      --usage              Show brief usage message");
        println!("  -h, --help               Show this help message");
        println!("  -V, --version            Print version information");
        return Ok(());
    }

    if args.dry_run {
        let (sources, dest) = args.resolve_paths();
        info!("dry-run: mv {:?} -> {:?}", sources, dest);
        return Ok(());
    }

    // TODO: Implement VCS-aware move logic (story 03-001)
    let (sources, dest) = args.resolve_paths();
    if sources.is_empty() {
        anyhow::bail!("missing file operand");
    }
    if dest.is_none() && args.target_directory.is_none() {
        let last = sources.last().unwrap().display();
        anyhow::bail!("missing destination file operand after {}", last);
    }

    info!("mv mode: sources={:?} dest={:?}", sources, dest);
    Ok(())
}

fn run_rm(args: RmArgs) -> Result<()> {
    // Handle --version flag
    if args.version {
        info!("--version flag triggered for rm mode");
        println!("smartfo rm {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Handle --usage flag
    if args.usage {
        info!("--usage flag triggered for rm mode");
        println!("Usage: rm [OPTION]... FILE...");
        println!("Remove (unlink) the FILE(s).");
        println!();
        println!("Options:");
        println!("  -f, --force              Ignore non-existent files, never prompt");
        println!("  -i                      Prompt before every removal");
        println!("  -I                      Prompt once before removing more than three files");
        println!("  -r, -R, --recursive     Remove directories and their contents recursively");
        println!("  -d, --dir               Remove empty directories");
        println!("      --preserve-root      Do not remove '/' (default)");
        println!("      --one-filesystem     Skip directories on different file systems");
        println!("      --plain              Disable all smart features; behave exactly like POSIX rm");
        println!("      --force-delete       Bypass trash and delete directly");
        println!("      --blocking           Wait for operation to complete");
        println!("      --sync               Fsync after operation");
        println!("      --reason=REASON      Annotate intent in the audit log");
        println!("      --json               Output operation metadata as JSON");
        println!("      --dry-run            Preview operations without executing");
        println!("      --usage              Show brief usage message");
        println!("  -h, --help               Show this help message");
        println!("  -V, --version            Print version information");
        return Ok(());
    }

    if args.dry_run {
        info!("dry-run: rm {:?}", args.paths);
        return Ok(());
    }

    if args.paths.is_empty() {
        anyhow::bail!("missing operand");
    }

    // Handle --plain mode (exact POSIX behavior, no smart features)
    if args.plain {
        return run_rm_plain(&args);
    }

    // Smart rm mode with VCS awareness and trash
    // TODO: Implement full smart rm logic (story 03-002)
    info!("rm mode: paths={:?}", args.paths);
    Ok(())
}

/// Run rm in plain POSIX mode (bypass all smart features)
fn run_rm_plain(args: &RmArgs) -> Result<()> {
    for path in &args.paths {
        if args.recursive || args.dir {
            // Remove directory recursively
            if path.is_dir() {
                std::fs::remove_dir_all(path)
                    .with_context(|| format!("Failed to remove directory: {}", path.display()))?;
            } else {
                std::fs::remove_file(path)
                    .with_context(|| format!("Failed to remove file: {}", path.display()))?;
            }
        } else {
            // Remove file only (POSIX rm default)
            std::fs::remove_file(path)
                .with_context(|| format!("Failed to remove file: {}", path.display()))?;
        }
    }
    Ok(())
}

fn run_install(args: &SmartfoArgs) -> Result<()> {
    // Handle --version flag
    if args.version {
        info!("--version flag triggered for install mode");
        println!("smartfo {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Handle --usage flag
    if args.usage {
        info!("--usage flag triggered for install mode");
        println!("Usage: smartfo [OPTIONS]");
        println!();
        println!("Options:");
        println!("      --install             Install symlinks and Git hooks");
        println!("      --hooks=TYPE          Hook types to install: client, server, or client,server");
        println!("      --no-hooks            Skip hook installation");
        println!("      --force               Overwrite existing files when installing");
        println!("      --usage               Show brief usage message");
        println!("  -h, --help               Show this help message");
        println!("  -V, --version            Print version information");
        println!();
        println!("Subcommands:");
        println!("  git-hook-client          Run client-side pre-commit hook");
        println!("  git-hook-server          Run server-side pre-receive hook");
        return Ok(());
    }

    info!("install mode: hooks={:?} no_hooks={} force={}", args.hooks, args.no_hooks, args.force);

    // Resolve symlink target directory
    let target_dir = if nix::unistd::Uid::effective().is_root() {
        // If running as root, try /usr/local/bin
        PathBuf::from("/usr/local/bin")
    } else {
        resolve_symlink_target_dir()?
    };

    info!("Symlink target directory: {}", target_dir.display());

    // Get the current executable path
    let current_exe = std::env::current_exe()
        .map_err(|_| anyhow::anyhow!("Failed to determine current executable path"))?;

    // Create symlinks for mv, rm, smv, srm
    let symlink_names = ["mv", "rm", "smv", "srm"];
    for name in &symlink_names {
        let symlink_path = target_dir.join(name);
        create_symlink(&current_exe, &symlink_path, args.force)?;
        info!("Created symlink: {} -> {}", symlink_path.display(), current_exe.display());
    }

    // Handle hook installation
    if !args.no_hooks {
        // Detect if we're inside a Git repository
        if let Some(repo_root) = detect_git_repo() {
            info!("Detected Git repository at: {}", repo_root.display());
            install_hooks(&repo_root, args)?;
        } else {
            info!("Not inside a Git repository, skipping hook installation");
        }
    } else {
        info!("Hook installation skipped due to --no-hooks flag");
    }

    Ok(())
}

/// Detect if the current directory is inside a Git repository
fn detect_git_repo() -> Option<PathBuf> {
    let current_dir = std::env::current_dir().ok()?;
    let mut dir = current_dir.as_path();

    loop {
        let git_dir = dir.join(".git");
        if git_dir.exists() {
            return Some(dir.to_path_buf());
        }

        match dir.parent() {
            Some(parent) => dir = parent,
            None => return None,
        }
    }
}

/// Install Git hooks based on the specified options
fn install_hooks(repo_root: &PathBuf, args: &SmartfoArgs) -> Result<()> {
    let hooks_dir = repo_root.join(".git/hooks");

    // Ensure hooks directory exists
    if !hooks_dir.exists() {
        std::fs::create_dir_all(&hooks_dir)?;
        info!("Created hooks directory: {}", hooks_dir.display());
    }

    // Determine which hooks to install
    let install_client = match &args.hooks {
        Some(hook_types) => {
            let types: Vec<&str> = hook_types.split(',').map(|s| s.trim()).collect();
            types.contains(&"client") || types.contains(&"client,server") || types.contains(&"server,client")
        }
        None => true, // Default: install both
    };

    let install_server = match &args.hooks {
        Some(hook_types) => {
            let types: Vec<&str> = hook_types.split(',').map(|s| s.trim()).collect();
            types.contains(&"server") || types.contains(&"client,server") || types.contains(&"server,client")
        }
        None => true, // Default: install both
    };

    // TODO: Implement actual hook installation (story 05-001)
    if install_client {
        info!("Would install pre-commit hook (implementation in story 05-001)");
    }
    if install_server {
        info!("Would install pre-receive hook (implementation in story 05-001)");
    }

    Ok(())
}

/// Create a symlink from source to target, handling existing files
fn create_symlink(source: &PathBuf, target: &PathBuf, force: bool) -> Result<()> {
    if target.exists() {
        // Check if existing symlink points to the same target (smartfo binary)
        if let Ok(link_target) = std::fs::read_link(target) {
            if link_target == *source {
                info!("Symlink already exists and points to smartfo: {}", target.display());
                return Ok(());
            }
            // Existing symlink points to something else
            if !force {
                anyhow::bail!(
                    "Existing symlink at {} points to {:?}, not smartfo binary. Use --force to overwrite.",
                    target.display(),
                    link_target
                );
            }
        } else {
            // Existing file is not a symlink (regular file, directory, etc.)
            if !force {
                anyhow::bail!(
                    "Non-smartfo file exists at {}: use --force to overwrite",
                    target.display()
                );
            }
        }

        // Remove existing file/symlink (force is true at this point)
        std::fs::remove_file(target)?;
        info!("Removed existing file: {}", target.display());
    }

    std::os::unix::fs::symlink(source, target)?;
    Ok(())
}

fn run_git_hook_client() -> Result<()> {
    // Detect the Git repository root
    let repo_root = detect_git_repo()
        .ok_or_else(|| anyhow::anyhow!("Not inside a Git repository"))?;

    hooks::run_pre_commit_hook(&repo_root)
}

fn run_git_hook_server() -> Result<()> {
    // Detect the Git repository root
    let repo_root = detect_git_repo()
        .ok_or_else(|| anyhow::anyhow!("Not inside a Git repository"))?;

    hooks::run_pre_receive_hook(&repo_root)
}

fn main() -> Result<()> {
    let mode = detect_mode();

    match mode.as_str() {
        "mv" | "smv" => {
            let args = MvArgs::parse();
            init_logging(args.json, if args.verbose { 1 } else { 0 }, false)?;
            run_mv(args)
        }
        "rm" | "srm" => {
            let args = RmArgs::parse();
            init_logging(args.json, 0, false)?;
            run_rm(args)
        }
        "smartfo" | _ => {
            let args = SmartfoArgs::parse();
            init_logging(false, 0, false)?;

            // Handle --version flag at root level
            if args.version {
                info!("--version flag triggered for smartfo");
                println!("smartfo {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }

            // Handle --usage flag at root level
            if args.usage {
                info!("--usage flag triggered for smartfo");
                println!("Usage: smartfo [OPTIONS]");
                println!();
                println!("Options:");
                println!("      --install             Install symlinks and Git hooks");
                println!("      --hooks=TYPE          Hook types to install: client, server, or client,server");
                println!("      --no-hooks            Skip hook installation");
                println!("      --force               Overwrite existing files when installing");
                println!("      --usage               Show brief usage message");
                println!("  -h, --help               Show this help message");
                println!("  -V, --version            Print version information");
                println!();
                println!("Subcommands:");
                println!("  git-hook-client          Run client-side pre-commit hook");
                println!("  git-hook-server          Run server-side pre-receive hook");
                return Ok(());
            }

            if let Some(cmd) = &args.command {
                match cmd {
                    SmartfoCommand::GitHookClient => run_git_hook_client(),
                    SmartfoCommand::GitHookServer => run_git_hook_server(),
                }
            } else if args.install {
                run_install(&args)
            } else {
                // No subcommand or install flag: print help
                use clap::CommandFactory;
                SmartfoArgs::command().print_help()?;
                println!();
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_resolve_symlink_target_dir_with_xdg_bin_home() {
        let temp_dir = TempDir::new().unwrap();
        let xdg_bin = temp_dir.path().join("bin");
        fs::create_dir_all(&xdg_bin).unwrap();

        std::env::set_var("XDG_BIN_HOME", xdg_bin.to_str().unwrap());
        let result = resolve_symlink_target_dir().unwrap();
        std::env::remove_var("XDG_BIN_HOME");

        assert_eq!(result, xdg_bin);
    }

    #[test]
    fn test_resolve_symlink_target_dir_creates_local_bin() {
        let temp_dir = TempDir::new().unwrap();
        let home = temp_dir.path();
        std::env::set_var("HOME", home.to_str().unwrap());
        std::env::remove_var("XDG_BIN_HOME");

        let result = resolve_symlink_target_dir().unwrap();
        std::env::remove_var("HOME");

        let expected_local_bin = home.join(".local/bin");
        assert_eq!(result, expected_local_bin);
        assert!(expected_local_bin.exists());
    }

    #[test]
    fn test_create_symlink_new() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("source");
        let target = temp_dir.path().join("target");

        fs::write(&source, "test").unwrap();
        create_symlink(&source, &target, false).unwrap();

        assert!(target.exists());
        assert_eq!(fs::read_link(&target).unwrap(), source);
    }

    #[test]
    fn test_create_symlink_existing_same_target() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("source");
        let target = temp_dir.path().join("target");

        fs::write(&source, "test").unwrap();
        create_symlink(&source, &target, false).unwrap();

        // Should succeed without error when symlink already points to correct target
        create_symlink(&source, &target, false).unwrap();
    }

    #[test]
    fn test_create_symlink_existing_different_target_no_force() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("source");
        let other_source = temp_dir.path().join("other");
        let target = temp_dir.path().join("target");

        fs::write(&source, "test").unwrap();
        fs::write(&other_source, "other").unwrap();
        create_symlink(&other_source, &target, false).unwrap();

        // Should fail without force when symlink points to different target
        let result = create_symlink(&source, &target, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_symlink_existing_different_target_with_force() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("source");
        let other_source = temp_dir.path().join("other");
        let target = temp_dir.path().join("target");

        fs::write(&source, "test").unwrap();
        fs::write(&other_source, "other").unwrap();
        create_symlink(&other_source, &target, false).unwrap();

        // Should succeed with force when symlink points to different target
        create_symlink(&source, &target, true).unwrap();
        assert_eq!(fs::read_link(&target).unwrap(), source);
    }

    #[test]
    fn test_create_symlink_existing_regular_file_no_force() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("source");
        let target = temp_dir.path().join("target");

        fs::write(&source, "test").unwrap();
        fs::write(&target, "regular file").unwrap();

        // Should fail without force when regular file exists
        let result = create_symlink(&source, &target, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_detect_git_repo() {
        let temp_dir = TempDir::new().unwrap();
        let repo = temp_dir.path();

        // Not a git repo
        std::env::set_current_dir(repo).unwrap();
        assert!(detect_git_repo().is_none());

        // Create .git directory
        let git_dir = repo.join(".git");
        fs::create_dir(&git_dir).unwrap();

        // Now it should be detected as a git repo
        let detected = detect_git_repo();
        assert!(detected.is_some());
        // Compare paths canonically to handle macOS path normalization
        assert_eq!(
            detected.unwrap().canonicalize().unwrap(),
            repo.canonicalize().unwrap()
        );
    }

    #[test]
    fn test_detect_git_repo_nested() {
        let temp_dir = TempDir::new().unwrap();
        let repo = temp_dir.path();
        let git_dir = repo.join(".git");
        fs::create_dir(&git_dir).unwrap();

        let nested_dir = repo.join("nested/deep");
        fs::create_dir_all(&nested_dir).unwrap();

        std::env::set_current_dir(&nested_dir).unwrap();
        let detected = detect_git_repo();
        assert!(detected.is_some());
        // Compare paths canonically to handle macOS path normalization
        assert_eq!(
            detected.unwrap().canonicalize().unwrap(),
            repo.canonicalize().unwrap()
        );
    }
}
