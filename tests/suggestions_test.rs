// Integration tests for contextual suggestion engine

use smartfo::output::suggestions::{Suggestion, SuggestionContext, SuggestionEngine, format_suggestions_as_help};

#[test]
fn test_suggestion_engine_list_git_repo() {
    let context = SuggestionContext::new("list")
        .with_git_repo(true)
        .with_daemon(true)
        .with_queue_depth(5);
    
    let suggestions = SuggestionEngine::generate(&context);
    
    assert!(!suggestions.is_empty());
    assert!(suggestions.len() <= 4);
    
    // Should suggest status and list with filters in git repo
    assert!(suggestions.iter().any(|s| s.command.contains("status")));
    assert!(suggestions.iter().any(|s| s.command.contains("list")));
    
    // Verify all suggestions are valid
    for suggestion in &suggestions {
        assert!(suggestion.is_valid());
    }
    
    // Verify sorted by relevance
    for i in 0..suggestions.len().saturating_sub(1) {
        assert!(suggestions[i].relevance >= suggestions[i + 1].relevance);
    }
}

#[test]
fn test_suggestion_engine_list_no_git() {
    let context = SuggestionContext::new("list")
        .with_git_repo(false)
        .with_daemon(false);
    
    let suggestions = SuggestionEngine::generate(&context);
    
    assert!(!suggestions.is_empty());
    assert!(suggestions.len() <= 4);
    
    // Should suggest install when not in git repo
    assert!(suggestions.iter().any(|s| s.command.contains("install")));
}

#[test]
fn test_suggestion_engine_status_with_queue() {
    let context = SuggestionContext::new("status")
        .with_git_repo(true)
        .with_daemon(true)
        .with_queue_depth(10);
    
    let suggestions = SuggestionEngine::generate(&context);
    
    assert!(!suggestions.is_empty());
    
    // Should suggest viewing the queue when there are pending operations
    assert!(suggestions.iter().any(|s| s.command.contains("list")));
    
    // High relevance for queue-related suggestions
    let queue_suggestions: Vec<_> = suggestions.iter()
        .filter(|s| s.command.contains("list"))
        .collect();
    
    if !queue_suggestions.is_empty() {
        assert!(queue_suggestions[0].relevance > 0.8);
    }
}

#[test]
fn test_suggestion_engine_status_no_queue() {
    let context = SuggestionContext::new("status")
        .with_git_repo(true)
        .with_daemon(true)
        .with_queue_depth(0);
    
    let suggestions = SuggestionEngine::generate(&context);
    
    assert!(!suggestions.is_empty());
    
    // Should not prioritize queue viewing when queue is empty
    let queue_suggestions: Vec<_> = suggestions.iter()
        .filter(|s| s.command.contains("list") && s.command.contains("pending"))
        .collect();
    
    assert!(queue_suggestions.is_empty() || queue_suggestions[0].relevance < 0.9);
}

#[test]
fn test_suggestion_engine_mv_git_repo() {
    let context = SuggestionContext::new("mv")
        .with_git_repo(true)
        .with_daemon(true);
    
    let suggestions = SuggestionEngine::generate(&context);
    
    assert!(!suggestions.is_empty());
    
    // Should suggest list and status after move
    assert!(suggestions.iter().any(|s| s.command.contains("list")));
    assert!(suggestions.iter().any(|s| s.command.contains("status")));
}

#[test]
fn test_suggestion_engine_rm() {
    let context = SuggestionContext::new("rm")
        .with_git_repo(true);
    
    let suggestions = SuggestionEngine::generate(&context);
    
    assert!(!suggestions.is_empty());
    
    // Should always suggest list after rm
    assert!(suggestions.iter().any(|s| s.command.contains("list")));
    
    // High relevance for list suggestion
    let list_suggestions: Vec<_> = suggestions.iter()
        .filter(|s| s.command.contains("list"))
        .collect();
    
    if !list_suggestions.is_empty() {
        assert!(list_suggestions[0].relevance > 0.8);
    }
}

#[test]
fn test_suggestion_engine_install() {
    let context = SuggestionContext::new("install")
        .with_git_repo(true);
    
    let suggestions = SuggestionEngine::generate(&context);
    
    assert!(!suggestions.is_empty());
    
    // Should suggest list and status after install
    assert!(suggestions.iter().any(|s| s.command.contains("list")));
    assert!(suggestions.iter().any(|s| s.command.contains("status")));
}

#[test]
fn test_suggestion_engine_noargs_git_repo() {
    let context = SuggestionContext::new("")
        .with_git_repo(true);
    
    let suggestions = SuggestionEngine::generate(&context);
    
    assert!(!suggestions.is_empty());
    
    // Should suggest list and status in git repo
    assert!(suggestions.iter().any(|s| s.command.contains("list")));
    assert!(suggestions.iter().any(|s| s.command.contains("status")));
}

