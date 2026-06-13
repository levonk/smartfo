use clap::Parser;
use clap::CommandFactory;
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
mod exit;
mod dry_run;
mod confirmation;
mod progress;
mod completions;
mod man;
mod health;
mod terminal;
mod tui;
mod secret;
mod resource;
mod privacy;
mod export;
use cli::{MvArgs, RmArgs, SmartfoArgs, SmartfoCommand, GitCommand, JobCommand, AgentCommand, InfoCommand, HealthCommand};
use vcs::detect_vcs;
use vcs::is_tracked;
use output::OutputFormat;
use output::schema::{SchemaRegistry, FieldSelector};
use output::aggregates::*;
use output::empty::{EmptyState, check_empty, EmptyContext};
use output::suggestions::{SuggestionContext, SuggestionEngine, format_suggestions_as_help};
use output::Pager;
use output::content_first::StateSummary;
use man::{ManPageType, generate_man_page};
use exit::{ExitCode, SignalHandler, error_category_to_exit_code, ErrorCategory};
use terminal::{get_terminal_size, TerminalSizeCache, wrap_text};
use tui::{edit_arguments, edit_config, install_tui, batch_operations, is_tui_supported};

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
                eprintln!("ERROR: Failed to parse field selector - {} - Use --help to see available fields", e);
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

/// Determine if daemon mode should be used based on flags and platform support
fn should_use_daemon(daemon_flag: bool, no_daemon_flag: bool) -> Result<bool> {
    // Check for conflicting flags (already validated in cli.rs, but double-check)
    if daemon_flag && no_daemon_flag {
        return Err(anyhow::anyhow!("Cannot specify both --daemon and --no-daemon"));
    }

    // --no-daemon explicitly disables daemon mode
    if no_daemon_flag {
        return Ok(false);
    }

    // --daemon explicitly enables daemon mode
    if daemon_flag {
        // Check platform support
        if !daemon::Daemon::is_daemon_supported() {
            // Load config to check if fallback should be quiet
            let config = config::resolve_config(None)?;
            if !config.behavior.daemon_fallback_quiet {
                eprintln!("WARNING: Daemon mode is not supported on this platform - Falling back to synchronous processing");
                eprintln!("Use --no-daemon to suppress this warning");
            }
            return Ok(false);
        }
        return Ok(true);
    }

    // Default: auto-spawn on first async operation (existing behavior)
    // This will be handled by the actual operation logic
    Ok(false)
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

    // Initialize logging using the module function (only if not already initialized)
    // This prevents panics when subcommands try to reinitialize logging
    let _guard = logging::try_init_logging(debug, quiet, format_str, None, None, None);

    Ok(())
}

