//! Windows-specific test configuration
//!
//! Platform-specific test settings for Windows platforms.

use super::TestConfig;

/// Get Windows-specific test configuration
pub fn windows_config() -> TestConfig {
    TestConfig {
        platform: super::Platform::Windows,
        supports_daemon: false,
        supports_unix_sockets: false,
        supports_signals: false,
        path_separator: ';',
        dir_separator: '\\',
    }
}

#[cfg(test)]
#[cfg(target_os = "windows")]
mod tests {
    use super::*;

    #[test]
    fn test_windows_config() {
        let config = windows_config();
        assert!(!config.supports_daemon);
        assert!(!config.supports_unix_sockets);
        assert!(!config.supports_signals);
        assert_eq!(config.path_separator, ';');
        assert_eq!(config.dir_separator, '\\');
    }
}
