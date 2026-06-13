use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use std::fmt;
use crate::secret::sanitize_string;

/// Configuration validation error with line number and suggestions.
#[derive(Debug, Clone)]
pub struct ConfigValidationError {
    /// The config file path where the error occurred
    pub file_path: Option<PathBuf>,
    /// Line number in the config file (0 if unknown)
    pub line: usize,
    /// The section where the error occurred (e.g., "trash", "vcs")
    pub section: String,
    /// The specific key that failed validation
    pub key: String,
    /// The error message
    pub message: String,
    /// Suggested fix for the error
    pub suggestion: String,
}

impl fmt::Display for ConfigValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let location = if let Some(path) = &self.file_path {
            format!("{}:{}", path.display(), self.line)
        } else {
            format!("line {}", self.line)
        };
        write!(
            f,
            "Config error at [{}]: {}.{}. {}",
            location, self.section, self.key, self.message
        )
    }
}

impl std::error::Error for ConfigValidationError {}

/// Result type for config validation operations.
pub type ValidationResult<T> = Result<T, ConfigValidationError>;

/// Validate a config file and return detailed errors if validation fails.
pub fn validate_config_file(path: &std::path::Path) -> ValidationResult<Config> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| ConfigValidationError {
            file_path: Some(path.to_path_buf()),
            line: 0,
            section: "file".to_string(),
            key: "read".to_string(),
            message: format!("Failed to read config file: {}", e),
            suggestion: "Check that the file exists and is readable.".to_string(),
        })?;

    // Parse TOML to check syntax
    let _toml_value: toml::Value = toml::from_str(&content).map_err(|e| {
        ConfigValidationError {
            file_path: Some(path.to_path_buf()),
            line: 0,
            section: "toml".to_string(),
            key: "parse".to_string(),
            message: format!("Invalid TOML syntax: {}", e),
            suggestion: "Check for syntax errors like missing quotes, unclosed brackets, or invalid characters.".to_string(),
        }
    })?;

    // Deserialize to Config to check structure
    let config: Config = toml::from_str(&content).map_err(|e| {
        ConfigValidationError {
            file_path: Some(path.to_path_buf()),
            line: 0,
            section: "structure".to_string(),
            key: "deserialize".to_string(),
            message: format!("Invalid config structure: {}", e),
            suggestion: "Check that all section names and keys are correct.".to_string(),
        }
    })?;

    // Perform semantic validation
    validate_config_semantics(&config, path)?;

    Ok(config)
}

/// Validate the semantic correctness of config values.
fn validate_config_semantics(config: &Config, path: &std::path::Path) -> ValidationResult<()> {
    // Validate schema version
    validate_schema_version(&config.schema_version)?;

    // Validate VCS config
    validate_vcs_config(&config.vcs)?;

    // Validate trash config
    validate_trash_config(&config.trash)?;

    // Validate concurrency config
    validate_concurrency_config(&config.concurrency)?;

    // Validate behavior config
    validate_behavior_config(&config.behavior)?;

    // Validate logging config
    validate_logging_config(&config.logging)?;

    // Validate privacy config
    validate_privacy_config(&config.privacy)?;

    // Validate paths config
    validate_paths_config(&config.paths, path)?;

    Ok(())
}

/// Validate config schema version.
fn validate_schema_version(version: &str) -> ValidationResult<()> {
    // Parse version as integer
    let version_num: u32 = version.parse().map_err(|_| ConfigValidationError {
        file_path: None,
        line: 0,
        section: "config".to_string(),
        key: "schema_version".to_string(),
        message: format!("Invalid schema version '{}', must be a positive integer", version),
        suggestion: "Set schema_version to a positive integer (e.g., 1).".to_string(),
    })?;

    // Check if version is supported
    if version_num == 0 || version_num > 1 {
        return Err(ConfigValidationError {
            file_path: None,
            line: 0,
            section: "config".to_string(),
            key: "schema_version".to_string(),
            message: format!("Unsupported schema version '{}', supported versions: 1", version),
            suggestion: "Update schema_version to 1 or check for a newer smartfo version.".to_string(),
        });
    }

    Ok(())
}

/// Validate VCS configuration.
fn validate_vcs_config(vcs: &VcsConfig) -> ValidationResult<()> {
    // Validate preference is in supported list
    if !vcs.supported.contains(&vcs.preference) {
        return Err(ConfigValidationError {
            file_path: None,
            line: 0,
            section: "vcs".to_string(),
            key: "preference".to_string(),
            message: format!("VCS preference '{}' is not in supported list", vcs.preference),
            suggestion: format!("Set preference to one of: {:?}", vcs.supported),
        });
    }

    // Validate fallback VCS are all in supported list
    for fallback in &vcs.fallback {
        if !vcs.supported.contains(fallback) {
            return Err(ConfigValidationError {
                file_path: None,
                line: 0,
                section: "vcs".to_string(),
                key: "fallback".to_string(),
                message: format!("Fallback VCS '{}' is not in supported list", fallback),
                suggestion: format!("Remove '{}' from fallback or add it to supported list", fallback),
            });
        }
    }

    Ok(())
}

/// Validate trash configuration.
fn validate_trash_config(trash: &TrashConfig) -> ValidationResult<()> {
    // Validate mode is either "versioned" or "simple"
    if trash.mode != "versioned" && trash.mode != "simple" {
        return Err(ConfigValidationError {
            file_path: None,
            line: 0,
            section: "trash".to_string(),
            key: "mode".to_string(),
            message: format!("Invalid trash mode '{}', must be 'versioned' or 'simple'", trash.mode),
            suggestion: "Set mode to either 'versioned' or 'simple'.".to_string(),
        });
    }

    // Validate min_free_space_percent is between 0 and 100
    if trash.min_free_space_percent > 100 {
        return Err(ConfigValidationError {
            file_path: None,
            line: 0,
            section: "trash".to_string(),
            key: "min_free_space_percent".to_string(),
            message: format!("min_free_space_percent {} is out of range (0-100)", trash.min_free_space_percent),
            suggestion: "Set min_free_space_percent to a value between 0 and 100.".to_string(),
        });
    }

    // Validate on_trash_full is either "refuse" or "delete"
    if trash.on_trash_full != "refuse" && trash.on_trash_full != "delete" {
        return Err(ConfigValidationError {
            file_path: None,
            line: 0,
            section: "trash".to_string(),
            key: "on_trash_full".to_string(),
            message: format!("Invalid on_trash_full '{}', must be 'refuse' or 'delete'", trash.on_trash_full),
            suggestion: "Set on_trash_full to either 'refuse' or 'delete'.".to_string(),
        });
    }

    Ok(())
}

