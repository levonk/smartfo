// Library exports for testing
pub mod config;
pub mod output;
pub mod hooks;
pub mod skill;
pub mod globbing;

// Re-export ColorMode for testing
pub use config::ColorMode;

// Re-export aggregate types for testing
pub use output::aggregates::{ListAggregate, OperationAggregate, QueueAggregate, DaemonAggregate, StatusAggregate, AggregateComputer};

// Re-export hooks types and functions for testing
pub use hooks::{SessionContext, SessionMetadata, AgentPlatform, cache_session_metadata, load_session_metadata, detect_git_repo_path, detect_agent_platform, install_claude_hooks, install_codex_hooks, resolve_smartfo_binary};

// Re-export skill types for testing
pub use skill::{SkillGenerator, GeneratedSkill, SkillMetadata, CommandDoc, FlagDoc, check_skill_stale};

// Re-export suggestion types for testing
pub use output::suggestions::{Suggestion, SuggestionContext, SuggestionEngine, format_suggestions_as_help};

// Re-export TOON types for testing
pub use output::toon::{ToonEncoder, to_string, ToonError};

// Re-export schema types for testing
pub use output::schema::{Schema, Field, FieldSelector};

// Re-export truncation types for testing
pub use output::truncation::{truncate, TruncatedContent};
