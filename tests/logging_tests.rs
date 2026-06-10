use smartfo::logging::{LogLevel, LogFormat};

#[test]
fn test_log_level_hierarchy() {
    // Test log level hierarchy: debug > verbose > info > warn > error
    let debug = LogLevel::Debug;
    let verbose = LogLevel::Verbose;
    let info = LogLevel::Info;
    let warn = LogLevel::Warn;
    let error = LogLevel::Error;
    let quiet = LogLevel::Quiet;

    // Test conversion to tracing levels
    assert_eq!(debug.to_tracing_level(), Some(tracing::Level::DEBUG));
    assert_eq!(verbose.to_tracing_level(), Some(tracing::Level::INFO));
    assert_eq!(info.to_tracing_level(), Some(tracing::Level::INFO));
    assert_eq!(warn.to_tracing_level(), Some(tracing::Level::WARN));
    assert_eq!(error.to_tracing_level(), Some(tracing::Level::ERROR));
    assert_eq!(quiet.to_tracing_level(), None); // Quiet suppresses all logging

    // Test filter string conversion
    assert_eq!(debug.to_filter_string(), "debug");
    assert_eq!(verbose.to_filter_string(), "info");
    assert_eq!(info.to_filter_string(), "info");
    assert_eq!(warn.to_filter_string(), "warn");
    assert_eq!(error.to_filter_string(), "error");
    assert_eq!(quiet.to_filter_string(), "off");
}

#[test]
fn test_log_level_from_cli_flags() {
    // Test --debug flag takes precedence
    assert_eq!(LogLevel::from_cli_flags(true, false), LogLevel::Debug);
    
    // Test --quiet flag
    assert_eq!(LogLevel::from_cli_flags(false, true), LogLevel::Quiet);
    
    // Test default (no flags)
    assert_eq!(LogLevel::from_cli_flags(false, false), LogLevel::Info);
    
    // Test --debug takes precedence over --quiet
    assert_eq!(LogLevel::from_cli_flags(true, true), LogLevel::Debug);
}

#[test]
fn test_quiet_flag_behavior() {
    // Test that quiet mode suppresses logging
    let quiet_level = LogLevel::Quiet;
    assert_eq!(quiet_level.to_tracing_level(), None);
    assert_eq!(quiet_level.to_filter_string(), "off");
}

#[test]
fn test_debug_flag_behavior() {
    // Test that debug mode enables detailed logging
    let debug_level = LogLevel::Debug;
    assert_eq!(debug_level.to_tracing_level(), Some(tracing::Level::DEBUG));
    assert_eq!(debug_level.to_filter_string(), "debug");
}

#[test]
fn test_verbose_flag_behavior() {
    // Test that verbose mode maps to INFO with more detail
    let verbose_level = LogLevel::Verbose;
    assert_eq!(verbose_level.to_tracing_level(), Some(tracing::Level::INFO));
    assert_eq!(verbose_level.to_filter_string(), "info");
}

#[test]
fn test_log_format_from_env() {
    // Test environment variable parsing for log format
    std::env::set_var("SMARTFO_LOG_FORMAT", "json");
    assert_eq!(LogFormat::from_env(), Some(LogFormat::Json));
    std::env::remove_var("SMARTFO_LOG_FORMAT");

    std::env::set_var("SMARTFO_LOG_FORMAT", "pretty");
    assert_eq!(LogFormat::from_env(), Some(LogFormat::Pretty));
    std::env::remove_var("SMARTFO_LOG_FORMAT");

    // Test invalid format returns None
    std::env::set_var("SMARTFO_LOG_FORMAT", "invalid");
    assert_eq!(LogFormat::from_env(), None);
    std::env::remove_var("SMARTFO_LOG_FORMAT");
}

#[test]
fn test_log_format_from_cli_arg() {
    // Test CLI argument parsing for log format
    assert_eq!(LogFormat::from_cli_arg("json"), Some(LogFormat::Json));
    assert_eq!(LogFormat::from_cli_arg("pretty"), Some(LogFormat::Pretty));
    assert_eq!(LogFormat::from_cli_arg("invalid"), None);
}
