use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::{debug, info};

/// Session context output for agent consumption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    /// Current working directory
    pub cwd: String,
    /// Git repository root (if in a repo)
    pub git_root: Option<String>,
    /// Smartfo audit log path (if in a repo)
    pub audit_log_path: Option<String>,
    /// Recent operations count
    pub recent_operations: u64,
    /// Queue size (if daemon is running)
    pub queue_size: Option<u64>,
    /// Session metadata
    pub metadata: SessionMetadata,
}

/// Session metadata for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    /// Session start time
    pub session_start: String,
    /// Last context update
    pub last_update: String,
    /// Context scope (directory, repo, global)
    pub scope: String,
}

impl SessionContext {
    /// Create a new session context for the current directory
    pub fn new() -> Result<Self> {
        let cwd = std::env::current_dir()
            .context("Failed to get current directory")?
            .display()
            .to_string();

        let git_root = detect_git_repo(&cwd);
        let audit_log_path = git_root.as_ref().map(|root| {
            PathBuf::from(root).join(".smartfo/audit/operations.jsonl")
                .display()
                .to_string()
        });

        let recent_operations = if let Some(ref root) = git_root {
            count_recent_operations(root)?
        } else {
            0
        };

        let queue_size = None; // TODO: Implement daemon queue size check

        let now = chrono::Utc::now().to_rfc3339();
        let metadata = SessionMetadata {
            session_start: now.clone(),
            last_update: now,
            scope: if git_root.is_some() { "repo".to_string() } else { "directory".to_string() },
        };

        Ok(Self {
            cwd,
            git_root,
            audit_log_path,
            recent_operations,
            queue_size,
            metadata,
        })
    }

    /// Convert to TOON format for token-efficient output
    /// Respects token budget if set via SMARTFO_TOKEN_BUDGET env var
    pub fn to_toon(&self) -> String {
        let mut toon = String::new();
        toon.push_str("(SessionContext\n");
        toon.push_str(&format!("  (cwd {})\n", crate::output::toon::escape_string(&self.cwd)));
        if let Some(ref root) = self.git_root {
            toon.push_str(&format!("  (git_root {})\n", crate::output::toon::escape_string(root)));
        }
        if let Some(ref audit) = self.audit_log_path {
            toon.push_str(&format!("  (audit_log_path {})\n", crate::output::toon::escape_string(audit)));
        }
        toon.push_str(&format!("  (recent_operations {})\n", self.recent_operations));
        if let Some(size) = self.queue_size {
            toon.push_str(&format!("  (queue_size {})\n", size));
        }
        toon.push_str("  (metadata\n");
        toon.push_str(&format!("    (session_start {})\n", crate::output::toon::escape_string(&self.metadata.session_start)));
        toon.push_str(&format!("    (last_update {})\n", crate::output::toon::escape_string(&self.metadata.last_update)));
        toon.push_str(&format!("    (scope {})\n", crate::output::toon::escape_string(&self.metadata.scope)));
        toon.push_str("  )\n");
        toon.push_str(")\n");
        
        // Apply token budget if set
        if let Ok(budget_str) = std::env::var("SMARTFO_TOKEN_BUDGET") {
            if let Ok(budget) = budget_str.parse::<usize>() {
                if toon.len() > budget {
                    // Truncate to fit budget, preserving structure
                    toon.truncate(budget.saturating_sub(20)); // Leave room for truncation marker
                    toon.push_str("...[truncated]");
                }
            }
        }
        
        toon
    }
}

/// Detect if the current directory is in a Git repository
pub fn detect_git_repo(cwd: &str) -> Option<String> {
    let mut current = PathBuf::from(cwd);
    
    loop {
        let git_dir = current.join(".git");
        if git_dir.exists() {
            return Some(current.display().to_string());
        }
        
        if !current.pop() {
            break;
        }
    }
    
    None
}

/// Detect if the current directory is inside a Git repository (returns PathBuf)
pub fn detect_git_repo_path() -> Option<PathBuf> {
    let current_dir = std::env::current_dir().ok()?;
    let mut dir = current_dir.as_path();

    loop {
        let git_dir = dir.join(".git");
        if git_dir.exists() {
            return Some(dir.to_path_buf());
        }

        match dir.parent() {
            Some(parent) => dir = parent,
            None => return None,
        }
    }
}

