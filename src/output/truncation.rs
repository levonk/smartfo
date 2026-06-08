// Content truncation for large text fields
// Reduces token consumption while preserving preview and metadata

use serde::Serialize;

/// Default truncation limit in characters
pub const DEFAULT_TRUNCATION_LIMIT: usize = 1000;

/// Truncation metadata
#[derive(Debug, Clone, Serialize)]
pub struct TruncationMetadata {
    /// Total character count of original content
    pub total_chars: usize,
    /// Whether content was truncated
    pub truncated: bool,
    /// Truncation limit used (in characters)
    pub limit: usize,
}

impl TruncationMetadata {
    /// Create truncation metadata
    pub fn new(total_chars: usize, limit: usize) -> Self {
        Self {
            total_chars,
            truncated: total_chars > limit,
            limit,
        }
    }
    
    /// Create metadata for non-truncated content
    pub fn not_truncated(total_chars: usize) -> Self {
        Self {
            total_chars,
            truncated: false,
            limit: total_chars,
        }
    }
}

/// Truncated content with metadata
#[derive(Debug, Clone, Serialize)]
pub struct TruncatedContent {
    /// Truncated content
    pub content: String,
    /// Truncation metadata
    pub metadata: TruncationMetadata,
    /// Help suggestion for viewing full content
    pub help_suggestion: Option<String>,
}

impl TruncatedContent {
    /// Create truncated content
    pub fn new(content: String, metadata: TruncationMetadata) -> Self {
        let help_suggestion = if metadata.truncated {
            Some("Use --full flag to see complete content".to_string())
        } else {
            None
        };
        Self { content, metadata, help_suggestion }
    }
    
    /// Get the full content (no truncation)
    pub fn full(content: String) -> Self {
        let total_chars = content.chars().count();
        Self {
            content,
            metadata: TruncationMetadata::not_truncated(total_chars),
            help_suggestion: None,
        }
    }
}

/// Truncate content to specified limit
pub fn truncate(content: &str, limit: usize) -> TruncatedContent {
    let total_chars = content.chars().count();
    
    if total_chars <= limit {
        return TruncatedContent::full(content.to_string());
    }
    
    // Use character-based truncation to handle Unicode properly
    let truncated: String = content.chars().take(limit).collect();
    let metadata = TruncationMetadata::new(total_chars, limit);
    
    TruncatedContent::new(truncated, metadata)
}

/// Truncate content with ellipsis indicator
pub fn truncate_with_ellipsis(content: &str, limit: usize) -> TruncatedContent {
    let total_chars = content.chars().count();
    
    if total_chars <= limit {
        return TruncatedContent::full(content.to_string());
    }
    
    // Reserve space for ellipsis
    let ellipsis = "...";
    let content_limit = limit.saturating_sub(ellipsis.len());
    
    if content_limit == 0 {
        // Just show ellipsis if limit is too small
        let metadata = TruncationMetadata::new(total_chars, limit);
        return TruncatedContent::new(ellipsis.to_string(), metadata);
    }
    
    // Use character-based truncation to handle Unicode properly
    let truncated: String = content.chars().take(content_limit).collect();
    let truncated_with_ellipsis = format!("{}{}", truncated, ellipsis);
    let metadata = TruncationMetadata::new(total_chars, limit);
    
    TruncatedContent::new(truncated_with_ellipsis, metadata)
}

/// Check if content should be truncated
pub fn should_truncate(content: &str, limit: usize) -> bool {
    content.chars().count() > limit
}

/// Get truncation limit from config or use default
pub fn get_truncation_limit(config_limit: Option<usize>) -> usize {
    config_limit.unwrap_or(DEFAULT_TRUNCATION_LIMIT)
}

/// Truncate a field value if it's a string and exceeds the limit
pub fn truncate_field_value(value: &str, limit: usize, enabled: bool) -> TruncatedContent {
    if !enabled {
        return TruncatedContent::full(value.to_string());
    }
    truncate_with_ellipsis(value, limit)
}

