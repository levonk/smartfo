//! Platform detection tests
//!
//! Tests for the platform detection utilities in the platform module.

use smartfo::tests::platform::{Platform, TestConfig};

#[test]
fn test_platform_detection() {
    let platform = Platform::current();
    // Should not be unknown on supported platforms
    assert_ne!(platform, Platform::Unknown);
}

#[test]
fn test_platform_properties() {
    let platform = Platform::current();

    // Test is_unix
    assert_eq!(platform.is_unix(), matches!(platform, Platform::Linux | Platform::MacOS));

    // Test is_windows
    assert_eq!(platform.is_windows(), matches!(platform, Platform::Windows));

    // Test supports_daemon
    assert_eq!(platform.supports_daemon(), platform.is_unix());

    // Test supports_unix_sockets
    assert_eq!(platform.supports_unix_sockets(), platform.is_unix());

    // Test supports_signals
    assert_eq!(platform.supports_signals(), platform.is_unix());
}

#[test]
fn test_path_separators() {
    let platform = Platform::current();

    if platform.is_windows() {
        assert_eq!(platform.path_separator(), ';');
        assert_eq!(platform.dir_separator(), '\\');
    } else {
        assert_eq!(platform.path_separator(), ':');
        assert_eq!(platform.dir_separator(), '/');
    }
}

#[test]
fn test_test_config() {
    let config = TestConfig::current();

    assert_eq!(config.platform, Platform::current());
    assert_eq!(config.supports_daemon, config.platform.supports_daemon());
    assert_eq!(config.supports_unix_sockets, config.platform.supports_unix_sockets());
    assert_eq!(config.supports_signals, config.platform.supports_signals());
    assert_eq!(config.path_separator, config.platform.path_separator());
    assert_eq!(config.dir_separator, config.platform.dir_separator());
}

// Integration tests for platform detection
#[test]
fn test_platform_name() {
    let platform = Platform::current();
    let name = platform.name();

    match platform {
        Platform::Linux => assert_eq!(name, "linux"),
        Platform::MacOS => assert_eq!(name, "macos"),
        Platform::Windows => assert_eq!(name, "windows"),
        Platform::Unknown => assert_eq!(name, "unknown"),
    }
}

#[test]
fn test_platform_consistency() {
    let platform = Platform::current();
    let config = TestConfig::current();

    // Ensure config platform matches detected platform
    assert_eq!(config.platform, platform);

    // Ensure all platform-specific properties are consistent
    assert_eq!(config.supports_daemon, platform.supports_daemon());
    assert_eq!(config.supports_unix_sockets, platform.supports_unix_sockets());
    assert_eq!(config.supports_signals, platform.supports_signals());
}

#[test]
fn test_daemon_support_consistency() {
    let platform = Platform::current();

    // Daemon support should be consistent with Unix support
    assert_eq!(platform.supports_daemon(), platform.is_unix());
    assert_eq!(platform.supports_fork(), platform.is_unix());
    assert_eq!(platform.supports_double_fork(), platform.is_unix());
}

#[test]
fn test_unix_socket_support_consistency() {
    let platform = Platform::current();

    // Unix socket support should be consistent with Unix support
    assert_eq!(platform.supports_unix_sockets(), platform.is_unix());
}

#[test]
fn test_signal_support_consistency() {
    let platform = Platform::current();

    // Signal support should be consistent with Unix support
    assert_eq!(platform.supports_signals(), platform.is_unix());
}

#[test]
fn test_filesystem_feature_consistency() {
    let platform = Platform::current();

    // Symlinks and file permissions should be consistent with Unix support
    assert_eq!(platform.supports_symlinks(), platform.is_unix());
    assert_eq!(platform.supports_file_permissions(), platform.is_unix());
}
