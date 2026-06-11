//! Platform detection and utilities for cross-platform testing
//!
//! This module provides utilities to detect the current platform and
//! handle platform-specific test scenarios for Linux, macOS, and Windows.

mod linux_config;
mod macos_config;
mod windows_config;

pub mod fixtures;

use std::env;

/// Represents the detected operating system platform
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Linux,
    MacOS,
    Windows,
    Unknown,
}

impl Platform {
    /// Detect the current platform at runtime
    pub fn current() -> Self {
        match env::consts::OS {
            "linux" => Platform::Linux,
            "macos" => Platform::MacOS,
            "windows" => Platform::Windows,
            _ => Platform::Unknown,
        }
    }

    /// Check if the current platform is Unix-like (Linux or macOS)
    pub fn is_unix(&self) -> bool {
        matches!(self, Platform::Linux | Platform::MacOS)
    }

    /// Check if the current platform is Windows
    pub fn is_windows(&self) -> bool {
        matches!(self, Platform::Windows)
    }

    /// Get the platform name as a string
    pub fn name(&self) -> &'static str {
        match self {
            Platform::Linux => "linux",
            Platform::MacOS => "macos",
            Platform::Windows => "windows",
            Platform::Unknown => "unknown",
        }
    }

    /// Check if daemon mode is supported on this platform
    pub fn supports_daemon(&self) -> bool {
        self.is_unix()
    }

    /// Check if Unix domain sockets are supported on this platform
    pub fn supports_unix_sockets(&self) -> bool {
        self.is_unix()
    }

    /// Check if signal handling is supported on this platform
    pub fn supports_signals(&self) -> bool {
        self.is_unix()
    }

    /// Get the appropriate path separator for the platform
    pub fn path_separator(&self) -> char {
        if self.is_windows() {
            ';'
        } else {
            ':'
        }
    }

    /// Get the appropriate directory separator for the platform
    pub fn dir_separator(&self) -> char {
        if self.is_windows() {
            '\\'
        } else {
            '/'
        }
    }

    /// Normalize a path for the current platform
    pub fn normalize_path(&self, path: &str) -> String {
        if self.is_windows() {
            path.replace('/', "\\")
        } else {
            path.replace('\\', "/")
        }
    }

    /// Check if a path is absolute for the current platform
    pub fn is_absolute_path(&self, path: &str) -> bool {
        if self.is_windows() {
            // Windows: C:\ or \\
            path.len() >= 3 && path.chars().nth(1) == Some(':') && path.chars().nth(2) == Some('\\')
                || path.starts_with("\\\\")
        } else {
            // Unix: starts with /
            path.starts_with('/')
        }
    }

    /// Get the appropriate temp directory for the platform
    pub fn temp_dir(&self) -> &'static str {
        if self.is_windows() {
            "C:\\Temp"
        } else {
            "/tmp"
        }
    }

    /// Get the appropriate home directory environment variable
    pub fn home_env_var(&self) -> &'static str {
        if self.is_windows() {
            "USERPROFILE"
        } else {
            "HOME"
        }
    }

    /// Get the appropriate socket directory for daemon communication
    pub fn socket_dir(&self) -> &'static str {
        if self.is_windows() {
            // Windows doesn't support Unix sockets, return a placeholder
            "C:\\Temp\\smartfo"
        } else {
            "/tmp/smartfo"
        }
    }

    /// Get the appropriate PID file directory for daemon
    pub fn pid_dir(&self) -> &'static str {
        if self.is_windows() {
            "C:\\Temp\\smartfo"
        } else {
            "/tmp/smartfo"
        }
    }

    /// Check if the platform supports forking for daemonization
    pub fn supports_fork(&self) -> bool {
        self.is_unix()
    }

    /// Check if the platform supports double-fork for daemonization
    pub fn supports_double_fork(&self) -> bool {
        self.is_unix()
    }

    /// Get the appropriate VCS command for the platform (e.g., git.exe on Windows)
    pub fn git_command(&self) -> &'static str {
        if self.is_windows() {
            "git.exe"
        } else {
            "git"
        }
    }

    /// Get the appropriate VCS command for the platform (e.g., hg.exe on Windows)
    pub fn hg_command(&self) -> &'static str {
        if self.is_windows() {
            "hg.exe"
        } else {
            "hg"
        }
    }

    /// Get the appropriate VCS command for the platform (e.g., svn.exe on Windows)
    pub fn svn_command(&self) -> &'static str {
        if self.is_windows() {
            "svn.exe"
        } else {
            "svn"
        }
    }

    /// Get the appropriate VCS command for the platform (e.g., jj.exe on Windows)
    pub fn jj_command(&self) -> &'static str {
        if self.is_windows() {
            "jj.exe"
        } else {
            "jj"
        }
    }

    /// Check if the platform supports symlinks
    pub fn supports_symlinks(&self) -> bool {
        self.is_unix()
    }

    /// Check if the platform supports file permissions (chmod)
    pub fn supports_file_permissions(&self) -> bool {
        self.is_unix()
    }
}