/// Count recent operations in the audit log
pub fn count_recent_operations(repo_root: &str) -> Result<u64> {
    let audit_path = PathBuf::from(repo_root).join(".smartfo/audit/operations.jsonl");
    
    if !audit_path.exists() {
        return Ok(0);
    }
    
    let content = fs::read_to_string(&audit_path)
        .with_context(|| format!("Failed to read audit log: {}", audit_path.display()))?;
    
    let count = content.lines().filter(|line| !line.trim().is_empty()).count() as u64;
    Ok(count)
}

/// Hook configuration for agent platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookConfig {
    /// Claude Code hook configuration
    pub claude: Option<ClaudeHookConfig>,
    /// Codex hook configuration
    pub codex: Option<CodexHookConfig>,
}

/// Claude Code hook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeHookConfig {
    /// Path to Claude settings file
    pub settings_path: PathBuf,
    /// Hook command to run
    pub hook_command: String,
}

/// Codex hook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexHookConfig {
    /// Path to Codex hooks file
    pub hooks_path: PathBuf,
    /// Hook command to run
    pub hook_command: String,
}

/// Install agent hooks for the current platform
pub fn install_agent_hooks() -> Result<()> {
    info!("Installing agent hooks");
    
    // Detect platform
    let platform = detect_agent_platform()?;
    
    match platform {
        AgentPlatform::ClaudeCode => install_claude_hooks()?,
        AgentPlatform::Codex => install_codex_hooks()?,
        AgentPlatform::Unknown => {
            anyhow::bail!("No supported agent platform detected");
        }
    }
    
    Ok(())
}

/// Detect the current agent platform
pub fn detect_agent_platform() -> Result<AgentPlatform> {
    // Check for Claude Code
    if let Ok(_) = std::env::var("CLAUDE_SESSION") {
        return Ok(AgentPlatform::ClaudeCode);
    }
    
    // Check for Codex
    if let Ok(_) = std::env::var("CODEX_SESSION") {
        return Ok(AgentPlatform::Codex);
    }
    
    // Check for Claude settings file
    let home = std::env::var("HOME").context("HOME not set")?;
    let claude_settings = PathBuf::from(home.clone()).join(".claude/settings.json");
    if claude_settings.exists() {
        return Ok(AgentPlatform::ClaudeCode);
    }
    
    // Check for Codex hooks file
    let codex_hooks = PathBuf::from(home).join(".codex/hooks.json");
    if codex_hooks.exists() {
        return Ok(AgentPlatform::Codex);
    }
    
    Ok(AgentPlatform::Unknown)
}

/// Agent platform type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentPlatform {
    ClaudeCode,
    Codex,
    Unknown,
}

/// Install Claude Code hooks
pub fn install_claude_hooks() -> Result<()> {
    info!("Installing Claude Code hooks");
    
    let home = std::env::var("HOME").context("HOME not set")?;
    let settings_path = PathBuf::from(home).join(".claude/settings.json");
    
    // Resolve smartfo binary path
    let smartfo_path = resolve_smartfo_binary()?;
    
    // Read existing settings or create new
    let mut settings: serde_json::Value = if settings_path.exists() {
        let content = fs::read_to_string(&settings_path)
            .with_context(|| format!("Failed to read Claude settings: {}", settings_path.display()))?;
        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse Claude settings: {}", settings_path.display()))?
    } else {
        serde_json::json!({})
    };
    
    // Add session-start hook
    if let Some(hooks) = settings.get_mut("hooks") {
        if let Some(session_hooks) = hooks.get_mut("session-start") {
            if let Some(hooks_array) = session_hooks.as_array_mut() {
                let hook_command = format!("{} --session-context", smartfo_path.display());
                if !hooks_array.iter().any(|h| h.as_str() == Some(&hook_command)) {
                    hooks_array.push(serde_json::json!(hook_command));
                    info!("Added session-start hook: {}", hook_command);
                }
            }
        }
    }
    
    // Add session-end hook
    if let Some(hooks) = settings.get_mut("hooks") {
        if let Some(session_hooks) = hooks.get_mut("session-end") {
            if let Some(hooks_array) = session_hooks.as_array_mut() {
                let hook_command = format!("{} --session-context", smartfo_path.display());
                if !hooks_array.iter().any(|h| h.as_str() == Some(&hook_command)) {
                    hooks_array.push(serde_json::json!(hook_command));
                    info!("Added session-end hook: {}", hook_command);
                }
            }
        }
    }
    
    // Write settings back
    let settings_json = serde_json::to_string_pretty(&settings)
        .context("Failed to serialize Claude settings")?;
    
    fs::write(&settings_path, settings_json)
        .with_context(|| format!("Failed to write Claude settings: {}", settings_path.display()))?;
    
    info!("Claude Code hooks installed successfully");
    Ok(())
}

