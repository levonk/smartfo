//! Linux-specific test configuration
//!
//! Platform-specific test settings for Linux platforms.

use super::TestConfig;

/// Get Linux-specific test configuration
pub fn linux_config() -> TestConfig {
    TestConfig {
        platform: super::Platform::Linux,
        supports_daemon: true,
        supports_unix_sockets: true,
        supports_signals: true,
        path_separator: ':',
        dir_separator: '/',
    }
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod tests {
    use super::*;

    #[test]
    fn test_linux_config() {
        let config = linux_config();
        assert!(config.supports_daemon);
        assert!(config.supports_unix_sockets);
        assert!(config.supports_signals);
        assert_eq!(config.path_separator, ':');
        assert_eq!(config.dir_separator, '/');
    }
}
