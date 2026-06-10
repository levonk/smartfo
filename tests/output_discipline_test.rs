//! Tests for output discipline (results to stdout, logs to stderr)

use std::io::Read;
use std::process::{Command, Stdio};
use tempfile::TempDir;
use smartfo::ColorMode;

#[test]
fn test_tty_color_detection() {
    // Test that color mode respects TTY detection
    // This is difficult to test in automated environment
    // We'll test the ColorMode logic instead
    let auto_mode = ColorMode::Auto;
    let always_mode = ColorMode::Always;
    let never_mode = ColorMode::Never;

    // Test parsing
    assert_eq!(ColorMode::from_str("auto"), Some(auto_mode));
    assert_eq!(ColorMode::from_str("always"), Some(always_mode));
    assert_eq!(ColorMode::from_str("never"), Some(never_mode));
    assert_eq!(ColorMode::from_str("invalid"), None);

    // Test should_color logic
    assert_eq!(always_mode.should_color(), true);
    assert_eq!(never_mode.should_color(), false);
    // Auto mode depends on TTY, which we can't easily test
}

#[test]
fn test_no_color_env_precedence() {
    // Test that NO_COLOR environment variable takes precedence
    let original_no_color = std::env::var("NO_COLOR");
    
    // Set NO_COLOR
    std::env::set_var("NO_COLOR", "1");
    
    let color_mode = ColorMode::determine(Some("always"), "auto");
    assert_eq!(color_mode, ColorMode::Never);
    
    // Restore original value
    match original_no_color {
        Ok(val) => std::env::set_var("NO_COLOR", val),
        Err(_) => std::env::remove_var("NO_COLOR"),
    }
}

#[test]
fn test_color_config_precedence() {
    // Test precedence: CLI flag > config > default
    let cli_override = ColorMode::determine(Some("never"), "always");
    assert_eq!(cli_override, ColorMode::Never);
    
    let config_only = ColorMode::determine(None, "always");
    assert_eq!(config_only, ColorMode::Always);
    
    let default = ColorMode::determine(None, "auto");
    assert_eq!(default, ColorMode::Auto);
}
