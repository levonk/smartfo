//! Structured error types for smartfo
//!
//! This module provides idempotent error handling where operations that
//! would fail due to existing state succeed silently instead.

use std::fmt;
use std::io;
use serde::Serialize;
use serde::Deserialize;

/// Smartfo-specific error types
#[derive(Debug)]
pub enum SmartfoError {
    /// Idempotent operation - state already exists in desired form
    AlreadyExists {
        /// What already exists
        what: String,
        /// Where it exists
        location: String,
    },
    
    /// Filesystem I/O error
    Io(io::Error),
    
    /// Configuration error
    Config(String),
    
    /// VCS operation error
    Vcs(String),
    
    /// Invalid arguments
    InvalidArgs(String),
    
    /// Operation not permitted
    PermissionDenied(String),
    
    /// Generic error with context
    Other(String),
}

impl SmartfoError {
    /// Check if this error represents an idempotent success (state already correct)
    pub fn is_idempotent(&self) -> bool {
        matches!(self, SmartfoError::AlreadyExists { .. })
    }
    
    /// Convert to a user-friendly message
    pub fn to_user_message(&self) -> String {
        match self {
            SmartfoError::AlreadyExists { what, location } => {
                format!("{} already exists at {}", what, location)
            }
            SmartfoError::Io(err) => {
                format!("I/O error: {}", err)
            }
            SmartfoError::Config(msg) => {
                format!("Configuration error: {}", msg)
            }
            SmartfoError::Vcs(msg) => {
                format!("VCS error: {}", msg)
            }
            SmartfoError::InvalidArgs(msg) => {
                format!("Invalid arguments: {}", msg)
            }
            SmartfoError::PermissionDenied(msg) => {
                format!("Permission denied: {}", msg)
            }
            SmartfoError::Other(msg) => {
                msg.clone()
            }
        }
    }
    
    /// Get actionable suggestion for the error
    pub fn suggestion(&self) -> Option<String> {
        match self {
            SmartfoError::AlreadyExists { .. } => {
                // No action needed - this is an idempotent success
                None
            }
            SmartfoError::Io(err) => {
                if err.kind() == io::ErrorKind::PermissionDenied {
                    Some("Check file permissions and try again".to_string())
                } else if err.kind() == io::ErrorKind::NotFound {
                    Some("Verify the path exists and is accessible".to_string())
                } else {
                    Some("Check the file system and try again".to_string())
                }
            }
            SmartfoError::Config(msg) => {
                Some(format!("Check your configuration file: {}", msg))
            }
            SmartfoError::Vcs(msg) => {
                Some(format!("VCS operation failed: {}", msg))
            }
            SmartfoError::InvalidArgs(msg) => {
                Some(format!("Review your arguments: {}", msg))
            }
            SmartfoError::PermissionDenied(msg) => {
                Some(format!("Check permissions: {}", msg))
            }
            SmartfoError::Other(_) => {
                None
            }
        }
    }
}

impl fmt::Display for SmartfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_user_message())
    }
}

impl std::error::Error for SmartfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SmartfoError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for SmartfoError {
    fn from(err: io::Error) -> Self {
        SmartfoError::Io(err)
    }
}

/// Result type alias for Smartfo operations
pub type Result<T> = std::result::Result<T, SmartfoError>;

/// Structured error output format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredError {
    /// Error type/category
    pub error_type: String,
    
    /// Human-readable error message
    pub message: String,
    
    /// Actionable suggestion (if available)
    pub suggestion: Option<String>,
    
    /// Whether this is an idempotent success (state already correct)
    pub idempotent: bool,
    
    /// Additional context
    pub context: Option<serde_json::Value>,
}

impl SmartfoError {
    /// Convert to structured error format
    pub fn to_structured(&self) -> StructuredError {
        StructuredError {
            error_type: self.error_type().to_string(),
            message: self.to_user_message(),
            suggestion: self.suggestion(),
            idempotent: self.is_idempotent(),
            context: self.context(),
        }
    }
    
    /// Get error type string
    fn error_type(&self) -> &str {
        match self {
            SmartfoError::AlreadyExists { .. } => "already_exists",
            SmartfoError::Io(_) => "io_error",
            SmartfoError::Config(_) => "config_error",
            SmartfoError::Vcs(_) => "vcs_error",
            SmartfoError::InvalidArgs(_) => "invalid_args",
            SmartfoError::PermissionDenied(_) => "permission_denied",
            SmartfoError::Other(_) => "other",
        }
    }
    
    /// Get additional context (if any)
    fn context(&self) -> Option<serde_json::Value> {
        match self {
            SmartfoError::AlreadyExists { what, location } => {
                Some(serde_json::json!({
                    "what": what,
                    "location": location
                }))
            }
            _ => None,
        }
    }
    
    /// Write structured error to stdout
    pub fn write_structured(&self) -> Result<()> {
        let structured = self.to_structured();
        let output = serde_json::to_string_pretty(&structured)
            .map_err(|e| SmartfoError::Other(format!("Failed to serialize error: {}", e)))?;
        println!("{}", output);
        Ok(())
    }
}

/// Helper for idempotent directory creation
pub fn create_dir_if_not_exists(path: &std::path::Path) -> Result<()> {
    if path.exists() {
        if path.is_dir() {
            // Already exists and is a directory - idempotent success
            return Ok(());
        } else {
            // Exists but is not a directory - error
            return Err(SmartfoError::AlreadyExists {
                what: "file".to_string(),
                location: path.display().to_string(),
            });
        }
    }
    
    std::fs::create_dir_all(path)?;
    Ok(())
}

