use clap::Parser;
use anyhow::{Result, Context};
use tracing::info;
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
mod git_hooks;
mod hooks;
mod install;
mod output;
mod logging;
mod error;
mod skill;
mod globbing;
use cli::{MvArgs, RmArgs, SmartfoArgs, SmartfoCommand};
use output::OutputFormat;
use output::schema::{SchemaRegistry, FieldSelector};
use output::aggregates::*;
use output::empty::{EmptyState, check_empty, EmptyContext};
use output::suggestions::{SuggestionContext, SuggestionEngine, format_suggestions_as_help};

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

/// Determine output format based on CLI flags and mode
fn determine_output_format(
    toon_flag: bool,
    format_flag: &Option<String>,
    agent_flag: bool,
    human_flag: bool,
) -> OutputFormat {
    // Explicit --toon flag takes precedence
    if toon_flag {
        return OutputFormat::Toon;
    }
    
    // Explicit --format flag
    if let Some(format) = format_flag {
        if let Some(parsed) = OutputFormat::parse(format) {
            return parsed;
        }
    }
    
    // Use mode-based defaults
    let output_mode = config::OutputMode::determine_mode(agent_flag, human_flag, config::OutputMode::Auto);
    match output_mode {
        config::OutputMode::Agent => OutputFormat::Toon,
        config::OutputMode::Human => OutputFormat::Human,
        config::OutputMode::Auto => {
            // Auto mode: use TOON if in agent session, otherwise human
            if config::OutputMode::detect_agent_session() {
                OutputFormat::Toon
            } else {
                OutputFormat::Human
            }
        }
    }
}

/// Determine field selector based on CLI flags and schema
fn determine_field_selector(
    fields_flag: &Option<String>,
    schema_name: &str,
) -> Option<FieldSelector> {
    let registry = SchemaRegistry::new();
    let schema = registry.get_or_default_schema(schema_name);
    
    if let Some(fields_str) = fields_flag {
        match FieldSelector::from_string(fields_str, schema) {
            Ok(selector) => Some(selector),
            Err(e) => {
                eprintln!("Error parsing fields: {}", e);
                eprintln!("Available fields: {}", 
                    schema.get_available_fields()
                        .iter()
                        .map(|f| f.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                Some(FieldSelector::from_schema(schema))
            }
        }
    } else {
        // Use default fields from schema
        Some(FieldSelector::from_schema(schema))
    }
}

fn setup_logging(debug: bool, quiet: bool, json: bool, color: Option<&str>) -> Result<()> {
    // Use the logging module's init_logging function
    let log_format = if json { Some("json") } else { None };
    let color_mode = config::ColorMode::determine(color, "auto");
    let use_ansi = color_mode.should_color();
    
    // Convert color mode to format string for logging module
    let format_str = if json { 
        Some("json") 
    } else if use_ansi {
        Some("pretty")
    } else {
        None
    };
    
    // Initialize logging using the module function
    let _guard = logging::init_logging(debug, quiet, format_str, None, None, None);
    
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
        println!("Glob Patterns:");
        println!("  *.txt                   Match all .txt files in current directory");
        println!("  **/*.rs                 Match all .rs files recursively");
        println!("  -                       Read paths from stdin (one per line)");
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
        println!("      --toon               Output in TOON format (token-efficient for agents)");
        println!("      --format=FORMAT      Output format: toon, json, or human");
        println!("      --fields=FIELDS      Select specific output fields (comma-separated)");
        println!("      --dry-run            Preview operations without executing");
        println!("      --usage              Show brief usage message");
        println!("  -h, --help               Show this help message");
        println!("  -V, --version            Print version information");
        return Ok(());
    }

    // Validate flags before proceeding
    if let Err(msg) = args.validate() {
        anyhow::bail!("{}", msg);
    }

    if args.dry_run {
        let (sources, dest) = args.resolve_paths()
            .context("Failed to resolve paths")?;
        info!("dry-run: mv {:?} -> {:?}", sources, dest);
        return Ok(());
    }

    // TODO: Implement VCS-aware move logic (story 03-001)
    let (sources, dest) = args.resolve_paths()
        .context("Failed to resolve paths")?;
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
        println!("Glob Patterns:");
        println!("  *.log                   Match all .log files in current directory");
        println!("  **/*.tmp                Match all .tmp files recursively");
        println!("  -                       Read paths from stdin (one per line)");
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
        println!("      --toon               Output in TOON format (token-efficient for agents)");
        println!("      --format=FORMAT      Output format: toon, json, or human");
        println!("      --fields=FIELDS      Select specific output fields (comma-separated)");
        println!("      --dry-run            Preview operations without executing");
        println!("      --usage              Show brief usage message");
        println!("  -h, --help               Show this help message");
        println!("  -V, --version            Print version information");
        return Ok(());
    }

    // Validate flags before proceeding
    if let Err(msg) = args.validate() {
        anyhow::bail!("{}", msg);
    }

    if args.dry_run {
        let paths = args.resolve_paths()
            .context("Failed to resolve paths")?;
        info!("dry-run: rm {:?}", paths);
        return Ok(());
    }

    let paths = args.resolve_paths()
        .context("Failed to resolve paths")?;
    
    if paths.is_empty() {
        anyhow::bail!("missing operand");
    }

    // Handle --plain mode (exact POSIX behavior, no smart features)
    if args.plain {
        return run_rm_plain(&args, &paths);
    }

    // Smart rm mode with VCS awareness and trash
    // TODO: Implement full smart rm logic (story 03-002)
    info!("rm mode: paths={:?}", paths);
    Ok(())
}

