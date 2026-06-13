// Contextual suggestion engine for CLI discovery
// Provides relevant next-step suggestions based on current state and context

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// A single suggestion with command, description, and relevance score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    /// The complete command with flags (e.g., "smartfo list --status pending")
    pub command: String,
    /// Human-readable description of what the command does
    pub description: String,
    /// Relevance score (0.0 to 1.0, higher is more relevant)
    pub relevance: f64,
}

impl Suggestion {
    /// Create a new suggestion
    pub fn new(command: impl Into<String>, description: impl Into<String>, relevance: f64) -> Self {
        Self {
            command: command.into(),
            description: description.into(),
            relevance,
        }
    }

    /// Validate the suggestion
    pub fn is_valid(&self) -> bool {
        !self.command.is_empty()
            && !self.description.is_empty()
            && self.relevance >= 0.0
            && self.relevance <= 1.0
    }
}

/// Context information for generating suggestions
#[derive(Debug, Clone)]
pub struct SuggestionContext {
    /// Current command being executed
    pub current_command: String,
    /// Current working directory
    pub current_dir: String,
    /// Whether in a git repository
    pub in_git_repo: bool,
    /// Whether daemon is running
    pub daemon_running: bool,
    /// Queue depth (number of pending operations)
    pub queue_depth: Option<usize>,
    /// Additional context data
    pub extra_context: HashMap<String, String>,
}

impl SuggestionContext {
    /// Create a new suggestion context
    pub fn new(current_command: impl Into<String>) -> Self {
        Self {
            current_command: current_command.into(),
            current_dir: std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .to_string_lossy()
                .to_string(),
            in_git_repo: false, // Will be detected by caller
            daemon_running: false, // Will be detected by caller
            queue_depth: None,
            extra_context: HashMap::new(),
        }
    }

    /// Set git repository status
    pub fn with_git_repo(mut self, in_git: bool) -> Self {
        self.in_git_repo = in_git;
        self
    }

    /// Set daemon status
    pub fn with_daemon(mut self, running: bool) -> Self {
        self.daemon_running = running;
        self
    }

    /// Set queue depth
    pub fn with_queue_depth(mut self, depth: usize) -> Self {
        self.queue_depth = Some(depth);
        self
    }

    /// Add extra context
    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra_context.insert(key.into(), value.into());
        self
    }
}

/// Suggestion engine that generates context-aware suggestions
pub struct SuggestionEngine;

impl SuggestionEngine {
    /// Generate suggestions for a given context
    pub fn generate(context: &SuggestionContext) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // Generate context-specific suggestions based on current command
        match context.current_command.as_str() {
            "list" => suggestions.extend(Self::suggestions_for_list(context)),
            "status" => suggestions.extend(Self::suggestions_for_status(context)),
            "mv" | "smv" => suggestions.extend(Self::suggestions_for_mv(context)),
            "rm" | "srm" => suggestions.extend(Self::suggestions_for_rm(context)),
            "install" => suggestions.extend(Self::suggestions_for_install(context)),
            "" | "smartfo" => suggestions.extend(Self::suggestions_for_noargs(context)),
            _ => suggestions.extend(Self::suggestions_for_generic(context)),
        }

        // Sort by relevance (highest first)
        suggestions.sort_by(|a, b| {
            b.relevance.partial_cmp(&a.relevance).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Limit to 2-4 suggestions
        suggestions.truncate(4);

        // Ensure at least 2 suggestions if possible
        if suggestions.len() < 2 && !suggestions.is_empty() {
            // Add generic suggestions if we have too few
            suggestions.extend(Self::generic_suggestions(context));
            suggestions.sort_by(|a, b| {
                b.relevance.partial_cmp(&a.relevance).unwrap_or(std::cmp::Ordering::Equal)
            });
            suggestions.truncate(4);
        }

        suggestions
    }

    /// Suggestions for list command
    fn suggestions_for_list(context: &SuggestionContext) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // Check if list is empty via extra context
        let is_empty = context.extra_context.get("list_empty").map(|v| v == "true").unwrap_or(false);

        if is_empty {
            suggestions.push(Suggestion::new(
                "smartfo install",
                "Install smartfo to start tracking operations",
                0.95,
            ));
            suggestions.push(Suggestion::new(
                "smartfo status",
                "Check daemon status and queue depth",
                0.9,
            ));
        }

        if context.in_git_repo {
            suggestions.push(Suggestion::new(
                "smartfo status",
                "Check daemon status and queue depth",
                0.9,
            ));
            suggestions.push(Suggestion::new(
                "smartfo list --status pending",
                "Show only pending operations",
                0.8,
            ));
        }

        if context.daemon_running {
            suggestions.push(Suggestion::new(
                "smartfo list --fields id,status,source",
                "Show minimal fields for quick overview",
                0.7,
            ));
        } else {
            suggestions.push(Suggestion::new(
                "smartfo install",
                "Install smartfo symlinks and hooks",
                0.8,
            ));
        }

        suggestions
    }