/// Truncate an optional field value
pub fn truncate_optional_field(value: &Option<String>, limit: usize, enabled: bool) -> Option<TruncatedContent> {
    value.as_ref().map(|v| truncate_field_value(v, limit, enabled))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_truncate_short_content() {
        let content = "short";
        let result = truncate(content, 1000);
        assert!(!result.metadata.truncated);
        assert_eq!(result.content, content);
        assert_eq!(result.metadata.total_chars, 5);
    }
    
    #[test]
    fn test_truncate_long_content() {
        let content = "a".repeat(2000);
        let result = truncate(&content, 1000);
        assert!(result.metadata.truncated);
        assert_eq!(result.content.len(), 1000);
        assert_eq!(result.metadata.total_chars, 2000);
        assert_eq!(result.metadata.limit, 1000);
        assert!(result.help_suggestion.is_some());
    }
    
    #[test]
    fn test_truncate_with_ellipsis() {
        let content = "a".repeat(2000);
        let result = truncate_with_ellipsis(&content, 1000);
        assert!(result.metadata.truncated);
        assert_eq!(result.content.len(), 1000);
        assert!(result.content.ends_with("..."));
        assert!(result.help_suggestion.is_some());
    }
    
    #[test]
    fn test_truncate_with_ellipsis_short() {
        let content = "short";
        let result = truncate_with_ellipsis(content, 1000);
        assert!(!result.metadata.truncated);
        assert_eq!(result.content, content);
        assert!(result.help_suggestion.is_none());
    }
    
    #[test]
    fn test_truncate_with_ellipsis_small_limit() {
        let content = "a".repeat(100);
        let result = truncate_with_ellipsis(&content, 2);
        assert!(result.metadata.truncated);
        assert_eq!(result.content, "...");
    }
    
    #[test]
    fn test_should_truncate() {
        assert!(should_truncate(&"a".repeat(2000), 1000));
        assert!(!should_truncate(&"a".repeat(500), 1000));
    }
    
    #[test]
    fn test_get_truncation_limit() {
        assert_eq!(get_truncation_limit(Some(500)), 500);
        assert_eq!(get_truncation_limit(None), DEFAULT_TRUNCATION_LIMIT);
    }
    
    #[test]
    fn test_truncation_metadata() {
        let metadata = TruncationMetadata::new(2000, 1000);
        assert!(metadata.truncated);
        assert_eq!(metadata.total_chars, 2000);
        assert_eq!(metadata.limit, 1000);
    }
    
    #[test]
    fn test_truncation_metadata_not_truncated() {
        let metadata = TruncationMetadata::not_truncated(500);
        assert!(!metadata.truncated);
        assert_eq!(metadata.total_chars, 500);
        assert_eq!(metadata.limit, 500);
    }
    
    #[test]
    fn test_truncate_field_value_enabled() {
        let content = "a".repeat(2000);
        let result = truncate_field_value(&content, 1000, true);
        assert!(result.metadata.truncated);
        assert_eq!(result.content.len(), 1000);
    }
    
    #[test]
    fn test_truncate_field_value_disabled() {
        let content = "a".repeat(2000);
        let result = truncate_field_value(&content, 1000, false);
        assert!(!result.metadata.truncated);
        assert_eq!(result.content.len(), 2000);
    }
    
    #[test]
    fn test_truncate_optional_field_some() {
        let content = "a".repeat(2000);
        let result = truncate_optional_field(&Some(content.to_string()), 1000, true);
        assert!(result.is_some());
        assert!(result.unwrap().metadata.truncated);
    }
    
    #[test]
    fn test_truncate_optional_field_none() {
        let result = truncate_optional_field(&None, 1000, true);
        assert!(result.is_none());
    }
    
    #[test]
    fn test_truncate_exact_limit() {
        let content = "a".repeat(1000);
        let result = truncate(&content, 1000);
        assert!(!result.metadata.truncated);
        assert_eq!(result.content.len(), 1000);
        assert_eq!(result.metadata.total_chars, 1000);
    }
    
    #[test]
    fn test_truncate_one_over_limit() {
        let content = "a".repeat(1001);
        let result = truncate(&content, 1000);
        assert!(result.metadata.truncated);
        assert_eq!(result.content.len(), 1000);
        assert_eq!(result.metadata.total_chars, 1001);
    }
    
    #[test]
    fn test_truncate_very_small_limit() {
        let content = "a".repeat(100);
        let result = truncate(&content, 5);
        assert!(result.metadata.truncated);
        assert_eq!(result.content.len(), 5);
        assert_eq!(result.metadata.total_chars, 100);
    }
    
    #[test]
    fn test_truncate_empty_string() {
        let content = "";
        let result = truncate(content, 1000);
        assert!(!result.metadata.truncated);
        assert_eq!(result.content.len(), 0);
        assert_eq!(result.metadata.total_chars, 0);
    }
    
    #[test]
    fn test_truncate_unicode_content() {
        let content = "🎉".repeat(500); // 500 emoji, each is 4 bytes
        let result = truncate(&content, 500);
        // Character-based truncation, not byte-based
        assert!(!result.metadata.truncated);
        assert_eq!(result.content.chars().count(), 500);
        assert_eq!(result.metadata.total_chars, 500);
        
        let result2 = truncate(&content, 400);
        assert!(result2.metadata.truncated);
        assert_eq!(result2.content.chars().count(), 400);
        assert_eq!(result2.metadata.total_chars, 500);
    }
}