/// Run rm in plain POSIX mode (bypass all smart features)
fn run_rm_plain(args: &RmArgs, paths: &[PathBuf]) -> Result<()> {
    for path in paths {
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

fn run_list(all: bool, limit: Option<usize>, args: &SmartfoArgs) -> Result<()> {
    info!("list command: all={}, limit={:?}", all, limit);
    
    // Determine output format
    let output_format = determine_output_format(args.toon, &args.format, args.agent, args.human);
    
    // Determine field selector
    let field_selector = determine_field_selector(&args.fields, "list");
    
    // Create sample data for demonstration (TODO: replace with actual queue/audit data)
    let items = vec![
        serde_json::json!({"id": "1", "type": "move", "status": "completed", "source": "/tmp/file1.txt"}),
        serde_json::json!({"id": "2", "type": "remove", "status": "pending", "source": "/tmp/file2.txt"}),
        serde_json::json!({"id": "3", "type": "move", "status": "completed", "source": "/tmp/file3.txt"}),
    ];
    
    // Check for empty state
    let empty_context = EmptyContext::new(if all {
        "all operations".to_string()
    } else {
        format!("operations (limit: {})", limit.unwrap_or_default())
    });
    
    if let Some(empty_state) = check_empty(&items, empty_context) {
        // Return empty state
        let output = serde_json::json!({
            "empty": empty_state,
        });
        
        let mut writer = output::OutputWriter::new(std::io::stdout(), output_format);
        writer.write(&output)?;
        return Ok(());
    }
    
    // Compute aggregate
    let total = if all { 100 } else { limit.unwrap_or(items.len()) };
    let aggregate = AggregateComputer::compute_list_aggregate(&items, total);
    
    // Generate contextual suggestions
    let in_git_repo = detect_git_repo().is_some();
    let daemon_running = daemon::Daemon::new().and_then(|d| d.ping_daemon()).unwrap_or(false);
    
    // Try to get queue depth using default queue path
    let queue_depth = if let Ok(daemon) = daemon::Daemon::new() {
        let xdg_data_home = std::env::var("XDG_DATA_HOME")
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").expect("HOME not set");
                format!("{}/.local/share", home)
            });
        let queue_path = std::path::PathBuf::from(xdg_data_home).join("smartfo/queue.db");
        queue::JobQueue::new(&queue_path)
            .and_then(|q| q.queue_depth())
            .map(|d| d as usize)
            .ok()
    } else {
        None
    };
    
    let suggestion_context = SuggestionContext::new("list")
        .with_git_repo(in_git_repo)
        .with_daemon(daemon_running)
        .with_queue_depth(queue_depth.unwrap_or(0));
    
    let suggestions = SuggestionEngine::generate(&suggestion_context);
    let help_suggestions = format_suggestions_as_help(&suggestions);
    
    // Create output with aggregate
    let output = serde_json::json!({
        "items": items,
        "aggregate": aggregate,
    });
    
    // Apply field selection if specified
    let output_data = if let Some(ref _selector) = field_selector {
        // For now, just return the output as-is since field selection needs structured data
        output
    } else {
        output
    };
    
    // Write output with suggestions
    let mut writer = output::OutputWriter::new(std::io::stdout(), output_format)
        .with_suggestions(help_suggestions);
    writer.write(&output_data)?;
    
    Ok(())
}