/// Helper for idempotent symlink creation
pub fn create_symlink_if_not_exists(
    source: &std::path::Path,
    target: &std::path::Path,
) -> Result<()> {
    if target.exists() {
        // Check if it's already the correct symlink
        if let Ok(existing_target) = std::fs::read_link(target) {
            if existing_target == source {
                // Already points to correct target - idempotent success
                return Ok(());
            }
        }
        
        // Exists but points elsewhere or is not a symlink
        return Err(SmartfoError::AlreadyExists {
            what: "symlink".to_string(),
            location: target.display().to_string(),
        });
    }
    
    std::os::unix::fs::symlink(source, target)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_idempotent_dir_creation() {
        let temp_dir = TempDir::new().unwrap();
        let dir = temp_dir.path().join("test");
        
        // First creation
        create_dir_if_not_exists(&dir).unwrap();
        assert!(dir.exists());
        
        // Second creation should succeed (idempotent)
        create_dir_if_not_exists(&dir).unwrap();
        assert!(dir.exists());
    }

    #[test]
    fn test_idempotent_dir_creation_file_conflict() {
        let temp_dir = TempDir::new().unwrap();
        let file = temp_dir.path().join("test");
        
        // Create a file instead of directory
        fs::write(&file, "test").unwrap();
        
        // Should error when trying to create directory over file
        let result = create_dir_if_not_exists(&file);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_idempotent());
    }

    #[test]
    fn test_idempotent_symlink_creation() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("source");
        let target = temp_dir.path().join("target");
        
        fs::write(&source, "test").unwrap();
        
        // First creation
        create_symlink_if_not_exists(&source, &target).unwrap();
        assert!(target.exists());
        
        // Second creation should succeed (idempotent)
        create_symlink_if_not_exists(&source, &target).unwrap();
        assert!(target.exists());
    }

    #[test]
    fn test_idempotent_symlink_wrong_target() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("source");
        let other_source = temp_dir.path().join("other");
        let target = temp_dir.path().join("target");
        
        fs::write(&source, "test").unwrap();
        fs::write(&other_source, "other").unwrap();
        
        // Create symlink to wrong target
        std::os::unix::fs::symlink(&other_source, &target).unwrap();
        
        // Should error when trying to create symlink to different target
        let result = create_symlink_if_not_exists(&source, &target);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_idempotent());
    }

    #[test]
    fn test_error_suggestions() {
        let io_err = SmartfoError::Io(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "access denied"
        ));
        assert!(io_err.suggestion().is_some());
        
        let already_exists = SmartfoError::AlreadyExists {
            what: "file".to_string(),
            location: "/path/to/file".to_string(),
        };
        assert!(already_exists.suggestion().is_none()); // No suggestion needed for idempotent success
    }
    
    #[test]
    fn test_structured_error_format() {
        let error = SmartfoError::AlreadyExists {
            what: "directory".to_string(),
            location: "/tmp/test".to_string(),
        };
        
        let structured = error.to_structured();
        assert_eq!(structured.error_type, "already_exists");
        assert!(structured.message.contains("directory"));
        assert!(structured.message.contains("/tmp/test"));
        assert!(structured.idempotent);
        assert!(structured.suggestion.is_none());
        assert!(structured.context.is_some());
    }
    
    #[test]
    fn test_structured_error_io() {
        let error = SmartfoError::Io(io::Error::new(
            io::ErrorKind::NotFound,
            "file not found"
        ));
        
        let structured = error.to_structured();
        assert_eq!(structured.error_type, "io_error");
        assert!(structured.message.contains("I/O error"));
        assert!(!structured.idempotent);
        assert!(structured.suggestion.is_some());
        assert!(structured.context.is_none());
    }
    
    #[test]
    fn test_structured_error_config() {
        let error = SmartfoError::Config("Invalid TOML".to_string());
        
        let structured = error.to_structured();
        assert_eq!(structured.error_type, "config_error");
        assert!(structured.message.contains("Configuration error"));
        assert!(!structured.idempotent);
        assert!(structured.suggestion.is_some());
    }
    
    #[test]
    fn test_error_type_strings() {
        let already_exists = SmartfoError::AlreadyExists {
            what: "test".to_string(),
            location: "/test".to_string(),
        };
        assert_eq!(already_exists.error_type(), "already_exists");
        
        let io_err = SmartfoError::Io(io::Error::new(io::ErrorKind::Other, "test"));
        assert_eq!(io_err.error_type(), "io_error");
        
        let config_err = SmartfoError::Config("test".to_string());
        assert_eq!(config_err.error_type(), "config_error");
        
        let vcs_err = SmartfoError::Vcs("test".to_string());
        assert_eq!(vcs_err.error_type(), "vcs_error");
        
        let invalid_args = SmartfoError::InvalidArgs("test".to_string());
        assert_eq!(invalid_args.error_type(), "invalid_args");
        
        let perm_denied = SmartfoError::PermissionDenied("test".to_string());
        assert_eq!(perm_denied.error_type(), "permission_denied");
        
        let other = SmartfoError::Other("test".to_string());
        assert_eq!(other.error_type(), "other");
    }
    
    #[test]
    fn test_error_context() {
        let error = SmartfoError::AlreadyExists {
            what: "symlink".to_string(),
            location: "/usr/local/bin/mv".to_string(),
        };
        
        let context = error.context();
        assert!(context.is_some());
        
        let context_obj = context.unwrap();
        assert_eq!(context_obj["what"], "symlink");
        assert_eq!(context_obj["location"], "/usr/local/bin/mv");
    }
    
    #[test]
    fn test_error_context_other_errors() {
        let io_err = SmartfoError::Io(io::Error::new(io::ErrorKind::Other, "test"));
        assert!(io_err.context().is_none());
        
        let config_err = SmartfoError::Config("test".to_string());
        assert!(config_err.context().is_none());
    }
}