fn run_mv(args: MvArgs) -> Result<()> {
    // Handle --version flag
    if args.version {
        info!("--version flag triggered for mv mode");
        println!("smartfo mv {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Handle TUI mode flags
    if args.interactive_tui || args.tui {
        info!("TUI mode triggered for mv");
        if !is_tui_supported() {
            eprintln!("ERROR: TUI mode requires a terminal");
            anyhow::bail!("TUI mode requires a terminal");
        }

        let (sources, dest) = args.resolve_paths()
            .context("Failed to resolve paths")?;

        let mut items: Vec<String> = sources.iter()
            .map(|p| p.display().to_string())
            .collect();

        if let Some(ref d) = dest {
            items.push(format!("-> {}", d.display()));
        }

        match edit_arguments(items) {
            Ok(result) => {
                info!("TUI result: {}", result);
                println!("TUI completed: {}", result);
                return Ok(());
            }
            Err(e) => {
                eprintln!("TUI error: {}", e);
                anyhow::bail!("TUI error: {}", e);
            }
        }
    }

    // Handle daemon flags
    let use_daemon = should_use_daemon(args.daemon, args.no_daemon)?;
    if use_daemon {
        info!("Daemon mode explicitly enabled via --daemon flag");
        // Pre-launch daemon and wait for it to be ready
        let daemon = daemon::Daemon::new()?;
        if let Ok(stream) = daemon.get_or_spawn_daemon() {
            info!("Daemon pre-launched successfully");
            println!("Daemon pre-launched successfully");
            println!("Use 'smartfo job list' to monitor background job progress");
            drop(stream); // Close the connection, daemon continues running
        } else {
            eprintln!("WARNING: Failed to pre-launch daemon, continuing without daemon");
        }
    } else if args.no_daemon {
        info!("Daemon mode explicitly disabled via --no-daemon flag");
    }

    // Handle --usage flag
    if args.usage {
        info!("--usage flag triggered for mv mode");

        // Create pager for long output
        let pager = Pager::new(args.no_pager, args.quiet, args.json, false);

        let usage_text = format!(
            "Usage: mv [OPTION]... SOURCE... DEST\n\
             Move (rename) SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n\
             \n\
             Glob Patterns:\n\
               *.txt                   Match all .txt files in current directory\n\
               **/*.rs                 Match all .rs files recursively\n\
               -                       Read paths from stdin (one per line)\n\
             \n\
             Options:\n\
               -f, --force              Do not prompt before overwriting\n\
               -i, --interactive        Prompt before overwrite\n\
               -n, --no-clobber         Do not overwrite an existing file\n\
               -v, --verbose            Explain what is being done\n\
               -T, --no-target-directory  Treat DEST as a normal file\n\
               -t, --target-directory=DIRECTORY  Move all SOURCE arguments into DIRECTORY\n\
                   --backup             Make a backup of each existing destination file\n\
                   --strip-trailing-slashes  Remove trailing slashes from SOURCE arguments\n\
                   --plain              Disable all smart features; behave exactly like POSIX mv\n\
                   --force-outside-vcs  Allow moving tracked files outside repo\n\
                   --async              Force async move even for small/same-fs files\n\
                   --blocking           Wait for operation to complete\n\
                   --sync               Fsync destination file and directory after operation\n\
                   --reason=REASON      Annotate intent in the audit log\n\
                   --json               Output operation metadata as JSON\n\
                   --toon               Output in TOON format (token-efficient for agents)\n\
                   --format=FORMAT      Output format: toon, json, or human\n\
                   --fields=FIELDS      Select specific output fields (comma-separated)\n\
                   --dry-run            Preview operations without executing\n\
                   --usage              Show brief usage message\n\
                   --no-pager           Disable pager for long output\n\
               -h, --help               Show this help message\n\
               -V, --version            Print version information"
        );

        // Wrap usage text based on terminal width
        let terminal_size = get_terminal_size();
        let wrapped_text = wrap_text(&usage_text, terminal_size.cols);
        let formatted_text = wrapped_text.join("\n");

        pager.page_content(&formatted_text)?;
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

        // Show what would be done
        println!("Dry-run mode: No changes will be made");
        println!();

        if let Some(ref dest_dir) = dest {
            if sources.len() > 1 {
                // Multiple sources, dest is a directory
                for source in &sources {
                    let dest_path = dest_dir.join(source.file_name().unwrap_or(source.as_os_str()));
                    println!("Would move: {} -> {}", source.display(), dest_path.display());

                    // Check if source is in a VCS repo
                    if let Ok(Some(vcs_info)) = vcs::detect_vcs(source) {
                        if let Ok(tracked) = vcs::is_tracked(&vcs_info, source) {
                            if tracked {
                                println!("  (VCS-native move would be used: {:?})", vcs_info.vcs_type());
                            }
                        }
                    }
                }
            } else {
                // Single source, dest is the target file/directory
                println!("Would move: {} -> {}", sources[0].display(), dest_dir.display());

                // Check if source is in a VCS repo
                if let Ok(Some(vcs_info)) = vcs::detect_vcs(&sources[0]) {
                    if let Ok(tracked) = vcs::is_tracked(&vcs_info, &sources[0]) {
                        if tracked {
                            println!("  (VCS-native move would be used: {:?})", vcs_info.vcs_type());
                        }
                    }
                }
            }
        } else if let Some(ref target_dir) = args.target_directory {
            // -t flag was used
            for source in &sources {
                let dest_path = target_dir.join(source.file_name().unwrap_or(source.as_os_str()));
                println!("Would move: {} -> {}", source.display(), dest_path.display());

                // Check if source is in a VCS repo
                if let Ok(Some(vcs_info)) = vcs::detect_vcs(source) {
                    if let Ok(tracked) = vcs::is_tracked(&vcs_info, source) {
                        if tracked {
                            println!("  (VCS-native move would be used: {:?})", vcs_info.vcs_type());
                        }
                    }
                }
            }
        }

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

    // Handle TUI mode flags
    if args.interactive_tui || args.tui {
        info!("TUI mode triggered for rm");
        if !is_tui_supported() {
            eprintln!("ERROR: TUI mode requires a terminal");
            anyhow::bail!("TUI mode requires a terminal");
        }

        let paths = args.resolve_paths()
            .context("Failed to resolve paths")?;

        let items: Vec<String> = paths.iter()
            .map(|p| p.display().to_string())
            .collect();

        match batch_operations(items) {
            Ok(result) => {
                info!("TUI result: {}", result);
                println!("TUI completed: {}", result);
                return Ok(());
            }
            Err(e) => {
                eprintln!("TUI error: {}", e);
                anyhow::bail!("TUI error: {}", e);
            }
        }
    }

    // Handle --man flag
    if args.man {
        info!("--man flag triggered for rm mode");
        let man_page = generate_man_page(ManPageType::Rm)?;

        // Create pager for long output
        let pager = Pager::new(args.no_pager, args.quiet, args.json, false);
        pager.page_content(&man_page)?;
        return Ok(());
    }

    // Handle daemon flags
    let use_daemon = should_use_daemon(args.daemon, args.no_daemon)?;
    if use_daemon {
        info!("Daemon mode explicitly enabled via --daemon flag");
        // Pre-launch daemon and wait for it to be ready
        let daemon = daemon::Daemon::new()?;
        if let Ok(stream) = daemon.get_or_spawn_daemon() {
            info!("Daemon pre-launched successfully");
            println!("Daemon pre-launched successfully");
            println!("Use 'smartfo job list' to monitor background job progress");
            drop(stream); // Close the connection, daemon continues running
        } else {
            eprintln!("WARNING: Failed to pre-launch daemon, continuing without daemon");
        }
    } else if args.no_daemon {
        info!("Daemon mode explicitly disabled via --no-daemon flag");
    }

    // Handle --usage flag
    if args.usage {
        info!("--usage flag triggered for rm mode");

        // Create pager for long output
        let pager = Pager::new(args.no_pager, args.quiet, args.json, false);

        let usage_text = format!(
            "Usage: rm [OPTION]... FILE...\n\
             Remove (unlink) the FILE(s).\n\
             \n\
             Glob Patterns:\n\
               *.log                   Match all .log files in current directory\n\
               **/*.tmp                Match all .tmp files recursively\n\
               -                       Read paths from stdin (one per line)\n\
             \n\
             Options:\n\
               -f, --force              Ignore non-existent files, never prompt\n\
               -i                      Prompt before every removal\n\
               -I                      Prompt once before removing more than three files\n\
               -r, -R, --recursive     Remove directories and their contents recursively\n\
               -d, --dir               Remove empty directories\n\
                   --preserve-root      Do not remove '/' (default)\n\
                   --one-filesystem     Skip directories on different file systems\n\
                   --plain              Disable all smart features; behave exactly like POSIX rm\n\
                   --force-delete       Bypass trash and delete directly\n\
                   --blocking           Wait for operation to complete\n\
                   --sync               Fsync after operation\n\
                   --reason=REASON      Annotate intent in the audit log\n\
                   --json               Output operation metadata as JSON\n\
                   --toon               Output in TOON format (token-efficient for agents)\n\
                   --format=FORMAT      Output format: toon, json, or human\n\
                   --fields=FIELDS      Select specific output fields (comma-separated)\n\
                   --dry-run            Preview operations without executing\n\
                   --usage              Show brief usage message\n\
                   --no-pager           Disable pager for long output\n\
               -h, --help               Show this help message\n\
               -V, --version            Print version information"
        );

        // Wrap usage text based on terminal width
        let terminal_size = get_terminal_size();
        let wrapped_text = wrap_text(&usage_text, terminal_size.cols);
        let formatted_text = wrapped_text.join("\n");

        pager.page_content(&formatted_text)?;
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

        // Show what would be done
        println!("Dry-run mode: No changes will be made");
        println!();

        for path in &paths {
            println!("Would remove: {}", path.display());

            // Check if path is in a VCS repo
            if let Ok(Some(vcs_info)) = vcs::detect_vcs(path) {
                if let Ok(tracked) = vcs::is_tracked(&vcs_info, path) {
                    if tracked {
                        println!("  (VCS-aware removal would be used: {:?})", vcs_info.vcs_type());
                    }
                }
            }

            // Check if path is a directory
            if path.is_dir() {
                if args.recursive || args.dir {
                    println!("  (Directory would be removed recursively)");
                } else {
                    println!("  (Would fail: -r or -d required for directory)");
                }
            }

            // Check if --force-delete is set
            if args.force_delete {
                println!("  (Would bypass trash and delete directly)");
            } else {
                println!("  (Would move to trash)");
            }
        }

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

fn run_health_check(http: bool, signal: bool) -> Result<()> {
    info!("health check command: http={}, signal={}", http, signal);

    let checker = health::HealthChecker::new()?;

    if signal {
        // Signal-based health check: send SIGUSR1 to daemon and read status file
        info!("Using signal-based health check");

        // Get daemon PID
        let daemon = daemon::Daemon::new()?;
        if let Some(pid) = daemon.read_pid_file()? {
            // Send SIGUSR1 to daemon
            use nix::sys::signal::{self, Signal};
            use nix::unistd::Pid;

            if let Err(e) = signal::kill(Pid::from_raw(pid as i32), Signal::SIGUSR1) {
                eprintln!("Failed to send SIGUSR1 to daemon: {}", e);
                std::process::exit(1);
            }

            // Wait a moment for the signal handler to write the status file
            std::thread::sleep(std::time::Duration::from_millis(100));

            // Read status from file
            let status = checker.read_status_file()?;
            println!("{:?}", status);
            std::process::exit(status.exit_code());
        } else {
            eprintln!("No daemon running (no PID file)");
            std::process::exit(1);
        }
    } else {
        // HTTP-based health check (default)
        info!("Using HTTP-based health check");
        let status = checker.check();
        println!("{:?}", status);
        std::process::exit(status.exit_code());
    }
}

fn run_install(args: &SmartfoArgs) -> Result<()> {
    // Handle --version flag
    if args.version {
        info!("--version flag triggered for install mode");
        println!("smartfo {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Handle TUI mode flags
    if args.interactive_tui || args.tui {
        info!("TUI mode triggered for install");
        if !is_tui_supported() {
            eprintln!("ERROR: TUI mode requires a terminal");
            anyhow::bail!("TUI mode requires a terminal");
        }

        match install_tui() {
            Ok(result) => {
                info!("TUI result: {}", result);
                println!("TUI completed: {}", result);
                // Continue with normal install after TUI
            }
            Err(e) => {
                eprintln!("TUI error: {}", e);
                anyhow::bail!("TUI error: {}", e);
            }
        }
    }

    // Handle --man flag
    if args.man {
        info!("--man flag triggered for install mode");
        let man_page = generate_man_page(ManPageType::Smartfo)?;

        // Create pager for long output
        let pager = Pager::new(args.no_pager, args.quiet, args.json, false);
        pager.page_content(&man_page)?;
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

    // Use the new install.rs module (includes hook installation)
    let installer = install::Installer::new()?;
    installer.install(args.force, args.hooks.clone(), args.no_hooks, args.force_hooks)?;

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
exec "{}" git hook-client
"#,
        smartfo_binary.display()
    );

    // Check if hook already exists
    if hook_path.exists() {
        // Check if it's a smartfo hook (support both old and new command format)
        if let Ok(content) = std::fs::read_to_string(&hook_path) {
            if content.contains("smartfo git hook-client") || content.contains("smartfo git-hook-client") {
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
exec "{}" git hook-server
"#,
        smartfo_binary.display()
    );

    // Check if hook already exists
    if hook_path.exists() {
        // Check if it's a smartfo hook (support both old and new command format)
        if let Ok(content) = std::fs::read_to_string(&hook_path) {
            if content.contains("smartfo git hook-server") || content.contains("smartfo git-hook-server") {
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

fn run_generate_completion(shell: Option<&str>) -> Result<i32> {
    use clap_complete::Shell;

    // If no shell specified, auto-detect available shells
    let shells_to_generate = if let Some(shell) = shell {
        vec![shell.to_lowercase()]
    } else {
        detect_available_shells()
    };

    let mode = detect_mode();

    for shell_name in shells_to_generate {
        let shell_enum = match shell_name.as_str() {
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            "fish" => Shell::Fish,
            _ => {
                eprintln!("ERROR: Unsupported shell '{}'. Supported shells: bash, zsh, fish", shell_name);
                continue;
            }
        };

        // Generate completion for smartfo mode based on argv[0]
        let mut buf = Vec::new();

        match mode.as_str() {
            "mv" | "smv" => {
                let mut cmd = cli::MvArgs::command();
                clap_complete::generate(shell_enum, &mut cmd, "mv", &mut buf);
            }
            "rm" | "srm" => {
                let mut cmd = cli::RmArgs::command();
                clap_complete::generate(shell_enum, &mut cmd, "rm", &mut buf);
            }
            "smartfo" | _ => {
                let mut cmd = cli::SmartfoArgs::command();
                clap_complete::generate(shell_enum, &mut cmd, "smartfo", &mut buf);
            }
        }

        println!("{}", String::from_utf8(buf).context("Completion script is not valid UTF-8")?);
    }

    Ok(0)
}

/// Detect available shells on the system
fn detect_available_shells() -> Vec<String> {
    let mut available = Vec::new();

    // Check for bash
    if std::path::Path::new("/bin/bash").exists() || which::which("bash").is_ok() {
        available.push("bash".to_string());
    }

    // Check for zsh
    if std::path::Path::new("/bin/zsh").exists() || which::which("zsh").is_ok() {
        available.push("zsh".to_string());
    }

    // Check for fish
    if which::which("fish").is_ok() {
        available.push("fish".to_string());
    }

    // If no shells detected, default to all three
    if available.is_empty() {
        available = vec!["bash".to_string(), "zsh".to_string(), "fish".to_string()];
    }

    available
}

fn run_main() -> Result<i32> {
    // Setup signal handler for graceful shutdown
    let signal_handler = SignalHandler::new();
    if let Err(e) = signal_handler.setup_handlers() {
        eprintln!("WARNING: Failed to setup signal handlers - {} - Graceful shutdown may not work correctly", e);
    // Detect terminal size on startup
    let terminal_size = get_terminal_size();
    info!("Terminal size detected: {}x{}", terminal_size.cols, terminal_size.rows);

    }

    let mode = detect_mode();
    let result = match mode.as_str() {
        "mv" | "smv" => {
            let args = MvArgs::parse();
            if let Err(e) = setup_logging(args.debug, args.quiet, args.json, args.color.as_deref()) {
                Err(e)
            } else {
                let output_format = determine_output_format(args.toon, &args.format, args.agent, args.human);
                info!("Output format: {:?}", output_format);
                run_mv(args)
            }
        }
        "rm" | "srm" => {
            let args = RmArgs::parse();
            if let Err(e) = setup_logging(args.debug, args.quiet, args.json, args.color.as_deref()) {
                Err(e)
            } else {
                let output_format = determine_output_format(args.toon, &args.format, args.agent, args.human);
                info!("Output format: {:?}", output_format);
                run_rm(args)
            }
        }
        "smartfo" | _ => {
            let args = SmartfoArgs::parse();
            if let Err(e) = setup_logging(args.debug, args.quiet, args.json, args.color.as_deref()) {
                Err(e)
            } else {
                let output_format = determine_output_format(args.toon, &args.format, args.agent, args.human);
                info!("Output format: {:?}", output_format);

                // Handle --version flag at root level
                if args.version {
                    info!("--version flag triggered for smartfo");
                    println!("smartfo {}", env!("CARGO_PKG_VERSION"));
                    return Ok(0);
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
                    println!("      --generate-completion=SHELL  Generate shell completion script (bash, zsh, fish)");
                    println!("      --toon               Output in TOON format (token-efficient for agents)");
                    println!("      --format=FORMAT      Output format: toon, json, or human");
                    println!("      --session-context     Output session context in TOON format for agent consumption");
                    println!("      --install-agent-hooks Install agent hooks for Claude Code or Codex");
                    println!("      --usage              Show brief usage message");
                    println!("  -h, --help               Show this help message");
                    println!("  -V, --version            Print version information");
                    println!();
                    println!("Subcommands:");
                    println!("  git                      Git hook commands");
                    println!("    hook-client           Run client-side pre-commit hook");
                    println!("    hook-server           Run server-side pre-receive hook");
                    println!("  job                      Job management commands");
                    println!("    list                  List background jobs with optional filtering");
                    println!("    cancel <id>           Cancel a specific background job by ID");
                    println!("  agent                    Agent integration commands");
                    println!("    session-context       Output session context in TOON format");
                    println!("    install-hooks         Install agent hooks for Claude Code or Codex");
                    println!("    generate-skill        Generate agent skill (SKILL.md) from CLI metadata");
                    println!("    check-skill           Check if generated skill is stale");
                    println!("  info                     Information and query commands");
                    println!("    list                  List operations and queue status");
                    println!("    status                Show daemon and queue status");
                    return Ok(0);
                }

                // Handle --init-config flag
                if args.init_config {
                    info!("--init-config flag triggered");
                    if args.interactive_tui || args.tui {
                        if !is_tui_supported() {
                            eprintln!("ERROR: TUI mode requires a terminal");
                            return Ok(1);
                        }

                        match edit_config() {
                            Ok(result) => {
                                info!("TUI config edit result: {}", result);
                                println!("TUI config edit completed: {}", result);
                            }
                            Err(e) => {
                                eprintln!("TUI error: {}", e);
                                return Ok(1);
                            }
                        }
                    }

                    let config_path = config::create_default_config_force(args.force)?;
                    println!("Created default config file at: {}", config_path.display());
                    return Ok(0);
                }

                // Handle --validate-config flag
                if args.validate_config {
                    info!("--validate-config flag triggered");
                    let config_path = config::default_config_path()
                        .ok_or_else(|| anyhow::anyhow!("Could not determine default config path"))?;

                    if !config_path.exists() {
                        eprintln!("Config file not found at: {}", config_path.display());
                        eprintln!("Run --init-config to create a default config file.");
                        return Ok(1);
                    }

                    match config::validate_config_file(&config_path) {
                        Ok(config) => {
                            println!("Config file is valid: {}", config_path.display());
                            println!();
                            println!("Configuration summary:");
                            println!("  VCS preference: {}", config.vcs.preference);
                            println!("  Trash mode: {}", config.trash.mode);
                            println!("  Max concurrent jobs: {}", config.concurrency.max_concurrent_jobs);
                            println!("  Log level: {}", config.logging.level);
                            return Ok(0);
                        }
                        Err(e) => {
                            eprintln!("Config validation failed:");
                            eprintln!("  {}", e);
                            eprintln!();
                            eprintln!("Suggestion: {}", e.suggestion);
                            return Ok(1);
                        }
                    }
                }

                // Handle --generate-completion flag
                if args.generate_completion.is_some() {
                    let shell = args.generate_completion.as_deref()
                        .filter(|s| !s.is_empty() && *s != "auto");
                    info!("--generate-completion flag triggered for shell: {:?}", shell);
                    return run_generate_completion(shell);
                }

                if let Some(cmd) = &args.command {
                    match cmd {
                        SmartfoCommand::Git(git_cmd) => {
                            match git_cmd {
                                GitCommand::HookClient => run_git_hook_client(),
                                GitCommand::HookServer => run_git_hook_server(),
                            }
                        },
                        SmartfoCommand::Job(job_cmd) => {
                            match job_cmd {
                                JobCommand::List { ids, quiet, debug } => {
                                    setup_logging(*debug, *quiet, args.json, args.color.as_deref())?;
                                    run_list_jobs(ids)
                                },
                                JobCommand::Cancel { job_id, quiet, debug } => {
                                    setup_logging(*debug, *quiet, args.json, args.color.as_deref())?;
                                    run_cancel_job(job_id)
                                },
                                JobCommand::Export { output, format, status, op_type, date_range } => {
                                    setup_logging(false, false, args.json, args.color.as_deref())?;
                                    run_export_jobs(output, format, status, op_type, date_range)
                                },
                                JobCommand::Import { input } => {
                                    setup_logging(false, false, args.json, args.color.as_deref())?;
                                    run_import_jobs(input)
                                },
                                JobCommand::Analyze { input } => {
                                    setup_logging(false, false, args.json, args.color.as_deref())?;
                                    run_analyze_jobs(input)
                                },
                            }
                        },
                        SmartfoCommand::Agent(agent_cmd) => {
                            match agent_cmd {
                                AgentCommand::SessionContext => run_session_context(),
                                AgentCommand::InstallHooks => run_install_agent_hooks(),
                                AgentCommand::GenerateSkill { output } => run_generate_skill(output),
                                AgentCommand::CheckSkill { skill_file } => run_check_skill(skill_file),
                            }
                        },
                        SmartfoCommand::Info(info_cmd) => {
                            match info_cmd {
                                InfoCommand::List { all, limit, quiet, debug } => {
                                    setup_logging(*debug, *quiet, args.json, args.color.as_deref())?;
                                    run_list(*all, *limit, &args)
                                },
                                InfoCommand::Status { detailed, quiet, debug } => {
                                    setup_logging(*debug, *quiet, args.json, args.color.as_deref())?;
                                    run_status(*detailed, &args)
                                },
                            }
                        },
                        SmartfoCommand::Health(health_cmd) => {
                            match health_cmd {
                                HealthCommand::Check { http, signal, quiet, debug } => {
                                    setup_logging(*debug, *quiet, args.json, args.color.as_deref())?;
                                    run_health_check(*http, *signal)
                                },
                            }
                        },
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
    };

    // Handle result and convert to appropriate exit code
    let exit_code = match result {
        Ok(()) => {
            if signal_handler.was_sigint_received() {
                // SIGINT was received during successful operation
                info!("SIGINT received, exiting with code 130");
                130
            } else {
                // Successful operation
                tracing::debug!("Operation completed successfully, exit code 0");
                0
            }
        }
        Err(e) => {
            // Error occurred
            if signal_handler.was_sigint_received() {
                // SIGINT was received during error
                info!("SIGINT received during error, exiting with code 130");
                130
            } else {
                // Convert error to appropriate exit code
                let exit_code_enum = if let Some(smartfo_err) = e.downcast_ref::<error::SmartfoError>() {
                    // Map SmartfoError to ErrorCategory
                    let category = match smartfo_err {
                        error::SmartfoError::InvalidArgs(_) => ErrorCategory::InvalidArgs,
                        error::SmartfoError::Config(_) => ErrorCategory::Config,
                        error::SmartfoError::PermissionDenied(_) => ErrorCategory::PermissionDenied,
                        error::SmartfoError::Vcs(_) => ErrorCategory::Vcs,
                        error::SmartfoError::Io(io_err) => {
                            match io_err.kind() {
                                std::io::ErrorKind::NotFound => ErrorCategory::IoNotFound,
                                std::io::ErrorKind::PermissionDenied => ErrorCategory::IoPermissionDenied,
                                _ => ErrorCategory::IoOther,
                            }
                        }
                        _ => ErrorCategory::Other,
                    };
                    error_category_to_exit_code(category)
                } else {
                    ExitCode::GenericError
                };

                let code = exit_code_enum.as_i32();
                tracing::debug!("Operation failed with exit code: {} ({})", code, exit_code_enum.description());

                // Print error message
                eprintln!("Error: {}", e);

                code
            }
        }
    };

    Ok(exit_code)
}

// Entry point that calls run_main and exits with the appropriate code
fn main() {
    let exit_code = run_main().unwrap_or_else(|e| {
        eprintln!("ERROR: Fatal error during initialization - {} - Check configuration and environment", e);
        1
    });
    std::process::exit(exit_code);
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

fn run_list_jobs(ids: &Option<String>) -> Result<()> {
    info!("list-jobs command: ids={:?}", ids);

    // Get queue path
    let xdg_data_home = std::env::var("XDG_DATA_HOME")
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME not set");
            format!("{}/.local/share", home)
        });
    let queue_path = std::path::PathBuf::from(xdg_data_home).join("smartfo/queue.db");

    // Try to load jobs from queue
    let queue = match queue::JobQueue::new(&queue_path) {
        Ok(q) => q,
        Err(e) => {
            eprintln!("Failed to open job queue: {}", e);
            eprintln!("No jobs to display");
            return Ok(());
        }
    };

    // Parse job IDs if provided
    let job_ids = ids.as_ref().map(|id_list| {
        id_list.split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>()
    });

    // List jobs
    let jobs = queue.list_jobs(job_ids.as_ref())
        .context("Failed to list jobs")?;

    if jobs.is_empty() {
        println!("No jobs found");
        return Ok(());
    }

    // Display jobs with status
    println!("Background Jobs:");
    println!();
    for job in jobs {
        let status_str = match job.status {
            queue::JobStatus::Queued => "Queued",
            queue::JobStatus::Running => "Running",
            queue::JobStatus::Done => "Done",
            queue::JobStatus::Failed => "Failed",
        };
        let op_type_str = match job.op_type {
            queue::OperationType::Move => "Move",
            queue::OperationType::Copy => "Copy",
            queue::OperationType::Delete => "Delete",
        };

        println!("  ID: {}", job.uuid);
        println!("  Type: {}", op_type_str);
        println!("  Status: {}", status_str);
        println!("  Source: {}", job.source.display());
        if let Some(ref dest) = job.dest {
            println!("  Destination: {}", dest.display());
        }
        println!("  Created: {}", job.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("  Retries: {}", job.retry_count);
        println!();
    }

    Ok(())
}

fn run_cancel_job(job_id: &str) -> Result<()> {
    info!("cancel-job command: job_id={}", job_id);

    // Parse job ID as UUID
    let uuid = uuid::Uuid::parse_str(job_id)
        .context("Invalid job ID format. Expected UUID format.")?;

    // Get queue path
    let xdg_data_home = std::env::var("XDG_DATA_HOME")
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME not set");
            format!("{}/.local/share", home)
        });
    let queue_path = std::path::PathBuf::from(xdg_data_home).join("smartfo/queue.db");

    // Try to load queue
    let queue = match queue::JobQueue::new(&queue_path) {
        Ok(q) => q,
        Err(e) => {
            eprintln!("Failed to open job queue: {}", e);
            eprintln!("Cannot cancel job");
            return Ok(());
        }
    };

    // Cancel the job
    let cancelled = queue.cancel_job(&uuid)
        .context("Failed to cancel job")?;

    if cancelled {
        println!("Job {} cancelled successfully", job_id);
    } else {
        eprintln!("Job {} not found", job_id);
    }

    Ok(())
}

fn run_export_jobs(
    output: &std::path::PathBuf,
    format: &Option<String>,
    status: &Option<String>,
    op_type: &Option<String>,
    date_range: &Option<String>,
) -> Result<()> {
    info!("export-jobs command: output={:?}, format={:?}", output, format);

    // Get queue path
    let xdg_data_home = std::env::var("XDG_DATA_HOME")
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME not set");
            format!("{}/.local/share", home)
        });
    let queue_path = std::path::PathBuf::from(xdg_data_home).join("smartfo/queue.db");

    // Parse export format
    let export_format = match format {
        None => export::ExportFormat::Json,
        Some(f) => export::ExportFormat::from_str(f)?,
    };

    // Parse filters
    let filters = export::ExportFilters {
        status: status.as_ref().and_then(|s| parse_job_status(s)),
        op_type: op_type.as_ref().and_then(|t| parse_operation_type(t)),
        date_range: date_range.as_ref().and_then(|d| parse_date_range(d)),
    };

    // Create export manager
    let export_manager = export::ExportManager::new(queue_path);

    // Export jobs
    export_manager.export_jobs(output, export_format, Some(filters))
        .context("Failed to export jobs")?;

    println!("Exported jobs to: {}", output.display());
    Ok(())
}

fn run_import_jobs(input: &std::path::PathBuf) -> Result<()> {
    info!("import-jobs command: input={:?}", input);

    // Get queue path
    let xdg_data_home = std::env::var("XDG_DATA_HOME")
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME not set");
            format!("{}/.local/share", home)
        });
    let queue_path = std::path::PathBuf::from(xdg_data_home).join("smartfo/queue.db");

    // Create export manager
    let export_manager = export::ExportManager::new(queue_path);

    // Import jobs
    let jobs = export_manager.import_jobs(input)
        .context("Failed to import jobs")?;

    println!("Imported {} jobs from: {}", jobs.len(), input.display());
    println!("Note: Jobs are not automatically re-enqueued. Use queue operations to re-enqueue if needed.");

    Ok(())
}

fn run_analyze_jobs(input: &std::path::PathBuf) -> Result<()> {
    info!("analyze-jobs command: input={:?}", input);

    // Get queue path
    let xdg_data_home = std::env::var("XDG_DATA_HOME")
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME not set");
            format!("{}/.local/share", home)
        });
    let queue_path = std::path::PathBuf::from(xdg_data_home).join("smartfo/queue.db");

    // Create export manager
    let export_manager = export::ExportManager::new(queue_path);

    // Analyze export
    let analysis = export_manager.analyze_export(input)
        .context("Failed to analyze export")?;

    println!("{}", analysis);

    Ok(())
}

/// Parse job status from string
fn parse_job_status(s: &str) -> Option<queue::JobStatus> {
    match s.to_lowercase().as_str() {
        "queued" => Some(queue::JobStatus::Queued),
        "running" => Some(queue::JobStatus::Running),
        "done" => Some(queue::JobStatus::Done),
        "failed" => Some(queue::JobStatus::Failed),
        _ => None,
    }
}

/// Parse operation type from string
fn parse_operation_type(s: &str) -> Option<queue::OperationType> {
    match s.to_lowercase().as_str() {
        "move" => Some(queue::OperationType::Move),
        "copy" => Some(queue::OperationType::Copy),
        "delete" => Some(queue::OperationType::Delete),
        _ => None,
    }
}

/// Parse date range from string (ISO 8601: START,END)
fn parse_date_range(s: &str) -> Option<(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return None;
    }

    let start = chrono::DateTime::parse_from_rfc3339(parts[0].trim())
        .ok()?
        .with_timezone(&chrono::Utc);
    let end = chrono::DateTime::parse_from_rfc3339(parts[1].trim())
        .ok()?
        .with_timezone(&chrono::Utc);

    Some((start, end))
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

    // Generate state summary using the new content_first module
    let state_summary = StateSummary::generate()?;

    // Output based on format
    match output_format {
        OutputFormat::Toon => {
            let toon_output = state_summary.format_toon();
            println!("{}", toon_output);
        }
        OutputFormat::Human => {
            let human_output = state_summary.format_human();
            println!("{}", human_output);
        }
        OutputFormat::Json => {
            let json_output = serde_json::to_string_pretty(&state_summary)?;
            println!("{}", json_output);
        }
    }

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
