// Library exports for testing
pub mod config;
pub mod output;
pub mod hooks;
pub mod skill;
pub mod globbing;
pub mod logging;
pub mod exit;
pub mod error;
pub mod dry_run;
pub mod confirmation;
pub mod progress;
pub mod worker;
pub mod queue;
pub mod trash;

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

// Re-export logging types for testing
pub use logging::{LogLevel, LogFormat, init_logging};

// Re-export exit code types for testing
pub use exit::{ExitCode, SignalHandler, error_category_to_exit_code, ErrorCategory};

// Re-export error types for testing

// Re-export confirmation types for testing
pub use confirmation::{ConfirmationState, confirm, confirm_batch, confirm_destructive};
pub use error::{SmartfoError, Result};
