//! Privacy mode with explicit ignore lists for identifiers
//!
//! This module provides privacy mode functionality to control what data is logged,
//! processed, or displayed. It distinguishes between "unknown" (logged but not assigned)
//! and "anonymous" (ignored entirely) identifiers, and provides configurable privacy
//! toggles to disable specific data collection.

use regex::Regex;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

/// Privacy mode level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrivacyMode {
    /// Normal mode - all data is logged and processed
    Normal,
    /// Privacy mode - sensitive data is anonymized or ignored
    Privacy,
    /// Strict mode - maximum privacy, minimal data collection
    Strict,
}

impl Default for PrivacyMode {
    fn default() -> Self {
        PrivacyMode::Normal
    }
}

/// Privacy toggle for specific data collection categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrivacyToggle {
    /// Log file paths
    LogPaths,
    /// Log user identifiers (usernames, UIDs)
    LogUserIds,
    /// Log hostnames
    LogHostnames,
    /// Log repository information
    LogRepoInfo,
    /// Log operation metadata (timestamps, UUIDs)
    LogMetadata,
    /// Log session context
    LogSessionContext,
}

impl PrivacyToggle {
    /// Get all privacy toggles
    pub fn all() -> Vec<PrivacyToggle> {
        vec![
            PrivacyToggle::LogPaths,
            PrivacyToggle::LogUserIds,
            PrivacyToggle::LogHostnames,
            PrivacyToggle::LogRepoInfo,
            PrivacyToggle::LogMetadata,
            PrivacyToggle::LogSessionContext,
        ]
    }

    /// Get the config key for this toggle
    pub fn config_key(&self) -> &'static str {
        match self {
            PrivacyToggle::LogPaths => "log_paths",
            PrivacyToggle::LogUserIds => "log_user_ids",
            PrivacyToggle::LogHostnames => "log_hostnames",
            PrivacyToggle::LogRepoInfo => "log_repo_info",
            PrivacyToggle::LogMetadata => "log_metadata",
            PrivacyToggle::LogSessionContext => "log_session_context",
        }
    }
}

/// Privacy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    /// Current privacy mode
    pub mode: PrivacyMode,
    /// Enabled privacy toggles (disabled toggles prevent data collection)
    pub enabled_toggles: Vec<PrivacyToggle>,
    /// Ignore patterns for identifiers to never log or process
    pub ignore_patterns: Vec<String>,
    /// Whether to distinguish between "unknown" and "anonymous"
    pub distinguish_unknown_anonymous: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        PrivacyConfig {
            mode: PrivacyMode::Normal,
            enabled_toggles: PrivacyToggle::all(),
            ignore_patterns: vec![],
            distinguish_unknown_anonymous: true,
        }
    }
}

/// Identifier treatment in privacy mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentifierTreatment {
    /// Log normally
    Normal,
    /// Log as "unknown" (logged but not assigned to specific entity)
    Unknown,
    /// Anonymous (ignored entirely, not logged)
    Anonymous,
}

/// Privacy manager for handling privacy mode operations
#[derive(Debug, Clone)]
pub struct PrivacyManager {
    config: PrivacyConfig,
    /// Compiled regex patterns from ignore list
    ignore_regexes: Vec<Regex>,
}

impl PrivacyManager {
    /// Create a new privacy manager with the given config
    pub fn new(config: PrivacyConfig) -> Result<Self, regex::Error> {
        let ignore_regexes = Self::compile_ignore_patterns(&config.ignore_patterns)?;
        Ok(PrivacyManager {
            config,
            ignore_regexes,
        })
    }

    /// Create a privacy manager with default config
    pub fn default() -> Self {
        Self::new(PrivacyConfig::default()).unwrap()
    }

    /// Compile ignore patterns into regex
    fn compile_ignore_patterns(patterns: &[String]) -> Result<Vec<Regex>, regex::Error> {
        patterns
            .iter()
            .map(|p| Regex::new(p))
            .collect()
    }

    /// Check if a privacy toggle is enabled
    pub fn is_toggle_enabled(&self, toggle: PrivacyToggle) -> bool {
        self.config.enabled_toggles.contains(&toggle)
    }

