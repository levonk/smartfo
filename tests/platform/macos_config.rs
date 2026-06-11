//! macOS-specific test configuration
//!
//! Platform-specific test settings for macOS platforms.

use super::TestConfig;

/// Get macOS-specific test configuration
pub fn macos_config() -> TestConfig {
    TestConfig {
        platform: super::Platform::MacOS,
        supports_daemon: true,
        supports_unix_sockets: true,
        supports_signals: true,
        path_separator: ':',
        dir_separator: '/',
    }
}

#[cfg(test)]
#[cfg(target_os = "macos")]
mod tests {
    use super::*;

    #[test]
    fn test_macos_config() {
        let config = macos_config();
        assert!(config.supports_daemon);
        assert!(config.supports_unix_sockets);
        assert!(config.supports_signals);
        assert_eq!(config.path_separator, ':');
        assert_eq!(config.dir_separator, '/');
    }
}
