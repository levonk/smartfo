use clap::Parser;
use anyhow::Result;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter, prelude::*};

mod cli;
use cli::{MvArgs, RmArgs, SmartfoArgs, SmartfoCommand};

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
    if args.dry_run {
        info!("dry-run: rm {:?}", args.paths);
        return Ok(());
    }

    // TODO: Implement trash enqueueing (story 03-002)
    if args.paths.is_empty() {
        anyhow::bail!("missing operand");
    }

    info!("rm mode: paths={:?}", args.paths);
    Ok(())
}

fn run_install(args: &SmartfoArgs) -> Result<()> {
    info!("install mode: hooks={:?} no_hooks={} force={}", args.hooks, args.no_hooks, args.force);

    // TODO: Implement symlink creation and hook installation (story 02-002)
    println!("smartfo: install mode not yet fully implemented");
    Ok(())
}

fn run_git_hook_client() -> Result<()> {
    // TODO: Implement pre-commit hook (story 05-001)
    info!("git-hook-client: verifying staged changes against audit log");
    Ok(())
}

fn run_git_hook_server() -> Result<()> {
    // TODO: Implement pre-receive hook (story 05-001)
    info!("git-hook-server: verifying incoming push against audit log");
    Ok(())
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
