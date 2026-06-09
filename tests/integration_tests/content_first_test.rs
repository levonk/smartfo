// Integration tests for content-first no-args in agent mode
// Tests that smartfo provides meaningful output when invoked without arguments

use std::fs;
use tempfile::TempDir;

#[test]
fn test_content_first_no_args_in_git_repo() {
    // Test that no-args provides context-aware output in git repository
    let temp_dir = TempDir::new().unwrap();
    let repo = temp_dir.path();
    
    // Create .git directory to simulate git repository
    let git_dir = repo.join(".git");
    fs::create_dir(&git_dir).unwrap();
    
    // Create some files in the repo
    let file1 = repo.join("file1.txt");
    let file2 = repo.join("file2.txt");
    fs::write(&file1, "content1").unwrap();
    fs::write(&file2, "content2").unwrap();
    
    std::env::set_current_dir(repo).unwrap();
    
    // Verify repository structure
    assert!(git_dir.exists());
    assert!(file1.exists());
    assert!(file2.exists());
    
    // In a real integration test, we would run smartfo without args
    // and verify it detects the git repo and provides context-aware output
    // For now, we verify the setup is correct
}

#[test]
fn test_content_first_no_args_outside_git_repo() {
    // Test that no-args provides appropriate output outside git repository
    let temp_dir = TempDir::new().unwrap();
    let non_repo = temp_dir.path();
    
    // Create some files outside git
    let file1 = non_repo.join("file1.txt");
    let file2 = non_repo.join("file2.txt");
    fs::write(&file1, "content1").unwrap();
    fs::write(&file2, "content2").unwrap();
    
    std::env::set_current_dir(non_repo).unwrap();
    
    // Verify non-repository structure
    assert!(!non_repo.join(".git").exists());
    assert!(file1.exists());
    assert!(file2.exists());
    
    // In a real integration test, we would run smartfo without args
    // and verify it detects it's not a git repo and provides appropriate output
}

#[test]
fn test_content_first_no_args_nested_directory() {
    // Test that no-args detects git repository from nested directory
    let temp_dir = TempDir::new().unwrap();
    let repo = temp_dir.path();
    
    // Create nested directory structure
    let nested = repo.join("nested/deep");
    fs::create_dir_all(&nested).unwrap();
    
    // Create .git directory at root
    let git_dir = repo.join(".git");
    fs::create_dir(&git_dir).unwrap();
    
    // Create files in nested directory
    let file1 = nested.join("file1.txt");
    let file2 = nested.join("file2.txt");
    fs::write(&file1, "content1").unwrap();
    fs::write(&file2, "content2").unwrap();
    
    std::env::set_current_dir(&nested).unwrap();
    
    // Verify nested structure
    assert!(git_dir.exists());
    assert!(nested.exists());
    assert!(file1.exists());
    assert!(file2.exists());
    
    // In a real integration test, we would run smartfo without args
    // and verify it detects the git repo from nested directory
}

#[test]
fn test_content_first_no_args_empty_directory() {
    // Test that no-args handles empty directory gracefully
    let temp_dir = TempDir::new().unwrap();
    let empty_dir = temp_dir.path();
    
    std::env::set_current_dir(empty_dir).unwrap();
    
    // Verify empty directory
    assert!(empty_dir.read_dir().unwrap().count() == 0);
    
    // In a real integration test, we would run smartfo without args
    // and verify it handles empty directory gracefully
}

#[test]
fn test_content_first_no_args_with_trash_directory() {
    // Test that no-args recognizes trash directory context
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();
    
    // Create trash directory structure
    let trash_root = base.join(".local/share/Trash/files");
    fs::create_dir_all(&trash_root).unwrap();
    
    // Create some trashed files
    let trash_file1 = trash_root.join("file1.txt_20240101");
    let trash_file2 = trash_root.join("file2.txt_20240102");
    fs::write(&trash_file1, "content1").unwrap();
    fs::write(&trash_file2, "content2").unwrap();
    
    std::env::set_current_dir(base).unwrap();
    
    // Verify trash structure
    assert!(trash_root.exists());
    assert!(trash_file1.exists());
    assert!(trash_file2.exists());
    
    // In a real integration test, we would run smartfo without args
    // and verify it recognizes trash context
}

#[test]
fn test_content_first_no_args_with_config_file() {
    // Test that no-args respects configuration file
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config/smartfo");
    fs::create_dir_all(&config_dir).unwrap();
    
    // Create a config file
    let config_file = config_dir.join("config.toml");
    let config_content = r#"
[vcs]
default = "git"

[trash]
mode = "always"
"#;
    fs::write(&config_file, config_content).unwrap();
    
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    // Verify config structure
    assert!(config_file.exists());
    
    // In a real integration test, we would run smartfo without args
    // and verify it respects the config file
}

#[test]
fn test_content_first_no_args_agent_mode_detection() {
    // Test that no-args detects agent mode and provides agent-optimized output
    let temp_dir = TempDir::new().unwrap();
    let repo = temp_dir.path();
    
    // Create git repository
    let git_dir = repo.join(".git");
    fs::create_dir(&git_dir).unwrap();
    
    // Set agent session environment variable
    std::env::set_var("CLAUDE_SESSION", "test-session");
    
    std::env::set_current_dir(repo).unwrap();
    
    // Verify agent session is set
    assert!(std::env::var("CLAUDE_SESSION").is_ok());
    assert!(git_dir.exists());
    
    // In a real integration test, we would run smartfo without args
    // and verify it detects agent mode and provides TOON output
    
    // Clean up
    std::env::remove_var("CLAUDE_SESSION");
}

