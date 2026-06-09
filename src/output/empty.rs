// Empty state formatting for CLI AXI
// Provides definitive empty states with context and consistent formatting

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Empty state context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyContext {
    /// Query scope (e.g., "all operations", "pending operations")
    pub scope: String,
    /// Filter criteria applied (e.g., "status=completed", "type=move")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, String>>,
    /// Total count of items in the broader scope (for context)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_scope_count: Option<usize>,
}

impl EmptyContext {
    /// Create a new empty context
    pub fn new(scope: impl Into<String>) -> Self {
        Self {
            scope: scope.into(),
            filters: None,
            total_scope_count: None,
        }
    }

    /// Add a filter criterion
    pub fn with_filter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut filters = self.filters.unwrap_or_default();
        filters.insert(key.into(), value.into());
        self.filters = Some(filters);
        self
    }

    /// Set the total scope count
    pub fn with_total_scope_count(mut self, count: usize) -> Self {
        self.total_scope_count = Some(count);
        self
    }
}

/// Empty state response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyState {
    /// Explicit count of results (always 0)
    pub count: usize,
    /// Human-readable message
    pub message: String,
    /// Context about the empty query
    pub context: EmptyContext,
}

impl EmptyState {
    /// Create a new empty state
    pub fn new(context: EmptyContext) -> Self {
        let message = Self::generate_message(&context);
        Self {
            count: 0,
            message,
            context,
        }
    }

    /// Generate a human-readable message based on context
    fn generate_message(context: &EmptyContext) -> String {
        let base_message = format!("0 results found in {}", context.scope);
        
        if let Some(ref filters) = context.filters {
            let filter_str: Vec<String> = filters
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            if !filter_str.is_empty() {
                return format!("{} with filters: {}", base_message, filter_str.join(", "));
            }
        }
        
        if let Some(total) = context.total_scope_count {
            return format!("{} (total scope: {} items)", base_message, total);
        }
        
        base_message
    }

    /// Create an empty state for list command
    pub fn for_list(all: bool, limit: Option<usize>) -> Self {
        let scope = if all {
            "all operations".to_string()
        } else {
            format!("operations (limit: {})", limit.unwrap_or_default())
        };
        
        let context = EmptyContext::new(scope);
        Self::new(context)
    }

    /// Create an empty state for status command
    pub fn for_status(detailed: bool) -> Self {
        let scope = if detailed {
            "detailed status".to_string()
        } else {
            "status summary".to_string()
        };
        
        let context = EmptyContext::new(scope);
        Self::new(context)
    }

    /// Create an empty state with custom filters
    pub fn with_filters(scope: impl Into<String>, filters: HashMap<String, String>) -> Self {
        let context = EmptyContext::new(scope).with_filters_map(filters);
        Self::new(context)
    }
}

impl EmptyContext {
    /// Add multiple filters at once
    pub fn with_filters_map(mut self, filters: HashMap<String, String>) -> Self {
        if !filters.is_empty() {
            self.filters = Some(filters);
        }
        self
    }
}

/// Check if a result set is empty and return appropriate empty state
pub fn check_empty<T>(items: &[T], context: EmptyContext) -> Option<EmptyState> {
    if items.is_empty() {
        Some(EmptyState::new(context))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_context_creation() {
        let context = EmptyContext::new("test scope");
        assert_eq!(context.scope, "test scope");
        assert!(context.filters.is_none());
        assert!(context.total_scope_count.is_none());
    }

    #[test]
    fn test_empty_context_with_filter() {
        let context = EmptyContext::new("test scope")
            .with_filter("status", "completed")
            .with_filter("type", "move");
        
        assert_eq!(context.scope, "test scope");
        let filters = context.filters.unwrap();
        assert_eq!(filters.get("status"), Some(&"completed".to_string()));
        assert_eq!(filters.get("type"), Some(&"move".to_string()));
    }

    #[test]
    fn test_empty_context_with_total_count() {
        let context = EmptyContext::new("test scope")
            .with_total_scope_count(100);
        
        assert_eq!(context.total_scope_count, Some(100));
    }

    #[test]
    fn test_empty_state_message_basic() {
        let context = EmptyContext::new("all operations");
        let state = EmptyState::new(context);
        
        assert_eq!(state.count, 0);
        assert_eq!(state.message, "0 results found in all operations");
    }

    #[test]
    fn test_empty_state_message_with_filters() {
        let mut filters = HashMap::new();
        filters.insert("status".to_string(), "completed".to_string());
        filters.insert("type".to_string(), "move".to_string());
        
        let context = EmptyContext::new("operations").with_filters_map(filters);
        let state = EmptyState::new(context);
        
        assert!(state.message.contains("0 results found in operations"));
        assert!(state.message.contains("status=completed"));
        assert!(state.message.contains("type=move"));
    }

    #[test]
    fn test_empty_state_message_with_total_count() {
        let context = EmptyContext::new("operations")
            .with_total_scope_count(100);
        let state = EmptyState::new(context);
        
        assert!(state.message.contains("0 results found in operations"));
        assert!(state.message.contains("total scope: 100 items"));
    }

    #[test]
    fn test_empty_state_for_list() {
        let state = EmptyState::for_list(true, None);
        assert_eq!(state.count, 0);
        assert!(state.message.contains("all operations"));
        
        let state = EmptyState::for_list(false, Some(10));
        assert!(state.message.contains("limit: 10"));
    }

    #[test]
    fn test_empty_state_for_status() {
        let state = EmptyState::for_status(true);
        assert!(state.message.contains("detailed status"));
        
        let state = EmptyState::for_status(false);
        assert!(state.message.contains("status summary"));
    }

    #[test]
    fn test_check_empty_with_items() {
        let items = vec![1, 2, 3];
        let context = EmptyContext::new("test");
        let result = check_empty(&items, context);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_check_empty_without_items() {
        let items: Vec<i32> = vec![];
        let context = EmptyContext::new("test");
        let result = check_empty(&items, context);
        
        assert!(result.is_some());
        let state = result.unwrap();
        assert_eq!(state.count, 0);
    }

    #[test]
    fn test_empty_state_serialization() {
        let context = EmptyContext::new("test scope")
            .with_filter("status", "completed");
        let state = EmptyState::new(context);
        
        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains("\"count\":0"));
        assert!(json.contains("\"message\""));
        assert!(json.contains("\"context\""));
    }
}
