use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

/// Top-level smartfo configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
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
    pub paths: PathsConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vcs: VcsConfig::default(),
            trash: TrashConfig::default(),
            concurrency: ConcurrencyConfig::default(),
            behavior: BehaviorConfig::default(),
            logging: LoggingConfig::default(),
            paths: PathsConfig::default(),
        }
    }
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

impl Default for ConcurrencyConfig {
    fn default() -> Self {
        Self {
            max_concurrent_jobs: default_max_concurrent_jobs(),
            network_limit_mbps: default_network_limit_mbps(),
            drive_detection: default_drive_detection(),
            network_concurrency: default_network_concurrency(),
        }
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

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            smart_mode: default_smart_mode(),
            async_threshold_mb: default_async_threshold_mb(),
            default_blocking: default_default_blocking(),
            sync_after_op: default_sync_after_op(),
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
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_json() -> bool {
    false
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            file: None,
            json: default_log_json(),
        }
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

/// Load config from a TOML file with environment variable expansion.
pub fn load_config_file(path: &std::path::Path) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let expanded = expand_env_vars(&content);
    let config: Config = toml::from_str(&expanded)?;
    Ok(config)
}

/// Resolve the effective config with the full precedence hierarchy:
/// 1. CLI flags (applied by caller after loading config)
/// 2. Environment variables (`SMARTFO_<SECTION>_<KEY>`)
/// 3. Config file
/// 4. Built-in defaults
pub fn resolve_config(config_path: Option<&std::path::Path>) -> anyhow::Result<Config> {
    // Start with defaults
    let mut config = Config::default();

    // Layer 3: Config file
    let file_path = config_path.map(PathBuf::from).or_else(|| default_config_path()).filter(|p| p.exists());
    if let Some(path) = file_path {
        let file_config = load_config_file(&path)?;
        config = merge_configs(config, file_config);
    }

    // Layer 2: Environment variables
    apply_env_overrides(&mut config)?;

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
        },
        logging: LoggingConfig {
            level: if file.logging.level != default_log_level() {
                file.logging.level
            } else {
                base.logging.level
            },
            file: file.logging.file.or(base.logging.file),
            json: file.logging.json,
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
    for (key, value) in env::vars() {
        if let Some(rest) = key.strip_prefix("SMARTFO_") {
            let parts: Vec<&str> = rest.splitn(2, '_').collect();
            if parts.len() != 2 {
                continue;
            }
            let (section, key_name) = (parts[0].to_lowercase(), parts[1].to_lowercase());

            match section.as_str() {
                "vcs" => match key_name.as_str() {
                    "preference" => config.vcs.preference = value,
                    _ => {}
                },
                "trash" => match key_name.as_str() {
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
                },
                "concurrency" => match key_name.as_str() {
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

[vcs]
preference = "git"
fallback = ["git", "jj", "hg", "svn"]
supported = ["git", "jj", "hg", "svn"]

[trash]
root = "$XDG_DATA_HOME/smartfo/trash"
mode = "versioned"
min_free_mb = 1024
min_free_space_percent = 20
on_trash_full = "refuse"
allow_last_version_cull = false
retention_days = 30
delete_ignored = true
preserve_tree = true
backup_vcs_committed = false

[concurrency]
max_concurrent_jobs = 4
network_limit_mbps = 0
drive_detection = true
network_concurrency = 2

[behavior]
smart_mode = true
async_threshold_mb = 100
default_blocking = false
sync_after_op = false

[logging]
level = "info"
# file = "$HOME/.local/share/smartfo/smartfo.log"
json = false

[paths]
# trash_root = "$HOME/.local/share/smartfo/trash"
audit_log = "$XDG_DATA_HOME/smartfo/audit/operations.jsonl"
cache_dir = "$XDG_CACHE_HOME/smartfo"
# config_dir = "$HOME/.config/smartfo"
"#
    .to_string()
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
    fn test_env_override() {
        env::set_var("SMARTFO_BEHAVIOR_DEFAULT_BLOCKING", "true");
        env::set_var("SMARTFO_CONCURRENCY_MAX_CONCURRENT_JOBS", "8");

        let config = resolve_config(None).unwrap();
        assert!(config.behavior.default_blocking);
        assert_eq!(config.concurrency.max_concurrent_jobs, 8);

        env::remove_var("SMARTFO_BEHAVIOR_DEFAULT_BLOCKING");
        env::remove_var("SMARTFO_CONCURRENCY_MAX_CONCURRENT_JOBS");
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
