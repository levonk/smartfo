//! Confirmation prompt utilities
//!
//! This module provides interactive confirmation prompts for destructive operations
//! with support for batch confirmation (yes to all, no to all) and quiet mode.

use anyhow::Result;
use dialoguer::{Confirm, theme::ColorfulTheme};
use std::io::{self, Write};

/// Confirmation state for batch operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationState {
    /// Not yet decided, prompt for each operation
    Prompt,
    /// Yes to all - auto-confirm all subsequent operations
    YesToAll,
    /// No to all - auto-reject all subsequent operations
    NoToAll,
}

impl Default for ConfirmationState {
    fn default() -> Self {
        ConfirmationState::Prompt
    }
}

/// Prompt user for confirmation of a destructive operation
///
/// # Arguments
/// * `message` - The confirmation message to display
/// * `force` - If true, bypass confirmation and return true
/// * `quiet` - If true, assume yes without prompting
/// * `state` - Current confirmation state for batch operations
/// * `dry_run` - If true, show prompt but don't actually wait for input
///
/// # Returns
/// * Ok(true) if confirmed
/// * Ok(false) if rejected
/// * Err if prompt fails
pub fn confirm(
    message: &str,
    force: bool,
    quiet: bool,
    state: &mut ConfirmationState,
    dry_run: bool,
) -> Result<bool> {
    // Quiet mode assumes yes
    if quiet {
        tracing::debug!("Quiet mode: auto-confirming operation");
        return Ok(true);
    }

    // Force flag bypasses confirmation
    if force {
        tracing::debug!("Force flag: bypassing confirmation");
        return Ok(true);
    }

    // Handle batch confirmation states
    match *state {
        ConfirmationState::YesToAll => {
            tracing::debug!("Batch confirmation: yes to all");
            return Ok(true);
        }
        ConfirmationState::NoToAll => {
            tracing::debug!("Batch confirmation: no to all");
            return Ok(false);
        }
        ConfirmationState::Prompt => {
            // Continue to prompt below
        }
    }

    // Dry run mode: show what would happen but don't prompt
    if dry_run {
        println!("{}", message);
        return Ok(true);
    }

    // Interactive prompt
    tracing::debug!("Prompting user for confirmation: {}", message);
    
    let result = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .interact_opt()?;

    match result {
        Some(true) => {
            tracing::debug!("User confirmed operation");
            Ok(true)
        }
        Some(false) => {
            tracing::debug!("User rejected operation");
            Ok(false)
        }
        None => {
            // User pressed Ctrl+C or similar
            tracing::debug!("User cancelled prompt");
            Ok(false)
        }
    }
}

/// Prompt user for batch confirmation with yes/no/all options
///
/// # Arguments
/// * `message` - The confirmation message to display
/// * `force` - If true, bypass confirmation and return YesToAll
/// * `quiet` - If true, assume YesToAll without prompting
///
/// # Returns
/// * Ok(ConfirmationState) with the user's choice
pub fn confirm_batch(message: &str, force: bool, quiet: bool) -> Result<ConfirmationState> {
    // Quiet mode assumes yes to all
    if quiet {
        tracing::debug!("Quiet mode: auto-confirming batch operation");
        return Ok(ConfirmationState::YesToAll);
    }

    // Force flag bypasses confirmation
    if force {
        tracing::debug!("Force flag: bypassing batch confirmation");
        return Ok(ConfirmationState::YesToAll);
    }

    // Interactive prompt with yes/no/all options
    tracing::debug!("Prompting user for batch confirmation: {}", message);
    
    println!("{}", message);
    print!("Proceed? [y/n/a] (y=yes, n=no, a=all): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let input = input.trim().to_lowercase();
    
    match input.as_str() {
        "y" | "yes" => {
            tracing::debug!("User confirmed single operation");
            Ok(ConfirmationState::Prompt)
        }
        "n" | "no" => {
            tracing::debug!("User rejected operation");
            Ok(ConfirmationState::NoToAll)
        }
        "a" | "all" => {
            tracing::debug!("User chose yes to all");
            Ok(ConfirmationState::YesToAll)
        }
        _ => {
            // Default to prompt on invalid input
            tracing::debug!("Invalid input, defaulting to prompt mode");
            Ok(ConfirmationState::Prompt)
        }
    }
}

/// Prompt for destructive operation with clear description
///
/// # Arguments
/// * `operation_type` - Type of operation (e.g., "delete", "overwrite")
/// * `target` - The file or directory being affected
/// * `force` - If true, bypass confirmation
/// * `quiet` - If true, assume yes
/// * `state` - Current confirmation state
/// * `dry_run` - If true, show prompt but don't wait
///
/// # Returns
/// * Ok(true) if confirmed
/// * Ok(false) if rejected
pub fn confirm_destructive(
    operation_type: &str,
    target: &str,
    force: bool,
    quiet: bool,
    state: &mut ConfirmationState,
    dry_run: bool,
) -> Result<bool> {
    let message = format!("{} '{}'?", operation_type, target);
    confirm(&message, force, quiet, state, dry_run)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confirmation_state_default() {
        let state = ConfirmationState::default();
        assert_eq!(state, ConfirmationState::Prompt);
    }

    #[test]
    fn test_force_bypass() {
        let mut state = ConfirmationState::Prompt;
        let result = confirm("Test message", true, false, &mut state, false);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_quiet_assume_yes() {
        let mut state = ConfirmationState::Prompt;
        let result = confirm("Test message", false, true, &mut state, false);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_yes_to_all() {
        let mut state = ConfirmationState::YesToAll;
        let result = confirm("Test message", false, false, &mut state, false);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_no_to_all() {
        let mut state = ConfirmationState::NoToAll;
        let result = confirm("Test message", false, false, &mut state, false);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_dry_run() {
        let mut state = ConfirmationState::Prompt;
        let result = confirm("Test message", false, false, &mut state, true);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