fn run_status(detailed: bool, args: &SmartfoArgs) -> Result<()> {
    info!("status command: detailed={}", detailed);
    
    // Determine output format
    let output_format = determine_output_format(args.toon, &args.format, args.agent, args.human);
    
    // Determine field selector
    let field_selector = determine_field_selector(&args.fields, "status");
    
    // Create sample aggregates for demonstration (TODO: replace with actual daemon/queue data)
    let operations = Some(AggregateComputer::compute_operation_aggregate(10, 7, 2));
    let queue = Some(AggregateComputer::compute_queue_aggregate(5, 2));
    let daemon = Some(AggregateComputer::compute_daemon_aggregate("running", Some(1234)));
    
    // Check for empty state (if all aggregates are None or empty)
    let is_empty = operations.is_none() && queue.is_none() && daemon.is_none();
    
    if is_empty {
        let empty_context = EmptyContext::new(if detailed {
            "detailed status".to_string()
        } else {
            "status summary".to_string()
        });
        
        let empty_state = EmptyState::new(empty_context);
        let output = serde_json::json!({
            "empty": empty_state,
        });
        
        let mut writer = output::OutputWriter::new(std::io::stdout(), output_format);
        writer.write(&output)?;
        return Ok(());
    }
    
    let status_aggregate = StatusAggregate::new(operations, queue, daemon);
    
    // Generate contextual suggestions
    let in_git_repo = detect_git_repo().is_some();
    let daemon_running = daemon::Daemon::new().and_then(|d| d.ping_daemon()).unwrap_or(false);
    
    // Try to get queue depth using default queue path
    let queue_depth = if let Ok(daemon) = daemon::Daemon::new() {
        let xdg_data_home = std::env::var("XDG_DATA_HOME")
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").expect("HOME not set");
                format!("{}/.local/share", home)
            });
        let queue_path = std::path::PathBuf::from(xdg_data_home).join("smartfo/queue.db");
        queue::JobQueue::new(&queue_path)
            .and_then(|q| q.queue_depth())
            .map(|d| d as usize)
            .ok()
    } else {
        None
    };
    
    let suggestion_context = SuggestionContext::new("status")
        .with_git_repo(in_git_repo)
        .with_daemon(daemon_running)
        .with_queue_depth(queue_depth.unwrap_or(0));
    
    let suggestions = SuggestionEngine::generate(&suggestion_context);
    let help_suggestions = format_suggestions_as_help(&suggestions);
    
    // Create output with aggregate
    let output = serde_json::json!({
        "status": status_aggregate,
    });
    
    // Apply field selection if specified
    let output_data = if let Some(ref _selector) = field_selector {
        // For now, just return the output as-is since field selection needs structured data
        output
    } else {
        output
    };
    
    // Write output with suggestions
    let mut writer = output::OutputWriter::new(std::io::stdout(), output_format)
        .with_suggestions(help_suggestions);
    writer.write(&output_data)?;
    
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
        println!("      --init-config         Initialize or recreate default config file");
        println!("      --uninstall           Uninstall smartfo (remove symlinks, completions, man pages)");
        println!("      --force-uninstall     Bypass confirmation prompts during uninstall");
        println!("      --usage               Show brief usage message");
        println!("  -h, --help               Show this help message");
        println!("  -V, --version            Print version information");
        println!();
        println!("Subcommands:");
        println!("  git-hook-client          Run client-side pre-commit hook");
        println!("  git-hook-server          Run server-side pre-receive hook");
        return Ok(());
    }

    // Handle --uninstall flag
    if args.uninstall {
        return run_uninstall(args);
    }

    // Initialize config if it doesn't exist
    if config::init_config_if_missing()? {
        info!("Created default config file");
    }

    info!("install mode: hooks={:?} no_hooks={} force={}", args.hooks, args.no_hooks, args.force);

    // Use the new install.rs module
    let installer = install::Installer::new()?;
    installer.install(args.force)?;

    // Handle hook installation (legacy logic for now, will be integrated into install.rs later)
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

