use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use tracing_appender::non_blocking::WorkerGuard;
use tracing::Level;
use std::io;
use crate::secret::sanitize_string;

/// Log level hierarchy for CLI flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Verbose,
    Info,
    Warn,
    Error,
    Quiet,
}

impl LogLevel {
    /// Convert LogLevel to tracing Level
    pub fn to_tracing_level(self) -> Option<Level> {
        match self {
            LogLevel::Debug => Some(Level::DEBUG),
            LogLevel::Verbose => Some(Level::INFO), // Verbose maps to INFO with more detail
            LogLevel::Info => Some(Level::INFO),
            LogLevel::Warn => Some(Level::WARN),
            LogLevel::Error => Some(Level::ERROR),
            LogLevel::Quiet => None, // Quiet suppresses all logging
        }
    }

    /// Resolve log level from CLI flags
    /// Priority: --debug > --quiet > default (info)
    pub fn from_cli_flags(debug: bool, quiet: bool) -> Self {
        if debug {
            LogLevel::Debug
        } else if quiet {
            LogLevel::Quiet
        } else {
            LogLevel::Info
        }
    }

    /// Convert to string for EnvFilter
    pub fn to_filter_string(self) -> String {
        match self {
            LogLevel::Debug => "debug".to_string(),
            LogLevel::Verbose => "info".to_string(),
            LogLevel::Info => "info".to_string(),
            LogLevel::Warn => "warn".to_string(),
            LogLevel::Error => "error".to_string(),
            LogLevel::Quiet => "off".to_string(),
        }
    }
}

/// Log output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    Pretty,
    Json,
}

impl LogFormat {
    pub fn from_env() -> Option<Self> {
        match std::env::var("SMARTFO_LOG_FORMAT").as_deref() {
            Ok("json") => Some(LogFormat::Json),
            Ok("pretty") => Some(LogFormat::Pretty),
            _ => None,
        }
    }

    pub fn from_cli_arg(arg: &str) -> Option<Self> {
        match arg {
            "json" => Some(LogFormat::Json),
            "pretty" => Some(LogFormat::Pretty),
            _ => None,
        }
    }
}

/// Initialize the tracing subscriber with configurable format and log level
///
/// Resolution order (highest wins):
/// 1. CLI flags (--debug, --quiet, --log-format)
/// 2. Environment variables (SMARTFO_LOG_LEVEL, SMARTFO_LOG_FORMAT, RUST_LOG)
/// 3. Config file values
/// 4. Built-in defaults
pub fn init_logging(
    debug: bool,
    quiet: bool,
    cli_format: Option<&str>,
    config_level: Option<&str>,
    config_format: Option<&str>,
    log_file: Option<&str>,
) -> Option<WorkerGuard> {
    // Resolve log format: CLI > env > config > default (pretty if TTY)
    let format = cli_format
        .and_then(|f| LogFormat::from_cli_arg(f))
        .or_else(LogFormat::from_env)
        .or_else(|| config_format.and_then(|f| match f {
            "json" => Some(LogFormat::Json),
            "pretty" => Some(LogFormat::Pretty),
            _ => None,
        }))
        .unwrap_or_else(|| {
            if atty::is(atty::Stream::Stderr) {
                LogFormat::Pretty
            } else {
                LogFormat::Json
            }
        });

    // Resolve log level: CLI flags > env > config > default (info)
    let log_level = LogLevel::from_cli_flags(debug, quiet);

    let env_filter = if log_level == LogLevel::Quiet {
        // Quiet mode suppresses all logging
        EnvFilter::new("off")
    } else {
        EnvFilter::try_from_default_env()
            .or_else(|_| {
                let level = log_level.to_filter_string();
                EnvFilter::try_new(&level)
            })
            .unwrap_or_else(|_| EnvFilter::new("info"))
    };

    let registry = tracing_subscriber::registry().with(env_filter);

    let guard = if let Some(path) = log_file {
        // File logging
        let file_appender = tracing_appender::rolling::never(
            std::path::Path::new(path).parent().unwrap_or(std::path::Path::new(".")),
            std::path::Path::new(path).file_name().unwrap_or(std::ffi::OsStr::new("smartfo.log")),
        );
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        let layer = match format {
            LogFormat::Json => fmt::layer().json().with_writer(non_blocking).boxed(),
            LogFormat::Pretty => fmt::layer().pretty().with_writer(non_blocking).boxed(),
        };

        registry.with(layer).init();
        Some(guard)
    } else {
        // Console logging only - explicitly use stderr for output discipline
        let layer = match format {
            LogFormat::Json => fmt::layer().json().with_writer(io::stderr).boxed(),
            LogFormat::Pretty => fmt::layer().pretty().with_writer(io::stderr).boxed(),
        };

        registry.with(layer).init();
        None
    };

    guard
}

/// Check if JSON output is requested (for machine-readable logs)
pub fn is_json_format() -> bool {
    LogFormat::from_env() == Some(LogFormat::Json)
}

/// Try to initialize logging, but don't panic if already initialized
/// This is useful for subcommands that may be called after logging is already set up
pub fn try_init_logging(
    debug: bool,
    quiet: bool,
    cli_format: Option<&str>,
    config_level: Option<&str>,
    config_format: Option<&str>,
    log_file: Option<&str>,
) -> Option<WorkerGuard> {
    // Use a static flag to track if logging has been initialized
    use std::sync::atomic::{AtomicBool, Ordering};
    static LOGGING_INITIALIZED: AtomicBool = AtomicBool::new(false);

    // Check if logging is already initialized
    if LOGGING_INITIALIZED.load(Ordering::SeqCst) {
        return None;
    }

    // Initialize logging normally
    let guard = init_logging(debug, quiet, cli_format, config_level, config_format, log_file);

    // Mark as initialized
    LOGGING_INITIALIZED.store(true, Ordering::SeqCst);

    guard
}