    /// Check if privacy mode is active
    pub fn is_privacy_mode(&self) -> bool {
        self.config.mode != PrivacyMode::Normal
    }

    /// Get the current privacy mode
    pub fn mode(&self) -> PrivacyMode {
        self.config.mode
    }

    /// Determine how to treat an identifier based on privacy settings
    pub fn treat_identifier(&self, identifier: &str) -> IdentifierTreatment {
        // Check if identifier matches any ignore pattern
        for regex in &self.ignore_regexes {
            if regex.is_match(identifier) {
                return IdentifierTreatment::Anonymous;
            }
        }

        // If not in ignore list, apply mode-based treatment
        match self.config.mode {
            PrivacyMode::Normal => IdentifierTreatment::Normal,
            PrivacyMode::Privacy => {
                if self.config.distinguish_unknown_anonymous {
                    IdentifierTreatment::Unknown
                } else {
                    IdentifierTreatment::Anonymous
                }
            }
            PrivacyMode::Strict => IdentifierTreatment::Anonymous,
        }
    }

    /// Sanitize a string based on privacy settings
    ///
    /// Returns "unknown" for unknown identifiers, "anonymous" for anonymous,
    /// or the original string for normal treatment.
    pub fn sanitize_identifier(&self, identifier: &str) -> String {
        match self.treat_identifier(identifier) {
            IdentifierTreatment::Normal => identifier.to_string(),
            IdentifierTreatment::Unknown => "unknown".to_string(),
            IdentifierTreatment::Anonymous => "anonymous".to_string(),
        }
    }

    /// Sanitize a file path based on privacy settings
    pub fn sanitize_path(&self, path: &str) -> String {
        if !self.is_toggle_enabled(PrivacyToggle::LogPaths) {
            return "<path redacted>".to_string();
        }

        // Apply identifier sanitization to path components
        let parts: Vec<&str> = path.split('/').collect();
        let sanitized: Vec<String> = parts
            .iter()
            .map(|part| self.sanitize_identifier(part))
            .collect();

        sanitized.join("/")
    }

    /// Sanitize a hostname based on privacy settings
    pub fn sanitize_hostname(&self, hostname: &str) -> String {
        if !self.is_toggle_enabled(PrivacyToggle::LogHostnames) {
            return "<hostname redacted>".to_string();
        }

        self.sanitize_identifier(hostname)
    }

    /// Sanitize a user ID based on privacy settings
    pub fn sanitize_user_id(&self, user_id: &str) -> String {
        if !self.is_toggle_enabled(PrivacyToggle::LogUserIds) {
            return "<user redacted>".to_string();
        }

        self.sanitize_identifier(user_id)
    }

    /// Sanitize repository information based on privacy settings
    pub fn sanitize_repo_info(&self, repo_info: &str) -> String {
        if !self.is_toggle_enabled(PrivacyToggle::LogRepoInfo) {
            return "<repo redacted>".to_string();
        }

        self.sanitize_identifier(repo_info)
    }

    /// Check if metadata should be logged
    pub fn should_log_metadata(&self) -> bool {
        self.is_toggle_enabled(PrivacyToggle::LogMetadata)
    }

    /// Check if session context should be logged
    pub fn should_log_session_context(&self) -> bool {
        self.is_toggle_enabled(PrivacyToggle::LogSessionContext)
    }

    /// Add an ignore pattern
    pub fn add_ignore_pattern(&mut self, pattern: String) -> Result<(), regex::Error> {
        let regex = Regex::new(&pattern)?;
        self.ignore_regexes.push(regex);
        self.config.ignore_patterns.push(pattern);
        Ok(())
    }

    /// Update the privacy config
    pub fn update_config(&mut self, config: PrivacyConfig) -> Result<(), regex::Error> {
        let ignore_regexes = Self::compile_ignore_patterns(&config.ignore_patterns)?;
        self.ignore_regexes = ignore_regexes;
        self.config = config;
        Ok(())
    }