#[test]
fn test_suggestion_engine_noargs_no_git() {
    let context = SuggestionContext::new("")
        .with_git_repo(false);
    
    let suggestions = SuggestionEngine::generate(&context);
    
    assert!(!suggestions.is_empty());
    
    // Should suggest install when not in git repo
    assert!(suggestions.iter().any(|s| s.command.contains("install")));
    
    // Should not suggest list when not in git repo
    assert!(!suggestions.iter().any(|s| s.command.contains("list")));
}

#[test]
fn test_suggestion_engine_generic_command() {
    let context = SuggestionContext::new("unknown_command")
        .with_git_repo(true);
    
    let suggestions = SuggestionEngine::generate(&context);
    
    assert!(!suggestions.is_empty());
    
    // Should provide generic suggestions
    assert!(suggestions.iter().any(|s| s.command.contains("--help")));
}

#[test]
fn test_suggestion_limit_to_4() {
    let context = SuggestionContext::new("list")
        .with_git_repo(true)
        .with_daemon(true)
        .with_queue_depth(10);
    
    let suggestions = SuggestionEngine::generate(&context);
    
    assert!(suggestions.len() <= 4);
}

#[test]
fn test_suggestion_minimum_2() {
    let context = SuggestionContext::new("list")
        .with_git_repo(false)
        .with_daemon(false);
    
    let suggestions = SuggestionEngine::generate(&context);
    
    // Should have at least 2 suggestions if possible
    assert!(suggestions.len() >= 2 || suggestions.is_empty());
}

#[test]
fn test_format_suggestions_as_help() {
    let suggestions = vec![
        Suggestion::new("smartfo list", "List operations", 0.9),
        Suggestion::new("smartfo status", "Check status", 0.8),
        Suggestion::new("smartfo install", "Install smartfo", 0.7),
    ];
    
    let help = format_suggestions_as_help(&suggestions);
    
    assert_eq!(help.len(), 3);
    assert!(help[0].contains("smartfo list"));
    assert!(help[0].contains("List operations"));
    assert!(help[1].contains("smartfo status"));
    assert!(help[1].contains("Check status"));
    assert!(help[2].contains("smartfo install"));
    assert!(help[2].contains("Install smartfo"));
}

#[test]
fn test_suggestion_context_builder() {
    let context = SuggestionContext::new("test")
        .with_git_repo(true)
        .with_daemon(false)
        .with_queue_depth(5)
        .with_context("custom_key", "custom_value");
    
    assert_eq!(context.current_command, "test");
    assert!(context.in_git_repo);
    assert!(!context.daemon_running);
    assert_eq!(context.queue_depth, Some(5));
    assert_eq!(context.extra_context.get("custom_key"), Some(&"custom_value".to_string()));
}

#[test]
fn test_suggestion_validation() {
    let valid = Suggestion::new("cmd", "desc", 0.5);
    assert!(valid.is_valid());
    
    let empty_command = Suggestion::new("", "desc", 0.5);
    assert!(!empty_command.is_valid());
    
    let empty_desc = Suggestion::new("cmd", "", 0.5);
    assert!(!empty_desc.is_valid());
    
    let negative_relevance = Suggestion::new("cmd", "desc", -0.1);
    assert!(!negative_relevance.is_valid());
    
    let high_relevance = Suggestion::new("cmd", "desc", 1.5);
    assert!(!high_relevance.is_valid());
}

#[test]
fn test_suggestion_relevance_sorting() {
    let context = SuggestionContext::new("list")
        .with_git_repo(true)
        .with_daemon(true);
    
    let suggestions = SuggestionEngine::generate(&context);
    
    // Verify suggestions are sorted by relevance (highest first)
    for i in 0..suggestions.len().saturating_sub(1) {
        assert!(suggestions[i].relevance >= suggestions[i + 1].relevance,
            "Suggestion {} should have relevance >= suggestion {}: {} vs {}",
            i, i + 1, suggestions[i].relevance, suggestions[i + 1].relevance);
    }
}

#[test]
fn test_suggestion_completeness() {
    let context = SuggestionContext::new("list")
        .with_git_repo(true);
    
    let suggestions = SuggestionEngine::generate(&context);
    
    // All suggestions should have complete commands
    for suggestion in &suggestions {
        assert!(!suggestion.command.is_empty());
        assert!(suggestion.command.starts_with("smartfo"));
    }
}

#[test]
fn test_suggestion_context_awareness() {
    // Test that suggestions differ based on context
    let git_context = SuggestionContext::new("").with_git_repo(true);
    let no_git_context = SuggestionContext::new("").with_git_repo(false);
    
    let git_suggestions = SuggestionEngine::generate(&git_context);
    let no_git_suggestions = SuggestionEngine::generate(&no_git_context);
    
    // Git context should suggest list, no-git should suggest install
    assert!(git_suggestions.iter().any(|s| s.command.contains("list")));
    assert!(no_git_suggestions.iter().any(|s| s.command.contains("install")));
}