    /// Suggestions for status command
    fn suggestions_for_status(context: &SuggestionContext) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        if let Some(depth) = context.queue_depth {
            if depth > 0 {
                suggestions.push(Suggestion::new(
                    "smartfo list",
                    "View queued operations",
                    0.95,
                ));
                suggestions.push(Suggestion::new(
                    "smartfo list --status pending",
                    "Show pending operations only",
                    0.85,
                ));
            }
        }

        if context.in_git_repo {
            suggestions.push(Suggestion::new(
                "smartfo list --format toon",
                "View operations in agent-optimized TOON format",
                0.75,
            ));
        }

        suggestions.push(Suggestion::new(
            "smartfo --help",
            "Show all available commands",
            0.6,
        ));

        suggestions
    }

    /// Suggestions for mv command
    fn suggestions_for_mv(context: &SuggestionContext) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        if context.in_git_repo {
            suggestions.push(Suggestion::new(
                "smartfo list",
                "View recent move operations",
                0.9,
            ));
            suggestions.push(Suggestion::new(
                "smartfo status",
                "Check operation queue and daemon status",
                0.8,
            ));
        }

        suggestions.push(Suggestion::new(
            "smartfo list --fields id,status,source,destination",
            "Show move operation details",
            0.7,
        ));

        if !context.daemon_running {
            suggestions.push(Suggestion::new(
                "smartfo install",
                "Install smartfo to enable daemon",
                0.75,
            ));
        }

        suggestions
    }

    /// Suggestions for rm command
    fn suggestions_for_rm(context: &SuggestionContext) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        suggestions.push(Suggestion::new(
            "smartfo list",
            "View trash and recent removals",
            0.95,
        ));

        if context.in_git_repo {
            suggestions.push(Suggestion::new(
                "smartfo list --status completed",
                "Show completed removal operations",
                0.85,
            ));
        }

        suggestions.push(Suggestion::new(
            "smartfo list --fields id,status,source",
            "Show removal operation details",
            0.75,
        ));

        suggestions
    }

    /// Suggestions for install command
    fn suggestions_for_install(context: &SuggestionContext) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        suggestions.push(Suggestion::new(
            "smartfo list",
            "View operation history",
            0.9,
        ));

        if context.in_git_repo {
            suggestions.push(Suggestion::new(
                "smartfo status",
                "Check daemon and queue status",
                0.85,
            ));
        }

        suggestions.push(Suggestion::new(
            "smartfo --help",
            "Explore all available commands",
            0.7,
        ));

        suggestions
    }

    /// Suggestions for no-args (smartfo invoked without command)
    fn suggestions_for_noargs(context: &SuggestionContext) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        if context.in_git_repo {
            suggestions.push(Suggestion::new(
                "smartfo list",
                "View operation history in this repository",
                0.95,
            ));
            suggestions.push(Suggestion::new(
                "smartfo status",
                "Check daemon status and queue depth",
                0.9,
            ));
        } else {
            suggestions.push(Suggestion::new(
                "smartfo install",
                "Install smartfo symlinks and hooks",
                0.9,
            ));
            suggestions.push(Suggestion::new(
                "smartfo --help",
                "Show all available commands and flags",
                0.85,
            ));
        }

        suggestions.push(Suggestion::new(
            "smartfo list --format toon",
            "Use agent-optimized TOON format",
            0.7,
        ));

        suggestions
    }

    /// Generic suggestions for unknown commands
    fn suggestions_for_generic(context: &SuggestionContext) -> Vec<Suggestion> {
        Self::generic_suggestions(context)
    }

    /// Generic fallback suggestions
    fn generic_suggestions(context: &SuggestionContext) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        suggestions.push(Suggestion::new(
            "smartfo --help",
            "Show all available commands",
            0.8,
        ));

        if context.in_git_repo {
            suggestions.push(Suggestion::new(
                "smartfo list",
                "View operation history",
                0.75,
            ));
        }

        suggestions.push(Suggestion::new(
            "smartfo status",
            "Check system status",
            0.7,
        ));

        suggestions
    }
}

