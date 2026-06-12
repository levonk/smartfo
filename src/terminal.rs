//! Terminal size detection and resize handling
//!
//! This module provides utilities for detecting terminal dimensions and
//! handling terminal resize events (SIGWINCH) for TUI mode and output formatting.

use anyhow::Result;
use libc::{winsize, TIOCGWINSZ};
use std::sync::{Arc, Mutex};
use tracing::{debug, trace, info};

/// Default terminal width for non-terminal output
pub const DEFAULT_WIDTH: usize = 80;

/// Default terminal height for non-terminal output
pub const DEFAULT_HEIGHT: usize = 24;

/// Minimum terminal width (below this, use simplified output)
pub const MIN_WIDTH: usize = 40;

/// Terminal size information
#[derive(Debug, Clone, Copy)]
pub struct TerminalSize {
    /// Number of columns (width)
    pub cols: usize,
    /// Number of rows (height)
    pub rows: usize,
}

impl TerminalSize {
    /// Create a new TerminalSize
    pub fn new(cols: usize, rows: usize) -> Self {
        Self { cols, rows }
    }

    /// Get default terminal size
    pub fn default() -> Self {
        Self {
            cols: DEFAULT_WIDTH,
            rows: DEFAULT_HEIGHT,
        }
    }

    /// Check if terminal is narrow (below minimum width)
    pub fn is_narrow(&self) -> bool {
        self.cols < MIN_WIDTH
    }

    /// Get width, clamped to minimum
    pub fn effective_width(&self) -> usize {
        self.cols.max(MIN_WIDTH)
    }
}

impl Default for TerminalSize {
    fn default() -> Self {
        Self::default()
    }
}

