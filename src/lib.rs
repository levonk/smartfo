// Library exports for testing
pub mod config;
pub mod output;
pub mod hooks;
pub mod skill;

// Re-export aggregate types for testing
pub use output::aggregates::{ListAggregate, OperationAggregate, QueueAggregate, DaemonAggregate, StatusAggregate, AggregateComputer};

// Re-export hooks types and functions for testing
pub use hooks::{SessionContext, SessionMetadata, AgentPlatform, cache_session_metadata, load_session_metadata, detect_git_repo_path, detect_agent_platform, install_claude_hooks, install_codex_hooks, resolve_smartfo_binary};

// Re-export skill types for testing
pub use skill::{SkillGenerator, GeneratedSkill, SkillMetadata, CommandDoc, FlagDoc, check_skill_stale};