fn run_uninstall(args: &SmartfoArgs) -> Result<()> {
    info!("uninstall mode: force_uninstall={}", args.force_uninstall);

    let installer = install::Installer::new()?;
    installer.uninstall(args.force_uninstall)?;

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

    // Resolve smartfo binary path for hook scripts
    let smartfo_binary = std::env::current_exe()
        .context("Failed to get smartfo binary path")?;
    
    let force_hooks = args.force_hooks;
    
    if install_client {
        install_pre_commit_hook(&hooks_dir, &smartfo_binary, force_hooks)?;
    }
    if install_server {
        install_pre_receive_hook(&hooks_dir, &smartfo_binary, force_hooks)?;
    }

    Ok(())
}

/// Install the pre-commit hook
fn install_pre_commit_hook(hooks_dir: &PathBuf, smartfo_binary: &PathBuf, force: bool) -> Result<()> {
    let hook_path = hooks_dir.join("pre-commit");
    
    // Hook script content
    let hook_script = format!(
        r#"#!/bin/sh
# smartfo pre-commit hook - blocks raw deletions and renames
# Generated by smartfo --install
exec "{}" git-hook-client
"#,
        smartfo_binary.display()
    );
    
    // Check if hook already exists
    if hook_path.exists() {
        // Check if it's a smartfo hook
        if let Ok(content) = std::fs::read_to_string(&hook_path) {
            if content.contains("smartfo git-hook-client") {
                info!("pre-commit hook already installed: {}", hook_path.display());
                return Ok(());
            }
        }
        
        // Existing non-smartfo hook
        if force {
            info!("Overwriting existing pre-commit hook: {}", hook_path.display());
        } else {
            info!("Skipping pre-commit hook installation - existing hook found: {}", hook_path.display());
            info!("Use --force-hooks to overwrite existing hooks");
            return Ok(());
        }
    }
    
    // Write the hook script
    std::fs::write(&hook_path, hook_script)
        .context("Failed to write pre-commit hook")?;
    
    // Make it executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&hook_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&hook_path, perms)?;
    }
    
    info!("Installed pre-commit hook: {}", hook_path.display());
    Ok(())
}

