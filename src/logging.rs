use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use tracing_appender::non_blocking::WorkerGuard;
use std::io;

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

    pub fn from_cli_arg(arg: Option<&str>) -> Option<Self> {
        match arg {
            Some("json") => Some(LogFormat::Json),
            Some("pretty") => Some(LogFormat::Pretty),
            _ => None,
        }
    }
}

/// Initialize the tracing subscriber with configurable format and log level
///
/// Resolution order (highest wins):
/// 1. CLI flags (--log-level, --log-format)
/// 2. Environment variables (SMARTFO_LOG_LEVEL, SMARTFO_LOG_FORMAT, RUST_LOG)
/// 3. Config file values
/// 4. Built-in defaults
pub fn init_logging(
    cli_level: Option<&str>,
    cli_format: Option<&str>,
    config_level: Option<&str>,
    config_format: Option<&str>,
    log_file: Option<&str>,
) -> Option<WorkerGuard> {
    // Resolve log format: CLI > env > config > default (pretty if TTY)
    let format = cli_format
        .and_then(LogFormat::from_cli_arg)
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

    // Resolve log level: CLI > env > config > default (info)
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| {
            let level = cli_level
                .or_else(|| std::env::var("SMARTFO_LOG_LEVEL").ok().as_deref())
                .or(config_level)
                .unwrap_or("info");
            EnvFilter::try_new(level)
        })
        .unwrap_or_else(|_| EnvFilter::new("info"));

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
        // Console logging only
        let layer = match format {
            LogFormat::Json => fmt::layer().json().boxed(),
            LogFormat::Pretty => fmt::layer().pretty().boxed(),
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