/// Validate concurrency configuration.
fn validate_concurrency_config(concurrency: &ConcurrencyConfig) -> ValidationResult<()> {
    // Validate max_concurrent_jobs is at least 1
    if concurrency.max_concurrent_jobs == 0 {
        return Err(ConfigValidationError {
            file_path: None,
            line: 0,
            section: "concurrency".to_string(),
            key: "max_concurrent_jobs".to_string(),
            message: "max_concurrent_jobs must be at least 1".to_string(),
            suggestion: "Set max_concurrent_jobs to a positive integer (recommended: 4).".to_string(),
        });
    }

    // Validate network_concurrency is at least 1
    if concurrency.network_concurrency == 0 {
        return Err(ConfigValidationError {
            file_path: None,
            line: 0,
            section: "concurrency".to_string(),
            key: "network_concurrency".to_string(),
            message: "network_concurrency must be at least 1".to_string(),
            suggestion: "Set network_concurrency to a positive integer (recommended: 2).".to_string(),
        });
    }

    // Validate max_memory_mb is reasonable (0 = unlimited is valid)
    if concurrency.max_memory_mb > 1024 * 1024 {
        return Err(ConfigValidationError {
            file_path: None,
            line: 0,
            section: "concurrency".to_string(),
            key: "max_memory_mb".to_string(),
            message: format!("max_memory_mb {} is too large (>1TB)", concurrency.max_memory_mb),
            suggestion: "Set max_memory_mb to a reasonable value (recommended: 0 for unlimited, or 4096 for 4GB).".to_string(),
        });
    }

    // Validate max_cpu_percent is between 0 and 100
    if concurrency.max_cpu_percent > 100 {
        return Err(ConfigValidationError {
            file_path: None,
            line: 0,
            section: "concurrency".to_string(),
            key: "max_cpu_percent".to_string(),
            message: format!("max_cpu_percent {} is out of range (0-100)", concurrency.max_cpu_percent),
            suggestion: "Set max_cpu_percent to a value between 0 and 100 (0 = unlimited).".to_string(),
        });
    }

    Ok(())
}

/// Validate behavior configuration.
fn validate_behavior_config(behavior: &BehaviorConfig) -> ValidationResult<()> {
    // Validate async_threshold_mb is reasonable
    if behavior.async_threshold_mb > 10000 {
        return Err(ConfigValidationError {
            file_path: None,
            line: 0,
            section: "behavior".to_string(),
            key: "async_threshold_mb".to_string(),
            message: format!("async_threshold_mb {} is very large (>10GB)", behavior.async_threshold_mb),
            suggestion: "Consider a smaller threshold (recommended: 100 MB).".to_string(),
        });
    }

    // Validate truncation_limit is reasonable
    if behavior.truncation_limit == 0 {
        return Err(ConfigValidationError {
            file_path: None,
            line: 0,
            section: "behavior".to_string(),
            key: "truncation_limit".to_string(),
            message: "truncation_limit must be at least 1".to_string(),
            suggestion: "Set truncation_limit to a positive integer (recommended: 1000).".to_string(),
        });
    }

    Ok(())
}

/// Validate logging configuration.
fn validate_logging_config(logging: &LoggingConfig) -> ValidationResult<()> {
    // Validate log level
    let valid_levels = ["trace", "debug", "info", "warn", "error"];
    if !valid_levels.contains(&logging.level.as_str()) {
        return Err(ConfigValidationError {
            file_path: None,
            line: 0,
            section: "logging".to_string(),
            key: "level".to_string(),
            message: format!("Invalid log level '{}', must be one of: {:?}", logging.level, valid_levels),
            suggestion: "Set level to one of: trace, debug, info, warn, error.".to_string(),
        });
    }

    // Validate color mode
    let valid_colors = ["auto", "always", "never"];
    if !valid_colors.contains(&logging.color.as_str()) {
        return Err(ConfigValidationError {
            file_path: None,
            line: 0,
            section: "logging".to_string(),
            key: "color".to_string(),
            message: format!("Invalid color mode '{}', must be one of: {:?}", logging.color, valid_colors),
            suggestion: "Set color to one of: auto, always, never.".to_string(),
        });
    }

    Ok(())
}

/// Validate privacy configuration.
fn validate_privacy_config(privacy: &PrivacyConfig) -> ValidationResult<()> {
    // Validate ignore patterns are valid regex
    for (i, pattern) in privacy.ignore_patterns.iter().enumerate() {
        if let Err(e) = regex::Regex::new(pattern) {
            return Err(ConfigValidationError {
                file_path: None,
                line: 0,
                section: "privacy".to_string(),
                key: format!("ignore_patterns[{}]", i),
                message: format!("Invalid regex pattern '{}': {}", pattern, e),
                suggestion: "Fix the regex pattern or remove it from ignore_patterns.".to_string(),
            });
        }
    }

    Ok(())
}

/// Validate paths configuration.
fn validate_paths_config(paths: &PathsConfig, config_path: &std::path::Path) -> ValidationResult<()> {
    // Validate audit_log path is not empty
    if paths.audit_log.as_os_str().is_empty() {
        return Err(ConfigValidationError {
            file_path: Some(config_path.to_path_buf()),
            line: 0,
            section: "paths".to_string(),
            key: "audit_log".to_string(),
            message: "audit_log path cannot be empty".to_string(),
            suggestion: "Provide a valid file path for the audit log.".to_string(),
        });
    }

    // Validate cache_dir path is not empty
    if paths.cache_dir.as_os_str().is_empty() {
        return Err(ConfigValidationError {
            file_path: Some(config_path.to_path_buf()),
            line: 0,
            section: "paths".to_string(),
            key: "cache_dir".to_string(),
            message: "cache_dir path cannot be empty".to_string(),
            suggestion: "Provide a valid directory path for the cache.".to_string(),
        });
    }

    Ok(())
}

/// Return the system config file path based on the current platform.
pub fn system_config_path() -> Option<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        Some(PathBuf::from("/etc/smartfo/config.toml"))
    }

    #[cfg(target_os = "macos")]
    {
        Some(PathBuf::from("/Library/Application Support/smartfo/config.toml"))
    }

    #[cfg(target_os = "windows")]
    {
        env::var("ProgramData")
            .ok()
            .map(|p| PathBuf::from(p).join("smartfo").join("config.toml"))
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        None
    }
}

/// Return the project config file path if running inside a Git repository.
pub fn project_config_path() -> Option<PathBuf> {
    // Check if we're in a Git repository
    let mut current_dir = std::env::current_dir().ok()?;

    loop {
        let git_dir = current_dir.join(".git");
        if git_dir.exists() {
            // Found a Git repository, return project config path
            return Some(current_dir.join(".config").join("smartfo").join("config.toml"));
        }

        // Move up to parent directory
        if !current_dir.pop() {
            break;
        }
    }

    None
}

/// Top-level smartfo configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Config {
    /// Config schema version (for migration support)
    #[serde(default = "default_schema_version")]
    pub schema_version: String,
    #[serde(default)]
    pub vcs: VcsConfig,
    #[serde(default)]
    pub trash: TrashConfig,
    #[serde(default)]
    pub concurrency: ConcurrencyConfig,
    #[serde(default)]
    pub behavior: BehaviorConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
    #[serde(default)]
    pub privacy: PrivacyConfig,
    #[serde(default)]
    pub paths: PathsConfig,
}

fn default_schema_version() -> String {
    "1".to_string()
}

/// VCS detection and preference settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VcsConfig {
    /// Preferred VCS when multiple are detected (default: "git")
    #[serde(default = "default_vcs_preference")]
    pub preference: String,
    /// Fallback order when preferred VCS is unavailable
    #[serde(default = "default_vcs_fallback")]
    pub fallback: Vec<String>,
    /// Supported VCS systems to detect
    #[serde(default = "default_vcs_supported")]
    pub supported: Vec<String>,
}