/// Install the pre-receive hook
fn install_pre_receive_hook(hooks_dir: &PathBuf, smartfo_binary: &PathBuf, force: bool) -> Result<()> {
    let hook_path = hooks_dir.join("pre-receive");
    
    // Hook script content
    let hook_script = format!(
        r#"#!/bin/sh
# smartfo pre-receive hook - blocks raw deletions and renames from pushes
# Generated by smartfo --install
exec "{}" git-hook-server
"#,
        smartfo_binary.display()
    );
    
    // Check if hook already exists
    if hook_path.exists() {
        // Check if it's a smartfo hook
        if let Ok(content) = std::fs::read_to_string(&hook_path) {
            if content.contains("smartfo git-hook-server") {
                info!("pre-receive hook already installed: {}", hook_path.display());
                return Ok(());
            }
        }
        
        // Existing non-smartfo hook
        if force {
            info!("Overwriting existing pre-receive hook: {}", hook_path.display());
        } else {
            info!("Skipping pre-receive hook installation - existing hook found: {}", hook_path.display());
            info!("Use --force-hooks to overwrite existing hooks");
            return Ok(());
        }
    }
    
    // Write the hook script
    std::fs::write(&hook_path, hook_script)
        .context("Failed to write pre-receive hook")?;
    
    // Make it executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&hook_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&hook_path, perms)?;
    }
    
    info!("Installed pre-receive hook: {}", hook_path.display());
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

    git_hooks::run_pre_commit_hook(&repo_root)
}

fn run_git_hook_server() -> Result<()> {
    // Detect the Git repository root
    let repo_root = detect_git_repo()
        .ok_or_else(|| anyhow::anyhow!("Not inside a Git repository"))?;

    git_hooks::run_pre_receive_hook(&repo_root)
}

fn run_session_context() -> Result<()> {
    info!("Generating session context");
    
    let context = hooks::SessionContext::new()
        .context("Failed to create session context")?;
    
    // Output in TOON format
    let toon_output = context.to_toon();
    println!("{}", toon_output);
    
    // Cache session metadata
    hooks::cache_session_metadata(&context)
        .context("Failed to cache session metadata")?;
    
    Ok(())
}

fn run_install_agent_hooks() -> Result<()> {
    info!("Installing agent hooks");
    
    hooks::install_agent_hooks()
        .context("Failed to install agent hooks")?;
    
    println!("Agent hooks installed successfully");
    Ok(())
}

fn main() -> Result<()> {
    let mode = detect_mode();

    match mode.as_str() {
        "mv" | "smv" => {
            let args = MvArgs::parse();
            setup_logging(args.debug, args.quiet, args.json, args.color.as_deref())?;
            let output_format = determine_output_format(args.toon, &args.format, args.agent, args.human);
            info!("Output format: {:?}", output_format);
            run_mv(args)
        }
        "rm" | "srm" => {
            let args = RmArgs::parse();
            setup_logging(args.debug, args.quiet, args.json, args.color.as_deref())?;
            let output_format = determine_output_format(args.toon, &args.format, args.agent, args.human);
            info!("Output format: {:?}", output_format);
            run_rm(args)
        }
        "smartfo" | _ => {
            let args = SmartfoArgs::parse();
            setup_logging(args.debug, args.quiet, args.json, args.color.as_deref())?;
            let output_format = determine_output_format(args.toon, &args.format, args.agent, args.human);
            info!("Output format: {:?}", output_format);

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
                println!("      --init-config         Initialize or recreate default config file");
                println!("      --toon               Output in TOON format (token-efficient for agents)");
                println!("      --format=FORMAT      Output format: toon, json, or human");
                println!("      --session-context     Output session context in TOON format for agent consumption");
                println!("      --install-agent-hooks Install agent hooks for Claude Code or Codex");
                println!("      --usage              Show brief usage message");
                println!("  -h, --help               Show this help message");
                println!("  -V, --version            Print version information");
                println!();
                println!("Subcommands:");
                println!("  git-hook-client          Run client-side pre-commit hook");
                println!("  git-hook-server          Run server-side pre-receive hook");
                println!("  list                     List operations and queue status (with aggregate counts)");
                println!("  status                   Show daemon and queue status (with aggregate information)");
                println!("  session-context          Output session context in TOON format for agent consumption");
                println!("  install-agent-hooks      Install agent hooks for Claude Code or Codex");
                println!("  generate-skill           Generate agent skill (SKILL.md) from CLI metadata");
                println!("  check-skill              Check if generated skill is stale");
                return Ok(());
            }

            // Handle --init-config flag
            if args.init_config {
                info!("--init-config flag triggered");
                let config_path = config::create_default_config()?;
                println!("Created default config file at: {}", config_path.display());
                return Ok(());
            }

            if let Some(cmd) = &args.command {
                match cmd {
                    SmartfoCommand::GitHookClient => run_git_hook_client(),
                    SmartfoCommand::GitHookServer => run_git_hook_server(),
                    SmartfoCommand::List { all, limit, quiet, debug } => {
                        // Reinitialize logging with subcommand flags
                        setup_logging(*debug, *quiet, args.json, args.color.as_deref())?;
                        run_list(*all, *limit, &args)
                    },
                    SmartfoCommand::Status { detailed, quiet, debug } => {
                        // Reinitialize logging with subcommand flags
                        setup_logging(*debug, *quiet, args.json, args.color.as_deref())?;
                        run_status(*detailed, &args)
                    },
                    SmartfoCommand::SessionContext => run_session_context(),
                    SmartfoCommand::InstallAgentHooks => run_install_agent_hooks(),
                    SmartfoCommand::GenerateSkill { output } => run_generate_skill(output),
                    SmartfoCommand::CheckSkill { skill_file } => run_check_skill(skill_file),
                }
            } else if args.install {
                run_install(&args)
            } else if args.session_context {
                run_session_context()
            } else if args.install_agent_hooks {
                run_install_agent_hooks()
            } else {
                // No subcommand or install flag: show content-first state summary
                run_noargs(&args)
            }
        }
    }
}

