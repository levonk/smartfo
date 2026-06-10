//! Dry-run mode support
//!
//! This module provides dry-run context for previewing operations without executing them.

/// Dry-run context to track dry-run state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DryRunContext {
    /// Whether dry-run mode is enabled
    pub enabled: bool,
}

impl DryRunContext {
    /// Create a new dry-run context
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Create a disabled dry-run context
    pub fn disabled() -> Self {
        Self { enabled: false }
    }

    /// Create an enabled dry-run context
    pub fn enabled() -> Self {
        Self { enabled: true }
    }

    /// Check if dry-run mode is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for DryRunContext {
    fn default() -> Self {
        Self::disabled()
    }
}
