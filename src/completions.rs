//! Shell completion generation for smartfo
//!
//! This module provides shell completion generation for bash, zsh, and fish
//! using clap_complete. Completions are generated for all modes (mv, rm, smartfo)
//! based on the argv[0] dispatch mechanism.

use clap::Command;
use clap_complete::{generate, Shell};
use std::io;
use tracing::{info, debug};

/// Generate shell completion script for the specified shell
///
/// # Arguments
/// * `cmd` - The clap Command to generate completions for
/// * `shell` - The target shell (bash, zsh, fish)
/// * `buf` - Output buffer to write the completion script to
///
/// # Example
/// ```no_run
/// use smartfo::completions::generate_completion;
/// use clap::Command;
///
/// let cmd = Command::new("smartfo");
/// let mut buf = Vec::new();
/// generate_completion(&mut cmd, Shell::Bash, &mut buf);
/// ```
pub fn generate_completion(cmd: &mut Command, shell: Shell, buf: &mut dyn io::Write) {
    info!("Generating {} completion script", shell);
    debug!("Command structure: {:?}", cmd.get_name());

    generate(shell, cmd, "smartfo", buf);
}

/// Generate completion script for bash
pub fn generate_bash_completion(cmd: &mut Command, buf: &mut dyn io::Write) {
    generate_completion(cmd, Shell::Bash, buf);
}

/// Generate completion script for zsh
pub fn generate_zsh_completion(cmd: &mut Command, buf: &mut dyn io::Write) {
    generate_completion(cmd, Shell::Zsh, buf);
}

/// Generate completion script for fish
pub fn generate_fish_completion(cmd: &mut Command, buf: &mut dyn io::Write) {
    generate_completion(cmd, Shell::Fish, buf);
}

/// Get all supported shells
pub fn supported_shells() -> Vec<Shell> {
    vec![Shell::Bash, Shell::Zsh, Shell::Fish]
}

/// Get shell name as string
pub fn shell_name(shell: Shell) -> &'static str {
    match shell {
        Shell::Bash => "bash",
        Shell::Zsh => "zsh",
        Shell::Fish => "fish",
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use clap::CommandFactory;

    #[derive(Parser, Debug)]
    struct TestCmd {
        #[arg(short, long)]
        test_flag: bool,
        #[arg(short = 'o', long)]
        option: Option<String>,
        #[arg(short = 'v', long)]
        value: Option<String>,
    }

    #[test]
    fn test_generate_bash_completion() {
        let mut cmd = TestCmd::command();
        let mut buf = Vec::new();
        generate_bash_completion(&mut cmd, &mut buf);

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("smartfo"));
        assert!(output.contains("test-flag"));
        assert!(output.contains("option"));
    }

    #[test]
    fn test_generate_zsh_completion() {
        let mut cmd = TestCmd::command();
        let mut buf = Vec::new();
        generate_zsh_completion(&mut cmd, &mut buf);

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("smartfo"));
        assert!(output.contains("test-flag"));
    }

    #[test]
    fn test_generate_fish_completion() {
        let mut cmd = TestCmd::command();
        let mut buf = Vec::new();
        generate_fish_completion(&mut cmd, &mut buf);

        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("smartfo"));
        assert!(output.contains("test-flag"));
    }

    #[test]
    fn test_supported_shells() {
        let shells = supported_shells();
        assert_eq!(shells.len(), 3);
        assert!(shells.contains(&Shell::Bash));
        assert!(shells.contains(&Shell::Zsh));
        assert!(shells.contains(&Shell::Fish));
    }

    #[test]
    fn test_shell_name() {
        assert_eq!(shell_name(Shell::Bash), "bash");
        assert_eq!(shell_name(Shell::Zsh), "zsh");
        assert_eq!(shell_name(Shell::Fish), "fish");
    }
}