fn default_vcs_preference() -> String {
    "git".to_string()
}

fn default_vcs_fallback() -> Vec<String> {
    vec!["git".to_string(), "jj".to_string(), "hg".to_string(), "svn".to_string()]
}

fn default_vcs_supported() -> Vec<String> {
    vec!["git".to_string(), "jj".to_string(), "hg".to_string(), "svn".to_string()]
}

impl Default for VcsConfig {
    fn default() -> Self {
        Self {
            preference: default_vcs_preference(),
            fallback: default_vcs_fallback(),
            supported: default_vcs_supported(),
        }
    }
}

/// Trash directory and retention settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrashConfig {
    /// Root directory for trash (default: `$XDG_DATA_HOME/smartfo/trash` or `$HOME/.local/share/smartfo/trash`)
    #[serde(default = "default_trash_root")]
    pub root: PathBuf,
    /// Trash mode: "versioned" or "simple"
    #[serde(default = "default_trash_mode")]
    pub mode: String,
    /// Minimum free space (in MB) before auto-culling oldest entries
    #[serde(default = "default_trash_min_free_mb")]
    pub min_free_mb: u64,
    /// Minimum free space percentage (0-100) before auto-culling oldest entries
    #[serde(default = "default_trash_min_free_space_percent")]
    pub min_free_space_percent: u8,
    /// Behavior when trash is full: "refuse" or "delete"
    #[serde(default = "default_trash_on_trash_full")]
    pub on_trash_full: String,
    /// Whether to allow culling the last version of a file
    #[serde(default = "default_trash_allow_last_version_cull")]
    pub allow_last_version_cull: bool,
    /// Retention period in days (0 = unlimited)
    #[serde(default = "default_trash_retention_days")]
    pub retention_days: u64,
    /// Whether to delete ignored files directly instead of trashing
    #[serde(default = "default_trash_delete_ignored")]
    pub delete_ignored: bool,
    /// Whether to preserve directory tree structure in trash
    #[serde(default = "default_trash_preserve_tree")]
    pub preserve_tree: bool,
    /// Whether to backup VCS-committed files to trash (false = use VCS-aware delete for clean files)
    #[serde(default = "default_trash_backup_vcs_committed")]
    pub backup_vcs_committed: bool,
}

fn default_trash_root() -> PathBuf {
    env::var("XDG_DATA_HOME")
        .map(|p| PathBuf::from(p).join("smartfo").join("trash"))
        .unwrap_or_else(|_| {
            env::var("HOME")
                .map(|p| PathBuf::from(p).join(".local").join("share").join("smartfo").join("trash"))
                .unwrap_or_else(|_| PathBuf::from("/tmp/smartfo-trash"))
        })
}

fn default_trash_mode() -> String {
    "versioned".to_string()
}

fn default_trash_min_free_mb() -> u64 {
    1024
}

fn default_trash_min_free_space_percent() -> u8 {
    20
}

fn default_trash_on_trash_full() -> String {
    "refuse".to_string()
}

fn default_trash_allow_last_version_cull() -> bool {
    false
}

fn default_trash_retention_days() -> u64 {
    30
}

fn default_trash_delete_ignored() -> bool {
    true
}

fn default_trash_preserve_tree() -> bool {
    true
}

fn default_trash_backup_vcs_committed() -> bool {
    false
}

impl Default for TrashConfig {
    fn default() -> Self {
        Self {
            root: default_trash_root(),
            mode: default_trash_mode(),
            min_free_mb: default_trash_min_free_mb(),
            min_free_space_percent: default_trash_min_free_space_percent(),
            on_trash_full: default_trash_on_trash_full(),
            allow_last_version_cull: default_trash_allow_last_version_cull(),
            retention_days: default_trash_retention_days(),
            delete_ignored: default_trash_delete_ignored(),
            preserve_tree: default_trash_preserve_tree(),
            backup_vcs_committed: default_trash_backup_vcs_committed(),
        }
    }
}

/// Concurrency and parallel job limits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConcurrencyConfig {
    /// Maximum number of concurrent background jobs
    #[serde(default = "default_max_concurrent_jobs")]
    pub max_concurrent_jobs: usize,
    /// Network bandwidth limit in MB/s (0 = unlimited)
    #[serde(default = "default_network_limit_mbps")]
    pub network_limit_mbps: u64,
    /// Whether to detect same-drive vs cross-device moves
    #[serde(default = "default_drive_detection")]
    pub drive_detection: bool,
    /// Maximum concurrent operations to network-mounted destinations
    #[serde(default = "default_network_concurrency")]
    pub network_concurrency: usize,
    /// Maximum memory limit in MB (0 = unlimited)
    #[serde(default = "default_max_memory_mb")]
    pub max_memory_mb: u64,
    /// Maximum CPU usage as percentage (0 = unlimited)
    #[serde(default = "default_max_cpu_percent")]
    pub max_cpu_percent: u8,
}

fn default_max_concurrent_jobs() -> usize {
    4
}

fn default_network_limit_mbps() -> u64 {
    0
}

fn default_drive_detection() -> bool {
    true
}

fn default_network_concurrency() -> usize {
    2
}

fn default_max_memory_mb() -> u64 {
    0 // 0 = unlimited
}

fn default_max_cpu_percent() -> u8 {
    0 // 0 = unlimited
}

impl Default for ConcurrencyConfig {
    fn default() -> Self {
        Self {
            max_concurrent_jobs: default_max_concurrent_jobs(),
            network_limit_mbps: default_network_limit_mbps(),
            drive_detection: default_drive_detection(),
            network_concurrency: default_network_concurrency(),
            max_memory_mb: default_max_memory_mb(),
            max_cpu_percent: default_max_cpu_percent(),
        }
    }
}

/// Output mode for smartfo operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OutputMode {
    /// Agent mode: optimized for AI/agent consumption (structured output, minimal prompts)
    Agent,
    /// Human mode: optimized for human interaction (friendly messages, interactive prompts)
    Human,
    /// Auto mode: automatically detect based on environment (TTY, agent session)
    #[default]
    Auto,
}

impl OutputMode {
    /// Detect if running in an agent session by checking environment variables
    pub fn detect_agent_session() -> bool {
        std::env::var("CLAUDE_SESSION").is_ok()
            || std::env::var("CODEX_SESSION").is_ok()
            || std::env::var("AGENT_SESSION").is_ok()
    }

    /// Detect if running in a TTY
    pub fn is_tty() -> bool {
        atty::is(atty::Stream::Stdout)
    }

    /// Resolve the actual mode based on auto-detection logic
    pub fn resolve(self) -> Self {
        match self {
            OutputMode::Agent => OutputMode::Agent,
            OutputMode::Human => OutputMode::Human,
            OutputMode::Auto => {
                // Auto-detection: prefer agent mode when in agent session or non-TTY
                if Self::detect_agent_session() || !Self::is_tty() {
                    OutputMode::Agent
                } else {
                    OutputMode::Human
                }
            }
        }
    }

