//! Progress indicators for long-running operations
//!
//! This module provides progress bars and spinners using indicatif,
//! with support for:
//! - Quiet mode (suppress all progress indicators)
//! - JSON mode (suppress or format appropriately)
//! - Progress bars for long-duration operations
//! - Spinners for short-duration operations
//! - Estimated time remaining

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Progress manager that handles progress indicators
pub struct ProgressManager {
    quiet: bool,
    json_mode: bool,
    is_tty: bool,
}

impl ProgressManager {
    /// Create a new progress manager
    ///
    /// # Arguments
    /// * `quiet` - If true, suppress all progress indicators
    /// * `json_mode` - If true, suppress progress indicators for JSON output
    pub fn new(quiet: bool, json_mode: bool) -> Self {
        let is_tty = atty::is(atty::Stream::Stderr);
        ProgressManager {
            quiet,
            json_mode,
            is_tty,
        }
    }

    /// Check if progress indicators should be shown
    fn should_show_progress(&self) -> bool {
        !self.quiet && !self.json_mode && self.is_tty
    }

    /// Create a progress bar for file copy operations
    ///
    /// # Arguments
    /// * `total_bytes` - Total number of bytes to copy
    /// * `message` - Message to display with the progress bar
    ///
    /// # Returns
    /// A progress bar (or a hidden one if progress is disabled)
    pub fn create_file_copy_progress(&self, total_bytes: u64, message: &str) -> ProgressBar {
        if !self.should_show_progress() {
            return ProgressBar::hidden();
        }

        let pb = ProgressBar::new(total_bytes);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .expect("Failed to create progress style")
                .progress_chars("#>-")
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }

    /// Create a progress bar for generic operations with percentage
    ///
    /// # Arguments
    /// * `total` - Total number of items/operations
    /// * `message` - Message to display with the progress bar
    ///
    /// # Returns
    /// A progress bar (or a hidden one if progress is disabled)
    pub fn create_percentage_progress(&self, total: u64, message: &str) -> ProgressBar {
        if !self.should_show_progress() {
            return ProgressBar::hidden();
        }

        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
                .expect("Failed to create progress style")
                .progress_chars("#>-")
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }

    /// Create a spinner for short-duration operations
    ///
    /// # Arguments
    /// * `message` - Message to display with the spinner
    ///
    /// # Returns
    /// A progress bar configured as a spinner (or hidden if progress is disabled)
    pub fn create_spinner(&self, message: &str) -> ProgressBar {
        if !self.should_show_progress() {
            return ProgressBar::hidden();
        }

        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .expect("Failed to create spinner style")
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }

    /// Create a progress bar for batch file operations
    ///
    /// # Arguments
    /// * `total_files` - Total number of files to process
    /// * `message` - Message to display with the progress bar
    ///
    /// # Returns
    /// A progress bar (or a hidden one if progress is disabled)
    pub fn create_batch_progress(&self, total_files: u64, message: &str) -> ProgressBar {
        if !self.should_show_progress() {
            return ProgressBar::hidden();
        }

        let pb = ProgressBar::new(total_files);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
                .expect("Failed to create batch progress style")
                .progress_chars("#>-")
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        pb
    }

    /// Finish a progress bar with a success message
    ///
    /// # Arguments
    /// * `pb` - The progress bar to finish
    /// * `message` - Success message to display
    pub fn finish_with_success(&self, pb: &ProgressBar, message: &str) {
        if self.should_show_progress() {
            pb.finish_with_message(message.to_string());
        } else {
            pb.finish();
        }
    }

    /// Finish a progress bar with an error message
    ///
    /// # Arguments
    /// * `pb` - The progress bar to finish
    /// * `message` - Error message to display
    pub fn finish_with_error(&self, pb: &ProgressBar, message: &str) {
        if self.should_show_progress() {
            pb.abandon_with_message(message.to_string());
        } else {
            pb.abandon();
        }
    }
}

impl Default for ProgressManager {
    fn default() -> Self {
        ProgressManager::new(false, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_manager_default() {
        let pm = ProgressManager::default();
        assert!(!pm.quiet);
        assert!(!pm.json_mode);
    }

    #[test]
    fn test_progress_manager_quiet_mode() {
        let pm = ProgressManager::new(true, false);
        assert!(pm.quiet);
        assert!(!pm.json_mode);
        assert!(!pm.should_show_progress());
    }

    #[test]
    fn test_progress_manager_json_mode() {
        let pm = ProgressManager::new(false, true);
        assert!(!pm.quiet);
        assert!(pm.json_mode);
        // In non-TTY environments, this might still be false
        // The test just verifies the flag is set
    }

    #[test]
    fn test_hidden_progress_bar() {
        let pm = ProgressManager::new(true, false);
        let pb = pm.create_file_copy_progress(1000, "Copying");
        assert!(pb.is_hidden());
    }

    #[test]
    fn test_spinner_creation() {
        let pm = ProgressManager::new(true, false);
        let pb = pm.create_spinner("Processing");
        assert!(pb.is_hidden());
    }

    #[test]
    fn test_percentage_progress_creation() {
        let pm = ProgressManager::new(true, false);
        let pb = pm.create_percentage_progress(100, "Processing");
        assert!(pb.is_hidden());
    }

    #[test]
    fn test_batch_progress_creation() {
        let pm = ProgressManager::new(true, false);
        let pb = pm.create_batch_progress(50, "Processing files");
        assert!(pb.is_hidden());
    }

    #[test]
    fn test_progress_bar_updates() {
        let pm = ProgressManager::new(true, false);
        let pb = pm.create_file_copy_progress(1000, "Copying");
        pb.set_position(500);
        assert_eq!(pb.position(), 500);
    }

    #[test]
    fn test_spinner_updates() {
        let pm = ProgressManager::new(true, false);
        let pb = pm.create_spinner("Processing");
        pb.inc(1);
        // Spinner doesn't have position, but should not panic
        pb.finish();
    }

    #[test]
    fn test_finish_with_success() {
        let pm = ProgressManager::new(true, false);
        let pb = pm.create_file_copy_progress(1000, "Copying");
        pm.finish_with_success(&pb, "Complete");
        // Should not panic
    }

    #[test]
    fn test_finish_with_error() {
        let pm = ProgressManager::new(true, false);
        let pb = pm.create_file_copy_progress(1000, "Copying");
        pm.finish_with_error(&pb, "Failed");
        // Should not panic
    }

    #[test]
    fn test_quiet_mode_suppresses_progress() {
        let pm = ProgressManager::new(true, false);
        let pb = pm.create_file_copy_progress(1000, "Copying");
        assert!(pb.is_hidden());
    }

    #[test]
    fn test_json_mode_suppresses_progress() {
        let pm = ProgressManager::new(false, true);
        let pb = pm.create_file_copy_progress(1000, "Copying");
        assert!(pb.is_hidden());
    }

    #[test]
    fn test_normal_mode_shows_progress() {
        let pm = ProgressManager::new(false, false);
        let pb = pm.create_file_copy_progress(1000, "Copying");
        // In non-TTY environments, this might still be hidden
        // The test just verifies the function doesn't panic
        pb.finish();
    }
}