/// Generate agent skill from CLI metadata
fn run_generate_skill(output: &Option<std::path::PathBuf>) -> Result<()> {
    info!("generate-skill command: output={:?}", output);
    
    let mut generator = skill::SkillGenerator::new();
    let skill = generator.generate_with_defaults()
        .context("Failed to generate skill")?;
    
    let markdown = skill.to_markdown();
    
    match output {
        Some(path) => {
            std::fs::write(path, &markdown)
                .context(format!("Failed to write skill to {}", path.display()))?;
            info!("Generated skill written to: {}", path.display());
        }
        None => {
            println!("{}", markdown);
        }
    }
    
    Ok(())
}

/// Check if generated skill is stale
fn run_check_skill(skill_file: &Option<std::path::PathBuf>) -> Result<()> {
    info!("check-skill command: skill_file={:?}", skill_file);
    
    let skill_path = skill_file.as_ref()
        .map(|p| p.as_path())
        .unwrap_or(std::path::Path::new("SKILL.md"));
    
    if !skill_path.exists() {
        anyhow::bail!("Skill file not found: {}", skill_path.display());
    }
    
    let content = std::fs::read_to_string(skill_path)
        .context(format!("Failed to read skill file: {}", skill_path.display()))?;
    
    let is_stale = skill::check_skill_stale(&content)
        .context("Failed to check skill staleness")?;
    
    if is_stale {
        anyhow::bail!("Skill is stale. Run 'smartfo generate-skill' to regenerate.");
    }
    
    info!("Skill is up to date");
    Ok(())
}

