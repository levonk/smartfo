//! Exit codes and signal handling for smartfo
//!
//! This module provides standard exit codes as specified in ADR #8,
//! along with signal handling for graceful shutdown.

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

// Static global state for signal handling (required for C-compatible signal handlers)
static SIGINT_RECEIVED: AtomicBool = AtomicBool::new(false);
static EXIT_CODE: AtomicI32 = AtomicI32::new(0);

/// Standard exit codes for smartfo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    /// Successful operation
    Success = 0,
    /// Generic error
    GenericError = 1,
    /// Usage error (invalid flags, missing arguments)
    UsageError = 2,
    /// Network error
    NetworkError = 3,
    /// Validation error (config validation, argument validation)
    ValidationError = 4,
    /// File not found error
    FileNotFound = 5,
    /// Permission denied error
    PermissionDenied = 6,
    /// VCS operation failure
    VcsFailed = 7,
    /// Daemon operation failure
    DaemonFailed = 8,
}

impl ExitCode {
    /// Get the integer value of the exit code
    pub fn as_i32(self) -> i32 {
        self as i32
    }
    
    /// Get a human-readable description of the exit code
    pub fn description(self) -> &'static str {
        match self {
            ExitCode::Success => "Operation completed successfully",
            ExitCode::GenericError => "Generic error occurred",
            ExitCode::UsageError => "Invalid usage (invalid flags or missing arguments)",
            ExitCode::NetworkError => "Network error occurred",
            ExitCode::ValidationError => "Validation error (config or argument validation failed)",
            ExitCode::FileNotFound => "File or directory not found",
            ExitCode::PermissionDenied => "Permission denied",
            ExitCode::VcsFailed => "VCS operation failed",
            ExitCode::DaemonFailed => "Daemon operation failed",
        }
    }
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> Self {
        code.as_i32()
    }
}

/// Global signal handler state
#[derive(Clone)]
pub struct SignalHandler;

impl SignalHandler {
    /// Create a new signal handler
    pub fn new() -> Self {
        Self
    }
    
    /// Check if SIGINT was received
    pub fn was_sigint_received(&self) -> bool {
        SIGINT_RECEIVED.load(Ordering::Relaxed)
    }
    
    /// Get the exit code
    pub fn get_exit_code(&self) -> i32 {
        EXIT_CODE.load(Ordering::Relaxed)
    }
    
    /// Set the exit code
    pub fn set_exit_code(&self, code: i32) {
        EXIT_CODE.store(code, Ordering::Relaxed);
    }
    
    /// Setup signal handlers for SIGINT and SIGTERM
    #[cfg(unix)]
    pub fn setup_handlers(&self) -> std::io::Result<()> {
        use nix::sys::signal::{self, SigHandler, Signal};
        use nix::unistd::getpid;
        
        // Setup SIGINT handler
        unsafe {
            signal::signal(Signal::SIGINT, SigHandler::Handler(handle_sigint))?;
        }
        
        // Setup SIGTERM handler for graceful shutdown
        unsafe {
            signal::signal(Signal::SIGTERM, SigHandler::Handler(handle_sigterm))?;
        }
        
        tracing::info!("Signal handlers installed for PID {}", getpid());
        Ok(())
    }
    
    /// No-op for non-Unix platforms
    #[cfg(not(unix))]
    pub fn setup_handlers(&self) -> std::io::Result<()> {
        tracing::warn!("Signal handling not supported on this platform");
        Ok(())
    }
}

/// C-compatible signal handler for SIGINT
#[cfg(unix)]
extern "C" fn handle_sigint(_signal: libc::c_int) {
    SIGINT_RECEIVED.store(true, Ordering::Relaxed);
    EXIT_CODE.store(130, Ordering::Relaxed); // Standard Unix exit code for SIGINT
}

/// C-compatible signal handler for SIGTERM
#[cfg(unix)]
extern "C" fn handle_sigterm(_signal: libc::c_int) {
    SIGINT_RECEIVED.store(true, Ordering::Relaxed);
    EXIT_CODE.store(0, Ordering::Relaxed); // Clean shutdown
}