/// Detect terminal size using ioctl TIOCGWINSZ
///
/// This function attempts to get the terminal size using the TIOCGWINSZ ioctl.
/// It falls back to environment variables (COLUMNS, LINES) if ioctl fails.
/// Returns None if running in a non-terminal environment.
pub fn detect_terminal_size() -> Option<TerminalSize> {
    // First check if we're in a terminal
    if !atty::is(atty::Stream::Stdout) {
        debug!("Not running in a terminal, using default size");
        return None;
    }

    // Try ioctl TIOCGWINSZ
    #[cfg(unix)]
    {
        use std::os::fd::AsRawFd;

        let stdout = std::io::stdout();
        let fd = stdout.as_raw_fd();

        let mut winsize: winsize = unsafe { std::mem::zeroed() };

        let result = unsafe { libc::ioctl(fd, TIOCGWINSZ, &mut winsize) };

        if result == 0 {
            let cols = winsize.ws_col as usize;
            let rows = winsize.ws_row as usize;

            if cols > 0 && rows > 0 {
                debug!("Detected terminal size via ioctl: {}x{}", cols, rows);
                return Some(TerminalSize::new(cols, rows));
            }
        } else {
            trace!("ioctl TIOCGWINSZ failed, trying environment variables");
        }
    }

    // Fallback to environment variables
    let cols = std::env::var("COLUMNS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_WIDTH);

    let rows = std::env::var("LINES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_HEIGHT);

    debug!(
        "Detected terminal size via environment variables: {}x{}",
        cols, rows
    );
    Some(TerminalSize::new(cols, rows))
}

/// Get terminal size with fallback to defaults
///
/// This function always returns a TerminalSize, using defaults if
/// running in a non-terminal environment or detection fails.
pub fn get_terminal_size() -> TerminalSize {
    detect_terminal_size().unwrap_or_else(TerminalSize::default)
}

/// Global terminal size cache for resize handling
#[derive(Debug)]
pub struct TerminalSizeCache {
    size: Arc<Mutex<TerminalSize>>,
}

impl TerminalSizeCache {
    /// Create a new terminal size cache
    pub fn new() -> Result<Self> {
        let size = get_terminal_size();
        debug!("Initialized terminal size cache with size: {}x{}", size.cols, size.rows);
        Ok(Self {
            size: Arc::new(Mutex::new(size)),
        })
    }

    /// Register SIGWINCH signal handler for resize events
    ///
    /// Note: This is a simplified implementation. For full TUI mode with interactive
    /// resize handling, a more sophisticated signal handler setup would be needed.
    /// This provides the infrastructure for future enhancement.
    pub fn register_resize_handler(&self) -> Result<()> {
        #[cfg(unix)]
        {
            debug!("SIGWINCH handler registration not yet implemented for interactive TUI mode");
            debug!("Terminal size will be updated on next output operation");
        }
        Ok(())
    }

    /// Get the current terminal size
    pub fn get(&self) -> TerminalSize {
        *self.size.lock().unwrap()
    }

    /// Update the terminal size (called on SIGWINCH)
    pub fn update(&self) {
        let new_size = get_terminal_size();
        let mut size = self.size.lock().unwrap();
        if size.cols != new_size.cols || size.rows != new_size.rows {
            debug!(
                "Terminal resized from {}x{} to {}x{}",
                size.cols, size.rows, new_size.cols, new_size.rows
            );
            *size = new_size;
        }
    }

    /// Check if terminal is narrow
    pub fn is_narrow(&self) -> bool {
        self.get().is_narrow()
    }

    /// Get effective width (clamped to minimum)
    pub fn effective_width(&self) -> usize {
        self.get().effective_width()
    }
}

impl Default for TerminalSizeCache {
    fn default() -> Self {
        Self::new().expect("Failed to initialize terminal size cache")
    }
}

/// Format text to fit within terminal width
///
/// Wraps text to fit within the specified width, preserving word boundaries.
/// Returns a vector of lines.
pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 || text.is_empty() {
        return vec![text.to_string()];
    }

    textwrap::wrap(text, width)
        .iter()
        .map(|s| s.to_string())
        .collect()
}

/// Truncate text to fit within terminal width with ellipsis
///
/// If text is longer than width, truncates it and adds "..." at the end.
pub fn truncate_text(text: &str, width: usize) -> String {
    if text.len() <= width {
        return text.to_string();
    }

    if width <= 3 {
        return "...".to_string();
    }

    format!("{}...", &text[..width - 3])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_size_default() {
        let size = TerminalSize::default();
        assert_eq!(size.cols, DEFAULT_WIDTH);
        assert_eq!(size.rows, DEFAULT_HEIGHT);
    }

    #[test]
    fn test_terminal_size_is_narrow() {
        let narrow = TerminalSize::new(30, 24);
        assert!(narrow.is_narrow());

        let wide = TerminalSize::new(80, 24);
        assert!(!wide.is_narrow());
    }

    #[test]
    fn test_terminal_size_effective_width() {
        let narrow = TerminalSize::new(30, 24);
        assert_eq!(narrow.effective_width(), MIN_WIDTH);

        let wide = TerminalSize::new(120, 24);
        assert_eq!(wide.effective_width(), 120);
    }

    #[test]
    fn test_wrap_text() {
        let text = "This is a long line that should be wrapped";
        let wrapped = wrap_text(text, 20);
        assert!(wrapped.len() > 1);
    }

    #[test]
    fn test_wrap_text_empty() {
        let wrapped = wrap_text("", 20);
        assert_eq!(wrapped, vec![""]);
    }

    #[test]
    fn test_truncate_text() {
        let text = "This is a long line";
        let truncated = truncate_text(text, 10);
        assert_eq!(truncated.len(), 10);
        assert!(truncated.ends_with("..."));
    }

    #[test]
    fn test_truncate_text_short() {
        let text = "Short";
        let truncated = truncate_text(text, 20);
        assert_eq!(truncated, "Short");
    }

    #[test]
    fn test_truncate_text_very_narrow() {
        let text = "Long text";
        let truncated = truncate_text(text, 2);
        assert_eq!(truncated, "...");
    }
}
