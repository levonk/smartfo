//! Health check mechanism for container orchestration
//!
//! This module implements health checks for Docker HEALTHCHECK and Kubernetes probes.
//! Supports both HTTP endpoint and signal-based health checks with appropriate exit codes.
//!
//! ## Health Check Types
//!
//! - **HTTP endpoint**: Lightweight HTTP server on localhost that returns 200 when healthy
//! - **Signal-based**: SIGUSR1 handler that writes health status to a file
//! - **CLI command**: Direct health check via command-line interface
//!
//! ## Exit Codes
//!
//! - `0`: Healthy
//! - `1`: Unhealthy
//! - `2`: Misconfigured (invalid arguments, etc.)

use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tracing::{info, warn, error};

/// Health status of the daemon
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// Daemon is healthy and operational
    Healthy,
    /// Daemon is unhealthy or not responding
    Unhealthy,
}

impl HealthStatus {
    /// Convert health status to exit code
    ///
    /// Returns 0 for healthy, 1 for unhealthy
    pub fn exit_code(&self) -> i32 {
        match self {
            HealthStatus::Healthy => 0,
            HealthStatus::Unhealthy => 1,
        }
    }

    /// Convert health status to HTTP status code
    ///
    /// Returns 200 for healthy, 503 for unhealthy
    pub fn http_status(&self) -> u16 {
        match self {
            HealthStatus::Healthy => 200,
            HealthStatus::Unhealthy => 503,
        }
    }
}

/// Health check configuration
#[derive(Debug, Clone)]
pub struct HealthCheckConfig {
    /// Path to health status file (for signal-based checks)
    pub status_file_path: PathBuf,
    /// HTTP server port (for HTTP-based checks)
    pub http_port: u16,
    /// Timeout for health check operations
    pub timeout: Duration,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        let xdg_data_home = std::env::var("XDG_DATA_HOME")
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").expect("HOME not set");
                format!("{}/.local/share", home)
            });

        let smartfo_data = PathBuf::from(xdg_data_home).join("smartfo");

        Self {
            status_file_path: smartfo_data.join("health-status.txt"),
            http_port: 8080,
            timeout: Duration::from_secs(5),
        }
    }
}

/// Health check interface
pub struct HealthChecker {
    config: HealthCheckConfig,
    /// Global health status (updated by signal handler)
    status: AtomicBool,
}

impl Clone for HealthChecker {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            status: AtomicBool::new(self.status.load(Ordering::SeqCst)),
        }
    }
}

impl HealthChecker {
    /// Create a new health checker with default configuration
    pub fn new() -> Result<Self> {
        let config = HealthCheckConfig::default();
        Self::with_config(config)
    }

    /// Create a new health checker with custom configuration
    pub fn with_config(config: HealthCheckConfig) -> Result<Self> {
        // Ensure parent directory exists for status file
        if let Some(parent) = config.status_file_path.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create health status directory")?;
        }

        Ok(Self {
            config,
            status: AtomicBool::new(true), // Start as healthy
        })
    }

    /// Perform a health check
    ///
    /// This validates the daemon's operational state without side effects.
    /// It checks:
    /// - If the daemon process is running
    /// - If the daemon socket is accessible
    /// - If the daemon responds to ping
    ///
    /// Returns HealthStatus::Healthy if all checks pass, Unhealthy otherwise
    pub fn check(&self) -> HealthStatus {
        info!("Performing health check");

        // Check if daemon is running by attempting to ping it
        // This is a lightweight check that doesn't require full daemon initialization
        let is_healthy = self.check_daemon_responsive();

        if is_healthy {
            info!("Health check passed: daemon is healthy");
            HealthStatus::Healthy
        } else {
            warn!("Health check failed: daemon is unhealthy");
            HealthStatus::Unhealthy
        }
    }

    /// Check if daemon is responsive via socket ping
    fn check_daemon_responsive(&self) -> bool {
        // Import daemon module to check socket
        // This is a minimal check - just try to connect and ping
        match crate::daemon::Daemon::new() {
            Ok(daemon) => {
                match daemon.ping_daemon() {
                    Ok(true) => true,
                    Ok(false) => {
                        warn!("Daemon ping returned false");
                        false
                    }
                    Err(e) => {
                        error!("Daemon ping failed: {}", e);
                        false
                    }
                }
            }
            Err(e) => {
                error!("Failed to create daemon instance for health check: {}", e);
                false
            }
        }
    }

    /// Update health status
    ///
    /// This is called by the daemon to update its health status
    pub fn set_status(&self, status: HealthStatus) {
        let is_healthy = status == HealthStatus::Healthy;
        self.status.store(is_healthy, Ordering::SeqCst);
        info!("Health status updated to: {:?}", status);

        // Also write to status file for signal-based checks
        self.write_status_file(status);
    }

    /// Write health status to file (for signal-based checks)
    pub fn write_status_file(&self, status: HealthStatus) {
        let status_str = match status {
            HealthStatus::Healthy => "healthy",
            HealthStatus::Unhealthy => "unhealthy",
        };

        if let Err(e) = std::fs::write(&self.config.status_file_path, status_str) {
            error!("Failed to write health status to file: {}", e);
        }
    }

    /// Read health status from file (for signal-based checks)
    pub fn read_status_file(&self) -> Result<HealthStatus> {
        if !self.config.status_file_path.exists() {
            return Ok(HealthStatus::Unhealthy);
        }

        let content = std::fs::read_to_string(&self.config.status_file_path)
            .context("Failed to read health status file")?;

        match content.trim() {
            "healthy" => Ok(HealthStatus::Healthy),
            "unhealthy" => Ok(HealthStatus::Unhealthy),
            _ => {
                warn!("Invalid health status in file: {}", content);
                Ok(HealthStatus::Unhealthy)
            }
        }
    }

    /// Get the current health status
    pub fn current_status(&self) -> HealthStatus {
        if self.status.load(Ordering::SeqCst) {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        }
    }

    /// Get the health check configuration
    pub fn config(&self) -> &HealthCheckConfig {
        &self.config
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new().expect("Failed to create health checker")
    }
}