/// Platform-specific test configuration
pub struct TestConfig {
    pub platform: Platform,
    pub supports_daemon: bool,
    pub supports_unix_sockets: bool,
    pub supports_signals: bool,
    pub path_separator: char,
    pub dir_separator: char,
}

impl TestConfig {
    /// Create a test configuration for the current platform
    pub fn current() -> Self {
        let platform = Platform::current();
        Self {
            platform,
            supports_daemon: platform.supports_daemon(),
            supports_unix_sockets: platform.supports_unix_sockets(),
            supports_signals: platform.supports_signals(),
            path_separator: platform.path_separator(),
            dir_separator: platform.dir_separator(),
        }
    }

    /// Skip the test if the platform is not the expected one
    pub fn skip_unless_platform(&self, expected: Platform) {
        if self.platform != expected {
            eprintln!("Skipping test: expected platform {:?}, got {:?}", expected, self.platform);
        }
    }

    /// Skip the test if the platform is Windows
    pub fn skip_if_windows(&self) {
        if self.platform.is_windows() {
            eprintln!("Skipping test: not supported on Windows");
        }
    }

    /// Skip the test if the platform is Unix
    pub fn skip_if_unix(&self) {
        if self.platform.is_unix() {
            eprintln!("Skipping test: not supported on Unix platforms");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_normalize_path() {
        let platform = Platform::current();

        if platform.is_windows() {
            assert_eq!(platform.normalize_path("foo/bar"), "foo\\bar");
            assert_eq!(platform.normalize_path("foo/bar/baz"), "foo\\bar\\baz");
        } else {
            assert_eq!(platform.normalize_path("foo\\bar"), "foo/bar");
            assert_eq!(platform.normalize_path("foo\\bar\\baz"), "foo/bar/baz");
        }
    }

    #[test]
    fn test_is_absolute_path() {
        let platform = Platform::current();

        if platform.is_windows() {
            assert!(platform.is_absolute_path("C:\\"));
            assert!(platform.is_absolute_path("C:\\foo"));
            assert!(platform.is_absolute_path("\\\\server\\share"));
            assert!(!platform.is_absolute_path("foo"));
            assert!(!platform.is_absolute_path("foo\\bar"));
        } else {
            assert!(platform.is_absolute_path("/"));
            assert!(platform.is_absolute_path("/foo"));
            assert!(platform.is_absolute_path("/foo/bar"));
            assert!(!platform.is_absolute_path("foo"));
            assert!(!platform.is_absolute_path("foo/bar"));
        }
    }

    #[test]
    fn test_temp_dir() {
        let platform = Platform::current();

        if platform.is_windows() {
            assert_eq!(platform.temp_dir(), "C:\\Temp");
        } else {
            assert_eq!(platform.temp_dir(), "/tmp");
        }
    }

    #[test]
    fn test_home_env_var() {
        let platform = Platform::current();

        if platform.is_windows() {
            assert_eq!(platform.home_env_var(), "USERPROFILE");
        } else {
            assert_eq!(platform.home_env_var(), "HOME");
        }
    }

    #[test]
    fn test_socket_dir() {
        let platform = Platform::current();

        if platform.is_windows() {
            assert_eq!(platform.socket_dir(), "C:\\Temp\\smartfo");
        } else {
            assert_eq!(platform.socket_dir(), "/tmp/smartfo");
        }
    }

    #[test]
    fn test_pid_dir() {
        let platform = Platform::current();

        if platform.is_windows() {
            assert_eq!(platform.pid_dir(), "C:\\Temp\\smartfo");
        } else {
            assert_eq!(platform.pid_dir(), "/tmp/smartfo");
        }
    }

    #[test]
    fn test_supports_fork() {
        let platform = Platform::current();
        assert_eq!(platform.supports_fork(), platform.is_unix());
    }

    #[test]
    fn test_supports_double_fork() {
        let platform = Platform::current();
        assert_eq!(platform.supports_double_fork(), platform.is_unix());
    }

    #[test]
    fn test_git_command() {
        let platform = Platform::current();

        if platform.is_windows() {
            assert_eq!(platform.git_command(), "git.exe");
        } else {
            assert_eq!(platform.git_command(), "git");
        }
    }

    #[test]
    fn test_hg_command() {
        let platform = Platform::current();

        if platform.is_windows() {
            assert_eq!(platform.hg_command(), "hg.exe");
        } else {
            assert_eq!(platform.hg_command(), "hg");
        }
    }

    #[test]
    fn test_svn_command() {
        let platform = Platform::current();

        if platform.is_windows() {
            assert_eq!(platform.svn_command(), "svn.exe");
        } else {
            assert_eq!(platform.svn_command(), "svn");
        }
    }

    #[test]
    fn test_jj_command() {
        let platform = Platform::current();

        if platform.is_windows() {
            assert_eq!(platform.jj_command(), "jj.exe");
        } else {
            assert_eq!(platform.jj_command(), "jj");
        }
    }

    #[test]
    fn test_supports_symlinks() {
        let platform = Platform::current();
        assert_eq!(platform.supports_symlinks(), platform.is_unix());
    }

    #[test]
    fn test_supports_file_permissions() {
        let platform = Platform::current();
        assert_eq!(platform.supports_file_permissions(), platform.is_unix());
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
}