impl Default for SignalHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert I/O error kind to appropriate exit code
pub fn io_error_to_exit_code(err: &std::io::Error) -> ExitCode {
    match err.kind() {
        std::io::ErrorKind::NotFound => ExitCode::FileNotFound,
        std::io::ErrorKind::PermissionDenied => ExitCode::PermissionDenied,
        _ => ExitCode::GenericError,
    }
}

/// Error category for exit code mapping (used in tests)
#[derive(Debug, Clone, Copy)]
pub enum ErrorCategory {
    InvalidArgs,
    Config,
    PermissionDenied,
    Vcs,
    IoNotFound,
    IoPermissionDenied,
    IoOther,
    Other,
}

/// Convert error category to appropriate exit code (used in tests)
pub fn error_category_to_exit_code(category: ErrorCategory) -> ExitCode {
    match category {
        ErrorCategory::InvalidArgs => ExitCode::UsageError,
        ErrorCategory::Config => ExitCode::ValidationError,
        ErrorCategory::PermissionDenied => ExitCode::PermissionDenied,
        ErrorCategory::Vcs => ExitCode::VcsFailed,
        ErrorCategory::IoNotFound => ExitCode::FileNotFound,
        ErrorCategory::IoPermissionDenied => ExitCode::PermissionDenied,
        ErrorCategory::IoOther => ExitCode::GenericError,
        ErrorCategory::Other => ExitCode::GenericError,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_exit_code_values() {
        assert_eq!(ExitCode::Success.as_i32(), 0);
        assert_eq!(ExitCode::GenericError.as_i32(), 1);
        assert_eq!(ExitCode::UsageError.as_i32(), 2);
        assert_eq!(ExitCode::NetworkError.as_i32(), 3);
        assert_eq!(ExitCode::ValidationError.as_i32(), 4);
        assert_eq!(ExitCode::FileNotFound.as_i32(), 5);
        assert_eq!(ExitCode::PermissionDenied.as_i32(), 6);
        assert_eq!(ExitCode::VcsFailed.as_i32(), 7);
        assert_eq!(ExitCode::DaemonFailed.as_i32(), 8);
    }
    
    #[test]
    fn test_exit_code_descriptions() {
        assert!(!ExitCode::Success.description().is_empty());
        assert!(!ExitCode::GenericError.description().is_empty());
        assert!(!ExitCode::UsageError.description().is_empty());
    }
    
    #[test]
    fn test_signal_handler_initial_state() {
        let handler = SignalHandler::new();
        assert!(!handler.was_sigint_received());
        // Note: exit code is global static state, so we don't assert initial value
        // since it may have been modified by other tests
    }
    
    #[test]
    fn test_signal_handler_set_exit_code() {
        let handler = SignalHandler::new();
        handler.set_exit_code(42);
        assert_eq!(handler.get_exit_code(), 42);
    }
    
    #[test]
    fn test_error_category_to_exit_code_mapping() {
        assert_eq!(
            error_category_to_exit_code(ErrorCategory::InvalidArgs),
            ExitCode::UsageError
        );
        
        assert_eq!(
            error_category_to_exit_code(ErrorCategory::Config),
            ExitCode::ValidationError
        );
        
        assert_eq!(
            error_category_to_exit_code(ErrorCategory::PermissionDenied),
            ExitCode::PermissionDenied
        );
        
        assert_eq!(
            error_category_to_exit_code(ErrorCategory::Vcs),
            ExitCode::VcsFailed
        );
    }
    
    #[test]
    fn test_io_error_to_exit_code() {
        let not_found = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        assert_eq!(
            io_error_to_exit_code(&not_found),
            ExitCode::FileNotFound
        );
        
        let permission_denied = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "test");
        assert_eq!(
            io_error_to_exit_code(&permission_denied),
            ExitCode::PermissionDenied
        );
        
        let other = std::io::Error::new(std::io::ErrorKind::Other, "test");
        assert_eq!(
            io_error_to_exit_code(&other),
            ExitCode::GenericError
        );
    }
}