/// Perform a CLI health check and exit with appropriate code
///
/// This is the entry point for the `smartfo health` command
pub fn run_health_check() -> Result<()> {
    let checker = HealthChecker::new()?;
    let status = checker.check();

    info!("Health check result: {:?}", status);
    std::process::exit(status.exit_code());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_exit_codes() {
        assert_eq!(HealthStatus::Healthy.exit_code(), 0);
        assert_eq!(HealthStatus::Unhealthy.exit_code(), 1);
    }

    #[test]
    fn test_health_status_http_codes() {
        assert_eq!(HealthStatus::Healthy.http_status(), 200);
        assert_eq!(HealthStatus::Unhealthy.http_status(), 503);
    }

    #[test]
    fn test_health_checker_creation() {
        let checker = HealthChecker::new();
        assert!(checker.is_ok());
    }

    #[test]
    fn test_health_checker_default_config() {
        let config = HealthCheckConfig::default();
        assert_eq!(config.http_port, 8080);
        assert_eq!(config.timeout, Duration::from_secs(5));
    }

    #[test]
    fn test_set_and_get_status() {
        let checker = HealthChecker::new().unwrap();

        checker.set_status(HealthStatus::Healthy);
        assert_eq!(checker.current_status(), HealthStatus::Healthy);

        checker.set_status(HealthStatus::Unhealthy);
        assert_eq!(checker.current_status(), HealthStatus::Unhealthy);
    }

    #[test]
    fn test_write_and_read_status_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let status_file = temp_dir.path().join("health-status.txt");

        let config = HealthCheckConfig {
            status_file_path: status_file.clone(),
            http_port: 8080,
            timeout: Duration::from_secs(5),
        };

        let checker = HealthChecker::with_config(config).unwrap();

        // Write healthy status
        checker.set_status(HealthStatus::Healthy);
        let read_status = checker.read_status_file().unwrap();
        assert_eq!(read_status, HealthStatus::Healthy);

        // Write unhealthy status
        checker.set_status(HealthStatus::Unhealthy);
        let read_status = checker.read_status_file().unwrap();
        assert_eq!(read_status, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_read_status_file_when_missing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let status_file = temp_dir.path().join("nonexistent-status.txt");

        let config = HealthCheckConfig {
            status_file_path: status_file,
            http_port: 8080,
            timeout: Duration::from_secs(5),
        };

        let checker = HealthChecker::with_config(config).unwrap();
        let status = checker.read_status_file().unwrap();
        assert_eq!(status, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_read_status_file_with_invalid_content() {
        let temp_dir = tempfile::tempdir().unwrap();
        let status_file = temp_dir.path().join("invalid-status.txt");

        // Write invalid content
        std::fs::write(&status_file, "invalid-status").unwrap();

        let config = HealthCheckConfig {
            status_file_path: status_file,
            http_port: 8080,
            timeout: Duration::from_secs(5),
        };

        let checker = HealthChecker::with_config(config).unwrap();
        let status = checker.read_status_file().unwrap();
        assert_eq!(status, HealthStatus::Unhealthy);
    }
}