    /// Get the current config
    pub fn config(&self) -> &PrivacyConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_mode_default() {
        let config = PrivacyConfig::default();
        assert_eq!(config.mode, PrivacyMode::Normal);
        assert!(config.distinguish_unknown_anonymous);
        assert_eq!(config.ignore_patterns.len(), 0);
    }

    #[test]
    fn test_privacy_manager_default() {
        let manager = PrivacyManager::default();
        assert_eq!(manager.mode(), PrivacyMode::Normal);
        assert!(!manager.is_privacy_mode());
    }

    #[test]
    fn test_privacy_mode_active() {
        let config = PrivacyConfig {
            mode: PrivacyMode::Privacy,
            ..Default::default()
        };
        let manager = PrivacyManager::new(config).unwrap();
        assert!(manager.is_privacy_mode());
        assert_eq!(manager.mode(), PrivacyMode::Privacy);
    }

    #[test]
    fn test_treat_identifier_normal_mode() {
        let manager = PrivacyManager::default();
        assert_eq!(
            manager.treat_identifier("test-user"),
            IdentifierTreatment::Normal
        );
    }

    #[test]
    fn test_treat_identifier_privacy_mode_unknown() {
        let config = PrivacyConfig {
            mode: PrivacyMode::Privacy,
            distinguish_unknown_anonymous: true,
            ..Default::default()
        };
        let manager = PrivacyManager::new(config).unwrap();
        assert_eq!(
            manager.treat_identifier("test-user"),
            IdentifierTreatment::Unknown
        );
    }

    #[test]
    fn test_treat_identifier_privacy_mode_anonymous() {
        let config = PrivacyConfig {
            mode: PrivacyMode::Privacy,
            distinguish_unknown_anonymous: false,
            ..Default::default()
        };
        let manager = PrivacyManager::new(config).unwrap();
        assert_eq!(
            manager.treat_identifier("test-user"),
            IdentifierTreatment::Anonymous
        );
    }

    #[test]
    fn test_treat_identifier_strict_mode() {
        let config = PrivacyConfig {
            mode: PrivacyMode::Strict,
            ..Default::default()
        };
        let manager = PrivacyManager::new(config).unwrap();
        assert_eq!(
            manager.treat_identifier("test-user"),
            IdentifierTreatment::Anonymous
        );
    }

    #[test]
    fn test_ignore_pattern_matching() {
        let config = PrivacyConfig {
            mode: PrivacyMode::Privacy,
            ignore_patterns: vec!["secret-.*".to_string()],
            ..Default::default()
        };
        let manager = PrivacyManager::new(config).unwrap();
        assert_eq!(
            manager.treat_identifier("secret-key"),
            IdentifierTreatment::Anonymous
        );
        assert_eq!(
            manager.treat_identifier("normal-user"),
            IdentifierTreatment::Unknown
        );
    }

    #[test]
    fn test_sanitize_identifier_normal() {
        let manager = PrivacyManager::default();
        assert_eq!(manager.sanitize_identifier("test-user"), "test-user");
    }

    #[test]
    fn test_sanitize_identifier_unknown() {
        let config = PrivacyConfig {
            mode: PrivacyMode::Privacy,
            distinguish_unknown_anonymous: true,
            ..Default::default()
        };
        let manager = PrivacyManager::new(config).unwrap();
        assert_eq!(manager.sanitize_identifier("test-user"), "unknown");
    }

    #[test]
    fn test_sanitize_identifier_anonymous() {
        let config = PrivacyConfig {
            mode: PrivacyMode::Privacy,
            distinguish_unknown_anonymous: false,
            ..Default::default()
        };
        let manager = PrivacyManager::new(config).unwrap();
        assert_eq!(manager.sanitize_identifier("test-user"), "anonymous");
    }

    #[test]
    fn test_sanitize_path() {
        let manager = PrivacyManager::default();
        assert_eq!(
            manager.sanitize_path("/home/user/document.txt"),
            "/home/user/document.txt"
        );
    }

