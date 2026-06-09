use smartfo::{SessionContext, cache_session_metadata, load_session_metadata, detect_git_repo_path};
use tempfile::TempDir;
use std::fs;

#[test]
fn test_session_context_creation() {
    let context = SessionContext::new().unwrap();
    
    // Verify basic fields are populated
    assert!(!context.cwd.is_empty());
    assert!(!context.metadata.session_start.is_empty());
    assert!(!context.metadata.last_update.is_empty());
    assert!(!context.metadata.scope.is_empty());
}

#[test]
fn test_session_context_to_toon() {
    let context = SessionContext::new().unwrap();
    let toon = context.to_toon();
    
    // Verify TOON format structure
    assert!(toon.contains("(SessionContext"));
    assert!(toon.contains("(cwd"));
    assert!(toon.contains("(metadata"));
    assert!(toon.contains("(scope"));
}

#[test]
fn test_session_context_token_budget() {
    let context = SessionContext::new().unwrap();
    
    // Set a small token budget
    std::env::set_var("SMARTFO_TOKEN_BUDGET", "100");
    let toon = context.to_toon();
    std::env::remove_var("SMARTFO_TOKEN_BUDGET");
    
    // Verify truncation
    assert!(toon.len() <= 120); // 100 + 20 for truncation marker
    assert!(toon.contains("[truncated]"));
}

#[test]
fn test_session_metadata_caching() {
    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().join("cache");
    std::env::set_var("XDG_CACHE_HOME", cache_dir.to_str().unwrap());
    
    let context = SessionContext::new().unwrap();
    cache_session_metadata(&context).unwrap();
    
    let loaded = load_session_metadata().unwrap();
    assert!(loaded.is_some());
    
    let loaded_context = loaded.unwrap();
    assert_eq!(loaded_context.cwd, context.cwd);
    assert_eq!(loaded_context.metadata.scope, context.metadata.scope);
    
    std::env::remove_var("XDG_CACHE_HOME");
}

#[test]
fn test_detect_git_repo_path() {
    let temp_dir = TempDir::new().unwrap();
    let repo_dir = temp_dir.path().join("test-repo");
    fs::create_dir_all(&repo_dir).unwrap();
    
    // Not a git repo yet
    std::env::set_current_dir(&repo_dir).unwrap();
    let result = smartfo::hooks::detect_git_repo_path();
    assert!(result.is_none());
    
    // Initialize git repo
    std::process::Command::new("git")
        .args(["init"])
        .current_dir(&repo_dir)
        .output()
        .unwrap();
    
    // Now it should detect the repo
    let result = detect_git_repo_path();
    assert!(result.is_some());
    assert_eq!(result.unwrap(), repo_dir);
}

#[test]
fn test_session_context_in_git_repo() {
    let temp_dir = TempDir::new().unwrap();
    let repo_dir = temp_dir.path().join("test-repo");
    fs::create_dir_all(&repo_dir).unwrap();
    
    // Initialize git repo
    std::process::Command::new("git")
        .args(["init"])
        .current_dir(&repo_dir)
        .output()
        .unwrap();
    
    std::env::set_current_dir(&repo_dir).unwrap();
    let context = SessionContext::new().unwrap();
    
    // Verify git context is detected
    assert!(context.git_root.is_some());
    assert_eq!(context.metadata.scope, "repo");
}

#[test]
fn test_session_context_outside_git_repo() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();
    
    let context = SessionContext::new().unwrap();
    
    // Verify no git context
    assert!(context.git_root.is_none());
    assert_eq!(context.metadata.scope, "directory");
}
