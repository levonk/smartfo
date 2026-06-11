//! Platform-specific path fixtures for testing
//!
//! Provides test path data for different platforms.

use super::super::Platform;

/// Get platform-specific test paths
pub fn test_paths(platform: Platform) -> Vec<&'static str> {
    match platform {
        Platform::Linux => vec![
            "/tmp/test",
            "/home/user/test",
            "/var/log/test",
            "/usr/local/bin/test",
        ],
        Platform::MacOS => vec![
            "/tmp/test",
            "/Users/user/test",
            "/var/log/test",
            "/usr/local/bin/test",
        ],
        Platform::Windows => vec![
            "C:\\Temp\\test",
            "C:\\Users\\user\\test",
            "C:\\Program Files\\test",
            "C:\\Windows\\System32\\test",
        ],
        Platform::Unknown => vec![],
    }
}

/// Get platform-specific invalid paths
pub fn invalid_paths(platform: Platform) -> Vec<&'static str> {
    match platform {
        Platform::Linux => vec![
            "",
            "/",
            "//",
            "/../../../etc/passwd",
        ],
        Platform::MacOS => vec![
            "",
            "/",
            "//",
            "/../../../etc/passwd",
        ],
        Platform::Windows => vec![
            "",
            "C:\\",
            "\\\\",
            "C:\\..\\..\\..\\Windows\\System32",
        ],
        Platform::Unknown => vec![],
    }
}
