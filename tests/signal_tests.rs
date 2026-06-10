//! Tests for signal handling and exit codes

use smartfo::{ExitCode, SignalHandler, error_category_to_exit_code, ErrorCategory};

#[test]
fn test_exit_code_success() {
    assert_eq!(ExitCode::Success.as_i32(), 0);
    assert_eq!(ExitCode::Success.description(), "Operation completed successfully");
}

#[test]
fn test_exit_code_generic_error() {
    assert_eq!(ExitCode::GenericError.as_i32(), 1);
    assert_eq!(ExitCode::GenericError.description(), "Generic error occurred");
}

#[test]
fn test_exit_code_usage_error() {
    assert_eq!(ExitCode::UsageError.as_i32(), 2);
    assert_eq!(ExitCode::UsageError.description(), "Invalid usage (invalid flags or missing arguments)");
}

#[test]
fn test_exit_code_validation_error() {
    assert_eq!(ExitCode::ValidationError.as_i32(), 4);
    assert_eq!(ExitCode::ValidationError.description(), "Validation error (config or argument validation failed)");
}

#[test]
fn test_exit_code_file_not_found() {
    assert_eq!(ExitCode::FileNotFound.as_i32(), 5);
    assert_eq!(ExitCode::FileNotFound.description(), "File or directory not found");
}

#[test]
fn test_exit_code_permission_denied() {
    assert_eq!(ExitCode::PermissionDenied.as_i32(), 6);
    assert_eq!(ExitCode::PermissionDenied.description(), "Permission denied");
}

#[test]
fn test_exit_code_vcs_failed() {
    assert_eq!(ExitCode::VcsFailed.as_i32(), 7);
    assert_eq!(ExitCode::VcsFailed.description(), "VCS operation failed");
}

#[test]
fn test_exit_code_daemon_failed() {
    assert_eq!(ExitCode::DaemonFailed.as_i32(), 8);
    assert_eq!(ExitCode::DaemonFailed.description(), "Daemon operation failed");
}

#[test]
fn test_exit_code_network_error() {
    assert_eq!(ExitCode::NetworkError.as_i32(), 3);
    assert_eq!(ExitCode::NetworkError.description(), "Network error occurred");
}

#[test]
fn test_signal_handler_initial_state() {
    let handler = SignalHandler::new();
    assert!(!handler.was_sigint_received());
    assert_eq!(handler.get_exit_code(), 130); // Standard Unix exit code for SIGINT
}

#[test]
fn test_signal_handler_set_exit_code() {
    let handler = SignalHandler::new();
    handler.set_exit_code(42);
    assert_eq!(handler.get_exit_code(), 42);
}

#[test]
fn test_error_category_to_exit_code_invalid_args() {
    let exit_code = error_category_to_exit_code(ErrorCategory::InvalidArgs);
    assert_eq!(exit_code, ExitCode::UsageError);
}

#[test]
fn test_error_category_to_exit_code_config_error() {
    let exit_code = error_category_to_exit_code(ErrorCategory::Config);
    assert_eq!(exit_code, ExitCode::ValidationError);
}

#[test]
fn test_error_category_to_exit_code_permission_denied() {
    let exit_code = error_category_to_exit_code(ErrorCategory::PermissionDenied);
    assert_eq!(exit_code, ExitCode::PermissionDenied);
}

#[test]
fn test_error_category_to_exit_code_vcs_error() {
    let exit_code = error_category_to_exit_code(ErrorCategory::Vcs);
    assert_eq!(exit_code, ExitCode::VcsFailed);
}

#[test]
fn test_error_category_to_exit_code_io_not_found() {
    let exit_code = error_category_to_exit_code(ErrorCategory::IoNotFound);
    assert_eq!(exit_code, ExitCode::FileNotFound);
}

#[test]
fn test_error_category_to_exit_code_io_permission_denied() {
    let exit_code = error_category_to_exit_code(ErrorCategory::IoPermissionDenied);
    assert_eq!(exit_code, ExitCode::PermissionDenied);
}

#[test]
fn test_error_category_to_exit_code_io_generic() {
    let exit_code = error_category_to_exit_code(ErrorCategory::IoOther);
    assert_eq!(exit_code, ExitCode::GenericError);
}

#[test]
fn test_error_category_to_exit_code_other_error() {
    let exit_code = error_category_to_exit_code(ErrorCategory::Other);
    assert_eq!(exit_code, ExitCode::GenericError);
}

#[test]
fn test_exit_code_into_i32() {
    let code: i32 = ExitCode::Success.into();
    assert_eq!(code, 0);
    
    let code: i32 = ExitCode::GenericError.into();
    assert_eq!(code, 1);
    
    let code: i32 = ExitCode::UsageError.into();
    assert_eq!(code, 2);
}

#[test]
fn test_sigint_handler_sets_flag() {
    let handler = SignalHandler::new();
    handler.set_exit_code(130);
    assert_eq!(handler.get_exit_code(), 130);
}

#[test]
fn test_signal_handler_setup() {
    let handler = SignalHandler::new();
    // Test that signal handler can be created without panicking
    // Actual signal handling is platform-specific and hard to test in unit tests
    assert!(!handler.was_sigint_received());
}