/// Install Codex hooks
pub fn install_codex_hooks() -> Result<()> {
    info!("Installing Codex hooks");
    
    let home = std::env::var("HOME").context("HOME not set")?;
    let hooks_path = PathBuf::from(home).join(".codex/hooks.json");
    
    // Resolve smartfo binary path
    let smartfo_path = resolve_smartfo_binary()?;
    
    // Read existing hooks or create new
    let mut hooks: serde_json::Value = if hooks_path.exists() {
        let content = fs::read_to_string(&hooks_path)
            .with_context(|| format!("Failed to read Codex hooks: {}", hooks_path.display()))?;
        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse Codex hooks: {}", hooks_path.display()))?
    } else {
        serde_json::json!({})
    };
    
    // Add session-start hook
    if let Some(session_hooks) = hooks.get_mut("session-start") {
        if let Some(hooks_array) = session_hooks.as_array_mut() {
            let hook_command = format!("{} --session-context", smartfo_path.display());
            if !hooks_array.iter().any(|h| h.as_str() == Some(&hook_command)) {
                hooks_array.push(serde_json::json!(hook_command));
                info!("Added session-start hook: {}", hook_command);
            }
        }
    }
    
    // Add session-end hook
    if let Some(session_hooks) = hooks.get_mut("session-end") {
        if let Some(hooks_array) = session_hooks.as_array_mut() {
            let hook_command = format!("{} --session-context", smartfo_path.display());
            if !hooks_array.iter().any(|h| h.as_str() == Some(&hook_command)) {
                hooks_array.push(serde_json::json!(hook_command));
                info!("Added session-end hook: {}", hook_command);
            }
        }
    }
    
    // Write hooks back
    let hooks_json = serde_json::to_string_pretty(&hooks)
        .context("Failed to serialize Codex hooks")?;
    
    fs::write(&hooks_path, hooks_json)
        .with_context(|| format!("Failed to write Codex hooks: {}", hooks_path.display()))?;
    
    info!("Codex hooks installed successfully");
    Ok(())
}

/// Resolve the smartfo binary path (PATH-verified with absolute fallback)
pub fn resolve_smartfo_binary() -> Result<PathBuf> {
    // First, try to find smartfo in PATH
    if let Ok(path) = which::which("smartfo") {
        debug!("Found smartfo in PATH: {}", path.display());
        return Ok(path);
    }
    
    // Fallback to current executable
    let current_exe = std::env::current_exe()
        .context("Failed to get current executable path")?;
    
    debug!("Using current executable as smartfo path: {}", current_exe.display());
    Ok(current_exe)
}

/// Cache session metadata for future context enrichment
pub fn cache_session_metadata(context: &SessionContext) -> Result<()> {
    let cache_dir = get_session_cache_dir()?;
    let cache_file = cache_dir.join("session-metadata.json");
    
    let metadata_json = serde_json::to_string_pretty(context)
        .context("Failed to serialize session metadata")?;
    
    fs::write(&cache_file, metadata_json)
        .with_context(|| format!("Failed to write session metadata cache: {}", cache_file.display()))?;
    
    debug!("Session metadata cached to: {}", cache_file.display());
    Ok(())
}

/// Get the session cache directory
pub fn get_session_cache_dir() -> Result<PathBuf> {
    let cache_dir = std::env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".cache")
        });
    
    let smartfo_cache = cache_dir.join("smartfo");
    std::fs::create_dir_all(&smartfo_cache)
        .with_context(|| format!("Failed to create cache directory: {}", smartfo_cache.display()))?;
    
    Ok(smartfo_cache)
}

/// Load cached session metadata
pub fn load_session_metadata() -> Result<Option<SessionContext>> {
    let cache_dir = get_session_cache_dir()?;
    let cache_file = cache_dir.join("session-metadata.json");
    
    if !cache_file.exists() {
        return Ok(None);
    }
    
    let content = fs::read_to_string(&cache_file)
        .with_context(|| format!("Failed to read session metadata cache: {}", cache_file.display()))?;
    
    let context: SessionContext = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse session metadata cache: {}", cache_file.display()))?;
    
    debug!("Loaded session metadata from cache");
    Ok(Some(context))
}