    #[test]
    fn test_sanitize_path_toggle_disabled() {
        let config = PrivacyConfig {
            enabled_toggles: vec![],
            ..Default::default()
        };
        let manager = PrivacyManager::new(config).unwrap();
        assert_eq!(
            manager.sanitize_path("/home/user/document.txt"),
            "<path redacted>"
        );
    }

    #[test]
    fn test_sanitize_hostname() {
        let manager = PrivacyManager::default();
        assert_eq!(manager.sanitize_hostname("myhost"), "myhost");
    }

    #[test]
    fn test_sanitize_hostname_toggle_disabled() {
        let config = PrivacyConfig {
            enabled_toggles: vec![],
            ..Default::default()
        };
        let manager = PrivacyManager::new(config).unwrap();
        assert_eq!(manager.sanitize_hostname("myhost"), "<hostname redacted>");
    }

    #[test]
    fn test_sanitize_user_id() {
        let manager = PrivacyManager::default();
        assert_eq!(manager.sanitize_user_id("user123"), "user123");
    }

    #[test]
    fn test_sanitize_user_id_toggle_disabled() {
        let config = PrivacyConfig {
            enabled_toggles: vec![],
            ..Default::default()
        };
        let manager = PrivacyManager::new(config).unwrap();
        assert_eq!(manager.sanitize_user_id("user123"), "<user redacted>");
    }

    #[test]
    fn test_is_toggle_enabled() {
        let config = PrivacyConfig {
            enabled_toggles: vec![PrivacyToggle::LogPaths, PrivacyToggle::LogUserIds],
            ..Default::default()
        };
        let manager = PrivacyManager::new(config).unwrap();
        assert!(manager.is_toggle_enabled(PrivacyToggle::LogPaths));
        assert!(manager.is_toggle_enabled(PrivacyToggle::LogUserIds));
        assert!(!manager.is_toggle_enabled(PrivacyToggle::LogHostnames));
    }

    #[test]
    fn test_should_log_metadata() {
        let manager = PrivacyManager::default();
        assert!(manager.should_log_metadata());

        let config = PrivacyConfig {
            enabled_toggles: vec![],
            ..Default::default()
        };
        let manager = PrivacyManager::new(config).unwrap();
        assert!(!manager.should_log_metadata());
    }

    #[test]
    fn test_add_ignore_pattern() {
        let mut manager = PrivacyManager::default();
        manager.add_ignore_pattern("test-.*".to_string()).unwrap();
        assert_eq!(manager.config().ignore_patterns.len(), 1);
        assert_eq!(
            manager.treat_identifier("test-user"),
            IdentifierTreatment::Anonymous
        );
    }

    #[test]
    fn test_update_config() {
        let mut manager = PrivacyManager::default();
        let new_config = PrivacyConfig {
            mode: PrivacyMode::Privacy,
            ..Default::default()
        };
        manager.update_config(new_config).unwrap();
        assert_eq!(manager.mode(), PrivacyMode::Privacy);
    }

    #[test]
    fn test_privacy_toggle_config_keys() {
        assert_eq!(PrivacyToggle::LogPaths.config_key(), "log_paths");
        assert_eq!(PrivacyToggle::LogUserIds.config_key(), "log_user_ids");
        assert_eq!(PrivacyToggle::LogHostnames.config_key(), "log_hostnames");
        assert_eq!(PrivacyToggle::LogRepoInfo.config_key(), "log_repo_info");
        assert_eq!(PrivacyToggle::LogMetadata.config_key(), "log_metadata");
        assert_eq!(
            PrivacyToggle::LogSessionContext.config_key(),
            "log_session_context"
        );
    }

    #[test]
    fn test_privacy_toggle_all() {
        let all = PrivacyToggle::all();
        assert_eq!(all.len(), 6);
        assert!(all.contains(&PrivacyToggle::LogPaths));
        assert!(all.contains(&PrivacyToggle::LogUserIds));
        assert!(all.contains(&PrivacyToggle::LogHostnames));
        assert!(all.contains(&PrivacyToggle::LogRepoInfo));
        assert!(all.contains(&PrivacyToggle::LogMetadata));
        assert!(all.contains(&PrivacyToggle::LogSessionContext));
    }
}