/// Run no-args invocation: show content-first state summary
fn run_noargs(args: &SmartfoArgs) -> Result<()> {
    info!("no-args invocation: showing content-first state summary");
    
    // Determine output format
    let output_format = determine_output_format(
        args.toon,
        &args.format,
        args.agent,
        args.human,
    );
    
    // Detect context
    let current_dir = std::env::current_dir()?;
    let git_repo = detect_git_repo();
    let is_in_git = git_repo.is_some();
    
    // Build context-aware state summary
    let mut state = serde_json::json!({
        "context": {
            "current_directory": current_dir.display().to_string(),
            "in_git_repository": is_in_git,
        }
    });
    
    // Add git repository info if in git
    if let Some(repo_root) = git_repo {
        state["context"]["git_repository_root"] = serde_json::Value::String(repo_root.display().to_string());
        
        // Try to get operations summary from queue
        let queue_result = get_queue_summary();
        match queue_result {
            Ok(summary) => {
                state["operations"] = summary;
            }
            Err(e) => {
                state["operations"] = serde_json::json!({
                    "error": format!("Failed to get queue summary: {}", e),
                    "use_list": "Run 'smartfo list' to see queued operations"
                });
            }
        }
    }
    
    // Add daemon status
    let daemon_status = get_daemon_status();
    state["daemon"] = daemon_status;
    
    // Generate contextual suggestions
    let daemon_running = daemon::Daemon::new().and_then(|d| d.ping_daemon()).unwrap_or(false);
    
    // Try to get queue depth using default queue path
    let queue_depth = if let Ok(daemon) = daemon::Daemon::new() {
        let xdg_data_home = std::env::var("XDG_DATA_HOME")
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").expect("HOME not set");
                format!("{}/.local/share", home)
            });
        let queue_path = std::path::PathBuf::from(xdg_data_home).join("smartfo/queue.db");
        queue::JobQueue::new(&queue_path)
            .and_then(|q| q.queue_depth())
            .map(|d| d as usize)
            .ok()
    } else {
        None
    };
    
    let suggestion_context = SuggestionContext::new("")
        .with_git_repo(is_in_git)
        .with_daemon(daemon_running)
        .with_queue_depth(queue_depth.unwrap_or(0));
    
    let suggestions = SuggestionEngine::generate(&suggestion_context);
    let help_suggestions = format_suggestions_as_help(&suggestions);
    
    // Write output with suggestions
    let mut writer = output::OutputWriter::new(std::io::stdout(), output_format)
        .with_suggestions(help_suggestions);
    writer.write(&state)?;
    
    Ok(())
}

/// Get queue summary for operations
fn get_queue_summary() -> Result<serde_json::Value> {
    // Try to get queue depth using default queue path
    let queue_path = std::path::PathBuf::from("/tmp/smartfo-queue.db");
    
    if !queue_path.exists() {
        return Ok(serde_json::json!({
            "queue_exists": false,
            "message": "No operation queue found"
        }));
    }
    
    let queue = queue::JobQueue::new(&queue_path)?;
    let depth = queue.queue_depth()?;
    
    Ok(serde_json::json!({
        "queue_exists": true,
        "queue_depth": depth,
        "queued_operations": depth,
        "use_list": format!("Run 'smartfo list' to see {} queued operation(s)", depth)
    }))
}

/// Get daemon status
fn get_daemon_status() -> serde_json::Value {
    let daemon = daemon::Daemon::new();
    
    match daemon {
        Ok(d) => {
            // Check if daemon is running by pinging it
            match d.ping_daemon() {
                Ok(true) => {
                    serde_json::json!({
                        "status": "running",
                        "message": "Daemon is active and accepting connections",
                        "use_status": "Run 'smartfo status' for detailed daemon status"
                    })
                }
                Ok(false) => {
                    serde_json::json!({
                        "status": "not_running",
                        "message": "Daemon is not currently running",
                        "use_status": "Run 'smartfo status' for detailed daemon status"
                    })
                }
                Err(e) => {
                    serde_json::json!({
                        "status": "unknown",
                        "error": format!("Failed to check daemon status: {}", e),
                        "use_status": "Run 'smartfo status' for detailed daemon status"
                    })
                }
            }
        }
        Err(e) => {
            serde_json::json!({
                "status": "error",
                "error": format!("Failed to initialize daemon: {}", e),
                "use_status": "Run 'smartfo status' for detailed daemon status"
            })
        }
    }
}

/// Custom exit code handling
pub fn exit_with_code(result: Result<()>) -> ! {
    match result {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            // Determine if this is a usage error (exit code 2) or other error (exit code 1)
            let is_usage_error = e.to_string().contains("Missing")
                || e.to_string().contains("Cannot specify")
                || e.to_string().contains("usage");
            
            if is_usage_error {
                std::process::exit(2);
            } else {
                std::process::exit(1);
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