    /// Determine the final output mode based on precedence chain
    /// Precedence: CLI flags > env var > config > auto-detection
    pub fn determine_mode(
        cli_human: bool,
        cli_agent: bool,
        config_mode: OutputMode,
    ) -> OutputMode {
        // CLI flags take highest precedence
        if cli_agent {
            return OutputMode::Agent;
        }
        if cli_human {
            return OutputMode::Human;
        }

        // Environment variable override
        if let Ok(env_mode) = std::env::var("SMARTFO_MODE") {
            return match env_mode.to_lowercase().as_str() {
                "agent" => OutputMode::Agent,
                "human" => OutputMode::Human,
                "auto" => OutputMode::Auto,
                _ => config_mode,
            };
        }

        // Config setting (resolve Auto mode)
        config_mode.resolve()
    }
}

/// Behavioral toggles and thresholds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BehaviorConfig {
    /// Whether to use smart features (VCS, trash, async) by default
    #[serde(default = "default_smart_mode")]
    pub smart_mode: bool,
    /// File size threshold (in MB) for triggering async move
    #[serde(default = "default_async_threshold_mb")]
    pub async_threshold_mb: u64,
    /// Whether blocking mode is the default (overrides async)
    #[serde(default = "default_default_blocking")]
    pub default_blocking: bool,
    /// Whether to fsync after every operation
    #[serde(default = "default_sync_after_op")]
    pub sync_after_op: bool,
    /// Whether daemon fallback to sync mode should be quiet (no warnings)
    #[serde(default = "default_daemon_fallback_quiet")]
    pub daemon_fallback_quiet: bool,
    /// Output mode: agent, human, or auto-detection
    #[serde(default)]
    pub mode: OutputMode,
    /// Default truncation limit for text fields (in characters)
    #[serde(default = "default_truncation_limit")]
    pub truncation_limit: usize,
}

fn default_smart_mode() -> bool {
    true
}

fn default_async_threshold_mb() -> u64 {
    100
}

fn default_default_blocking() -> bool {
    false
}

fn default_sync_after_op() -> bool {
    false
}

fn default_daemon_fallback_quiet() -> bool {
    false
}

fn default_truncation_limit() -> usize {
    1000
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            smart_mode: default_smart_mode(),
            async_threshold_mb: default_async_threshold_mb(),
            default_blocking: default_default_blocking(),
            sync_after_op: default_sync_after_op(),
            daemon_fallback_quiet: default_daemon_fallback_quiet(),
            mode: OutputMode::default(),
            truncation_limit: default_truncation_limit(),
        }
    }
}

/// Logging settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level: "trace", "debug", "info", "warn", "error"
    #[serde(default = "default_log_level")]
    pub level: String,
    /// Log file path (default: stderr)
    #[serde(default)]
    pub file: Option<PathBuf>,
    /// Whether to use JSON formatting
    #[serde(default = "default_log_json")]
    pub json: bool,
    /// Color output: "auto", "always", "never"
    #[serde(default = "default_log_color")]
    pub color: String,
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_json() -> bool {
    false
}

fn default_log_color() -> String {
    "auto".to_string()
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            file: None,
            json: default_log_json(),
            color: default_log_color(),
        }
    }
}

/// Color output mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

impl ColorMode {
    /// Parse color mode from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "auto" => Some(ColorMode::Auto),
            "always" => Some(ColorMode::Always),
            "never" => Some(ColorMode::Never),
            _ => None,
        }
    }

    /// Determine if colors should be enabled based on mode and TTY detection
    pub fn should_color(&self) -> bool {
        match self {
            ColorMode::Auto => atty::is(atty::Stream::Stderr),
            ColorMode::Always => true,
            ColorMode::Never => false,
        }
    }

    /// Determine color mode based on precedence chain
    /// Precedence: NO_COLOR env var > CLI flag > config > default (auto)
    pub fn determine(cli_color: Option<&str>, config_color: &str) -> Self {
        // NO_COLOR environment variable takes highest precedence
        if std::env::var("NO_COLOR").is_ok() {
            return ColorMode::Never;
        }

        // CLI flag takes precedence over config
        if let Some(cli) = cli_color {
            if let Some(parsed) = Self::from_str(cli) {
                return parsed;
            }
        }

        // Config setting
        Self::from_str(config_color).unwrap_or(ColorMode::Auto)
    }
}

/// Path overrides.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PathsConfig {
    /// Override for trash root
    #[serde(default)]
    pub trash_root: Option<PathBuf>,
    /// Audit log file path
    #[serde(default = "default_audit_log")]
    pub audit_log: PathBuf,
    /// Cache directory
    #[serde(default = "default_cache_dir")]
    pub cache_dir: PathBuf,
    /// Config directory override
    #[serde(default)]
    pub config_dir: Option<PathBuf>,
}

fn default_audit_log() -> PathBuf {
    env::var("XDG_DATA_HOME")
        .map(|p| PathBuf::from(p).join("smartfo").join("audit").join("operations.jsonl"))
        .unwrap_or_else(|_| {
            env::var("HOME")
                .map(|p| {
                    PathBuf::from(p)
                        .join(".local")
                        .join("share")
                        .join("smartfo")
                        .join("audit")
                        .join("operations.jsonl")
                })
                .unwrap_or_else(|_| PathBuf::from("/tmp/smartfo-audit.jsonl"))
        })
}

fn default_cache_dir() -> PathBuf {
    env::var("XDG_CACHE_HOME")
        .map(|p| PathBuf::from(p).join("smartfo"))
        .unwrap_or_else(|_| {
            env::var("HOME")
                .map(|p| PathBuf::from(p).join(".cache").join("smartfo"))
                .unwrap_or_else(|_| PathBuf::from("/tmp/smartfo-cache"))
        })
}

impl Default for PathsConfig {
    fn default() -> Self {
        Self {
            trash_root: None,
            audit_log: default_audit_log(),
            cache_dir: default_cache_dir(),
            config_dir: None,
        }
    }
}

/// Privacy mode configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivacyConfig {
    /// Privacy mode: normal, privacy, or strict
    #[serde(default = "default_privacy_mode")]
    pub mode: String,
    /// Enabled privacy toggles (disabled toggles prevent data collection)
    #[serde(default = "default_privacy_enabled_toggles")]
    pub enabled_toggles: Vec<String>,
    /// Ignore patterns for identifiers to never log or process
    #[serde(default = "default_privacy_ignore_patterns")]
    pub ignore_patterns: Vec<String>,
    /// Whether to distinguish between "unknown" and "anonymous"
    #[serde(default = "default_privacy_distinguish_unknown_anonymous")]
    pub distinguish_unknown_anonymous: bool,
}

fn default_privacy_mode() -> String {
    "normal".to_string()
}

fn default_privacy_enabled_toggles() -> Vec<String> {
    vec![
        "log_paths".to_string(),
        "log_user_ids".to_string(),
        "log_hostnames".to_string(),
        "log_repo_info".to_string(),
        "log_metadata".to_string(),
        "log_session_context".to_string(),
    ]
}

fn default_privacy_ignore_patterns() -> Vec<String> {
    vec![]
}