/// Format suggestions as a help[] array for TOON output
pub fn format_suggestions_as_help(suggestions: &[Suggestion]) -> Vec<String> {
    suggestions
        .iter()
        .map(|s| format!("{} - {}", s.command, s.description))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggestion_creation() {
        let suggestion = Suggestion::new("smartfo list", "List operations", 0.9);
        assert!(suggestion.is_valid());
        assert_eq!(suggestion.command, "smartfo list");
        assert_eq!(suggestion.description, "List operations");
        assert_eq!(suggestion.relevance, 0.9);
    }

    #[test]
    fn test_suggestion_validation() {
        let valid = Suggestion::new("cmd", "desc", 0.5);
        assert!(valid.is_valid());

        let empty_command = Suggestion::new("", "desc", 0.5);
        assert!(!empty_command.is_valid());

        let empty_desc = Suggestion::new("cmd", "", 0.5);
        assert!(!empty_desc.is_valid());

        let invalid_relevance = Suggestion::new("cmd", "desc", 1.5);
        assert!(!invalid_relevance.is_valid());
    }

    #[test]
    fn test_suggestion_context_creation() {
        let context = SuggestionContext::new("list");
        assert_eq!(context.current_command, "list");
        assert!(!context.in_git_repo);
        assert!(!context.daemon_running);
    }

    #[test]
    fn test_suggestion_context_builder() {
        let context = SuggestionContext::new("list")
            .with_git_repo(true)
            .with_daemon(true)
            .with_queue_depth(5)
            .with_context("key", "value");

        assert!(context.in_git_repo);
        assert!(context.daemon_running);
        assert_eq!(context.queue_depth, Some(5));
        assert_eq!(context.extra_context.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_suggestions_for_list() {
        let context = SuggestionContext::new("list")
            .with_git_repo(true)
            .with_daemon(true);

        let suggestions = SuggestionEngine::generate(&context);

        assert!(!suggestions.is_empty());
        assert!(suggestions.len() <= 4);

        // Verify suggestions are sorted by relevance
        for i in 0..suggestions.len().saturating_sub(1) {
            assert!(suggestions[i].relevance >= suggestions[i + 1].relevance);
        }
    }

    #[test]
    fn test_suggestions_for_status_with_queue() {
        let context = SuggestionContext::new("status")
            .with_git_repo(true)
            .with_queue_depth(10);

        let suggestions = SuggestionEngine::generate(&context);

        assert!(!suggestions.is_empty());
        // Should suggest viewing the queue
        assert!(suggestions.iter().any(|s| s.command.contains("list")));
    }

    #[test]
    fn test_suggestions_for_noargs_git_repo() {
        let context = SuggestionContext::new("")
            .with_git_repo(true);

        let suggestions = SuggestionEngine::generate(&context);

        assert!(!suggestions.is_empty());
        // Should suggest list and status in git repo
        assert!(suggestions.iter().any(|s| s.command.contains("list")));
        assert!(suggestions.iter().any(|s| s.command.contains("status")));
    }

    #[test]
    fn test_suggestions_for_noargs_no_git() {
        let context = SuggestionContext::new("")
            .with_git_repo(false);

        let suggestions = SuggestionEngine::generate(&context);

        assert!(!suggestions.is_empty());
        // Should suggest install when not in git repo
        assert!(suggestions.iter().any(|s| s.command.contains("install")));
    }

    #[test]
    fn test_suggestions_limited_to_4() {
        let context = SuggestionContext::new("list")
            .with_git_repo(true)
            .with_daemon(true)
            .with_queue_depth(5);

        let suggestions = SuggestionEngine::generate(&context);
        assert!(suggestions.len() <= 4);
    }

    #[test]
    fn test_format_suggestions_as_help() {
        let suggestions = vec![
            Suggestion::new("smartfo list", "List operations", 0.9),
            Suggestion::new("smartfo status", "Check status", 0.8),
        ];

        let help = format_suggestions_as_help(&suggestions);

        assert_eq!(help.len(), 2);
        assert!(help[0].contains("smartfo list"));
        assert!(help[0].contains("List operations"));
        assert!(help[1].contains("smartfo status"));
        assert!(help[1].contains("Check status"));
    }

    #[test]
    fn test_suggestions_for_empty_list() {
        let context = SuggestionContext::new("list")
            .with_context("list_empty", "true")
            .with_git_repo(false);

        let suggestions = SuggestionEngine::generate(&context);

        assert!(!suggestions.is_empty());
        // Should suggest install when list is empty
        assert!(suggestions.iter().any(|s| s.command.contains("install")));
    }
}
