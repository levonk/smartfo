//! Resource monitoring and limiting
//!
//! This module provides functionality to monitor and enforce resource limits
//! for memory and CPU usage in the daemon process.

use anyhow::{Context, Result};
use sysinfo::{Pid, ProcessRefreshKind, System};
use tracing::{info, warn, error};

/// Resource limits configuration
#[derive(Debug, Clone, Copy)]
pub struct ResourceLimits {
    /// Maximum memory limit in MB (0 = unlimited)
    pub max_memory_mb: u64,
    /// Maximum CPU usage as percentage (0 = unlimited)
    pub max_cpu_percent: u8,
}

impl ResourceLimits {
    /// Create new resource limits
    pub fn new(max_memory_mb: u64, max_cpu_percent: u8) -> Self {
        Self {
            max_memory_mb,
            max_cpu_percent,
        }
    }

    /// Create unlimited resource limits
    pub fn unlimited() -> Self {
        Self {
            max_memory_mb: 0,
            max_cpu_percent: 0,
        }
    }

    /// Check if memory limiting is enabled
    pub fn is_memory_limited(&self) -> bool {
        self.max_memory_mb > 0
    }

    /// Check if CPU limiting is enabled
    pub fn is_cpu_limited(&self) -> bool {
        self.max_cpu_percent > 0
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self::unlimited()
    }
}

/// Resource monitor for tracking process resource usage
pub struct ResourceMonitor {
    system: System,
    limits: ResourceLimits,
    pid: Pid,
}

impl ResourceMonitor {
    /// Create a new resource monitor for the current process
    pub fn new(limits: ResourceLimits) -> Self {
        let mut system = System::new_all();
        let pid = Pid::from_u32(std::process::id());
        system.refresh_process_specifics(pid, ProcessRefreshKind::everything());
        
        Self {
            system,
            limits,
            pid,
        }
    }

    /// Refresh resource usage data
    pub fn refresh(&mut self) {
        self.system.refresh_process_specifics(self.pid, ProcessRefreshKind::everything());
    }

    /// Get current memory usage in MB
    pub fn get_memory_usage_mb(&self) -> u64 {
        if let Some(process) = self.system.process(self.pid) {
            // Convert bytes to MB
            process.memory() / (1024 * 1024)
        } else {
            0
        }
    }

    /// Get current CPU usage as percentage
    pub fn get_cpu_usage_percent(&self) -> f32 {
        if let Some(process) = self.system.process(self.pid) {
            process.cpu_usage()
        } else {
            0.0
        }
    }

    /// Check if memory limit is exceeded
    pub fn is_memory_limit_exceeded(&self) -> bool {
        if !self.limits.is_memory_limited() {
            return false;
        }
        
        let usage = self.get_memory_usage_mb();
        usage > self.limits.max_memory_mb
    }

    /// Check if CPU limit is exceeded
    pub fn is_cpu_limit_exceeded(&self) -> bool {
        if !self.limits.is_cpu_limited() {
            return false;
        }
        
        let usage = self.get_cpu_usage_percent();
        usage > self.limits.max_cpu_percent as f32
    }

    /// Check if any resource limit is exceeded
    pub fn is_any_limit_exceeded(&self) -> bool {
        self.is_memory_limit_exceeded() || self.is_cpu_limit_exceeded()
    }

    /// Get resource limit violation message
    pub fn get_violation_message(&self) -> Option<String> {
        if self.is_memory_limit_exceeded() {
            Some(format!(
                "Memory limit exceeded: {} MB used > {} MB limit",
                self.get_memory_usage_mb(),
                self.limits.max_memory_mb
            ))
        } else if self.is_cpu_limit_exceeded() {
            Some(format!(
                "CPU limit exceeded: {:.1}% used > {}% limit",
                self.get_cpu_usage_percent(),
                self.limits.max_cpu_percent
            ))
        } else {
            None
        }
    }

    /// Log current resource usage
    pub fn log_usage(&self) {
        info!(
            "Resource usage: Memory = {} MB, CPU = {:.1}%",
            self.get_memory_usage_mb(),
            self.get_cpu_usage_percent()
        );
    }

    /// Log resource limits
    pub fn log_limits(&self) {
        info!(
            "Resource limits: Memory = {} MB ({}), CPU = {}% ({})",
            self.limits.max_memory_mb,
            if self.limits.is_memory_limited() { "limited" } else { "unlimited" },
            self.limits.max_cpu_percent,
            if self.limits.is_cpu_limited() { "limited" } else { "unlimited" }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_limits_unlimited() {
        let limits = ResourceLimits::unlimited();
        assert!(!limits.is_memory_limited());
        assert!(!limits.is_cpu_limited());
    }

    #[test]
    fn test_resource_limits_limited() {
        let limits = ResourceLimits::new(4096, 80);
        assert!(limits.is_memory_limited());
        assert!(limits.is_cpu_limited());
        assert_eq!(limits.max_memory_mb, 4096);
        assert_eq!(limits.max_cpu_percent, 80);
    }

    #[test]
    fn test_resource_limits_default() {
        let limits = ResourceLimits::default();
        assert!(!limits.is_memory_limited());
        assert!(!limits.is_cpu_limited());
    }

    #[test]
    fn test_resource_monitor_creation() {
        let limits = ResourceLimits::new(4096, 80);
        let monitor = ResourceMonitor::new(limits);
        // Should not panic
        let _ = monitor;
    }

    #[test]
    fn test_resource_monitor_refresh() {
        let limits = ResourceLimits::new(4096, 80);
        let mut monitor = ResourceMonitor::new(limits);
        monitor.refresh();
        // Should not panic
    }

    #[test]
    fn test_get_memory_usage() {
        let limits = ResourceLimits::unlimited();
        let monitor = ResourceMonitor::new(limits);
        let usage = monitor.get_memory_usage_mb();
        // Should return a value (may be 0 if process not found)
        // Just ensure it doesn't panic
        let _ = usage;
    }

    #[test]
    fn test_get_cpu_usage() {
        let limits = ResourceLimits::unlimited();
        let monitor = ResourceMonitor::new(limits);
        let usage = monitor.get_cpu_usage_percent();
        // Should return a value (may be 0.0 if process not found)
        // Just ensure it doesn't panic
        let _ = usage;
    }

    #[test]
    fn test_unlimited_limits_not_exceeded() {
        let limits = ResourceLimits::unlimited();
        let monitor = ResourceMonitor::new(limits);
        assert!(!monitor.is_memory_limit_exceeded());
        assert!(!monitor.is_cpu_limit_exceeded());
        assert!(!monitor.is_any_limit_exceeded());
    }

    #[test]
    fn test_log_usage() {
        let limits = ResourceLimits::unlimited();
        let monitor = ResourceMonitor::new(limits);
        monitor.log_usage();
        // Should not panic
    }

    #[test]
    fn test_log_limits() {
        let limits = ResourceLimits::new(4096, 80);
        let monitor = ResourceMonitor::new(limits);
        monitor.log_limits();
        // Should not panic
    }
}