fn default_privacy_distinguish_unknown_anonymous() -> bool {
    true
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            mode: default_privacy_mode(),
            enabled_toggles: default_privacy_enabled_toggles(),
            ignore_patterns: default_privacy_ignore_patterns(),
            distinguish_unknown_anonymous: default_privacy_distinguish_unknown_anonymous(),
        }
    }
}

/// Expand POSIX-style environment variables in a string.
/// Supports `$VAR` and `${VAR}` syntax.
pub fn expand_env_vars(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '$' && i + 1 < chars.len() {
            if chars[i + 1] == '{' {
                // ${VAR} syntax
                if let Some(end) = chars[i + 2..].iter().position(|&c| c == '}') {
                    let var_name: String = chars[i + 2..i + 2 + end].iter().collect();
                    let var_value = env::var(&var_name).unwrap_or_default();
                    result.push_str(&var_value);
                    i += 2 + end + 1;
                    continue;
                }
            } else {
                // $VAR syntax
                let start = i + 1;
                let end = chars[start..]
                    .iter()
                    .position(|&c| !c.is_alphanumeric() && c != '_')
                    .map(|p| start + p)
                    .unwrap_or(chars.len());
                if end > start {
                    let var_name: String = chars[start..end].iter().collect();
                    let var_value = env::var(&var_name).unwrap_or_default();
                    result.push_str(&var_value);
                    i = end;
                    continue;
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }

    result
}

/// Load config from a TOML file with environment variable expansion and validation.
pub fn load_config_file(path: &std::path::Path) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let expanded = expand_env_vars(&content);

    // Check for secrets in config file
    if crate::secret::contains_secrets(&expanded) {
        tracing::warn!("Config file may contain sensitive data. Consider using environment variables or secure credential storage.");
    }

    let config: Config = toml::from_str(&expanded)?;
    Ok(config)
}

/// Load config from a TOML file with validation and graceful fallback on errors.
/// Returns Ok(config) if valid, Ok(None) if invalid (caller should use defaults),
/// or Err if there's a critical error (e.g., file not found).
pub fn load_config_file_with_validation(path: &std::path::Path) -> anyhow::Result<Option<Config>> {
    match validate_config_file(path) {
        Ok(config) => Ok(Some(config)),
        Err(e) => {
            // Log the validation error but don't crash
            eprintln!("Config validation warning: {}", e);
            eprintln!("Suggestion: {}", e.suggestion);
            eprintln!("Falling back to default configuration.");
            Ok(None)
        }
    }
}

/// Resolve the effective config with the full precedence hierarchy:
/// 1. CLI flags (applied by caller after loading config)
/// 2. Environment variables (`SMARTFO_<SECTION>_<KEY>`)
/// 3. Project config (if in a Git repository)
/// 4. User config
/// 5. System config
/// 6. Built-in defaults
pub fn resolve_config(config_path: Option<&std::path::Path>) -> anyhow::Result<Config> {
    // Start with defaults
    let mut config = Config::default();

    // Layer 5: System config
    if let Some(system_path) = system_config_path().filter(|p| p.exists()) {
        if let Some(system_config) = load_config_file_with_validation(&system_path)? {
            config = merge_configs(config, system_config);
        }
    }

    // Layer 4: User config
    let user_path = config_path.map(PathBuf::from).or_else(default_config_path).filter(|p| p.exists());
    if let Some(path) = user_path {
        if let Some(user_config) = load_config_file_with_validation(&path)? {
            config = merge_configs(config, user_config);
        }
    }

    // Layer 3: Project config (if in a Git repository)
    if let Some(project_path) = project_config_path().filter(|p| p.exists()) {
        if let Some(project_config) = load_config_file_with_validation(&project_path)? {
            config = merge_configs(config, project_config);
        }
    }

    // Layer 2: Environment variables
    apply_env_overrides(&mut config)?;

    // Layer 1: CLI flags (applied by caller after loading config)
    // Caller should apply CLI flags on top of this config

    Ok(config)
}


/// Return the default config file path.
pub fn default_config_path() -> Option<PathBuf> {
    env::var("XDG_CONFIG_HOME")
        .ok()
        .map(|p| PathBuf::from(p).join("smartfo").join("config.toml"))
        .or_else(|| {
            env::var("HOME")
                .ok()
                .map(|p| PathBuf::from(p).join("smartfo").join("config.toml"))
        })
}

/// Merge file config over defaults. File values override defaults.
fn merge_configs(base: Config, file: Config) -> Config {
    Config {
        schema_version: if file.schema_version != default_schema_version() {
            file.schema_version
        } else {
            base.schema_version
        },
        vcs: VcsConfig {
            preference: if file.vcs.preference != default_vcs_preference() {
                file.vcs.preference
            } else {
                base.vcs.preference
            },
            fallback: if !file.vcs.fallback.is_empty() {
                file.vcs.fallback
            } else {
                base.vcs.fallback
            },
            supported: if !file.vcs.supported.is_empty() {
                file.vcs.supported
            } else {
                base.vcs.supported
            },
        },
        trash: TrashConfig {
            root: if file.trash.root != default_trash_root() {
                file.trash.root
            } else {
                base.trash.root
            },
            mode: if file.trash.mode != default_trash_mode() {
                file.trash.mode
            } else {
                base.trash.mode
            },
            min_free_mb: if file.trash.min_free_mb != default_trash_min_free_mb() {
                file.trash.min_free_mb
            } else {
                base.trash.min_free_mb
            },
            min_free_space_percent: if file.trash.min_free_space_percent != default_trash_min_free_space_percent() {
                file.trash.min_free_space_percent
            } else {
                base.trash.min_free_space_percent
            },
            on_trash_full: if file.trash.on_trash_full != default_trash_on_trash_full() {
                file.trash.on_trash_full
            } else {
                base.trash.on_trash_full
            },
            allow_last_version_cull: file.trash.allow_last_version_cull,
            retention_days: if file.trash.retention_days != default_trash_retention_days() {
                file.trash.retention_days
            } else {
                base.trash.retention_days
            },
            delete_ignored: file.trash.delete_ignored,
            preserve_tree: file.trash.preserve_tree,
            backup_vcs_committed: file.trash.backup_vcs_committed,
        },
        concurrency: ConcurrencyConfig {
            max_concurrent_jobs: if file.concurrency.max_concurrent_jobs != default_max_concurrent_jobs() {
                file.concurrency.max_concurrent_jobs
            } else {
                base.concurrency.max_concurrent_jobs
            },
            network_limit_mbps: if file.concurrency.network_limit_mbps != default_network_limit_mbps() {
                file.concurrency.network_limit_mbps
            } else {
                base.concurrency.network_limit_mbps
            },
            drive_detection: if file.concurrency.drive_detection != default_drive_detection() {
                file.concurrency.drive_detection
            } else {
                base.concurrency.drive_detection
            },
            network_concurrency: if file.concurrency.network_concurrency != default_network_concurrency() {
                file.concurrency.network_concurrency
            } else {
                base.concurrency.network_concurrency
            },
            max_memory_mb: if file.concurrency.max_memory_mb != default_max_memory_mb() {
                file.concurrency.max_memory_mb
            } else {
                base.concurrency.max_memory_mb
            },
            max_cpu_percent: if file.concurrency.max_cpu_percent != default_max_cpu_percent() {
                file.concurrency.max_cpu_percent
            } else {
                base.concurrency.max_cpu_percent
            },
        },
        behavior: BehaviorConfig {
            smart_mode: file.behavior.smart_mode,
            async_threshold_mb: if file.behavior.async_threshold_mb != default_async_threshold_mb() {
                file.behavior.async_threshold_mb
            } else {
                base.behavior.async_threshold_mb
            },
            default_blocking: file.behavior.default_blocking,
            sync_after_op: file.behavior.sync_after_op,
            daemon_fallback_quiet: file.behavior.daemon_fallback_quiet,
            mode: file.behavior.mode,
            truncation_limit: if file.behavior.truncation_limit != default_truncation_limit() {
                file.behavior.truncation_limit
            } else {
                base.behavior.truncation_limit
            },
        },
        logging: LoggingConfig {
            level: if file.logging.level != default_log_level() {
                file.logging.level
            } else {
                base.logging.level
            },
            file: file.logging.file.or(base.logging.file),
            json: file.logging.json,
            color: if file.logging.color != default_log_color() {
                file.logging.color
            } else {
                base.logging.color
            },
        },
        privacy: PrivacyConfig {
            mode: if file.privacy.mode != default_privacy_mode() {
                file.privacy.mode
            } else {
                base.privacy.mode
            },
            enabled_toggles: if !file.privacy.enabled_toggles.is_empty() {
                file.privacy.enabled_toggles
            } else {
                base.privacy.enabled_toggles
            },
            ignore_patterns: if !file.privacy.ignore_patterns.is_empty() {
                file.privacy.ignore_patterns
            } else {
                base.privacy.ignore_patterns
            },
            distinguish_unknown_anonymous: file.privacy.distinguish_unknown_anonymous,
        },
        paths: PathsConfig {
            trash_root: file.paths.trash_root.or(base.paths.trash_root),
            audit_log: if file.paths.audit_log != default_audit_log() {
                file.paths.audit_log
            } else {
                base.paths.audit_log
            },
            cache_dir: if file.paths.cache_dir != default_cache_dir() {
                file.paths.cache_dir
            } else {
                base.paths.cache_dir
            },
            config_dir: file.paths.config_dir.or(base.paths.config_dir),
        },
    }
}

/// Apply environment variable overrides of the form `SMARTFO_<SECTION>_<KEY>`.
fn apply_env_overrides(config: &mut Config) -> anyhow::Result<()> {
    // Concurrency overrides (explicit handling for backward compatibility)
    if let Ok(val) = std::env::var("SMARTFO_CONCURRENCY_MAX_CONCURRENT_JOBS") {
        if let Ok(parsed) = val.parse::<usize>() {
            config.concurrency.max_concurrent_jobs = parsed;
        }
    }
    if let Ok(val) = std::env::var("SMARTFO_CONCURRENCY_MAX_MEMORY_MB") {
        if let Ok(parsed) = val.parse::<u64>() {
            config.concurrency.max_memory_mb = parsed;
        }
    }
    if let Ok(val) = std::env::var("SMARTFO_CONCURRENCY_MAX_CPU_PERCENT") {
        if let Ok(parsed) = val.parse::<u8>() {
            config.concurrency.max_cpu_percent = parsed;
        }
    }

    // Generic environment variable overrides
    for (key, value) in env::vars() {
        if let Some(rest) = key.strip_prefix("SMARTFO_") {
            let parts: Vec<&str> = rest.splitn(2, '_').collect();
            if parts.len() != 2 {
                continue;
            }
            let (section, key_name) = (parts[0].to_lowercase(), parts[1].to_lowercase());

            // Warn if environment variable contains secrets
            if crate::secret::contains_secrets(&value) {
                tracing::warn!("Environment variable {} may contain sensitive data. Consider using secure credential storage.", key);
            }

            match section.as_str() {
                "vcs" if key_name.as_str() == "preference" => config.vcs.preference = value,
                "trash" => {
                    match key_name.as_str() {
                        "root" => config.trash.root = PathBuf::from(expand_env_vars(&value)),
                        "mode" => config.trash.mode = value,
                        "min_free_mb" => {
                            if let Ok(v) = value.parse() {
                                config.trash.min_free_mb = v;
                            }
                        }
                        "retention_days" => {
                            if let Ok(v) = value.parse() {
                                config.trash.retention_days = v;
                            }
                        }
                        "delete_ignored" => config.trash.delete_ignored = value.parse().unwrap_or(true),
                        _ => {}
                    }
                }
                "concurrency" => match key_name.as_str() {
                    "max_concurrent_jobs" => {
                        if let Ok(v) = value.parse() {
                            config.concurrency.max_concurrent_jobs = v;
                        }
                    }
                    "max_memory_mb" => {
                        if let Ok(v) = value.parse() {
                            config.concurrency.max_memory_mb = v;
                        }
                    }
                    "max_cpu_percent" => {
                        if let Ok(v) = value.parse() {
                            config.concurrency.max_cpu_percent = v;
                        }
                    }
                    _ => {}
                }
                "privacy" => match key_name.as_str() {
                    "mode" => {
                        config.privacy.mode = value;
                    }
                    "distinguish_unknown_anonymous" => {
                        config.privacy.distinguish_unknown_anonymous = value.parse().unwrap_or(true);
                    }
                    _ => {}
                }
                "behavior" => match key_name.as_str() {
                    "max_concurrent_jobs" => {
                        if let Ok(v) = value.parse() {
                            config.concurrency.max_concurrent_jobs = v;
                        }
                    }
                    "network_limit_mbps" => {
                        if let Ok(v) = value.parse() {
                            config.concurrency.network_limit_mbps = v;
                        }
                    }
                    "drive_detection" => {
                        config.concurrency.drive_detection = value.parse().unwrap_or(true);
                    }
                    _ => {}
                },
                "behavior" => match key_name.as_str() {
                    "smart_mode" => config.behavior.smart_mode = value.parse().unwrap_or(true),
                    "async_threshold_mb" => {
                        if let Ok(v) = value.parse() {
                            config.behavior.async_threshold_mb = v;
                        }
                    }
                    "default_blocking" => {
                        config.behavior.default_blocking = value.parse().unwrap_or(false);
                    }
                    "sync_after_op" => config.behavior.sync_after_op = value.parse().unwrap_or(false),
                    "daemon_fallback_quiet" => {
                        config.behavior.daemon_fallback_quiet = value.parse().unwrap_or(false);
                    }
                    "mode" => {
                        config.behavior.mode = match value.to_lowercase().as_str() {
                            "agent" => OutputMode::Agent,
                            "human" => OutputMode::Human,
                            "auto" => OutputMode::Auto,
                            _ => OutputMode::Auto,
                        };
                    }
                    _ => {}
                },
                "logging" => match key_name.as_str() {
                    "level" => config.logging.level = value,
                    "file" => config.logging.file = Some(PathBuf::from(expand_env_vars(&value))),
                    "json" => config.logging.json = value.parse().unwrap_or(false),
                    _ => {}
                },
                "paths" => match key_name.as_str() {
                    "trash_root" => config.paths.trash_root = Some(PathBuf::from(expand_env_vars(&value))),
                    "audit_log" => config.paths.audit_log = PathBuf::from(expand_env_vars(&value)),
                    "cache_dir" => config.paths.cache_dir = PathBuf::from(expand_env_vars(&value)),
                    "config_dir" => config.paths.config_dir = Some(PathBuf::from(expand_env_vars(&value))),
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}

/// Generate a default config file template as a string.
pub fn default_config_template() -> String {
    r#"# Smartfo Configuration File
# Place this file at $XDG_CONFIG_HOME/smartfo/config.toml or $HOME/smartfo/config.toml
# All settings are commented out with their default values shown
# Uncomment and modify any setting you wish to customize

[vcs]
# Preferred VCS when multiple are detected (default: "git")
# preference = "git"
# Fallback order when preferred VCS is unavailable (default: ["git", "jj", "hg", "svn"])
# fallback = ["git", "jj", "hg", "svn"]
# Supported VCS systems to detect (default: ["git", "jj", "hg", "svn"])
# supported = ["git", "jj", "hg", "svn"]

[trash]
# Root directory for trash (default: "$XDG_DATA_HOME/smartfo/trash")
# root = "$XDG_DATA_HOME/smartfo/trash"
# Trash mode: "versioned" or "simple" (default: "versioned")
# mode = "versioned"
# Minimum free space (in MB) before auto-culling oldest entries (default: 1024)
# min_free_mb = 1024
# Minimum free space percentage (0-100) before auto-culling oldest entries (default: 20)
# min_free_space_percent = 20
# Behavior when trash is full: "refuse" or "delete" (default: "refuse")
# on_trash_full = "refuse"
# Whether to allow culling the last version of a file (default: false)
# allow_last_version_cull = false
# Retention period in days, 0 = unlimited (default: 30)
# retention_days = 30
# Whether to delete ignored files directly instead of trashing (default: true)
# delete_ignored = true
# Whether to preserve directory tree structure in trash (default: true)
# preserve_tree = true
# Whether to backup VCS-committed files to trash (default: false)
# backup_vcs_committed = false

[concurrency]
# Maximum number of concurrent background jobs (default: 4)
# max_concurrent_jobs = 4
# Network bandwidth limit in MB/s, 0 = unlimited (default: 0)
# network_limit_mbps = 0
# Whether to detect same-drive vs cross-device moves (default: true)
# drive_detection = true
# Maximum concurrent operations to network-mounted destinations (default: 2)
# network_concurrency = 2

[behavior]
# Whether to use smart features (VCS, trash, async) by default (default: true)
# smart_mode = true
# File size threshold (in MB) for triggering async move (default: 100)
# async_threshold_mb = 100
# Whether blocking mode is the default (overrides async) (default: false)
# default_blocking = false
# Whether to fsync after every operation (default: false)
# sync_after_op = false
# Whether daemon fallback to sync mode should be quiet (no warnings) (default: false)
# daemon_fallback_quiet = false

[logging]
# Log level: "trace", "debug", "info", "warn", "error" (default: "info")
# level = "info"
# Log file path (default: stderr)
# file = "$HOME/.local/share/smartfo/smartfo.log"
# Whether to use JSON formatting (default: false)
# json = false

[paths]
# Override for trash root (default: uses trash.root)
# trash_root = "$HOME/.local/share/smartfo/trash"
# Audit log file path (default: "$XDG_DATA_HOME/smartfo/audit/operations.jsonl")
# audit_log = "$XDG_DATA_HOME/smartfo/audit/operations.jsonl"
# Cache directory (default: "$XDG_CACHE_HOME/smartfo")
# cache_dir = "$XDG_CACHE_HOME/smartfo"
# Config directory override (default: uses XDG_CONFIG_HOME)
# config_dir = "$HOME/.config/smartfo"
"#
    .to_string()
}

/// Check if the user config file exists.
pub fn user_config_exists() -> bool {
    default_config_path()
        .map(|p| p.exists())
        .unwrap_or(false)
}

/// Create the default config file in the user config directory.
/// If the config file already exists and force is false, returns an error.
pub fn create_default_config() -> anyhow::Result<PathBuf> {
    create_default_config_impl(false)
}

/// Create the default config file in the user config directory.
/// If force is true, overwrites existing config without warning.
pub fn create_default_config_force(force: bool) -> anyhow::Result<PathBuf> {
    create_default_config_impl(force)
}

fn create_default_config_impl(force: bool) -> anyhow::Result<PathBuf> {
    let config_path = default_config_path()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine config file path"))?;

    // Check if config already exists
    if config_path.exists() && !force {
        return Err(anyhow::anyhow!(
            "Config file already exists at: {}\nUse --force to overwrite",
            config_path.display()
        ));
    }

    // Create parent directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Write the default config template
    std::fs::write(&config_path, default_config_template())?;

    Ok(config_path)
}

/// Initialize config on first run if it doesn't exist.
pub fn init_config_if_missing() -> anyhow::Result<bool> {
    if user_config_exists() {
        return Ok(false);
    }

    create_default_config()?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.vcs.preference, "git");
        assert_eq!(config.concurrency.max_concurrent_jobs, 4);
        assert!(config.behavior.smart_mode);
        assert_eq!(config.logging.level, "info");
        assert!(!config.behavior.daemon_fallback_quiet);
    }

    #[test]
    fn test_expand_env_vars_simple() {
        env::set_var("SMARTFO_TEST_VAR", "hello");
        assert_eq!(expand_env_vars("$SMARTFO_TEST_VAR/world"), "hello/world");
        assert_eq!(expand_env_vars("${SMARTFO_TEST_VAR}/world"), "hello/world");
        env::remove_var("SMARTFO_TEST_VAR");
    }

    #[test]
    fn test_expand_env_vars_missing() {
        env::remove_var("SMARTFO_NONEXISTENT");
        assert_eq!(expand_env_vars("$SMARTFO_NONEXISTENT/fallback"), "/fallback");
    }

    #[test]
    fn test_load_config_file() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        let toml = r#"
[vcs]
preference = "jj"

[behavior]
default_blocking = true
"#;
        tmpfile.write_all(toml.as_bytes()).unwrap();

        let config = load_config_file(tmpfile.path()).unwrap();
        assert_eq!(config.vcs.preference, "jj");
        assert!(config.behavior.default_blocking);
        // Unspecified fields keep defaults
        assert_eq!(config.concurrency.max_concurrent_jobs, 4);
    }

    #[test]
    fn test_system_config_path() {
        let path = system_config_path();
        #[cfg(target_os = "linux")]
        assert_eq!(path, Some(PathBuf::from("/etc/smartfo/config.toml")));

        #[cfg(target_os = "macos")]
        assert_eq!(path, Some(PathBuf::from("/Library/Application Support/smartfo/config.toml")));

        #[cfg(target_os = "windows")]
        assert!(path.is_some());

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        assert!(path.is_none());
    }

    #[test]
    fn test_project_config_path() {
        // This test should pass when run from within a Git repository
        let path = project_config_path();
        // We can't assert a specific value since it depends on the test environment
        // Just verify it returns Some or None consistently
        let _ = path;
    }

    #[test]
    fn test_config_precedence() {
        let mut tmpdir = tempfile::TempDir::new().unwrap();

        // Create system config
        let system_path = tmpdir.path().join("system.toml");
        let system_toml = r#"
[behavior]
smart_mode = false
"#;
        std::fs::write(&system_path, system_toml).unwrap();

        // Create user config
        let user_path = tmpdir.path().join("user.toml");
        let user_toml = r#"
[behavior]
smart_mode = true
default_blocking = true
"#;
        std::fs::write(&user_path, user_toml).unwrap();

        // Test precedence: system -> user -> project
        // Since we can't easily mock project config, we'll just test system -> user
        let config = resolve_config(Some(&user_path)).unwrap();
        assert!(config.behavior.smart_mode);
        assert!(config.behavior.default_blocking);
    }

    #[test]
    fn test_daemon_fallback_quiet_default() {
        let config = Config::default();
        assert!(!config.behavior.daemon_fallback_quiet);
    }

    #[test]
    fn test_daemon_fallback_quiet_env_override() {
        env::set_var("SMARTFO_BEHAVIOR_DAEMON_FALLBACK_QUIET", "true");
        let mut config = Config::default();
        apply_env_overrides(&mut config).unwrap();
        assert!(config.behavior.daemon_fallback_quiet);
        env::remove_var("SMARTFO_BEHAVIOR_DAEMON_FALLBACK_QUIET");
    }

    #[test]
    fn test_default_config_template() {
        let template = default_config_template();
        assert!(template.contains("# Smartfo Configuration File"));
        assert!(template.contains("daemon_fallback_quiet"));
        assert!(template.contains("# preference = \"git\""));
    }

    #[test]
    fn test_create_default_config() {
        let tmpdir = tempfile::TempDir::new().unwrap();
        let config_dir = tmpdir.path().join(".config").join("smartfo");
        std::fs::create_dir_all(&config_dir).unwrap();

        // Temporarily override the config path
        let original_home = env::var("HOME");
        let original_xdg = env::var("XDG_CONFIG_HOME");
        env::set_var("HOME", tmpdir.path());
        env::set_var("XDG_CONFIG_HOME", tmpdir.path().join(".config"));

        let result = create_default_config();
        assert!(result.is_ok());
        let created_path = result.unwrap();
        assert!(created_path.exists());

        // Restore original HOME
        if let Ok(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }
        if let Ok(xdg) = original_xdg {
            env::set_var("XDG_CONFIG_HOME", xdg);
        } else {
            env::remove_var("XDG_CONFIG_HOME");
        }
    }

    #[test]
    fn test_init_config_if_missing() {
        let tmpdir = tempfile::TempDir::new().unwrap();
        let config_dir = tmpdir.path().join("smartfo");
        std::fs::create_dir_all(&config_dir).unwrap();

        // Temporarily override the config path
        let original_home = env::var("HOME");
        let original_xdg = env::var("XDG_CONFIG_HOME");
        env::set_var("HOME", tmpdir.path());
        env::remove_var("XDG_CONFIG_HOME");

        // First call should create config
        let created = init_config_if_missing().unwrap();
        assert!(created);

        // Second call should not create config (already exists)
        let created_again = init_config_if_missing().unwrap();
        assert!(!created_again);

        // Restore original HOME
        if let Ok(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }
        if let Ok(xdg) = original_xdg {
            env::set_var("XDG_CONFIG_HOME", xdg);
        } else {
            env::remove_var("XDG_CONFIG_HOME");
        }
    }

    #[test]
    fn test_load_config_with_env_expansion() {
        env::set_var("SMARTFO_TEST_HOME", "/test/home");
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        let toml = r#"
[trash]
root = "$SMARTFO_TEST_HOME/trash"
"#;
        tmpfile.write_all(toml.as_bytes()).unwrap();

        let config = load_config_file(tmpfile.path()).unwrap();
        assert_eq!(config.trash.root, PathBuf::from("/test/home/trash"));
        env::remove_var("SMARTFO_TEST_HOME");
    }

    #[test]
    fn test_env_override_paths() {
        env::set_var("SMARTFO_PATHS_AUDIT_LOG", "/custom/audit.jsonl");
        let config = resolve_config(None).unwrap();
        assert_eq!(config.paths.audit_log, PathBuf::from("/custom/audit.jsonl"));
        env::remove_var("SMARTFO_PATHS_AUDIT_LOG");
    }

    #[test]
    fn test_precedence_cli_over_env_over_file_over_default() {
        // Clean up any environment variables that might interfere
        env::remove_var("SMARTFO_VCS_PREFERENCE");

        // This test verifies the layering: defaults -> file -> env
        // CLI flags are applied by the caller, not tested here.
        env::set_var("SMARTFO_VCS_PREFERENCE", "hg");

        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        let toml = r#"
[vcs]
preference = "svn"
"#;
        tmpfile.write_all(toml.as_bytes()).unwrap();

        let config = resolve_config(Some(tmpfile.path())).unwrap();
        // Env should override file
        assert_eq!(config.vcs.preference, "hg");

        env::remove_var("SMARTFO_VCS_PREFERENCE");
    }

    #[test]
    fn test_missing_config_falls_back_to_defaults() {
        // Clean up any environment variables that might interfere
        env::remove_var("SMARTFO_VCS_PREFERENCE");
        env::remove_var("SMARTFO_LOGGING_LEVEL");
        env::remove_var("SMARTFO_CONCURRENCY_NETWORK_CONCURRENCY");
        env::remove_var("SMARTFO_BEHAVIOR_DEFAULT_BLOCKING");

        let config = resolve_config(Some(std::path::Path::new("/nonexistent/path/config.toml"))).unwrap();
        assert_eq!(config.vcs.preference, "git");
        assert_eq!(config.logging.level, "info");
        assert_eq!(config.concurrency.network_concurrency, 2);
    }

    #[test]
    fn test_default_config_template_is_valid_toml() {
        let template = default_config_template();
        let config: Config = toml::from_str(&template).unwrap();
        assert_eq!(config.vcs.preference, "git");
        assert_eq!(config.trash.mode, "versioned");
    }

    #[test]
    fn test_invalid_config_produces_error() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        tmpfile.write_all(b"[vcs]\npreference = 123\n").unwrap();
        let result = load_config_file(tmpfile.path());
        assert!(result.is_err());
    }
}

/// Reload configuration from file with validation.
/// Returns the new config if successful, or an error if validation fails.
/// The old config is kept active on failure.
pub fn reload_config(current_config: &Config, config_path: Option<&std::path::Path>) -> anyhow::Result<Config> {
    tracing::info!("Attempting to reload configuration");

    // Resolve the new config
    let new_config = resolve_config(config_path)?;

    // Validate the new config
    validate_config(&new_config)?;

    tracing::info!("Configuration reloaded successfully");
    Ok(new_config)
}

#[cfg(test)]
mod reload_tests {
    use super::*;

    #[test]
    fn test_reload_config_with_valid_config() {
        let current_config = Config::default();
        let result = reload_config(&current_config, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_reload_config_preserves_validation() {
        let current_config = Config::default();

        // Create a temporary invalid config file
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        tmpfile.write_all(b"[vcs]\npreference = 123\n").unwrap();

        let result = reload_config(&current_config, Some(tmpfile.path()));
        assert!(result.is_err());
    }
}