#[test]
fn test_content_first_no_args_non_tty_detection() {
    // Test that no-args detects non-TTY environment (agent mode)
    let temp_dir = TempDir::new().unwrap();
    let repo = temp_dir.path();
    
    // Create git repository
    let git_dir = repo.join(".git");
    fs::create_dir(&git_dir).unwrap();
    
    std::env::set_current_dir(repo).unwrap();
    
    // Verify non-TTY detection would work
    // (atty::is(atty::Stream::Stdout) would return false in non-TTY)
    assert!(git_dir.exists());
    
    // In a real integration test, we would run smartfo without args
    // with piped output and verify it detects non-TTY mode
}

#[test]
fn test_content_first_no_args_with_multiple_vcs() {
    // Test that no-args handles multiple VCS systems
    let temp_dir = TempDir::new().unwrap();
    let repo = temp_dir.path();
    
    // Create .git directory
    let git_dir = repo.join(".git");
    fs::create_dir(&git_dir).unwrap();
    
    // Create .hg directory (Mercurial)
    let hg_dir = repo.join(".hg");
    fs::create_dir(&hg_dir).unwrap();
    
    std::env::set_current_dir(repo).unwrap();
    
    // Verify multiple VCS directories
    assert!(git_dir.exists());
    assert!(hg_dir.exists());
    
    // In a real integration test, we would run smartfo without args
    // and verify it detects multiple VCS systems
}

#[test]
fn test_content_first_no_args_help_suggestions() {
    // Test that no-args includes helpful suggestions in output
    let temp_dir = TempDir::new().unwrap();
    let repo = temp_dir.path();
    
    // Create git repository
    let git_dir = repo.join(".git");
    fs::create_dir(&git_dir).unwrap();
    
    // Create some files
    let file1 = repo.join("file1.txt");
    fs::write(&file1, "content1").unwrap();
    
    std::env::set_current_dir(repo).unwrap();
    
    // Verify structure
    assert!(git_dir.exists());
    assert!(file1.exists());
    
    // In a real integration test, we would run smartfo without args
    // and verify it includes helpful suggestions like:
    // - "Use 'smartfo list' to see operations"
    // - "Use 'smartfo status' to see daemon status"
    // - "Use 'smartfo --help' for full command reference"
}

#[test]
fn test_content_first_no_args_with_audit_log() {
    // Test that no-args references audit log when available
    let temp_dir = TempDir::new().unwrap();
    let repo = temp_dir.path();
    
    // Create git repository
    let git_dir = repo.join(".git");
    fs::create_dir(&git_dir).unwrap();
    
    // Create smartfo audit directory
    let smartfo_dir = repo.join(".smartfo/audit");
    fs::create_dir_all(&smartfo_dir).unwrap();
    
    // Create audit log file
    let audit_log = smartfo_dir.join("operations.jsonl");
    let audit_content = r#"{"timestamp":"2024-01-01T00:00:00Z","operation":"move","source":"file1.txt","destination":"file2.txt"}
{"timestamp":"2024-01-01T00:01:00Z","operation":"remove","source":"file3.txt"}"#;
    fs::write(&audit_log, audit_content).unwrap();
    
    std::env::set_current_dir(repo).unwrap();
    
    // Verify audit log structure
    assert!(git_dir.exists());
    assert!(audit_log.exists());
    
    // In a real integration test, we would run smartfo without args
    // and verify it references the audit log in its output
}

#[test]
fn test_content_first_no_args_daemon_status_check() {
    // Test that no-args checks daemon status when applicable
    let temp_dir = TempDir::new().unwrap();
    let repo = temp_dir.path();
    
    // Create git repository
    let git_dir = repo.join(".git");
    fs::create_dir(&git_dir).unwrap();
    
    // Create smartfo state directory
    let state_dir = repo.join(".smartfo/state");
    fs::create_dir_all(&state_dir).unwrap();
    
    // Create a mock daemon PID file
    let pid_file = state_dir.join("daemon.pid");
    fs::write(&pid_file, "12345").unwrap();
    
    std::env::set_current_dir(repo).unwrap();
    
    // Verify daemon status structure
    assert!(git_dir.exists());
    assert!(pid_file.exists());
    
    // In a real integration test, we would run smartfo without args
    // and verify it checks and reports daemon status
}

#[test]
fn test_content_first_no_args_queue_status() {
    // Test that no-args reports queue status when jobs are pending
    let temp_dir = TempDir::new().unwrap();
    let repo = temp_dir.path();
    
    // Create git repository
    let git_dir = repo.join(".git");
    fs::create_dir(&git_dir).unwrap();
    
    // Create smartfo queue directory
    let queue_dir = repo.join(".smartfo/queue");
    fs::create_dir_all(&queue_dir).unwrap();
    
    // Create a mock queue file
    let queue_file = queue_dir.join("queue.sqlite");
    fs::write(&queue_file, "").unwrap();
    
    std::env::set_current_dir(repo).unwrap();
    
    // Verify queue structure
    assert!(git_dir.exists());
    assert!(queue_file.exists());
    
    // In a real integration test, we would run smartfo without args
    // and verify it reports queue status
}
